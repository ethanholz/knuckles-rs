#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ModelRecord {
    pub serial: u32,
}

impl ModelRecord {
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
