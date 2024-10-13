#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CrystalRecord {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub alpha: f32,
    pub beta: f32,
    pub gamma: f32,
    pub space_group: String,
    pub z: u32,
}

impl CrystalRecord {
    pub fn new(str: &str) -> CrystalRecord {
        CrystalRecord {
            a: str[6..15].trim().parse().unwrap_or_default(),
            b: str[15..24].trim().parse().unwrap_or_default(),
            c: str[24..33].trim().parse().unwrap_or_default(),
            alpha: str[33..40].trim().parse().unwrap_or_default(),
            beta: str[40..47].trim().parse().unwrap_or_default(),
            gamma: str[47..54].trim().parse().unwrap_or_default(),
            space_group: str[55..66].trim().to_string(),
            z: str[66..70].trim().parse().unwrap_or_default(),
        }
    }
}

impl From<&str> for CrystalRecord {
    fn from(str: &str) -> Self {
        CrystalRecord::new(str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_crystal_line_test() {
        const LINE: &str =
            "CRYST1   52.000   58.600   61.900  90.00  90.00  90.00 P 21 21 21    8          ";
        let record = CrystalRecord::new(LINE);
        assert_eq!(record.a, 52.0);
        assert_eq!(record.b, 58.6);
        assert_eq!(record.c, 61.9);
        assert_eq!(record.alpha, 90.0);
        assert_eq!(record.beta, 90.0);
        assert_eq!(record.gamma, 90.0);
        assert_eq!(record.space_group, "P 21 21 21");
        assert_eq!(record.z, 8);
    }
}
