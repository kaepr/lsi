/// Heap image to be loaded by interpreter
const IMAGEFILE: &str = "ls9.image";
/// Source file to load, when no default image exists
const IMAGESRC: &str = "ls9.ls9";
/// Number of nodes
const NNODES: usize = 262144;
/// Number of vector cells
const NVCELLS: usize = 262144;
/// Maximum I/O ports that can be opened
const NPORTS: usize = 20;
/// Maximum token length
const TOKLEN: usize = 80;
/// Growth delta for internal objects
const CHUNKSIZE: usize = 1024;
/// Maximum number of recursive calls to macro expander
const MXMAX: usize = 2000;
/// Number of references to print in case of errors.
const NTRACE: usize = 10;
/// Maximum depth of structure for printer.
const PRDEPTH: usize = 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Cell(i32);

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// struct TypeTag(i32);

#[repr(i32)]
enum TypeTag {
    Bytecode = -10,
    Catchtag = -11,
    Char = -12,
    Closure = -13,
    Fixnum = -14,
    Inport = -15,
    Outport = -16,
    String = -17,
    Symbol = -18,
    Vector = -19
}

impl From<TypeTag> for Cell {
    fn from(type_tag: TypeTag) -> Self {
        Cell(type_tag as i32)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecialObject {
    Nil = -1,
    True = -2,
    EofMark = -3,
    Undef = -4,
    RParen = -5,
    Dot = -6,
}

impl From<SpecialObject> for Cell {
    fn from(object: SpecialObject) -> Self {
        Cell(object as i32)
    }
}

impl From<usize> for Cell {
    fn from(value: usize) -> Self {
        Cell(value as i32)
    }
}

impl From<Cell> for usize {
    fn from(cell: Cell) -> Self {
        cell.0 as usize
    }
}

impl Cell {
    pub fn new(value: i32) -> Self {
        Cell(value)
    }

    pub fn is_special(&self) -> bool {
        self.0 < 0
    }

    pub fn is_type_tag(&self) -> bool {
        (-19..=-10).contains(&self.0)
    }

    pub fn value(&self) -> i32 {
        self.0
    }

    pub fn as_special(&self) -> Option<SpecialObject> {
        match self.0 {
            -1 => Some(SpecialObject::Nil),
            -2 => Some(SpecialObject::True),
            -3 => Some(SpecialObject::EofMark),
            -4 => Some(SpecialObject::Undef),
            -5 => Some(SpecialObject::RParen),
            -6 => Some(SpecialObject::Dot),
            _ => None,
        }
    }

    pub fn as_type_tag(&self) -> Option<TypeTag> {
        match self.0 {
            -10 => Some(TypeTag::Bytecode),
            -11 => Some(TypeTag::Catchtag),
            -12 => Some(TypeTag::Char),
            -13 => Some(TypeTag::Closure),
            -14 => Some(TypeTag::Fixnum),
            -15 => Some(TypeTag::Inport),
            -16 => Some(TypeTag::Outport),
            -17 => Some(TypeTag::String),
            -18 => Some(TypeTag::Symbol),
            -19 => Some(TypeTag::Vector),
            _ => None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Tag(u8);

impl Tag {
    const ATOM: u8 = 0x01;
    const MARK: u8 = 0x02;
    const TRAV: u8 = 0x04;
    const VECTOR: u8 = 0x08;
    const PORT: u8 = 0x10;
    const USED: u8 = 0x20;
    const LOCK: u8 = 0x40;
    const CONST: u8 = 0x80;

    fn new() -> Self {
        Self(0)
    }

    fn with_flag(mut self, flag: u8) -> Self {
        self.0 |= flag;
        self
    }

    fn has_flag(&self, flag: u8) -> bool {
        (self.0 & flag) != 0
    }

    fn set_flag(&mut self, flag: u8) {
        self.0 |= flag;
    }

    fn clear_flag(&mut self, flag: u8) {
        self.0 &= !flag
    }

    fn raw(&self) -> u8 {
        self.0
    }
}

#[derive(Default)]
struct Node {
    car: Cell,
    cdr: Cell,
    tag: Tag,
}

impl Node {
   pub fn new(car: Cell, cdr: Cell, tag: Tag) -> Self {
       Node {
           car: car,
           cdr: cdr,
           tag: tag
       }
   }
}

struct Pool {
    nodes: Vec<Node>,
    free_list: Vec<Node>,
    vectors: Vec<Cell>,
    free_vec: Vec<Cell>,
}

impl Pool {
    fn tag(&self, n: usize) -> Tag {
        self.nodes[n].tag
    }

    fn car(&self, x: usize) -> Cell {
        self.nodes[x].car
    }

    fn cdr(&self, x: usize) -> Cell {
        self.nodes[x].cdr
    }

    fn caar(&self, x: usize) -> Cell {
        self.car(self.cdr(x).into())
    }

    fn cadr(&self, x: usize) -> Cell {
        self.car(self.cdr(x).into())
    }

    fn cddr(&self, x: usize) -> Cell {
        self.cdr(self.cdr(x).into())
    }

    fn cons() {
        unimplemented!()
    }

    fn mkatom() {
        unimplemented!()
    }

    fn cons3() {
        unimplemented!()
    }

    fn portno(&self, n: usize) -> Cell {
        self.cadr(n)
    }

    fn string(&self, n: usize) -> *const u8 {
        let idx: usize = self.cdr(n).into();
        let cell_value = self.vectors[idx].value();
        cell_value as *const *mut i32 as *const u8
    }

}

enum Opcode {
    /// Abstract machine opcodes
    ILL,
    APPLIS,
    APPLIST,
    APPLY,
    TAILAPP,
    QUOTE,
    ARG,
    REF,
    PUSH,
    PUSHTRUE,
    PUSHVAL,
    POP,
    DROP,
    JMP,
    BRF,
    BRT,
    HALT,
    CATCHSTAR,
    THROWSTAR,
    CLOSURE,
    MKENV,
    PROPENV,
    CPREF,
    CPRAG,
    ENTER,
    ENTCOL,
    RETURN,
    SETARG,
    SETREF,
    MACRO,
    /// Inlined lisp functions
    ABS,
    ALPHAC,
    ATOM,
    BITOP,
    CAAR,
    CADR,
    CAR,
    CDAR,
    CDDR,
    CDR,
    CEQUAL,
    CGRTR,
    CGTEQ,
    CHAR,
    CHARP,
    CHARVAL,
    CLESS,
    CLOSEPORT,
    CLTEQ,
    CMDLINE,
    CONC,
    CONS,
    CONSTP,
    CTAPG,
    DELETE,
    DIV,
    DOWNCASE,
    DUMPIMAGE,
    EOFP,
    EQ,
    EQUAL,
    ERROR,
    ERROR2,
    ERRPORT,
    EVAL,
    EXISTSP,
    FIXP,
    FORMAT,
    FUNP,
    GC,
    GENSYM,
    GRTR,
    GTEQ,
    INPORT,
    INPORTP,
    LESS,
    LISTSTR,
    LISTVEC,
    LOAD,
    LOWERC,
    LTEQ,
    MAX,
    MIN,
    MINUS,
    MKSTR,
    MKVEC,
    MX,
    MX1,
    NCONC,
    NEGATE,
    NRECONC,
    NULL,
    NUMERIC,
    NUMSTR,
    OBTAB,
    OPENINFILE,
    OPENOUTFILE,
    OUTPORT,
    OUTPORTP,
    PAIR,
    PEEKC,
    PLUS,
    PRIN,
    PRINC,
    QUIT,
    READ,
    READC,
    RECONC,
    REM,
    SCONC,
    SEQUAL,
    SETCAR,
    SETCDR,
    SETINPORT,
    SETOUTPORT,
    SFILL,
    SGRTR,
    SGTEQ,
    SIEQUAL,
    SIGRTR,
    SIGTEQ,
    SILESS,
    SILTEQ,
    SLESS,
    SLTEQ,
    SREF,
    SSET,
    SSIZE,
    STRINGP,
    STRLIST,
    STRNUM,
    SUBSTR,
    SUBVEC,
    SYMBOL,
    SYMBOLP,
    SYMNAME,
    SYMTAB,
    SYSCMD,
    TIMES,
    UNTAG,
    UPCASE,
    UPPERC,
    VCONC,
    VECLIST,
    VECTORP,
    VFILL,
    VREF,
    VSET,
    VSIZE,
    WHITEC,
    WRITEC
}

fn main() {
    println!("Hello, world!");
}
