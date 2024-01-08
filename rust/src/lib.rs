use wasm_bindgen::prelude::*;
extern crate console_error_panic_hook;

use wasm_bindgen_file_reader::WebSysFile;
use seq_io::fasta::{Reader, Record};
// WebSysFile doesn't have the Send trait, required by needletail
// use needletail::{parser::FastaReader, FastxReader};
//TODO use flate2 to support gz reading
// use flate2::read::GzDecoder;

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
pub struct SkaRef {
    k: usize,
    kmers: Vec<u128>,
    count: usize,
}

#[wasm_bindgen]
impl SkaRef {
    pub fn new(ref_file: web_sys::File) -> Self {
        init_panic_hook();

        let mut wf = WebSysFile::new(ref_file);
        let idx = idx_ref(&mut wf);
        log(&format!("Indexed reference with return value {}", idx));

        // Placeholder
        let k = 31;
        let size = 3000000;
        let kmers = vec![0; size];
        Self {k, kmers, count: 0}
    }

    pub fn map(&mut self, input_file: String, rev_reads: Option<String>) -> Vec<usize> {
        if rev_reads.is_some() {
            log(&format!("Detected paired fastq input files"));
        }
        let idx = map_to_ref(&input_file);
        let mut result: f64 = 0.0;
        // Take some time
        for i in 0..50000000 {
            result += ((i % 201) * (i % 7)) as f64;
        }
        self.count += 1;
        log(&format!("Mapped file #{} ({}) with return value {}", self.count, input_file, idx));
        vec![1; 10]
    }
}

pub fn idx_ref(ref_file: &mut WebSysFile) -> String {
    let mut reader = Reader::new(ref_file);
    let record = reader.next().unwrap().unwrap();

    let fasta_id: String = record.id().unwrap().into();
    log(&format!("Read fasta record {}: {}", fasta_id, String::from_utf8(record.full_seq()[0..10].to_vec()).unwrap()));

    fasta_id
}

pub fn map_to_ref(input_file: &str) -> usize {
    1
}
