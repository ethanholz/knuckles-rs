//! # knuckles-parse
//!
//! A fast and efficient PDB (Protein Data Bank) file parser written in Rust.
//!
//! This library provides functionality to parse PDB files into structured Rust data types,
//! with support for parallel processing, Python bindings, and serialization.
//!
//! ## Features
//!
//! - **Fast parsing**: Optimized for performance with optional parallel processing
//! - **Comprehensive record support**: Handles all major PDB record types
//! - **Python bindings**: Optional Python integration via PyO3
//! - **Serialization**: Optional JSON serialization support via Serde
//! - **Parallel processing**: Optional multi-threaded parsing with Rayon
//!
//! ## Example
//!
//! ```rust
//! use knuckles_parse::{pdbreader_single, Record};
//!
//! let pdb_content = "ATOM      1  N   ALA A   1      20.154  16.967  27.462  1.00 11.18           N";
//! let records = pdbreader_single(pdb_content);
//!
//! match &records[0] {
//!     Record::Atom(atom) => {
//!         println!("Atom name: {}", atom.name);
//!         println!("Coordinates: ({}, {}, {})", atom.x, atom.y, atom.z);
//!     }
//!     _ => {}
//! }
//! ```

pub mod records;
use records::Record;

#[cfg(feature = "python")]
use pyo3::prelude::*;

/// Parse PDB file contents using parallel processing.
///
/// This function processes PDB file contents line-by-line using parallel processing
/// to extract structured record data. It automatically handles atom serial number
/// assignment for atoms that don't have them, which is necessary for some PDB files
/// with more than 99,999 atoms.
///
/// # Arguments
///
/// * `contents` - The complete PDB file contents as a string
///
/// # Returns
///
/// A vector of [`Record`] variants representing the parsed PDB records.
///
/// # Features
///
/// This function is only available when the `parallel` feature is enabled.
///
/// # Example
///
/// ```rust
/// use knuckles_parse::pdbreader_parallel;
///
/// let pdb_content = "ATOM      1  N   ALA A   1      20.154  16.967  27.462  1.00 11.18           N\n\
///                    ATOM      2  CA  ALA A   1      20.987  18.149  27.890  1.00 11.85           C";
/// let records = pdbreader_parallel(pdb_content);
/// println!("Parsed {} records", records.len());
/// ```
#[cfg(feature = "parallel")]
pub fn pdbreader_parallel(contents: &str) -> Vec<Record> {
    use rayon::prelude::*;

    let lines: Vec<&str> = contents.lines().collect();
    let mut record: Vec<Record> = lines
        .par_iter()
        .filter_map(|line| {
            if line.len() < 6 {
                return None;
            }
            Record::try_from(*line).ok()
        })
        .collect();

    // We then comb through the records and assign serial numbers to atoms that
    // don't have them. This is necessary for some PDB files, which have more than 99999 atoms.
    // NOTE: We don't need to use a second pass in the single threaded version because we can do it
    // in the same pass.
    let mut last = 0;
    for atom in record.iter_mut() {
        if let Record::Atom(atom) = atom {
            if atom.serial == 0 {
                last += 1;
                atom.serial = last;
            } else {
                last = atom.serial;
            }
        }
    }
    record
}

/// Parse PDB file contents using single-threaded processing.
///
/// This function processes PDB file contents line-by-line in a single thread
/// to extract structured record data. It handles atom serial number assignment
/// during the parsing process, making it more efficient than the parallel version
/// for smaller files.
///
/// # Arguments
///
/// * `contents` - The complete PDB file contents as a string
///
/// # Returns
///
/// A vector of [`Record`] variants representing the parsed PDB records.
/// Note: Currently only returns ATOM records, filtering out other record types.
///
/// # Example
///
/// ```rust
/// use knuckles_parse::pdbreader_single;
///
/// let pdb_content = "ATOM      1  N   ALA A   1      20.154  16.967  27.462  1.00 11.18           N\n\
///                    HETATM    2  O   HOH A   2      15.123  12.456  30.789  1.00 25.50           O";
/// let records = pdbreader_single(pdb_content);
/// println!("Parsed {} atom records", records.len());
/// ```
pub fn pdbreader_single(contents: &str) -> Vec<Record> {
    let mut last = 0;
    contents
        .lines()
        .filter_map(|line| {
            if line.len() < 6 {
                return None;
            }
            let record = Record::try_from(line);
            if let Ok(Record::Atom(mut atom)) = record {
                if atom.serial == 0 {
                    last += 1;
                    atom.serial = last;
                } else {
                    last = atom.serial;
                }
                Some(Record::Atom(atom))
            } else {
                None
            }
            // Record::try_from(line).ok()
        })
        .collect()
}

#[cfg(feature = "python")]
#[pymodule(name = "knuckles_parse")]
mod knuckles_parse {
    use super::*;
    #[pymodule_export]
    use crate::records::anisotropic::AnisotropicRecord;
    #[pymodule_export]
    use crate::records::atom::AtomRecord;
    #[pymodule_export]
    use crate::records::connect::ConnectRecord;
    #[pymodule_export]
    use crate::records::crystal::CrystalRecord;
    #[pymodule_export]
    use crate::records::dbref::DBRefRecord;
    #[pymodule_export]
    use crate::records::het::HetRecord;
    #[pymodule_export]
    use crate::records::hetnam::HetnamRecord;
    #[pymodule_export]
    use crate::records::model::ModelRecord;
    #[pymodule_export]
    use crate::records::modres::ModresRecord;
    #[pymodule_export]
    use crate::records::mtrixn::MtrixnRecord;
    #[pymodule_export]
    use crate::records::nummdl::NummdlRecord;
    #[pymodule_export]
    use crate::records::origxn::OrigxnRecord;
    #[pymodule_export]
    use crate::records::scalen::ScalenRecord;
    #[pymodule_export]
    use crate::records::seqadv::SeqAdvRecord;
    #[pymodule_export]
    use crate::records::seqres::SeqresRecord;
    #[pymodule_export]
    use crate::records::term::TermRecord;
    #[pymodule_export]
    use crate::records::Record;

    /// Creates a list of PDB records from a string
    #[pyfunction]
    fn pdbreader(contents: &str) -> Vec<Record> {
        pdbreader_parallel(contents)
    }

    #[pyfunction]
    fn version() -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }
}
