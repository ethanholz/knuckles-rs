pub mod anisotropic;
pub mod atom;
pub mod connect;
pub mod crystal;
pub mod dbref;
pub mod het;
pub mod hetnam;
pub mod model;
pub mod modres;
pub mod mtrixn;
pub mod nummdl;
pub mod origxn;
pub mod scalen;
pub mod seqadv;
pub mod seqres;
pub mod term;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "python", pyclass)]
pub enum Record {
    Anisou(anisotropic::AnisotropicRecord),
    Atom(atom::AtomRecord),
    Connect(connect::ConnectRecord),
    Crystal(crystal::CrystalRecord),
    DBRef(dbref::DBRefRecord),
    Het(het::HetRecord),
    Hetatm(atom::AtomRecord),
    Hetnam(hetnam::HetnamRecord),
    Nummdl(nummdl::NummdlRecord),
    MtrixN(mtrixn::MtrixN),
    Model(model::ModelRecord),
    Modres(modres::ModresRecord),
    OrigxN(origxn::OrigxN),
    ScaleN(scalen::ScaleN),
    Seqres(seqres::SeqresRecord),
    Seqadv(seqadv::SeqAdvRecord),
    Term(term::TermRecord),
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
