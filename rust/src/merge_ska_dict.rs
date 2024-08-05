use core::mem::swap;
use core::panic;
use std::mem;

use hashbrown::HashMap;
use indicatif::ProgressIterator;
use rayon::vec;
use crate::QualOpts;
use crate::QualFilter;

use crate::ska_dict::SkaDict;

use wasm_bindgen_file_reader::WebSysFile;

// Tuple for name and fasta or paired fastq input
pub type InputFastx = (String, String, Option<String>);

// Merged dictionary with names, and middle bases in [`Vec<u8>`] in the same order.
#[derive(Debug)]
pub struct MergeSkaDict {
    // K-mer size
    k: usize,
    // Whether reverse complement split k-mers were used
    rc: bool,
    // Total number of samples supported (some may be empty)
    n_samples: usize,
    // Sample names (some may be empty)
    names: Vec<String>,
    // Dictionary of split k-mers, and middle base vectors
    split_kmers: HashMap<u128, Vec<u8>>,
}

impl MergeSkaDict {
    // Create an empty merged dictionary, to be used with [`MergeSkaDict::merge()`]
    // or [`MergeSkaDict::append()`].
    pub fn new(k: usize, n_samples: usize, rc: bool) -> Self {
        Self {
            k,
            rc,
            n_samples,
            names: vec!["".to_string(); n_samples],
            split_kmers: HashMap::default(),
        }
    }

    // Directly add name and merged dictionary content
    //
    // Used when creating this struct from a [`crate::merge_ska_array::MergeSkaArray`]
    pub fn build_from_array<'a>(
        &'a mut self,
        names: &'a mut Vec<String>,
        split_kmers: &mut HashMap<u128, Vec<u8>>,
    ) {
        swap(names, &mut self.names);
        swap(split_kmers, &mut self.split_kmers);
    }

    // Add a single [`crate::ska_dict::SkaDict`](`SkaDict`) to the merged dictionary
    //
    // NB: this is not a real append, and to work the input must have a unique
    // `sample_idx < n_samples` set.
    //
    // # Panics
    //
    // If k-mer length or reverse complement do not match
    pub fn append(&mut self, other: &SkaDict) {
        if other.kmer_len() != self.k {
            panic!(
                "K-mer lengths do not match: {} {}",
                other.kmer_len(),
                self.k
            );
        }
        if other.rc() != self.rc {
            panic!("Strand use inconsistent");
        }
        self.names[other.idx()] = other.name().clone();
        if self.ksize() == 0 {
            for (kmer, base) in other.kmers() {
                let mut base_vec: Vec<u8> = vec![0; self.n_samples];
                base_vec[other.idx()] = *base;
                self.split_kmers.insert(*kmer, base_vec);
            }
        } else {
            for (kmer, base) in other.kmers() {
                self.split_kmers
                    .entry(*kmer)
                    .and_modify(|b| {
                        b[other.idx()] = *base;
                    })
                    .or_insert_with(|| {
                        let mut new_base_vec: Vec<u8> = vec![0; self.n_samples];
                        new_base_vec[other.idx()] = *base;
                        new_base_vec
                    });
            }
        }
    }

    // Combine with another [`MergeSkaDict`] with non-overlapping samples
    //
    // Used when building, when individual dicts have been joined using append
    // (CLI `ska build`)
    //
    // # Panics
    //
    // If k-mer length or reverse complement do not match
    pub fn merge<'a>(&'a mut self, other: &'a mut MergeSkaDict) {
        if other.k != self.k {
            panic!("K-mer lengths do not match: {} {}", other.k, self.k);
        }
        if other.rc() != self.rc {
            panic!("Strand use inconsistent");
        }
        if other.ksize() > 0 {
            if self.ksize() == 0 {
                swap(&mut other.names, &mut self.names);
                swap(&mut other.split_kmers, &mut self.split_kmers);
            } else {
                for name_it in other.names.iter_mut().zip(self.names.iter_mut()) {
                    let (other_name, self_name) = name_it;
                    if self_name.is_empty() {
                        swap(self_name, other_name);
                    }
                }

                for (kmer, other_vec) in &mut other.split_kmers {
                    self.split_kmers
                        .entry(*kmer)
                        .and_modify(|self_vec| {
                            // Vectorises to VORPS (I've checked!)
                            for base_it in other_vec.iter().zip(self_vec.iter_mut()) {
                                *base_it.1 |= *base_it.0;
                            }
                        })
                        .or_insert_with(|| mem::take(other_vec));
                }
            }
        }
    }

    // K-mer length of split-kmers
    pub fn kmer_len(&self) -> usize {
        self.k
    }

    // Whether reverse complement has been used
    pub fn rc(&self) -> bool {
        self.rc
    }

    // Sample names
    pub fn names(&self) -> &Vec<String> {
        &self.names
    }

    // Split k-mer dictionary
    pub fn kmer_dict(&self) -> &HashMap<u128, Vec<u8>> {
        &self.split_kmers
    }

    // Total number of split k-mers
    pub fn ksize(&self) -> usize {
        self.split_kmers.len()
    }

    // Number of samples
    pub fn nsamples(&self) -> usize {
        self.n_samples
    }
}


// Create a [`MergeSkaDict`] from input FASTA/FASTQ files
//
// First build [`SkaDict`] for each input.

pub fn build_and_merge(
    input_files: Vec<web_sys::File>,
    k: usize,
    rc: bool,
    file_types: Vec<String>,
    proportion_reads: Option<f64>,
) -> MergeSkaDict {

    let total_size = input_files.len();
    let mut merged_dict = MergeSkaDict::new(k, total_size, rc);

    for (idx, file) in input_files.iter().enumerate() {
        let mut wf1 = WebSysFile::new(file.clone());
        let ska_dict = SkaDict::new(
            k,
            0,
            &mut wf1,
            "",
            rc,
            &QualOpts {
                min_count: 1,
                min_qual: 0,
                qual_filter: QualFilter::NoFilter,
            },
            file_types[idx].as_str(),
            proportion_reads,
        );

        merged_dict.append(&ska_dict);
    }
    merged_dict
}
