//! Type for building a split k-mer dictionary from a fasta/fastq file.
//!
//! The dictionary has the concatenated left and right part of the split k-mer
//! (bit-encoded DNA bases) as keys, and ASCII IUPAC bases as values for the
//! middle base.
//!
//! Prefer to use [`crate::merge_ska_dict::build_and_merge`] over this
//! function directly.
//!
//! If you want to use this directly, build with [`SkaDict::new()`].
//!
//! These should then be converted or merged into to [`crate::merge_ska_dict::MergeSkaDict`], which
//! lets you do more useful things with them.
//!
//! Note that you should use an appropriate `sample_idx` when building if you
//! know how many you will be merging, which will let you use [`crate::merge_ska_dict::MergeSkaDict::merge()`].
//! Otherwise you will need to use the slower [`crate::merge_ska_dict::MergeSkaDict::extend()`]

use std::io::Read;
use std::cmp::Ordering;

use hashbrown::HashMap;

use crate::fastx::open_fasta;

pub mod split_kmer;
use super::QualOpts;
use crate::ska_dict::split_kmer::SplitKmer;

pub mod bit_encoding;
use crate::ska_dict::bit_encoding::{decode_base, IUPAC};

pub mod bloom_filter;
use crate::ska_dict::bloom_filter::KmerFilter;

pub mod nthash;

/// Holds the split-kmer dictionary, and basic information such as k-mer size.
#[derive(Debug, Clone, Default)]
pub struct SkaDict {
    /// K-mer size
    k: usize,
    /// Whether reverse-complement was counted
    rc: bool,
    /// Sample index, if being used in a merge
    sample_idx: usize,
    /// Sample name
    name: String,
    /// Split k-mer dictionary split-k:middle-base
    split_kmers: HashMap<u128, u8>,
    /// A bloom filter for counting from fastq files
    kmer_filter: KmerFilter,
    qual_opts: QualOpts,
    is_reads: bool,
}

impl SkaDict
{
    /// Adds a split-kmer and middle base to dictionary.
    fn add_to_dict(&mut self, kmer: u128, base: u8) {
        self.split_kmers
            .entry(kmer)
            .and_modify(|b| *b = IUPAC[base as usize * 256 + *b as usize])
            .or_insert(decode_base(base));
    }

    /// Adds a split k-mer which is a self-rc to the dict
    /// This requires amibguity of middle_base + rc(middle_base) to be added
    fn add_palindrome_to_dict(&mut self, kmer: u128, base: u8) {
        self.split_kmers
            .entry(kmer)
            .and_modify(|b| {
                *b = match b {
                    b'W' => {
                        if base == 0 || base == 2 {
                            b'W'
                        } else {
                            b'N'
                        }
                    }
                    b'S' => {
                        if base == 0 || base == 2 {
                            b'N'
                        } else {
                            b'S'
                        }
                    }
                    b'N' => b'N',
                    _ => panic!("Palindrome middle base not W/S: {}", *b as char),
                }
            })
            .or_insert(match base {
                0 | 2 => b'W', // A or T
                1 | 3 => b'S', // C or G
                _ => panic!("Base encoding error: {}", base as char),
            });
    }

    /// Iterates through all the k-mers from an input fastx file and adds them
    /// to the dictionary
    pub fn add_file_kmers<F: Read>(&mut self, file: &mut F) {
        // TODO: deal with fastq (is_reads)
        let mut reader = open_fasta(file);
        while let Some(record) = reader.next() {
            let seqrec = record.expect("Invalid FASTA/Q record");
            let kmer_opt = SplitKmer::new(
                seqrec.full_seq(),
                seqrec.full_seq().len(),
                None,
                self.k,
                self.rc,
                self.qual_opts.min_qual,
                self.qual_opts.qual_filter,
                self.is_reads,
            );
            if let Some(mut kmer_it) = kmer_opt {
                if !self.is_reads
                    || (kmer_it.middle_base_qual()
                        && Ordering::is_eq(self.kmer_filter.filter(&kmer_it)))
                {
                    let (kmer, base, _rc) = kmer_it.get_curr_kmer();
                    if kmer_it.self_palindrome() {
                        self.add_palindrome_to_dict(kmer, base);
                    } else {
                        self.add_to_dict(kmer, base);
                    }
                }
                while let Some((kmer, base, _rc)) = kmer_it.get_next_kmer() {
                    if !self.is_reads
                        || (kmer_it.middle_base_qual()
                            && Ordering::is_eq(self.kmer_filter.filter(&kmer_it)))
                    {
                        if kmer_it.self_palindrome() {
                            self.add_palindrome_to_dict(kmer, base);
                        } else {
                            self.add_to_dict(kmer, base);
                        }
                    }
                }
            }
        }
    }

