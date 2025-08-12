//! PDB record type definitions and parsers.
//!
//! This module contains all the supported PDB record types and their parsing implementations.
//! Each record type has its own submodule with specific parsing logic and data structures.

/// Anisotropic temperature factor records (ANISOU)
pub mod anisotropic;
/// Atom coordinate records (ATOM/HETATM)
pub mod atom;
/// Connectivity records (CONECT)
pub mod connect;
/// Crystal structure parameters (CRYST1)
pub mod crystal;
/// Database reference records (DBREF)
pub mod dbref;
/// Hetero-compound records (HET)
pub mod het;
/// Hetero-compound name records (HETNAM)
pub mod hetnam;
/// Model records (MODEL)
pub mod model;
/// Modified residue records (MODRES)
pub mod modres;
/// Matrix transformation records (MTRIX1/2/3)
pub mod mtrixn;
/// Number of models records (NUMMDL)
pub mod nummdl;
/// Original coordinate system transformation records (ORIGX1/2/3)
pub mod origxn;
/// Scale matrix records (SCALE1/2/3)
pub mod scalen;
/// Sequence differences records (SEQADV)
pub mod seqadv;
/// Residue sequence records (SEQRES)
pub mod seqres;
/// Chain termination records (TER)
pub mod term;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "python")]
use pyo3::prelude::*;

/// Represents all supported PDB record types.
///
/// This enum encompasses all the different types of records that can be found in a PDB file.
/// Each variant contains the specific data structure for that record type.
///
/// # Record Types
///
/// - `Anisou` - Anisotropic temperature factor records
/// - `Atom` - Standard atom coordinate records
/// - `Connect` - Connectivity records showing bonds between atoms
/// - `Crystal` - Crystallographic unit cell parameters
/// - `DBRef` - Database reference records
/// - `Het` - Hetero-compound records
/// - `Hetatm` - Hetero-atom coordinate records (uses same structure as `Atom`)
/// - `Hetnam` - Hetero-compound name records
/// - `Nummdl` - Number of models in the file
/// - `MtrixN` - Transformation matrix records (N = 1, 2, or 3)
/// - `Model` - Model records for multi-model structures
/// - `Modres` - Modified residue records
/// - `OrigxN` - Original coordinate system transformation (N = 1, 2, or 3)
/// - `ScaleN` - Scale matrix records (N = 1, 2, or 3)
/// - `Seqres` - Residue sequence records
/// - `Seqadv` - Sequence differences from database
/// - `Term` - Chain termination records
/// - `Endmdl` - End of model marker (no associated data)
///
/// # Example
///
/// ```rust
/// use knuckles_parse::records::Record;
///
/// let line = "ATOM      1  N   ALA A   1      20.154  16.967  27.462  1.00 11.18           N";
/// let record = Record::try_from(line).unwrap();
///
/// match record {
///     Record::Atom(atom) => println!("Found atom: {}", atom.name),
///     Record::Hetatm(hetatm) => println!("Found hetatm: {}", hetatm.name),
///     _ => println!("Other record type"),
/// }
/// ```
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "python", pyclass)]
pub enum Record {
    /// Anisotropic temperature factor record
    Anisou(anisotropic::AnisotropicRecord),
    /// Standard atom coordinate record
    Atom(atom::AtomRecord),
    /// Connectivity record showing bonds between atoms
    Connect(connect::ConnectRecord),
    /// Crystallographic unit cell parameters
    Crystal(crystal::CrystalRecord),
    /// Database reference record
    DBRef(dbref::DBRefRecord),
    /// Hetero-compound record
    Het(het::HetRecord),
    /// Hetero-atom coordinate record (same structure as Atom)
    Hetatm(atom::AtomRecord),
    /// Hetero-compound name record
    Hetnam(hetnam::HetnamRecord),
    /// Number of models record
    Nummdl(nummdl::NummdlRecord),
    /// Transformation matrix record (MTRIX1, MTRIX2, or MTRIX3)
    MtrixN(mtrixn::MtrixN),
    /// Model record for multi-model structures
    Model(model::ModelRecord),
    /// Modified residue record
    Modres(modres::ModresRecord),
    /// Original coordinate system transformation (ORIGX1, ORIGX2, or ORIGX3)
    OrigxN(origxn::OrigxN),
    /// Scale matrix record (SCALE1, SCALE2, or SCALE3)
    ScaleN(scalen::ScaleN),
    /// Residue sequence record
    Seqres(seqres::SeqresRecord),
    /// Sequence differences from database
    Seqadv(seqadv::SeqAdvRecord),
    /// Chain termination record
    Term(term::TermRecord),
    /// End of model marker
    Endmdl(),
}
#[cfg(feature = "python")]
macro_rules! debug_match {
    ($self:expr, { $($variant:ident($value:ident)),* }) => {
        match $self {
            $(Self::$variant($value) => format!("{:?}", $value),)*
            Self::Endmdl() => "ENDMDL".to_string(),
        }
    };
}

