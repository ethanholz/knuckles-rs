#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "python")]
use knuckles_macro::pydefault;

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "python", pyclass(get_all, set_all))]
#[cfg_attr(feature = "python", pydefault)]
pub struct HetRecord {
    pub het_id: String,
    pub chain_id: char,
    pub seq_num: i32,
    pub i_code: Option<char>,
    pub num_het_atoms: i32,
    pub text: Option<String>,
}

impl HetRecord {
    pub fn new(line: &str) -> Self {
        HetRecord {
            het_id: line[7..10].trim().to_string(),
            chain_id: line.chars().nth(12).unwrap(),
            seq_num: line[13..17].trim().parse().unwrap(),
            i_code: line[17..18].trim().parse().ok(),
            num_het_atoms: line[21..26].trim().parse().unwrap(),
            text: line
                .get(31..71)
                .map(|str| str.trim().to_string())
                .filter(|item| !item.is_empty()),
        }
    }
}

impl From<&str> for HetRecord {
    fn from(value: &str) -> Self {
        HetRecord::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_het_record() {
        let line = "HET    UDP  A1457      25                     ";
        let record = HetRecord::new(line);
        assert_eq!(record.het_id, "UDP");
        assert_eq!(record.chain_id, 'A');
        assert_eq!(record.seq_num, 1457);
        assert_eq!(record.i_code, None);
        assert_eq!(record.num_het_atoms, 25);
        assert_eq!(record.text, None);

        let line = "HET    UNK  A 161       1     ";
        let record = HetRecord::new(line);
        assert_eq!(record.het_id, "UNK");
        assert_eq!(record.chain_id, 'A');
        assert_eq!(record.seq_num, 161);
        assert_eq!(record.i_code, None);
        assert_eq!(record.num_het_atoms, 1);
        assert_eq!(record.text, None);
    }

    #[test]
    fn test_from_str() {
        let line = "HET    UDP  A1457      25                     ";
        let record: HetRecord = line.into();
        assert_eq!(record.het_id, "UDP");
        assert_eq!(record.chain_id, 'A');
        assert_eq!(record.seq_num, 1457);
        assert_eq!(record.i_code, None);
        assert_eq!(record.num_het_atoms, 25);
        assert_eq!(record.text, None);
    }
}
