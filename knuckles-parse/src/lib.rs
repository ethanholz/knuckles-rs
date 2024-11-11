pub mod records;
use records::Record;

#[cfg(feature = "python")]
use pyo3::prelude::*;

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