#[cfg(feature = "python")]
#[pymethods]
impl Record {
    #[new]
    fn python_new(str: &str) -> Self {
        match Self::try_from(str) {
            Ok(item) => item,
            Err(_) => panic!("Unable to parse line"),
        }
    }
    #[getter]
    fn record(&self, py: Python) -> Py<PyAny> {
        match self {
            Self::Atom(atom) => atom.clone().into_pyobject(py).unwrap().into_any().into(),
            Self::Anisou(anisou) => anisou.clone().into_pyobject(py).unwrap().into_any().into(),
            Self::Connect(connect) => connect.clone().into_pyobject(py).unwrap().into_any().into(),
            Self::Crystal(crystal) => crystal.clone().into_pyobject(py).unwrap().into_any().into(),
            Self::DBRef(dbref) => dbref.clone().into_pyobject(py).unwrap().into_any().into(),
            Self::Endmdl() => py.None(),
            Self::Hetatm(atom) => atom.clone().into_pyobject(py).unwrap().into_any().into(),
            Self::Het(het) => het.clone().into_pyobject(py).unwrap().into_any().into(),
            Self::Hetnam(hetnam) => hetnam.clone().into_pyobject(py).unwrap().into_any().into(),
            Self::Nummdl(nummdl) => nummdl.clone().into_pyobject(py).unwrap().into_any().into(),
            Self::Model(model) => model.clone().into_pyobject(py).unwrap().into_any().into(),
            Self::Modres(modres) => modres.clone().into_pyobject(py).unwrap().into_any().into(),
            Self::MtrixN(mtrix) => mtrix.clone().into_pyobject(py).unwrap().into_any().into(),
            Self::OrigxN(origxn) => origxn.clone().into_pyobject(py).unwrap().into_any().into(),
            Self::ScaleN(scalen) => scalen.clone().into_pyobject(py).unwrap().into_any().into(),
            Self::Seqadv(seqadv) => seqadv.clone().into_pyobject(py).unwrap().into_any().into(),
            Self::Seqres(seqres) => seqres.clone().into_pyobject(py).unwrap().into_any().into(),
            Self::Term(term) => term.clone().into_pyobject(py).unwrap().into_any().into(),
        }
    }

    fn __repr__(&self) -> String {
        debug_match!(
            self,
            {
                Anisou(anisou),
                Atom(atom),
                Connect(connect),
                Crystal(crystal),
                DBRef(dbref),
                Het(het),
                Hetatm(atom),
                Hetnam(hetnam),
                Model(model),
                Modres(modres),
                MtrixN(mtrix),
                Nummdl(nummdl),
                OrigxN(origxn),
                ScaleN(scalen),
                Seqadv(seqadv),
                Seqres(seqres),
                Term(term)
            }

        )
    }
}

impl TryFrom<&str> for Record {
    type Error = &'static str;

