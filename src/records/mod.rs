pub mod anisotropic;
pub mod atom;
pub mod connect;
pub mod crystal;
pub mod dbref;
pub mod model;
pub mod modres;
pub mod mtrixn;
pub mod origxn;
pub mod scalen;
pub mod seqres;
pub mod term;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Record {
    Anisou(anisotropic::AnisotropicRecord),
    Atom(atom::AtomRecord),
    Connect(connect::ConnectRecord),
    Crystal(crystal::CrystalRecord),
    DBRef(dbref::DBRefRecord),
    Hetatm(atom::AtomRecord),
    MtrixN(mtrixn::MtrixN),
    Model(model::ModelRecord),
    Modres(modres::ModresRecord),
    OrigxN(origxn::OrigxN),
    ScaleN(scalen::ScaleN),
    Seqres(seqres::SeqresRecord),
    Term(term::TermRecord),
    Endmdl(),
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
                "MTRIX1" | "MTRIX2" | "MTRIX3" => Ok(Record::MtrixN(mtrixn::MtrixN::from(line))),
                "MODEL " => Ok(Record::Model(model::ModelRecord::from(line))),
                "MODRES" => Ok(Record::Modres(modres::ModresRecord::from(line))),
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
            Record::MtrixN(mtrix) => write!(f, "{:?}", mtrix),
            Record::Model(model) => write!(f, "{:?}", model),
            Record::Modres(modres) => write!(f, "{:?}", modres),
            Record::OrigxN(origxn) => write!(f, "{:?}", origxn),
            Record::ScaleN(scalen) => write!(f, "{:?}", scalen),
            Record::Seqres(seqres) => write!(f, "{:?}", seqres),
            Record::Term(term) => write!(f, "{:?}", term),
        }
    }
}

// impl From<&str> for Record {
//     fn from(line: &str) -> Self {
//         Self::try_from(line).unwrap()
//     }
// }
