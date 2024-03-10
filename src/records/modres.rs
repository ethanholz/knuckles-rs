#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ModresRecord {
    pub id_code: String,
    pub res_name: String,
    pub chain_id: char,
    pub seq_num: i32,
    pub i_code: Option<char>,
    pub std_res_name: String,
    pub comment: String,
}

impl ModresRecord {
    pub fn from(line: &str) -> Self {
        Self {
            id_code: line[7..11].trim().to_string(),
            res_name: line[12..15].trim().to_string(),
            chain_id: line.chars().nth(16).unwrap(),
            seq_num: line[18..22].trim().parse().unwrap(),
            i_code: line[22..23].trim().parse().ok(),
            std_res_name: line[24..27].trim().to_string(),
            comment: line[29..].trim().to_string(),
        }
    }
}

impl From<&str> for ModresRecord {
    fn from(line: &str) -> Self {
        Self::from(line)
    }
}
