#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "python")]
use knuckles_macro::pydefault;

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "python",
    pyclass(get_all, set_all, module = "kncukles_parse")
)]
#[cfg_attr(feature = "python", pydefault)]
/// A record for an atom in a PDB file
pub struct AtomRecord {
    pub serial: u32,
    pub name: String,
    pub alt_loc: Option<char>,
    pub res_name: String,
    pub chain_id: Option<char>,
    pub res_seq: i16,
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
        let mut radix = 10;
        let serial = str[6..11].trim();
        if serial.chars().any(|c| c.is_ascii_alphabetic()) {
            radix = 16;
        }
        AtomRecord {
            // TODO: add support for parsing serial numbers > 99999
            serial: u32::from_str_radix(serial, radix).unwrap_or_default(),
            name: str[12..16].trim().to_string(),
            alt_loc: str[16..17].trim().parse::<char>().ok(),
            res_name: str[17..20].trim().to_string(),
            chain_id: str[21..22].trim().parse::<char>().ok(),
            res_seq: str[22..26].trim().parse().unwrap(),
            i_code: str[26..27].trim().parse().ok(),
            x: str[30..38].trim().parse().unwrap(),
            y: str[38..46].trim().parse().unwrap(),
            z: str[46..54].trim().parse().unwrap(),
            occupancy: str[54..60].trim().parse().unwrap(),
            temp_factor: str[60..66].trim().parse().unwrap(),
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

    #[test]
    fn parse_atom_line_hex_test() {
        const LINE: &str =
            "ATOM  186a0  CA  GLY A  67      26.731  62.085   4.078  0.00  7.83           C  ";
        let record = AtomRecord::new(LINE);
        assert_eq!(record.serial, 100000);
        assert_eq!(record.name, "CA");
        assert_eq!(record.alt_loc, None);
        assert_eq!(record.res_name, "GLY");
        assert_eq!(record.res_seq, 67);
        assert_eq!(record.x, 26.731);
        assert_eq!(record.y, 62.085);
        assert_eq!(record.z, 4.078);
        assert_eq!(record.occupancy, 0.00);
        assert_eq!(record.temp_factor, 7.83);
        assert_eq!(record.entry, None);
        assert_eq!(record.element, Some("C".to_string()));
        assert_eq!(record.charge, None);
    }

    #[test]
    fn parse_atom_from_str() {
        const LINE: &str =
            "ATOM  186a0  CA  GLY A  67      26.731  62.085   4.078  0.00  7.83           C  ";
        let record = AtomRecord::from(LINE);
        assert_eq!(record.serial, 100000);
        assert_eq!(record.name, "CA");
        assert_eq!(record.alt_loc, None);
        assert_eq!(record.res_name, "GLY");
        assert_eq!(record.res_seq, 67);
        assert_eq!(record.x, 26.731);
        assert_eq!(record.y, 62.085);
        assert_eq!(record.z, 4.078);
        assert_eq!(record.occupancy, 0.00);
        assert_eq!(record.temp_factor, 7.83);
        assert_eq!(record.entry, None);
        assert_eq!(record.element, Some("C".to_string()));
        assert_eq!(record.charge, None);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_serde_serialization() {
        const LINE: &str =
            "ATOM  186a0  CA  GLY A  67      26.731  62.085   4.078  0.00  7.83           C  ";
        let record = AtomRecord::from(LINE);
        let serialized = serde_json::to_string(&record).expect("Serialization failed");
        assert_eq!(
            serialized,
            r#"{"serial":100000,"name":"CA","alt_loc":null,"res_name":"GLY","chain_id":"A","res_seq":67,"i_code":null,"x":26.731,"y":62.085,"z":4.078,"occupancy":0.0,"temp_factor":7.83,"element":"C","charge":null,"entry":null}"#
        );
        let deserialized: AtomRecord =
            serde_json::from_str(&serialized).expect("Deserialization failed");
        assert_eq!(record, deserialized);
    }
}
