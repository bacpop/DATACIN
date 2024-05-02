use std::io::Read;

use hashbrown::hash_set::Entry::*;
use hashbrown::HashSet;
use seq_io::fasta::Record;

use super::{log, QualFilter};
use crate::fastx::open_fasta;
use crate::ska_dict::split_kmer::SplitKmer;

/// A split k-mer in the reference sequence encapsulated with positional data.
#[derive(Debug, Clone)]
pub struct RefKmer {
    /// Encoded split k-mer
    pub kmer: u128,
    /// Middle base
    pub base: u8,
    /// Position in the chromosome
    pub pos: usize,
    /// Index of the chromosome
    pub chrom: usize,
    /// Whether on the reverse strand
    pub rc: bool,
}

/// A reference sequence, a list of its split k-mers, and optionally mapping information.
///
/// The default to after building with [`RefSka::new()`] will be a list of split-kmers,
/// as [`Vec<RefKmer>`], along with the reference sequence itself for lookup purposes.
///
/// After running [`RefSka::map()`] against a [`MergeSkaDict`] mapped middle
/// bases and positions will also be populated.
pub struct RefSka {
    /// k-mer size
    pub k: usize,
    /// reverse complement,
    pub rc: bool,
    /// Concatenated list of split k-mers
    split_kmer_pos: Vec<RefKmer>,
    /// Replace ambiguous bases with N
    ambig_mask: bool,
    /// Chromosome names
    chrom_names: Vec<String>,
    /// Sequence, indexed by chromosome, then position
    seq: Vec<Vec<u8>>,
    /// Where repeats should be masked with 'N'
    repeat_coors: Vec<usize>,
    /// Iterator index
    index: usize,
}

impl RefSka {
    pub fn new<F: Read>(
        k: usize,
        file: &mut F,
        rc: bool,
        ambig_mask: bool,
        repeat_mask: bool,
    ) -> Self {
        let mut reader = open_fasta(file);

        if !(5..=63).contains(&k) || k % 2 == 0 {
            panic!("Invalid k-mer length");
        }

        let mut split_kmer_pos = Vec::new();
        let mut seq = Vec::new();
        let mut chrom_names = Vec::new();
        let mut singles = HashSet::new();
        let mut repeats = HashSet::new();

        let mut chrom = 0;
        while let Some(record) = reader.next() {
            let seqrec = record.expect("Invalid FASTA record");
            chrom_names.push(seqrec.id().unwrap().to_owned());
            split_kmer_pos.reserve(seqrec.full_seq().len());

            let kmer_opt = SplitKmer::new(
                seqrec.seq().to_vec(),
                seqrec.seq().len(),
                None,
                k,
                rc,
                0,
                QualFilter::NoFilter,
                false,
            );
            if let Some(mut kmer_it) = kmer_opt {
                let (kmer, base, rc) = kmer_it.get_curr_kmer();
                let mut pos = kmer_it.get_middle_pos();
                split_kmer_pos.push(RefKmer {
                    kmer,
                    base,
                    pos,
                    chrom,
                    rc,
                });
                if repeat_mask {
                    Self::track_repeats(kmer, &mut singles, &mut repeats);
                }
                while let Some((kmer, base, rc)) = kmer_it.get_next_kmer() {
                    pos = kmer_it.get_middle_pos();
                    split_kmer_pos.push(RefKmer {
                        kmer,
                        base,
                        pos,
                        chrom,
                        rc,
                    });
                    if repeat_mask {
                        Self::track_repeats(kmer, &mut singles, &mut repeats);
                    }
                }
            }
            chrom += 1;
            seq.push(seqrec.seq().to_vec());
        }
        if split_kmer_pos.is_empty() {
            panic!("No valid sequence");
        }

        // Find the repeat ranges, and intersect them
        let mut repeat_coors = Vec::new();
        if repeat_mask {
            let half_split_len = (k - 1) / 2;
            let mut last_chrom = 0;
            let mut last_end = 0;
            let mut chrom_offset = 0;
            for sk in &split_kmer_pos {
                if sk.chrom > last_chrom {
                    chrom_offset += seq[last_chrom].len();
                    last_chrom = sk.chrom;
                }
                if repeats.contains(&sk.kmer) {
                    let start = sk.pos - half_split_len + chrom_offset;
                    let end = sk.pos + half_split_len + chrom_offset;
                    let range = if start > last_end || start == 0 {
                        std::ops::Range {
                            start,
                            end: end + 1,
                        }
                    } else {
                        std::ops::Range {
                            start: last_end + 1,
                            end: end + 1,
                        }
                    };
                    for pos in range {
                        repeat_coors.push(pos);
                    }
                    last_chrom = sk.chrom;
                    last_end = end;
                }
            }
            if cfg!(debug_assertions) {
                log(&format!(
                    "Masking {} unique split k-mer repeats spanning {} bases",
                    repeats.len(),
                    repeat_coors.len()
                ));
            }
        }

        Self {
            k,
            rc,
            seq,
            ambig_mask,
            chrom_names,
            split_kmer_pos,
            repeat_coors,
            index: 0,
        }
    }

    pub fn nk(&self) -> usize {
        self.split_kmer_pos.len()
    }

    pub fn kmer_iter(&self) -> impl Iterator<Item = &RefKmer> + '_ {
        self.split_kmer_pos.iter()
    }

    // Keeps track of split k-mers in the ref, any found before are moved
    // to the repeats set
    fn track_repeats(kmer: u128, singles: &mut HashSet<u128>, repeats: &mut HashSet<u128>) {
        if let Vacant(rep_kmer) = repeats.entry(kmer) {
            match singles.entry(kmer) {
                Vacant(single_entry) => single_entry.insert(),
                Occupied(_) => {
                    rep_kmer.insert();
                }
            }
        }
    }

    pub fn len(&self) -> usize {
        self.seq.iter().map(|x| x.len()).sum()
    }

    pub fn get_seq(&self) ->  &Vec<Vec<u8>> {
        &self.seq
    }
}
