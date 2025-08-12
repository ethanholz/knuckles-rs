#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
use knuckles_macro::pydefault;

/// Represents a CRYST1 record containing crystallographic unit cell parameters.
///
/// This record specifies the unit cell parameters and space group for the crystal structure.
/// It is essential for understanding the crystallographic context of the atomic coordinates.
///
/// # Fields
///
/// - `a`, `b`, `c`: Unit cell dimensions in Ångströms
/// - `alpha`, `beta`, `gamma`: Unit cell angles in degrees
/// - `space_group`: Space group symbol
/// - `z`: Number of polymeric chains in the unit cell
///
/// # Example
///
/// ```rust
/// use knuckles_parse::records::crystal::CrystalRecord;
///
/// let line = "CRYST1   52.000   58.600   61.900  90.00  90.00  90.00 P 21 21 21    8";
/// let crystal = CrystalRecord::from(line);
///
/// assert_eq!(crystal.a, 52.0);
/// assert_eq!(crystal.space_group, "P 21 21 21");
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "python", pyclass(get_all, set_all))]
#[cfg_attr(feature = "python", pydefault)]
pub struct CrystalRecord {
    /// Unit cell dimension a in Ångströms
    pub a: f32,
    /// Unit cell dimension b in Ångströms
    pub b: f32,
    /// Unit cell dimension c in Ångströms
    pub c: f32,
    /// Unit cell angle alpha in degrees
    pub alpha: f32,
    /// Unit cell angle beta in degrees
    pub beta: f32,
    /// Unit cell angle gamma in degrees
    pub gamma: f32,
    /// Space group symbol
    pub space_group: String,
    /// Number of polymeric chains in the unit cell
    pub z: u32,
}

impl CrystalRecord {
    /// Create a new CrystalRecord by parsing a CRYST1 line.
    ///
    /// Parses the unit cell parameters and space group information from the fixed-width
    /// fields in the CRYST1 record.
    ///
    /// # Arguments
    ///
    /// * `str` - A CRYST1 line from a PDB file
    ///
    /// # Returns
    ///
    /// A new `CrystalRecord` with parsed crystallographic parameters.
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
