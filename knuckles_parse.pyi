from enum import Enum
from typing import Optional, Union

class AtomRecord:
    """A class to represent an Atom Record in a PDB file.

    Parameters
    ----------
    line : str
    A string representing the line in the PDB file.

    Attributes
    ----------
    serial : int
        The serial number of the atom.
    name : str
        The name of the atom.
    alt_loc : str
        The alternate location indicator.
    res_name : str
        The name of the residue.
    chain_id : str
        The chain identifier.
    res_seq : int
        The sequence number of the residue.
    i_code : str
        The insertion code.
    x : float
        The x-coordinate of the atom.
    y : float
        The y-coordinate of the atom.
    z : float
        The z-coordinate of the atom.
    occupancy : float
        The occupancy of the atom.
    temp_factor : float
        The temperature factor of the atom.
    element : Optional[str]
        The element symbol.
    charge : Optional[str]
        The charge on the atom.
    entry : Optional[str]
        The entry in the PDB file.

    """

    serial: int
    name: str
    alt_loc: Optional[str]
    res_name: str
    chain_id: Optional[str]
    res_seq: int
    i_code: Optional[str]
    x: float
    y: float
    z: float
    occupancy: float
    temp_factor: float
    element: Optional[str]
    charge: Optional[str]
    entry: Optional[str]

    def __repr__(self) -> str: ...

class AnisotropicRecord:
    """A class to represent an Anisotropic Record in a PDB file.

    Parameters
    ----------
    line : str
        A string representing a line in the PDB file.

    Attributes
    ----------
    serial : int
        The serial number of the atom.
    name : str
        The name of the atom.
    alt_loc : str
        The alternate location indicator.
    res_name : str
        The name of the residue.
    chain_id : str
        The chain identifier.
    res_seq : int
        The sequence number of the residue.
    i_code : str
        The insertion code.
    u00 : int
        The U(1,1) element of the anisotropic temperature factor.
    u11 : int
        The U(2,2) element of the anisotropic temperature factor.
    u22 : int
        The U(3,3) element of the anisotropic temperature factor.
    u01 : int
        The U(1,2) element of the anisotropic temperature factor.
    u02 : int
        The U(1,3) element of the anisotropic temperature factor.
    u12 : int
        The U(2,3) element of the anisotropic temperature factor.
    element : Optional[str]
        The element symbol.
    charge : Optional[str]
        The charge on the atom.

    """

    serial: int
    name: str
    alt_loc: Optional[str]
    res_name: str
    chain_id: Optional[str]
    res_seq: int
    i_code: Optional[str]
    u00: int
    u11: int
    u22: int
    u01: int
    u02: int
    u12: int
    element: Optional[str]
    charge: Optional[str]

    def __repr__(self) -> str: ...

class ConnectRecord:
    """A class to represent a Connect Record in a PDB file.

    Parameters
    ----------
    line : str
        A string representing a line in the PDB file.

    Attributes
    ----------
    serial : int
        The base serial number of an atom.
    connected : list[Optional[int]]
        A list of serial numbers of atoms that are connected to the atom with the given serial number.

    """

    serial: int
    connected: list[Optional[int]]

    def __repr__(self) -> str: ...

class CrystalRecord:
    """A class to represent a Crystal Record in a PDB file.

    Parameters
    ----------
    line : str
        A string representing a line in the PDB file.

    Attributes
    ----------
    a : float
        The length of the unit cell edge a. (Angstroms)
    b : float
        The length of the unit cell edge b. (Angstroms)
    c : float
        The length of the unit cell edge c. (Angstroms)
    alpha : float
        The angle between the unit cell edge b and c. (Degrees)
    beta : float
        The angle between the unit cell edge a and c. (Degrees)
    gamma : float
        The angle between the unit cell edge a and b. (Degrees)
    space_group : str
        The space group symbol.
    z : int
        The number of molecules in the unit cell. (Z value)

    """

    a: float
    b: float
    c: float
    alpha: float
    beta: float
    gamma: float
    space_group: str
    z: int

    def __repr__(self) -> str: ...

