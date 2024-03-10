#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::dbref::DBType;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SeqAdvRecord {
    pub id_code: String,
    pub res_name: String,
    pub chain_id: char,
    pub seq_num: i32,
    pub i_code: Option<char>,
    pub database: DBType,
    pub db_accession: String,
    pub db_res: Option<String>,
    pub db_seq: Option<i32>,
    pub conflict: String,
}

impl SeqAdvRecord {
    pub fn new(line: &str) -> Self {
        // let db_res = line[39..42].trim().parse::<i32>().ok();
        let database = DBType::new(&line[24..28]);
        SeqAdvRecord {
            id_code: line[7..11].trim().to_string(),
            res_name: line[12..15].trim().to_string(),
            chain_id: line.chars().nth(16).unwrap(),
            seq_num: line[18..22].trim().parse().unwrap(),
            i_code: line[22..23].trim().parse().ok(),
            database,
            db_accession: line[29..38].trim().to_string(),
            db_res: line
                .get(39..42)
                .map(|str| str.trim().to_string())
                .filter(|item| !item.is_empty()),
            db_seq: line[43..48].trim().parse().ok(),
            conflict: line[49..].trim().to_string(),
        }
    }
}

impl From<&str> for SeqAdvRecord {
    fn from(value: &str) -> Self {
        SeqAdvRecord::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_seqdv_record() {
        let line = "SEQADV 3ABC MET A   -1  UNP  P10725              EXPRESSION TAG";
        let record = SeqAdvRecord::new(line);
        assert_eq!("3ABC", record.id_code);
        assert_eq!("MET", record.res_name);
        assert_eq!('A', record.chain_id);
        assert_eq!(-1, record.seq_num);
        assert_eq!(None, record.i_code);
        assert_eq!(DBType::UNP, record.database);
        assert_eq!("P10725", record.db_accession);
        assert_eq!(None, record.db_res);
        assert_eq!(None, record.db_seq);
        assert_eq!("EXPRESSION TAG", record.conflict);

        let line = "SEQADV 3ABC GLY A   50  UNP  P10725    VAL    50 ENGINEERED";
        let record = SeqAdvRecord::new(line);
        assert_eq!("3ABC", record.id_code);
        assert_eq!("GLY", record.res_name);
        assert_eq!('A', record.chain_id);
        assert_eq!(50, record.seq_num);
        assert_eq!(None, record.i_code);
        assert_eq!(DBType::UNP, record.database);
        assert_eq!("P10725", record.db_accession);
        assert_eq!(Some("VAL".to_string()), record.db_res);
        assert_eq!(Some(50), record.db_seq);
        assert_eq!("ENGINEERED", record.conflict);

        let line = "SEQADV 2OKW LEU A   64  NOR  NOR00669  PHE    14 SEE REMARK 999";
        let record = SeqAdvRecord::new(line);
        assert_eq!("2OKW", record.id_code);
        assert_eq!("LEU", record.res_name);
        assert_eq!('A', record.chain_id);
        assert_eq!(64, record.seq_num);
        assert_eq!(None, record.i_code);
        assert_eq!(DBType::NORINE, record.database);
        assert_eq!("NOR00669", record.db_accession);
        assert_eq!(Some("PHE".to_string()), record.db_res);
        assert_eq!(Some(14), record.db_seq);
        assert_eq!("SEE REMARK 999", record.conflict);
    }
}
