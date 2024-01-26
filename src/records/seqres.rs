#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SeqresRecord {
    pub ser_num: u32,
    pub chain_id: char,
    pub num_res: u32,
    pub res_names: Vec<String>,
}

impl SeqresRecord {
    pub fn new(str: &str) -> Self {
        Self {
            ser_num: str[7..10].trim().parse().unwrap(),
            chain_id: str.chars().nth(11).unwrap(),
            num_res: str[13..17].trim().parse().unwrap(),
            res_names: str[19..]
                .split_whitespace()
                .map(|s| s.to_string())
                .collect(),
        }
    }
}

impl From<&str> for SeqresRecord {
    fn from(line: &str) -> Self {
        Self::new(line)
    }
}
