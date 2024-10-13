pub mod records;
use records::Record;

#[cfg(feature = "parallel")]
pub fn pdbreader(contents: &str) -> Vec<Record> {
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
