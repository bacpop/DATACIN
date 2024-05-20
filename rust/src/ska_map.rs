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
    /// Positions of mapped bases as (chrom, pos)
    mapped_bases: Vec<Variant>,
}

impl SkaMap {
    pub fn new<F: Read>(
        reference: &RefSka,
        file1: &mut F,
        file2: Option<&mut F>,
        file_type: &str,
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
        );
        if let Some(second_file) = file2 {
            query_ska.add_file_kmers(second_file, file_type);
        }
        let mut mapped_bases = Vec::new();
        for ref_kmer in reference.kmer_iter() {
            if let Some(kmer_match) = query_ska.kmers().get(&ref_kmer.kmer) {
                mapped_bases.push(Variant {
                    chrom: ref_kmer.chrom,
                    pos: ref_kmer.pos,
                    base: *kmer_match,
                })
            }
        }
        Self { mapped_bases }
    }

    pub fn mapped_bases(&self) -> &Vec<Variant> {
        &self.mapped_bases
    }
}