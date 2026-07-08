#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "python")]
use knuckles_macro::pydefault;

#[cfg(feature = "python")]
use pyo3::prelude::*;

/// HEADER record containing PDB classification, deposition date, and identifier.
#[allow(non_snake_case)]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "python", pyclass(get_all, set_all))]
#[cfg_attr(feature = "python", pydefault)]
pub struct HeaderRecord {
    /// Classification of the molecule (columns 11-50).
    pub classification: String,
    /// Deposition date (columns 51-59).
    pub depDate: String,
    /// PDB identifier code (columns 63-66).
    pub idCode: String,
}

impl HeaderRecord {
    pub fn new(line: &str) -> Self {
        HeaderRecord {
            classification: line[10..50].trim().to_string(),
            depDate: line[50..59].trim().to_string(),
            idCode: line[62..66].trim().to_string(),
        }
    }
}

impl From<&str> for HeaderRecord {
    fn from(line: &str) -> Self {
        HeaderRecord::new(line)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_header_record_test() {
        const LINE: &str =
            "HEADER    PHOTOSYNTHESIS                          28-MAR-07   2UXK              ";
        let record = HeaderRecord::new(LINE);
        assert_eq!(record.classification, "PHOTOSYNTHESIS");
        assert_eq!(record.depDate, "28-MAR-07");
        assert_eq!(record.idCode, "2UXK");

        const LINE2: &str =
            "HEADER    TRANSFERASE/TRANSFERASE INHIBITOR       17-SEP-04   1XH6              ";
        let record = HeaderRecord::new(LINE2);
        assert_eq!(record.classification, "TRANSFERASE/TRANSFERASE INHIBITOR");
        assert_eq!(record.depDate, "17-SEP-04");
        assert_eq!(record.idCode, "1XH6");

        const LINE3: &str =
            "HEADER    MEMBRANE PROTEIN, TRANSPORT PROTEIN     20-JUL-06   2HRT              ";
        let record = HeaderRecord::new(LINE3);
        assert_eq!(record.classification, "MEMBRANE PROTEIN, TRANSPORT PROTEIN");
        assert_eq!(record.depDate, "20-JUL-06");
        assert_eq!(record.idCode, "2HRT");
    }
}
