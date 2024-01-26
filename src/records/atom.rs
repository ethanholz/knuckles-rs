#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AtomRecord {
    pub serial: u32,
    pub name: String,
    pub alt_loc: Option<char>,
    pub res_name: String,
    pub chain_id: Option<char>,
    pub res_seq: u16,
    pub i_code: Option<char>,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub occupancy: f32,
    pub temp_factor: f32,
    pub element: Option<String>,
    pub charge: Option<String>,
    pub entry: Option<String>,
}

impl AtomRecord {
    pub fn new(str: &str) -> AtomRecord {
        AtomRecord {
            // TODO: add support for parsing serial numbers > 99999
            serial: str[6..11].trim().parse::<u32>().unwrap_or_default(),
            name: str[12..16].trim().to_string(),
            alt_loc: str[16..17].trim().parse::<char>().ok(),
            res_name: str[17..20].trim().to_string(),
            chain_id: str[21..22].trim().parse::<char>().ok(),
            res_seq: str[22..26].trim().parse::<u16>().unwrap(),
            i_code: str[26..27].trim().parse::<char>().ok(),
            x: str[30..38].trim().parse::<f32>().unwrap(),
            y: str[38..46].trim().parse::<f32>().unwrap(),
            z: str[46..54].trim().parse::<f32>().unwrap(),
            occupancy: str[54..60].trim().parse::<f32>().unwrap(),
            temp_factor: str[60..66].trim().parse::<f32>().unwrap(),
            entry: str
                .get(72..76)
                .map(|str| str.trim().to_string())
                .filter(|item| !item.is_empty()),
            element: str
                .get(77..80)
                .map(|str| str.trim().to_string())
                .filter(|item| !item.is_empty()),
            charge: str
                .get(78..80)
                .map(|str| str.trim().to_string())
                .filter(|item| !item.is_empty()),
        }
    }
}

impl From<&str> for AtomRecord {
    fn from(str: &str) -> Self {
        AtomRecord::new(str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_atom_line_test() {
        const LINE: &str =
            "ATOM     17  NE2 GLN     2      25.562  32.733   1.806  1.00 19.49      1UBQ    ";
        let record = AtomRecord::new(LINE);
        assert_eq!(record.serial, 17);
        assert_eq!(record.name, "NE2");
        assert_eq!(record.alt_loc, None);
        assert_eq!(record.res_name, "GLN");
        assert_eq!(record.res_seq, 2);
        assert_eq!(record.x, 25.562);
        assert_eq!(record.y, 32.733);
        assert_eq!(record.z, 1.806);
        assert_eq!(record.occupancy, 1.00);
        assert_eq!(record.temp_factor, 19.49);
        assert_eq!(record.entry, Some("1UBQ".to_string()));
        assert_eq!(record.element, None);
        assert_eq!(record.charge, None);
    }
}
