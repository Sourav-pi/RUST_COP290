use std::collections::HashSet;

#[derive(Debug, Clone)]
enum Formula {
    Literal(i32),
    Arithmetic(Operation, CellRef, CellRef),
    RangeFunction(FunctionType, RangeRef),
    Sleep(u32),
}

#[derive(Debug, Clone)]
struct RangeRef {
    start: (usize, usize),
    end: (usize, usize)
}

#[derive(Debug, Clone)]
enum Operation { Add, Sub, Mul, Div }
#[derive(Clone)]
struct Cell {
    value: i32,
    formula: Formula,
    dependents: HashSet<CellRef>,
    is_error: bool,
}

// Placeholder types for Formula and CellRef
struct Formula; 
struct CellRef;

fn main() {
    let rows = 10; // Number of rows
    let cols = 10; // Number of columns

    let default_cell = Cell {
        value: 0,
        formula: Formula, // Replace with actual formula initialization
        dependents: HashSet::new(),
        is_error: false,
    };

    let grid: Vec<Vec<Cell>> = vec![vec![default_cell.clone(); cols]; rows];

    println!("Spreadsheet initialized with {} rows and {} columns", rows, cols);
}
