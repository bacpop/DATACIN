use std::io::Read;

use crate::ska_dict::SkaDict;
use crate::QualFilter;
use crate::QualOpts;

#[derive(Debug, Clone, Default)]
pub struct SkaAlign {
    /// Positions of mapped bases as (chrom, pos)
    queries_ska: Vec<SkaDict>,
    pairwise_distances: Vec<Vec<usize>>,
    k: usize,
}

impl SkaAlign {
    pub fn new(k: usize) -> Self {
        Self { queries_ska: Vec::new(), pairwise_distances: Vec::new(), k }
    }

    pub fn add_file<F: Read>(
        &mut self,
        file: &mut F,
        file_type: &str,
        proportions_reads: Option<f64>,
    ) {
        self.queries_ska.push(SkaDict::new(                                                                                      
            self.k,
            0,
            file,
            "",
            false,
            &QualOpts {
                min_count: 1,
                min_qual: 0,
                qual_filter: QualFilter::NoFilter,
            },
            file_type,
            proportions_reads
        ));
    }

    pub fn align(&mut self) -> Vec<Vec<usize>> {
        let mut pairwise_distances = vec![vec![0; self.queries_ska.len()]; self.queries_ska.len()];
        for i in 0..self.queries_ska.len() {
            for j in (i+1)..self.queries_ska.len() {
                for ref_kmer in self.queries_ska[i].kmer_iter() {
                    if let Some(kmer_match) = self.queries_ska[j].kmers().get(&ref_kmer.0) {
                        if *kmer_match != *ref_kmer.1 {
                            pairwise_distances[i][j] += 1;
                        }
                    }
                }
            }
        }
        let cloned_pairwise_distances = pairwise_distances.clone();
        self.pairwise_distances = cloned_pairwise_distances;
        pairwise_distances
    }
}