    /// Parse a PDB line into the appropriate Record variant.
    ///
    /// This method examines the first 6 characters of the line to determine the record type
    /// and then delegates to the appropriate record-specific parser.
    ///
    /// # Arguments
    ///
    /// * `line` - A single line from a PDB file
    ///
    /// # Returns
    ///
    /// - `Ok(Record)` - Successfully parsed record
    /// - `Err(&'static str)` - Error message if parsing failed
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The line is shorter than 6 characters
    /// - The record type is not recognized
    /// - The line format is invalid for the detected record type
    ///
    /// # Example
    ///
    /// ```rust
    /// use knuckles_parse::records::Record;
    ///
    /// let line = "ATOM      1  N   ALA A   1      20.154  16.967  27.462  1.00 11.18           N";
    /// let record = Record::try_from(line).unwrap();
    /// ```
    fn try_from(line: &str) -> Result<Self, Self::Error> {
        match &line.get(0..6) {
            Some(item) => match *item {
                "ANISOU" => Ok(Record::Anisou(anisotropic::AnisotropicRecord::from(line))),
                "ATOM  " => Ok(Record::Atom(atom::AtomRecord::from(line))),
                "CONECT" => Ok(Record::Connect(connect::ConnectRecord::from(line))),
                "CRYST1" => Ok(Record::Crystal(crystal::CrystalRecord::from(line))),
                "DBREF " => Ok(Record::DBRef(dbref::DBRefRecord::from(line))),
                "ENDMDL" => Ok(Record::Endmdl()),
                "HETATM" => Ok(Record::Hetatm(atom::AtomRecord::from(line))),
                "HET   " => Ok(Record::Het(het::HetRecord::from(line))),
                "HETNAM" => Ok(Record::Hetnam(hetnam::HetnamRecord::from(line))),
                "MTRIX1" | "MTRIX2" | "MTRIX3" => Ok(Record::MtrixN(mtrixn::MtrixN::from(line))),
                "MODEL " => Ok(Record::Model(model::ModelRecord::from(line))),
                "MODRES" => Ok(Record::Modres(modres::ModresRecord::from(line))),
                "NUMMDL" => Ok(Record::Nummdl(nummdl::NummdlRecord::from(line))),
                "ORIGX1" | "ORIGX2" | "ORIGX3" => Ok(Record::OrigxN(origxn::OrigxN::from(line))),
                "SCALE1" | "SCALE2" | "SCALE3" => Ok(Record::ScaleN(scalen::ScaleN::from(line))),
                "SEQRES" => Ok(Record::Seqres(seqres::SeqresRecord::from(line))),
                "TER   " => Ok(Record::Term(term::TermRecord::from(line))),
                _ => Err("Unknown record type"),
            },
            None => Err("Unable to parse line"),
        }
    }
}

impl std::fmt::Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[cfg(feature = "serde")]
        match serde_json::to_string(&self) {
            Ok(item) => write!(f, "{}", item),
            Err(_) => Err(std::fmt::Error),
        }
        #[cfg(not(feature = "serde"))]
        match self {
            Record::Anisou(anisotropic) => write!(f, "{:?}", anisotropic),
            Record::Atom(atom) => write!(f, "{:?}", atom),
            Record::Connect(connect) => write!(f, "{:?}", connect),
            Record::Crystal(crystal) => write!(f, "{:?}", crystal),
            Record::DBRef(dbref) => write!(f, "{:?}", dbref),
            Record::Endmdl() => write!(f, "ENDMDL"),
            Record::Hetatm(atom) => write!(f, "{:?}", atom),
            Record::Hetnam(hetnam) => write!(f, "{:?}", hetnam),
            Record::Het(het) => write!(f, "{:?}", het),
            Record::MtrixN(mtrix) => write!(f, "{:?}", mtrix),
            Record::Model(model) => write!(f, "{:?}", model),
            Record::Modres(modres) => write!(f, "{:?}", modres),
            Record::Nummdl(nummdl) => write!(f, "{:?}", nummdl),
            Record::OrigxN(origxn) => write!(f, "{:?}", origxn),
            Record::ScaleN(scalen) => write!(f, "{:?}", scalen),
            Record::Seqres(seqres) => write!(f, "{:?}", seqres),
            Record::Seqadv(seqadv) => write!(f, "{:?}", seqadv),
            Record::Term(term) => write!(f, "{:?}", term),
        }
    }
}

// impl From<&str> for Record {
//     fn from(line: &str) -> Self {
//         Self::try_from(line).unwrap()
//     }
// }
