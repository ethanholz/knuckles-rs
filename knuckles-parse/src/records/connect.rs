#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
use knuckles_macro::pydefault;

/// Represents a CONECT record specifying connectivity between atoms.
///
/// CONECT records specify the bonds between atoms that are not implied by
/// the chemical structure. These are particularly important for hetero-compounds,
/// metal coordination, and disulfide bonds.
///
/// # Fields
///
/// - `serial`: Serial number of the first atom in the connectivity
/// - `connected`: Array of up to 4 connected atom serial numbers
///
/// # Example
///
/// ```rust
/// use knuckles_parse::records::connect::ConnectRecord;
///
/// let line = "CONECT  413  412  414                                                           ";
/// let connect = ConnectRecord::from(line);
///
/// assert_eq!(connect.serial, 413);
/// assert_eq!(connect.connected[0], Some(412));
/// assert_eq!(connect.connected[1], Some(414));
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "python", pyclass(get_all, set_all))]
#[cfg_attr(feature = "python", pydefault)]
pub struct ConnectRecord {
    /// Serial number of the atom for which connectivity is being specified
    pub serial: u32,
    /// Array of up to 4 connected atom serial numbers
    pub connected: [Option<u32>; 4],
}

impl ConnectRecord {
    /// Create a new ConnectRecord by parsing a CONECT line.
    ///
    /// Parses the atom serial number and up to 4 connected atoms from the
    /// fixed-width fields in the CONECT record.
    ///
    /// # Arguments
    ///
    /// * `str` - A CONECT line from a PDB file
    ///
    /// # Returns
    ///
    /// A new `ConnectRecord` with parsed connectivity information.
    pub fn new(str: &str) -> ConnectRecord {
        ConnectRecord {
            serial: str[6..11].trim().parse::<u32>().unwrap_or_default(),
            connected: [
                str[11..16].trim().parse::<u32>().ok(),
                str[16..21].trim().parse::<u32>().ok(),
                str[21..26].trim().parse::<u32>().ok(),
                str[26..31].trim().parse::<u32>().ok(),
            ],
        }
    }
}

impl From<&str> for ConnectRecord {
    fn from(str: &str) -> Self {
        ConnectRecord::new(str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_connect_line_test() {
        const LINE: &str =
            "CONECT  413  412  414                                                           ";
        let record = ConnectRecord::new(LINE);
        assert_eq!(record.serial, 413);
        assert_eq!(record.connected, [Some(412), Some(414), None, None]);
    }
}
