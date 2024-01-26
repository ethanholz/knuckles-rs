#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum OrigxN {
    Origx1(OrigxnRecord),
    Origx2(OrigxnRecord),
    Origx3(OrigxnRecord),
}

impl OrigxN {
    pub fn new(str: &str) -> OrigxN {
        let record = OrigxnRecord::from(str);
        match record.n {
            1 => OrigxN::Origx1(record),
            2 => OrigxN::Origx2(record),
            3 => OrigxN::Origx3(record),
            _ => panic!("Invalid ORIGXN record"),
        }
    }
}

impl From<&str> for OrigxN {
    fn from(str: &str) -> Self {
        OrigxN::new(str)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OrigxnRecord {
    pub n: u16,
    pub origxn: [f32; 3],
    pub tn: f32,
}

impl OrigxnRecord {
    pub fn new(str: &str) -> OrigxnRecord {
        OrigxnRecord {
            n: str.chars().nth(5).unwrap() as u16 - 48,
            origxn: [
                str[10..20].trim().parse().unwrap_or_default(),
                str[20..30].trim().parse().unwrap_or_default(),
                str[30..40].trim().parse().unwrap_or_default(),
            ],
            tn: str[45..55].trim().parse().unwrap_or_default(),
        }
    }
}

impl From<&str> for OrigxnRecord {
    fn from(str: &str) -> Self {
        OrigxnRecord::new(str)
    }
}

impl std::fmt::Display for OrigxnRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = format!("{:1}", self.n);
        let origxn = format!(
            "{:10.6}{:10.6}{:10.6}",
            self.origxn[0], self.origxn[1], self.origxn[2]
        );
        let tn = format!("     {:10.5}", self.tn);
        write!(f, "{:<1$}", format!("ORIGX{}    {}{}", n, origxn, tn), 80)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_origxn_line_test() {
        const LINE: &str =
            "ORIGX1      0.963457  0.136613  0.230424       16.61000                         ";
        let record = OrigxnRecord::new(LINE);
        assert_eq!(record.n, 1);
        assert_eq!(record.origxn, [0.963457, 0.136613, 0.230424]);
        assert_eq!(record.tn, 16.61);
    }

    #[test]
    fn parse_origxn_test() {
        const LINE: &str =
            "ORIGX1      0.963457  0.136613  0.230424       16.61000                         ";
        let record = OrigxN::new(LINE);
        match record {
            OrigxN::Origx1(record) => {
                assert_eq!(record.n, 1);
                assert_eq!(record.origxn, [0.963457, 0.136613, 0.230424]);
                assert_eq!(record.tn, 16.61);
            }
            _ => panic!("Wrong record type"),
        }
    }

    #[test]
    fn origxn_record_display_test() {
        const LINE: &str =
            "ORIGX1      0.963457  0.136613  0.230424       16.61000                         ";
        let record = OrigxnRecord::new(LINE);
        assert_eq!(format!("{}", record), LINE);
    }
}
