
#[derive(Debug, Clone, Default)]
pub struct Variant {
    chrom: usize,
    pos: usize,
    base: u8
}

#[derive(Debug, Clone, Default)]
pub struct SkaMap {
    /// Positions of mapped bases as (chrom, pos)
    mapped_bases: Vec<Variant>,
}

impl SkaMap {
    pub fn new() -> Self {
        todo!()
    }
}