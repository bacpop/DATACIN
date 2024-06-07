use std::io::Read;

use crate::ska_dict::SkaDict;
use crate::ska_ref::RefSka;
use crate::QualFilter;
use crate::QualOpts;

#[derive(Debug, Clone, Default)]
pub struct Variant {
    pub chrom: usize,
    pub pos: usize,
    pub base: u8,
}

#[derive(Debug, Clone, Default)]
pub struct SkaMap {
    /// Positions of mapped bases as (chrom, pos, base)
    mapped_bases: Vec<Variant>,
}

impl SkaMap {
    pub fn new<F: Read>(
        reference: &RefSka,
        file1: &mut F,
        file2: Option<&mut F>,
        file_type: &str,
        proportions_reads: Option<f64>,
    ) -> Self {
        // TODO - two files. Should just be able to add an 'add more k-mers' method on the struct to accept second file if given
        let mut query_ska = SkaDict::new(
            reference.k,
            0,
            file1,
            "",
            reference.rc,
            &QualOpts {
                min_count: 1,
                min_qual: 0,
                qual_filter: QualFilter::NoFilter,
            },
            file_type,
            proportions_reads,
        );
        if let Some(second_file) = file2 {
            query_ska.add_file_kmers(second_file, file_type, proportions_reads);
        }
        let mut mapped_bases = Vec::new();
        for ref_kmer in reference.kmer_iter() {
            if let Some(kmer_match) = query_ska.kmers().get(&ref_kmer.kmer) {
                if ref_kmer.rc{
                    mapped_bases.push(Variant {
                        chrom: ref_kmer.chrom,
                        pos: ref_kmer.pos,
                        base: RC_IUPAC[*kmer_match as usize],
                    })
                } else {
                    mapped_bases.push(Variant {
                        chrom: ref_kmer.chrom,
                        pos: ref_kmer.pos,
                        base: *kmer_match,
                    })
                }
            }
        }
        Self { mapped_bases }
    }

    pub fn mapped_bases(&self) -> &Vec<Variant> {
        &self.mapped_bases
    }
}

/// Lookup table which gives reverse complement of a single IUPAC code (ASCII/`u8`).
pub const RC_IUPAC: [u8; 256] = [
    b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-',
    b'-', // 0-15
    b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-',
    b'-', // 16-31
    b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-',
    b'-', // 32-47
    b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-',
    b'-', // 48-63
    b'-', b'T', b'V', b'G', b'H', b'-', b'-', b'C', b'D', b'-', b'-', b'M', b'-', b'K', b'N',
    b'-', // 64-79
    b'-', b'-', b'Y', b'S', b'A', b'-', b'B', b'W', b'-', b'R', b'-', b'-', b'-', b'-', b'-',
    b'-', // 80-95
    b'-', b'T', b'V', b'G', b'H', b'-', b'-', b'C', b'D', b'-', b'-', b'M', b'-', b'K', b'N',
    b'-', // 96-111
    b'-', b'-', b'Y', b'S', b'A', b'-', b'B', b'W', b'-', b'R', b'-', b'-', b'-', b'-', b'-',
    b'-', // 112-127
    b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-',
    b'-', // 128-143
    b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-',
    b'-', // 144-159
    b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-',
    b'-', // 160-175
    b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-',
    b'-', // 176-191
    b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-',
    b'-', // 192-207
    b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-',
    b'-', // 208-223
    b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-',
    b'-', // 224-239
    b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'-',
    b'-', // 240-255
];