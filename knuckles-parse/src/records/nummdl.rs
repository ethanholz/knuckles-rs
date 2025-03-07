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
pub struct NummdlRecord {
    pub count: u32,
}

impl NummdlRecord {
    pub fn new(str: &str) -> NummdlRecord {
        NummdlRecord {
            count: str[10..14].trim().parse::<u32>().unwrap_or_default(),
        }
    }
}

impl From<&str> for NummdlRecord {
    fn from(str: &str) -> Self {
        NummdlRecord::new(str)
    }
}

impl std::fmt::Display for NummdlRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:<1$}", format!("NUMMDL    {:>4}", self.count), 80)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_model_record_new() {
        let line = "NUMMDL       1";
        let model = NummdlRecord::new(line);
        assert_eq!(model.count, 1);
    }
    #[test]
    fn test_model_record_from() {
        let line = "NUMMDL       1";
        let model = NummdlRecord::from(line);
        assert_eq!(model.count, 1);
    }
    #[test]
    fn test_model_record_display() {
        let model = NummdlRecord { count: 1 };
        let result =
            "NUMMDL       1                                                                  ";
        assert_eq!(format!("{}", model), result);
    }
}
