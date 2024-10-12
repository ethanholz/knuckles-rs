#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DBRefRecord {
    pub id_code: String,
    pub chain_id: char,
    pub seq_begin: u32,
    pub insert_begin: Option<char>,
    pub seq_end: u32,
    pub insert_end: Option<char>,
    pub database: DBType,
    pub db_accession: String,
    pub db_id_code: String,
    pub db_seq_begin: u32,
    pub i_dbns_beg: Option<char>,
    pub db_seq_end: u32,
    pub db_ins_end: Option<char>,
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DBType {
    GB,
    NORINE,
    PDB,
    UNP,
}

impl DBType {
    pub fn new(str: &str) -> Self {
        match str.trim() {
            "GB" => DBType::GB,
            "NORINE" | "NOR" => DBType::NORINE,
            "PDB" => DBType::PDB,
            "UNP" => DBType::UNP,
            _ => panic!("Unknown DBType"),
        }
    }
}

impl DBRefRecord {
    pub fn new(str: &str) -> Self {
        let database = DBType::new(&str[26..32]);
        let mut i_dbns_beg: Option<char> = None;
        let mut db_ins_end: Option<char> = None;
        if let DBType::PDB = database {
            i_dbns_beg = Some(str.chars().nth(60).unwrap());
            db_ins_end = Some(str.chars().nth(67).unwrap());
        }
        DBRefRecord {
            id_code: str[7..11].trim().to_string(),
            chain_id: str.chars().nth(12).unwrap(),
            seq_begin: str[14..18].trim().parse().unwrap(),
            insert_begin: str[18..19].trim().parse().ok(),
            seq_end: str[20..24].trim().parse().unwrap(),
            insert_end: str[24..25].trim().parse().ok(),
            database,
            db_accession: str[33..41].trim().to_string(),
            db_id_code: str[42..54].trim().to_string(),
            db_seq_begin: str[55..60].trim().parse().unwrap(),
            i_dbns_beg,
            db_seq_end: str[62..67].trim().parse().unwrap(),
            db_ins_end,
        }
    }
}

impl From<&str> for DBRefRecord {
    fn from(line: &str) -> Self {
        Self::new(line)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_dbref() {
        let line = "DBREF  2JHQ A    1   226  UNP    Q9KPK8   UNG_VIBCH        1    226";
        let record = DBRefRecord::new(line);
        assert_eq!("2JHQ", record.id_code);
        assert_eq!('A', record.chain_id);
        assert_eq!(1, record.seq_begin);
        assert_eq!(None, record.insert_begin);
        assert_eq!(226, record.seq_end);
        assert_eq!(None, record.insert_end);
        assert_eq!(DBType::UNP, record.database);
        assert_eq!("Q9KPK8", record.db_accession);
        assert_eq!("UNG_VIBCH", record.db_id_code);
        assert_eq!(1, record.db_seq_begin);
        assert_eq!(None, record.i_dbns_beg);
        assert_eq!(226, record.db_seq_end);
        assert_eq!(None, record.db_ins_end);
        let line = "DBREF  2JHQ A    1   226  PDB    Q9KPK8   UNG_VIBCH        1A   226B";
        let record = DBRefRecord::new(line);
        assert_eq!("2JHQ", record.id_code);
        assert_eq!('A', record.chain_id);
        assert_eq!(1, record.seq_begin);
        assert_eq!(None, record.insert_begin);
        assert_eq!(226, record.seq_end);
        assert_eq!(None, record.insert_end);
        assert_eq!(DBType::PDB, record.database);
        assert_eq!("Q9KPK8", record.db_accession);
        assert_eq!("UNG_VIBCH", record.db_id_code);
        assert_eq!(1, record.db_seq_begin);
        assert_eq!(Some('A'), record.i_dbns_beg);
        assert_eq!(226, record.db_seq_end);
        assert_eq!(Some('B'), record.db_ins_end);
    }
}
