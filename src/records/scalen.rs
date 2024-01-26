#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ScaleN {
    Scale1(ScalenRecord),
    Scale2(ScalenRecord),
    Scale3(ScalenRecord),
}

impl ScaleN {
    pub fn new(str: &str) -> Self {
        let record = ScalenRecord::from(str);
        match record.n {
            1 => ScaleN::Scale1(record),
            2 => ScaleN::Scale2(record),
            3 => ScaleN::Scale3(record),
            _ => panic!("Invalid SCALEn record"),
        }
    }
}

impl From<&str> for ScaleN {
    fn from(str: &str) -> Self {
        ScaleN::new(str)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ScalenRecord {
    pub n: u16,
    pub scalen: [f32; 3],
    pub un: f32,
}

impl ScalenRecord {
    pub fn new(str: &str) -> ScalenRecord {
        ScalenRecord {
            n: str.chars().nth(5).unwrap() as u16 - 48,
            scalen: [
                str[10..20].trim().parse().unwrap_or_default(),
                str[20..30].trim().parse().unwrap_or_default(),
                str[30..40].trim().parse().unwrap_or_default(),
            ],
            un: str[45..55].trim().parse().unwrap_or_default(),
        }
    }
}

impl From<&str> for ScalenRecord {
    fn from(str: &str) -> Self {
        ScalenRecord::new(str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_scalen_line_test() {
        const LINE: &str =
            "SCALE1      0.019231  0.000000  0.000000        0.00000                         ";
        let record = ScalenRecord::new(LINE);
        assert_eq!(record.n, 1);
        assert_eq!(record.scalen, [0.019231, 0.0, 0.0]);
        assert_eq!(record.un, 0.0);
    }

    #[test]
    fn parse_scalen_test() {
        const LINE: &str =
            "SCALE1      0.019231  0.000000  0.000000        0.00000                         ";
        let record = ScaleN::new(LINE);
        match record {
            ScaleN::Scale1(record) => {
                assert_eq!(record.n, 1);
                assert_eq!(record.scalen, [0.019231, 0.0, 0.0]);
                assert_eq!(record.un, 0.0);
            }
            _ => panic!("Wrong record type"),
        }
    }
}
