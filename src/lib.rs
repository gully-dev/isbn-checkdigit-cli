//! Check digit calculation and validation for ISBN-10 and the family of
//! retail barcodes (UPC-A, EAN-8, EAN-13, ISBN-13) that all share the GS1
//! algorithm. See the `isbn10` and `gs1` modules for the two algorithms.

pub mod gs1;
pub mod isbn10;
