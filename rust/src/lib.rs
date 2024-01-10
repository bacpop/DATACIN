
use wasm_bindgen::prelude::*;
use wasm_bindgen_file_reader::WebSysFile;
extern crate console_error_panic_hook;

pub mod fastx;
pub mod ska_ref;
use crate::ska_ref::RefSka;
pub mod ska_dict;
pub mod ska_map;
use crate::ska_map::SkaMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum QualFilter {
    /// Ignore quality scores in reads
    NoFilter,
    /// Filter middle bases below quality threshold
    Middle,
    /// Filter entire k-mer when any base below quality threshold
    Strict,
}

/// Quality filtering options for FASTQ files
pub struct QualOpts {
    /// Minimum k-mer count across reads to be added
    pub min_count: u16,
    /// Minimum base quality to be added
    pub min_qual: u8,
    /// [`QualFilter`]: apply quality across whole k-mer, just middle base, or not at all
    pub qual_filter: QualFilter,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct SkaData {
    reference: RefSka,
    mapped: Vec<SkaMap>,
}

#[wasm_bindgen]
impl SkaData {
    pub fn new(ref_file: web_sys::File) -> Self {
        if cfg!(debug_assertions) {
            init_panic_hook();
        }

        let k = 31;
        let rc = true;
        let ambig_mask = false;
        let repeat_mask = false;

        let mut wf = WebSysFile::new(ref_file);
        let reference = RefSka::new(k, &mut wf, rc, ambig_mask, repeat_mask);
        log(&format!("Indexed reference: {} split k-mers", reference.nk()));

        Self {reference, mapped: Vec::new()}
    }

    pub fn map(&mut self, input_file: String, rev_reads: Option<String>) -> Vec<usize> {
        if rev_reads.is_some() {
            log(&format!("Detected paired fastq input files"));
        }
        todo!();
    }
}

