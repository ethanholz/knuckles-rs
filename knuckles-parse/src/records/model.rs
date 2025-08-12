#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
use knuckles_macro::pydefault;

/// Represents a MODEL record for multi-model PDB structures.
///
/// MODEL records are used in PDB files that contain multiple structural models
/// of the same molecule (e.g., NMR structures, molecular dynamics snapshots).
/// Each model is identified by a serial number.
///
/// # Fields
///
/// - `serial`: Model serial number
///
/// # Example
///
/// ```rust
/// use knuckles_parse::records::model::ModelRecord;
///
/// let line = "MODEL        1                                                                  ";
/// let model = ModelRecord::from(line);
/// assert_eq!(model.serial, 1);
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "python", pyclass(get_all, set_all))]
#[cfg_attr(feature = "python", pydefault)]
pub struct ModelRecord {
    /// Model serial number
    pub serial: u32,
}

impl ModelRecord {
    /// Create a new ModelRecord by parsing a MODEL line.
    pub fn new(str: &str) -> ModelRecord {
        ModelRecord {
            serial: str[10..14].trim().parse::<u32>().unwrap_or_default(),
        }
    }
}

impl From<&str> for ModelRecord {
    fn from(str: &str) -> Self {
        ModelRecord::new(str)
    }
}

impl std::fmt::Display for ModelRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:<1$}", format!("MODEL     {:>4}", self.serial), 80)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_model_record_new() {
        let line = "MODEL        1";
        let model = ModelRecord::new(line);
        assert_eq!(model.serial, 1);
    }
    #[test]
    fn test_model_record_from() {
        let line = "MODEL        1";
        let model = ModelRecord::from(line);
        assert_eq!(model.serial, 1);
    }
    #[test]
    fn test_model_record_display() {
        let model = ModelRecord { serial: 1 };
        let result =
            "MODEL        1                                                                  ";
        assert_eq!(format!("{}", model), result);
    }
}
