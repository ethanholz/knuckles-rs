#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
use knuckles_macro::pydefault;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "python", pyclass(get_all, set_all))]
#[cfg_attr(feature = "python", pydefault)]
pub struct TermRecord {
    pub serial: u32,
    pub res_name: String,
    pub chain_id: char,
    pub res_seq: i16,
    pub i_code: Option<char>,
}

impl TermRecord {
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

    #[test]
    #[cfg(feature = "serde")]
    fn test_serde_serialization() {
        let str =
            "TER     297      ALA A  18                                                      ";
        let record = TermRecord::new(str);
        let json = serde_json::to_string(&record).unwrap();
        assert_eq!(
            json,
            r#"{"serial":297,"res_name":"ALA","chain_id":"A","res_seq":18,"i_code":null}"#
        );
        let deserialized: TermRecord = serde_json::from_str(&json).unwrap();
        assert_eq!(record, deserialized);
    }
}
