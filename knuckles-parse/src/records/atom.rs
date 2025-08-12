#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "python")]
use knuckles_macro::pydefault;

#[cfg(feature = "python")]
use pyo3::prelude::*;

/// Represents an ATOM or HETATM record from a PDB file.
///
/// This structure contains all the information for an atom coordinate record,
/// including position, occupancy, temperature factors, and identification data.
///
/// # PDB Format
///
/// ATOM records contain atomic coordinate data for standard amino acid and nucleic acid residues.
/// HETATM records contain atomic coordinate data for non-standard residues, water molecules,
/// and other hetero-compounds.
///
/// # Fields
///
/// - `serial`: Atom serial number (1-99999, or hexadecimal for >99999)
/// - `name`: Atom name (e.g., "CA", "N", "O")
/// - `alt_loc`: Alternative location indicator
/// - `res_name`: Residue name (e.g., "ALA", "GLY", "HOH")
/// - `chain_id`: Chain identifier
/// - `res_seq`: Residue sequence number
/// - `i_code`: Insertion code for residues
/// - `x`, `y`, `z`: Atomic coordinates in Ångströms
/// - `occupancy`: Occupancy value (0.0-1.0)
/// - `temp_factor`: Temperature factor (B-factor)
/// - `element`: Element symbol
/// - `charge`: Formal charge
/// - `entry`: PDB entry identifier
///
/// # Example
///
/// ```rust
/// use knuckles_parse::records::atom::AtomRecord;
///
/// let line = "ATOM      1  N   ALA A   1      20.154  16.967  27.462  1.00 11.18           N";
/// let atom = AtomRecord::from(line);
///
/// assert_eq!(atom.serial, 1);
/// assert_eq!(atom.name, "N");
/// assert_eq!(atom.res_name, "ALA");
/// assert_eq!(atom.x, 20.154);
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "python",
    pyclass(get_all, set_all, module = "kncukles_parse")
)]
#[cfg_attr(feature = "python", pydefault)]
pub struct AtomRecord {
    /// Atom serial number (1-99999, or hexadecimal for larger values)
    pub serial: u32,
    /// Atom name (e.g., "CA", "N", "O")
    pub name: String,
    /// Alternative location indicator
    pub alt_loc: Option<char>,
    /// Residue name (e.g., "ALA", "GLY", "HOH")
    pub res_name: String,
    /// Chain identifier
    pub chain_id: Option<char>,
    /// Residue sequence number
    pub res_seq: i16,
    /// Insertion code for residues
    pub i_code: Option<char>,
    /// X coordinate in Ångströms
    pub x: f32,
    /// Y coordinate in Ångströms
    pub y: f32,
    /// Z coordinate in Ångströms
    pub z: f32,
    /// Occupancy value (0.0-1.0)
    pub occupancy: f32,
    /// Temperature factor (B-factor)
    pub temp_factor: f32,
    /// Element symbol
    pub element: Option<String>,
    /// Formal charge
    pub charge: Option<String>,
    /// PDB entry identifier
    pub entry: Option<String>,
}

impl AtomRecord {
    /// Create a new AtomRecord by parsing a PDB ATOM or HETATM line.
    ///
    /// This method parses fixed-width fields according to the PDB format specification.
    /// It handles both decimal and hexadecimal serial numbers (for atoms > 99999).
    ///
    /// # Arguments
    ///
    /// * `str` - A single ATOM or HETATM line from a PDB file
    ///
    /// # Returns
    ///
    /// A new `AtomRecord` with all fields parsed from the input line.
    ///
    /// # Panics
    ///
    /// Panics if required numeric fields cannot be parsed (coordinates, occupancy, etc.)
    ///
    /// # Example
    ///
    /// ```rust
    /// use knuckles_parse::records::atom::AtomRecord;
    ///
    /// let line = "ATOM      1  N   ALA A   1      20.154  16.967  27.462  1.00 11.18           N";
    /// let atom = AtomRecord::new(line);
    /// assert_eq!(atom.name, "N");
    /// ```
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
}