    /// Build a split-kmer dictionary from input fastx file(s)
    ///
    /// Prefer to use [`crate::merge_ska_dict::build_and_merge()`] over this
    /// function directly.
    ///
    /// # Examples
    ///
    /// To build with a FASTA
    /// ```
    /// use ska::ska_dict::SkaDict;
    /// use ska::{QualOpts, QualFilter};
    ///
    /// let k = 31;
    /// let sample_idx = 0;
    /// let quality = QualOpts {min_count: 1, min_qual: 0, qual_filter: QualFilter::NoFilter};
    /// let ska_dict = SkaDict::<u64>::new(k, sample_idx, (&"tests/test_files_in/test_1.fa", None), "test_1", true, &quality);
    /// ```
    ///
    /// With FASTQ pair, only allowing k-mers with a count over 2, and where all
    /// bases have a PHRED score of 20 or more
    /// ```
    /// use ska::ska_dict::SkaDict;
    /// use ska::{QualOpts, QualFilter};
    ///
    /// let quality = QualOpts {min_count: 2, min_qual: 20, qual_filter: QualFilter::Middle};
    /// let k = 9;
    /// let sample_idx = 0;
    /// let ska_dict = SkaDict::<u64>::new(k, sample_idx,
    ///                             (&"tests/test_files_in/test_1_fwd.fastq.gz",
    ///                             Some(&"tests/test_files_in/test_2_fwd.fastq.gz".to_string())),
    ///                             "sample1", true, &quality);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - k-mer length is invalid (<5, >63 or even)
    /// - Input file cannot be read
    /// - Input file contains invalid fastx record
    /// - Input file contains no valid sequence to find at least on split k-mer
    #[allow(clippy::too_many_arguments)]
    pub fn new<F: Read>(
        k: usize,
        sample_idx: usize,
        input_file: &mut F,
        name: &str,
        rc: bool,
        qual: &QualOpts,
    ) -> Self {
        if !(5..=63).contains(&k) || k % 2 == 0 {
            panic!("Invalid k-mer length");
        }

        let mut sk_dict = Self {
            k,
            rc,
            sample_idx,
            name: name.to_string(),
            split_kmers: HashMap::default(),
            kmer_filter: KmerFilter::new(qual.min_count),
            qual_opts: *qual,
            is_reads: false,
        };

        // TODO Check if we're working with reads, and initalise the CM filter if so
        // and self.is_reads
        /*
        let mut reader_peek =
            parse_fastx_file(files.0).unwrap_or_else(|_| panic!("Invalid path/file: {}", files.0));
        let seq_peek = reader_peek
            .next()
            .expect("Invalid FASTA/Q record")
            .expect("Invalid FASTA/Q record");
        let mut is_reads = false;
        if seq_peek.format() == Format::Fastq {
            sk_dict.kmer_filter.init();
            is_reads = true;
        }
        */

        // Build the dict
        sk_dict.add_file_kmers(input_file);

        if sk_dict.ksize() == 0 {
            panic!("File has no valid sequence");
        }
        sk_dict
    }

    /// K-mer length used for split k-mers
    pub fn kmer_len(&self) -> usize {
        self.k
    }

    /// Whether reverse-complement was counted
    pub fn rc(&self) -> bool {
        self.rc
    }

    /// Total number of split k-mers in the dictionary
    pub fn ksize(&self) -> usize {
        self.split_kmers.len()
    }

    /// Sample index for merging
    pub fn idx(&self) -> usize {
        self.sample_idx
    }

    /// Split k-mer dictonary
    pub fn kmers(&self) -> &HashMap<u128, u8> {
        &self.split_kmers
    }

    /// Sample name
    pub fn name(&self) -> &String {
        &self.name
    }
}
