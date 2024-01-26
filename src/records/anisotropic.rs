#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AnisotropicRecord {
    pub serial: u32,
    pub name: String,
    pub alt_loc: Option<char>,
    pub res_name: String,
    pub chain_id: char,
    pub res_seq: u16,
    pub i_code: Option<char>,
    pub u00: i32,
    pub u11: i32,
    pub u22: i32,
    pub u01: i32,
    pub u02: i32,
    pub u12: i32,
    pub element: Option<String>,
    pub charge: Option<String>,
}

impl AnisotropicRecord {
    pub fn new(str: &str) -> AnisotropicRecord {
        AnisotropicRecord {
            serial: str[6..11].trim().parse::<u32>().unwrap_or_default(),
            name: str[12..16].trim().to_string(),
            alt_loc: str[16..17].trim().parse::<char>().ok(),
            res_name: str[17..20].trim().to_string(),
            chain_id: str[21..22].trim().parse::<char>().unwrap(),
            res_seq: str[22..26].trim().parse::<u16>().unwrap(),
            i_code: str[26..27].trim().parse::<char>().ok(),
            u00: str[28..35].trim().parse().unwrap(),
            u11: str[35..42].trim().parse().unwrap(),
            u22: str[42..49].trim().parse().unwrap(),
            u01: str[49..56].trim().parse().unwrap(),
            u02: str[56..63].trim().parse().unwrap(),
            u12: str[63..70].trim().parse().unwrap(),
            element: str
                .get(76..78)
                .map(|str| str.trim().to_string())
                .filter(|item| !item.is_empty()),
            charge: str
                .get(78..80)
                .map(|str| str.trim().to_string())
                .filter(|item| !item.is_empty()),
        }
    }
}

impl From<&str> for AnisotropicRecord {
    fn from(str: &str) -> Self {
        AnisotropicRecord::new(str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_anisou_line_test() {
        const LINE: &str =
            "ANISOU    1  N   MET A   1      688   1234    806    -19    -49    178       N  ";
        let record = AnisotropicRecord::new(LINE);
        assert_eq!(record.serial, 1);
        assert_eq!(record.name, "N");
        assert_eq!(record.res_name, "MET");
        assert_eq!(record.chain_id, 'A');
        assert_eq!(record.res_seq, 1);
        assert_eq!(record.u00, 688);
        assert_eq!(record.u11, 1234);
        assert_eq!(record.u22, 806);
        assert_eq!(record.u01, -19);
        assert_eq!(record.u02, -49);
        assert_eq!(record.u12, 178);
        assert_eq!(record.element, Some("N".to_string()));
        const LINE2: &str =
            "ANISOU    1  N   MET A   1      688   1234    806    -19    -49    178          ";
        let record = AnisotropicRecord::new(LINE2);
        assert_eq!(record.serial, 1);
        assert_eq!(record.name, "N");
        assert_eq!(record.res_name, "MET");
        assert_eq!(record.chain_id, 'A');
        assert_eq!(record.res_seq, 1);
        assert_eq!(record.u00, 688);
        assert_eq!(record.u11, 1234);
        assert_eq!(record.u22, 806);
        assert_eq!(record.u01, -19);
        assert_eq!(record.u02, -49);
        assert_eq!(record.u12, 178);
        assert_eq!(record.element, None);
    }
}
