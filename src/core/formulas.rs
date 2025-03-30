struct Cell {
    value: i32,
    formula: Formula,
    dependents: HashSet<CellRef>,
    is_error: bool,
}
