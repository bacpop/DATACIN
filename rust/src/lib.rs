use wasm_bindgen::prelude::*;
use wasm_bindgen_file_reader::WebSysFile;
extern crate console_error_panic_hook;

pub mod fastx;
pub mod ska_ref;
use crate::ska_ref::RefSka;
pub mod ska_dict;
pub mod ska_map;
pub mod ska_align;

use crate::ska_map::SkaMap;
use crate::ska_align::SkaAlign;

use json;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum QualFilter {
    /// Ignore quality scores in reads
    #[default]
    NoFilter,
    /// Filter middle bases below quality threshold
    Middle,
    /// Filter entire k-mer when any base below quality threshold
    Strict,
}

/// Quality filtering options for FASTQ files
#[derive(Copy, Clone, Debug, Default)]
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
    reference_string: Vec<String>,
    mapped: Vec<SkaMap>,
}

#[wasm_bindgen]
impl SkaData {
    pub fn new(ref_file: web_sys::File, k: usize) -> Self {
        if cfg!(debug_assertions) {
            init_panic_hook();
        }

        let rc = true;
        let ambig_mask = false;
        let repeat_mask = false;

        let mut wf = WebSysFile::new(ref_file);
        let reference = RefSka::new(k, &mut wf, rc, ambig_mask, repeat_mask);
        log(&format!(
            "Indexed reference: {} split k-mers",
            reference.nk()
        ));
        let reference_string = reconstruct_sequence(&reference);

        Self {
            reference,
            reference_string,
            mapped: Vec::new(),
        }
    }

    pub fn map(&mut self, input_file: web_sys::File, rev_reads: Option<web_sys::File>, proportion_reads: Option<f64>) -> String {
        if rev_reads.is_some() {
            log(&format!("Detected paired fastq input files"));
        }
        log(&format!("Mapping reads to reference"));
        let file_name = input_file.name();
        let file_type = file_name.split('.').nth(1).unwrap();
        let mut wf1 = WebSysFile::new(input_file);

        if let Some(second_file) = rev_reads {
            self.mapped.push(SkaMap::new(
                &self.reference,
                &mut wf1,
                Some(&mut WebSysFile::new(second_file)),
                file_type,
                proportion_reads,
            ));
        } else {
            self.mapped
                .push(SkaMap::new(&self.reference, &mut wf1, None, file_type, proportion_reads));
        };

        log("Reads mapped successfully!");
        
        let mut results = json::JsonValue::new_array();

        results["Mapped sequences"] = json::JsonValue::new_array();

        let ref whole_mapping = self.reference.pseudoalignment(self.mapped[self.mapped.len() - 1].mapped_bases())[0];

        let mut current_length = 0;
        for chr in 0..self.reference_string.len() {
            let chr_mapping: String = whole_mapping[current_length..current_length + self.reference_string[chr].len()].to_string();
            let _ = results["Mapped sequences"].push(chr_mapping);
            current_length += self.reference_string[chr].len();
        }

        results["Number of variants"] = self.mapped[self.mapped.len() - 1].mapped_bases().len().into();
        // The result is a concatenated string of all the mapped bases

        let mut count_mapped_bases = 0;
        let mut count_total_bases = 0;
        for base in whole_mapping.chars() {
            if base != '-' {
                count_mapped_bases += 1;
                count_total_bases += 1;
            } else {
                count_total_bases += 1;
            }
        }

        results["Coverage"] = (count_mapped_bases as f64 / count_total_bases as f64).into();

        log("Results computed successfully!");

        return results.dump();
    }

    pub fn get_reference(&self) -> String {
        return self.reference_string.join("\n");
    }
}

pub fn reconstruct_sequence(reference: &RefSka) -> Vec<String> {
    let sequence_u8 = reference.get_seq();
    let mut sequence_string = Vec::new();

    for sequence in sequence_u8 {
        let mut current_seq= "".to_string();
        for base in sequence {
            if *base != 10 {
                current_seq.push(*base as char);
            }
            
        }
        sequence_string.push(current_seq);
    }

    return sequence_string;
}

#[wasm_bindgen]
pub struct AlignData {
    alignment: SkaAlign,
    file_names: Vec<String>,
}

#[wasm_bindgen]
impl AlignData {
    pub fn new(k: usize) -> Self {
        Self {
            alignment: SkaAlign::new(k),
            file_names: Vec::new(),
        }
    }

    pub fn align(&mut self, input_files: Vec<web_sys::File>, proportion_reads: Option<f64>) -> String {
        log(&format!("Aligning reads"));

        let mut wf1: WebSysFile;

        for input_file in input_files {
            let file_name = input_file.name();
            self.file_names.push(file_name.clone().split('.').nth(0).unwrap().to_string());
            let file_type = file_name.split('.').nth(1).unwrap();   
            wf1 = WebSysFile::new(input_file);
            
            self.alignment.add_file(
                &mut wf1,
                file_type,
                proportion_reads,
            );
        }

        if self.alignment.get_size() <= 2 {
            let mut results = json::JsonValue::new_array();
            results["newick"] = "Not enough sequences to align".into();
            results["names"] = json::JsonValue::new_array();
            for name in &self.file_names {
                let _ = results["names"].push(name.to_string());
            }

            return results.dump()
        }

        let newick = self.alignment.align(&self.file_names);

        let mut results = json::JsonValue::new_array();

        results["newick"] = newick.into();
        results["names"] = json::JsonValue::new_array();
            for name in &self.file_names {
                let _ = results["names"].push(name.to_string());
            }
        results.dump()
    }
}
