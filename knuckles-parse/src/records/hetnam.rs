#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "python")]
use knuckles_macro::pydefault;

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "python", pyclass(get_all, set_all))]
#[cfg_attr(feature = "python", pydefault)]
pub struct HetnamRecord {
    pub continuation: Option<String>,
    pub het_id: String,
    pub text: String,
}

impl HetnamRecord {
    pub fn new(line: &str) -> Self {
        HetnamRecord {
            continuation: line
                .get(8..10)
                .map(|str| str.trim().to_string())
                .filter(|item| !item.is_empty()),
            het_id: line[11..15].trim().to_string(),
            text: line[15..].trim().to_string(), // TODO: This should only parse up to 70 chars but
                                                 // it may fail if the line is less than 70 chars
                                                 // using 15..70
        }
    }
}

impl From<&str> for HetnamRecord {
    fn from(value: &str) -> Self {
        HetnamRecord::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hetnam_new() {
        let line = "HETNAM     NAG N-ACETYL-D-GLUCOSAMINE";
        let record = HetnamRecord::new(line);
        assert_eq!(record.continuation, None);
        assert_eq!(record.het_id, "NAG");
        assert_eq!(record.text, "N-ACETYL-D-GLUCOSAMINE");

        let line = "HETNAM  2  SAD DINUCLEOTIDE";
        let record = HetnamRecord::new(line);
        assert_eq!(record.continuation, Some("2".to_string()));
        assert_eq!(record.het_id, "SAD");
        assert_eq!(record.text, "DINUCLEOTIDE");
    }

    #[test]
    fn test_hetnam_from() {
        let line = "HETNAM     NAG N-ACETYL-D-GLUCOSAMINE";
        let record = HetnamRecord::from(line);
        assert_eq!(record.continuation, None);
        assert_eq!(record.het_id, "NAG");
        assert_eq!(record.text, "N-ACETYL-D-GLUCOSAMINE");
        let line = "HETNAM  2  SAD DINUCLEOTIDE";
        let record = HetnamRecord::from(line);
        assert_eq!(record.continuation, Some("2".to_string()));
        assert_eq!(record.het_id, "SAD");
        assert_eq!(record.text, "DINUCLEOTIDE");
    }
}
