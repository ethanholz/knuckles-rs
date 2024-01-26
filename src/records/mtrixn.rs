#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MtrixN {
    Mtrix1(MtrixnRecord),
    Mtrix2(MtrixnRecord),
    Mtrix3(MtrixnRecord),
}

impl MtrixN {
    pub fn new(str: &str) -> Self {
        let record = MtrixnRecord::from(str);
        match record.n {
            1 => MtrixN::Mtrix1(record),
            2 => MtrixN::Mtrix2(record),
            3 => MtrixN::Mtrix3(record),
            _ => panic!("Invalid MTRIXn record"),
        }
    }
}

impl From<&str> for MtrixN {
    fn from(str: &str) -> Self {
        MtrixN::new(str)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MtrixnRecord {
    pub n: u32,
    pub serial_number: u32,
    pub matrix: [f32; 3],
    pub vn: f32,
    pub i_given: bool,
}

impl MtrixnRecord {
    pub fn from(str: &str) -> Self {
        Self {
            n: str.chars().nth(5).unwrap() as u32 - 48,
            serial_number: str[7..10].trim().parse().unwrap(),
            matrix: [
                str[10..20].trim().parse().unwrap(),
                str[20..30].trim().parse().unwrap(),
                str[30..40].trim().parse().unwrap(),
            ],
            vn: str[45..55].trim().parse().unwrap(),
            i_given: str.chars().nth(59).unwrap() == '1',
        }
    }
}

impl From<&str> for MtrixnRecord {
    fn from(line: &str) -> Self {
        Self::from(line)
    }
}

impl std::fmt::Display for MtrixnRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = format!("{:1}", self.n);
        let serial_number = format!("{:3}", self.serial_number);
        let origxn = format!(
            "{:10.6}{:10.6}{:10.6}",
            self.matrix[0], self.matrix[1], self.matrix[2]
        );
        let vn = format!("     {:10.5}", self.vn);
        let i_given = format!("{:1}", self.i_given as u32);
        write!(
            f,
            "{:<1$}",
            format!("MTRIX{} {}{}{}{:>5}", n, serial_number, origxn, vn, i_given),
            80
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mtrixn_record() {
        let line =
            "MTRIX1   1 -1.000000  0.000000  0.000000        0.00000    1                   ";
        let record = MtrixnRecord::from(line);
        assert_eq!(record.n, 1);
        assert_eq!(record.serial_number, 1);
        assert_eq!(record.matrix, [-1., 0., 0.]);
        assert_eq!(record.vn, 0.);
        assert_eq!(record.i_given, true);
        let line2 = "MTRIX2   1  0.000000  1.000000  0.000000        0.00000    1";
        let record = MtrixnRecord::from(line2);

        assert_eq!(record.n, 2);
        assert_eq!(record.serial_number, 1);
        assert_eq!(record.matrix, [0., 1., 0.]);
        assert_eq!(record.vn, 0.);
        assert_eq!(record.i_given, true);
    }

    #[test]
    fn test_mtrixn() {
        let line = "MTRIX1   1 -1.000000  0.000000  0.000000        0.00000    1";
        let record = MtrixN::from(line);
        match record {
            MtrixN::Mtrix1(record) => {
                assert_eq!(record.n, 1);
                assert_eq!(record.serial_number, 1);
                assert_eq!(record.matrix, [-1., 0., 0.]);
                assert_eq!(record.vn, 0.);
                assert_eq!(record.i_given, true);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn test_mtrixn_display() {
        let line =
            "MTRIX1   1 -1.000000  0.000000  0.000000        0.00000    1                    ";
        let record = MtrixnRecord::from(line);
        assert_eq!(format!("{}", record), line);
    }
}
