#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
use knuckles_macro::pydefault;

/// Represents a TER (termination) record indicating the end of a chain.
///
/// TER records are used to indicate the end of a list of ATOM/HETATM records for a chain.
///
/// # Fields
///
/// - `serial`: Serial number of the terminating atom
/// - `res_name`: Residue name of the terminating residue  
/// - `chain_id`: Chain identifier
/// - `res_seq`: Residue sequence number
/// - `i_code`: Insertion code
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "python", pyclass(get_all, set_all))]
#[cfg_attr(feature = "python", pydefault)]
pub struct TermRecord {
    /// Serial number of the terminating atom
    pub serial: u32,
    /// Residue name of the terminating residue
    pub res_name: String,
    /// Chain identifier
    pub chain_id: char,
    /// Residue sequence number
    pub res_seq: i16,
    /// Insertion code
    pub i_code: Option<char>,
}

impl TermRecord {
    /// Create a new TermRecord by parsing a TER line.
    pub fn new(str: &str) -> TermRecord {
        TermRecord {
            serial: str[6..11].trim().parse().unwrap_or_default(),
            res_name: str[17..20].trim().to_string(),
            chain_id: str[21..22].parse().unwrap_or_default(),
            res_seq: str[22..26].trim().parse().unwrap_or_default(),
            i_code: str
                .get(26..27)
                .map(|s| s.parse().unwrap_or_default())
                .filter(|item| *item != ' '),
        }
    }
}

impl From<&str> for TermRecord {
    fn from(str: &str) -> Self {
        TermRecord::new(str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let str =
            "TER     297      ALA A  18                                                      ";
        let record = TermRecord::new(str);
        assert_eq!(record.serial, 297);
        assert_eq!(record.res_name, "ALA");
        assert_eq!(record.chain_id, 'A');
        assert_eq!(record.res_seq, 18);
        assert_eq!(record.i_code, None);
    }

    #[test]
    fn test_from() {
        let str =
            "TER     297      ALA A  18                                                      ";
        let record = TermRecord::from(str);
        assert_eq!(record.serial, 297);
        assert_eq!(record.res_name, "ALA");
        assert_eq!(record.chain_id, 'A');
        assert_eq!(record.res_seq, 18);
        assert_eq!(record.i_code, None);
    }
}
