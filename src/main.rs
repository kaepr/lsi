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

fn main() {
    println!("Hello, world!");
}