class DBRefRecord:
    """A class to represent a DBREF Record in a PDB file.

    Parameters
    ----------
    line : str
        A string representing a line in the PDB file.

    Attributes
    ----------
    id_code : str
        ID code of this entry.
    chain_id : char
        Chain identifier.
    seq_begin : int
        Initial sequence number of the PDB sequence segment.
    insert_begin : Optional[char]
        Initial insertion code of the PDB sequence segment.
    seq_end : int
        Ending sequence number of the PDB sequence segment.
    insert_end : Optional[char]
        Ending insertion code of the PDB sequence segment.
    database : DBType
        The database name.
    db_accession : str
        Accession code of the database.
    db_id_code : str
        ID code of the database.
    db_seq_begin : int
        Initial sequence number of the database sequence segment.
    i_dbns_beg : Optional[char]
        Initial insertion code of the database sequence segment.
    db_seq_end : int
        Ending sequence number of the database sequence segment.
    db_ins_end : Optional[char]
        Ending insertion code of the database sequence segment.

    """

    id_code: str
    chain_id: str
    seq_begin: int
    insert_begin: Optional[str]
    seq_end: int
    insert_end: Optional[str]
    database: DBType
    db_accession: str
    db_id_code: str
    db_seq_begin: int
    i_dbns_beg: Optional[str]
    db_seq_end: int
    db_ins_end: Optional[str]

    def __repr__(self) -> str: ...

class DBType(Enum):
    """An enumeration to represent the database type in a DBREF Record."""

    GB = ...
    NORINE = ...
    PDB = ...
    UNP = ...

    def __repr__(self) -> str: ...

class HetRecord:
    """A class to represent a HET Record in a PDB file.

    Parameters
    ----------
    line : str
        A string representing a line in the PDB file.

    Attributes
    ----------
    het_id : str
        The heterogen identifier.
    chain_id : str
        The chain identifier.
    seq_num : int
        The sequence number.
    i_code : Option[str]
        The insertion code.
    num_het_atoms: int
        The number of HETATM records for the group.
    text: Optional[str]
        The text.

    """

    het_id: str
    chain_id: str
    seq_num: int
    i_code: Optional[str]
    num_het_atoms: int
    text: Optional[str]

    def __repr__(self) -> str: ...

# pub struct HetnamRecord {
#     pub continuation: Option<String>,
#     pub het_id: String,
#     pub text: String,
# }
class HetnamRecord:
    """A class to represent a HETNAM Record in a PDB file.

    Parameters
    ----------
    line : str
        A string representing a line in the PDB file.

    Attributes
    ----------
    continuation : Optional[str]
        The continuation line.
    het_id : str
        The heterogen identifier.
    text : str
        The text.

    """

    continuation: Optional[str]
    het_id: str
    text: str

    def __repr__(self) -> str: ...

class ModelRecord:
    """A class to represent a Model Record in a PDB file.

    Parameters
    ----------
    line : str
        A string representing a line in the PDB file.

    Attributes
    ----------
    serial_number : int
        The serial number of the model.

    """

    serial_number: int

    def __repr__(self) -> str: ...

class ModresRecord:
    """A class to represent a MODRES Record in a PDB file.

    Parameters
    ----------
    line : str
        A string representing a line in the PDB file.

    Attributes
    ----------
    id_code : str
        ID code of this entry.
    res_name : str
        Residue name used in the PDB file.
    chain_id : str
        Chain identifier.
    seq_num : int
        Sequence number.
    i_code : Optional[str]
        Insertion code.
    std_res : str
        Standard residue name.
    comment : str
        Comment.

    """

    id_code: str
    res_name: str
    chain_id: str
    seq_num: int
    i_code: Optional[str]
    std_res: str
    comment: str

    def __repr__(self) -> str: ...

class MtrixnRecord:
    """A class to represent an MTRIXn Record in a PDB file.

    Parameters
    ----------
    line : str
        A string representing a line in the PDB file.

    Attributes
    ----------
    n : int
        N value of the MTRIXn record. (n = 1, 2, 3)
    serial_number : int
        Serial number of the transformation matrix.
    matrix : list[float]
        The transformation matrix.
    vn : float
        The vector.
    i_given : bool
        The I given flag.

    """

    n: int
    serial_number: int
    matrix: list[float]
    vn: float
    i_given: bool

    def __repr__(self) -> str: ...

