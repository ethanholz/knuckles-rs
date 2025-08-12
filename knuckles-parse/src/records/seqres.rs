#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
use knuckles_macro::pydefault;

/// Represents a SEQRES record containing sequence information.
///
/// SEQRES records contain the amino acid or nucleic acid sequence of residues
/// in each chain of the macromolecule. Multiple SEQRES records may be used
/// for a single chain if the sequence is long.
///
/// # Fields
///
/// - `ser_num`: Serial number of the SEQRES record for the current chain
/// - `chain_id`: Chain identifier
/// - `num_res`: Total number of residues in the chain
/// - `res_names`: List of residue names in this record
///
/// # Example
///
/// ```rust
/// use knuckles_parse::records::seqres::SeqresRecord;
///
/// let line = "SEQRES   1 A  147  THR SER ASN PHE ALA ASP GLY LYS ASP ALA ILE LEU GLU";
/// let seqres = SeqresRecord::from(line);
/// assert_eq!(seqres.chain_id, 'A');
/// assert_eq!(seqres.num_res, 147);
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "python", pyclass(get_all, set_all))]
#[cfg_attr(feature = "python", pydefault)]
pub struct SeqresRecord {
    /// Serial number of this SEQRES record for the current chain
    pub ser_num: u32,
    /// Chain identifier
    pub chain_id: char,
    /// Total number of residues in the chain
    pub num_res: i16,
    /// List of residue names in this SEQRES record
    pub res_names: Vec<String>,
}

impl SeqresRecord {
    /// Create a new SeqresRecord by parsing a SEQRES line.
    pub fn new(str: &str) -> Self {
        Self {
            ser_num: str[7..10].trim().parse().unwrap(),
            chain_id: str.chars().nth(11).unwrap(),
            num_res: str[13..17].trim().parse().unwrap(),
            res_names: str[19..]
                .split_whitespace()
                .map(|s| s.to_string())
                .collect(),
        }
    }
}

impl From<&str> for SeqresRecord {
    fn from(line: &str) -> Self {
        Self::new(line)
    }
}
