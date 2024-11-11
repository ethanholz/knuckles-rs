#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
use knuckles_macro::pydefault;

#[derive(Debug, Clone)]
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
            i_code: str.get(26..27).map(|s| s.parse().unwrap_or_default()),
        }
    }
}

impl From<&str> for TermRecord {
    fn from(str: &str) -> Self {
        TermRecord::new(str)
    }
}