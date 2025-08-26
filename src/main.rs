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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Cell(i32);

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

impl Cell {
    pub fn new(value: i32) -> Self {
        Cell(value)
    }

    pub fn is_special(&self) -> bool {
        self.0 < 0
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
}

enum Tag {}

struct Node {
    car: Cell,
    cdr: Cell,
    tag: Tag,
}

struct NodePool {
    nodes: Vec<Node>,
}

struct VectorPool {
    vectors: Vec<Cell>,
}

fn main() {
    println!("Hello, world!");
}