class NummdlRecord:
    """A class to represent a NUMMDL Record in a PDB file.

    Parameters
    ----------
    line : str
        A string representing a line in the PDB file.

    Attributes
    ----------
    num_models : int
        The number of models.

    """

    num_models: int

    def __repr__(self) -> str: ...

class OrigxnRecord:
    """A class to represent an ORIGXn Record in a PDB file.

    Parameters
    ----------
    line : str
        A string representing a line in the PDB file.

    Attributes
    ----------
    n : int
        N value of the ORIGXn record. (n = 1, 2, 3)
    o : list[float]
        The origin.
    tn: float
        The translation.

    """

    n: int
    o: list[float]
    tn: float

    def __repr__(self) -> str: ...

class ScalenRecord:
    """A class to represent a SCALEn Record in a PDB file.

    Parameters
    ----------
    line : str
        A string representing a line in the PDB file

    Attributes
    ----------
    n : int
        N value of the SCALEn record. (n = 1, 2, 3)
    scalen : list[float]
        The scaling matrix.
    un : float
        The unit.

    """

    n: int
    scalen: list[float]
    un: float

    def __repr__(self) -> str: ...

class SeqAdvRecord:
    """A class to represent a SEQADV Record in a PDB file.

    Parameters
    ----------
    line : str
        A string representing a line in the PDB file.

    Attributes
    ----------
    id_code : str
        ID code of this entry.
    res_name : str
        Residue name used in the PDB file.
    chain_id : str
        Chain identifier.
    seq_num : int
        Sequence number.
    i_code : Optional[str]
        Insertion code.
    database : DBType
        Database name.
    db_accession : str
        Accession code of the database.
    db_res : Optional[str]
        Residue name in the database.
    db_seq : Optional[int]
        Sequence number in the database.
    conflict : str
        Conflict comment.

    """

    id_code: str
    res_name: str
    chain_id: str
    seq_num: int
    i_code: Optional[str]
    database: DBType
    db_accession: str
    db_res: Optional[str]
    db_seq: Optional[int]
    conflict: str

    def __repr__(self) -> str: ...

class SeqresRecord:
    """A class to represent a SEQRES Record in a PDB file.

    Parameters
    ----------
    line : str
        A string representing a line in the PDB file.

    Attributes
    ----------
    ser_num : int
        Serial number of the SEQRES record.
    chain_id : str
        Chain identifier.
    num_res : int
        Number of residues in the chain.
    res_names : list[str]
        List of residue names.

    """

    ser_num: int
    chain_id: str
    num_res: int
    res_names: list[str]
    res_names: list[str]

    def __repr__(self) -> str: ...

class TermRecord:
    """A class to represent a TER Record in a PDB file.

    Parameters
    ----------
    line : str
        A string representing a line in the PDB file.

    Attributes
    ----------
    serial : int
        The serial number of the atom.

    """

    serial: int

    def __repr__(self) -> str: ...

class Record:
    """A class to represents a single line in a PDB file.

    Wraps around the specific record types like AtomRecord, AnisotropicRecord, etc.

    Parameters
    ----------
    line : str
        A string representing the line in the PDB file.

    """

    @property
    def record(
        self,
    ) -> Union[
        AnisotropicRecord,
        AtomRecord,
        ConnectRecord,
        CrystalRecord,
        DBRefRecord,
        HetRecord,
        HetnamRecord,
        MtrixnRecord,
        ModelRecord,
        ModresRecord,
        NummdlRecord,
        OrigxnRecord,
        ScalenRecord,
        SeqresRecord,
        SeqAdvRecord,
        TermRecord,
        None,
    ]:
        """Getter method that returns the specific variant of the Record.

        Returns
        -------
        Union[
                AnisotropicRecord, AtomRecord, ConnectRecord, CrystalRecord,
                DBRefRecord, HetRecrd, MtrixnRecord, ModelRecord, ModresRecord,
                NummdlRecord, OrigxnRecord, ScalenRecord, SeqresRecord, SeqAdvRecord,
                TermRecord, HetnamRecord, None]
            The specific variant of the Record.

        """
        ...

def pdbreader(contents: str) -> list[Record]:
    """Read the contents of a PDB file.

    Parameters
    ----------
    contents : str
        The contents of the PDB file.

    Returns
    -------
    list[Record]
        A list of Record objects.

    """
    ...
