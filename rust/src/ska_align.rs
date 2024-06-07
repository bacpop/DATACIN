use std::io::Read;

use crate::ska_dict::SkaDict;
use crate::QualFilter;
use crate::QualOpts;

use speedytree::DistanceMatrix;
use speedytree::{NeighborJoiningSolver, Canonical};

#[derive(Debug, Clone, Default)]
pub struct SkaAlign {
    /// Positions of mapped bases as (chrom, pos)
    queries_ska: Vec<SkaDict>,
    k: usize,
}

impl SkaAlign {
    pub fn new(k: usize) -> Self {
        Self { queries_ska: Vec::new(), k }
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

    pub fn align(&mut self, file_names: &Vec<String>) -> String {
        let mut pairwise_distances = vec![vec![0; self.queries_ska.len()]; self.queries_ska.len()];

        let mut phylip_format = "".to_string();
        phylip_format += format!("{}\n", self.queries_ska.len()).as_str();

        for i in 0..self.queries_ska.len() {
            phylip_format += format!("{}\t", file_names[i]).as_str();
            for j in 0..self.queries_ska.len() { // Do it on only half of the matrix
                for ref_kmer in self.queries_ska[i].kmer_iter() {
                    if let Some(kmer_match) = self.queries_ska[j].kmers().get(&ref_kmer.0) {
                        if *kmer_match != *ref_kmer.1 {
                            pairwise_distances[i][j] += 1;
                        }
                    }
                }
                phylip_format += format!("{}\t", pairwise_distances[i][j]).as_str();
            }
            phylip_format += "\n";
        }

        let d = DistanceMatrix::read_from_phylip(phylip_format.as_bytes()).unwrap();
        let tree = NeighborJoiningSolver::<Canonical>::default(d.clone())
            .solve()
            .unwrap();
        let newick = speedytree::to_newick(&tree);
        newick
    }

    pub fn get_size(&self) -> usize {
        self.queries_ska.len()
    }
}