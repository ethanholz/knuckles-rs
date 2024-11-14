#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
use knuckles_macro::pydefault;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "python", pyclass(get_all, set_all))]
#[cfg_attr(feature = "python", pydefault)]
pub struct ConnectRecord {
    pub serial: u32,
    pub connected: [Option<u32>; 4],
}

impl ConnectRecord {
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
