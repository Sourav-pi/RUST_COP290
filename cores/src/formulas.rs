use std::mem;
use std::collections::HashSet;
mod sheet;
use sheet::{Cell, Formula, CellRef, Operation, FunctionType, RangeRef};

// struct Cell {
//     value: i32,
//     formula: Formula,
//     dependents: HashSet<CellRef>,
//     is_error: bool,
// }
// Modified functions to match the Cell structure from sheet.rs

// fn addition(x1: usize, y1: usize, x2: usize, y2: usize, row: usize, col: usize, arr: &Vec<Vec<Cell>>) -> i32 {
//     if x1 >= row || x2 >= row || y1 >= col || y2 >= col {
//         return 0;
//     }
    
//     return arr[x1][y1].value + arr[x2][y2].value;
// }

// fn subtraction(x1: usize, y1: usize, x2: usize, y2: usize, row: usize, col: usize, arr: &Vec<Vec<Cell>>) -> i32 {
//     if x1 >= row || x2 >= row || y1 >= col || y2 >= col {
//         return 0;
//     }
    
//     return arr[x1][y1].value - arr[x2][y2].value;
// }

// fn multiply(x1: usize, y1: usize, x2: usize, y2: usize, row: usize, col: usize, arr: &Vec<Vec<Cell>>) -> i32 {
//     if x1 >= row || x2 >= row || y1 >= col || y2 >= col {
//         return 0;
//     }
    
//     return arr[x1][y1].value * arr[x2][y2].value;
// }

// fn maximum_range(x1: usize, y1: usize, x2: usize, y2: usize, row: usize, col: usize, arr: &Vec<Vec<Cell>>, tgt: &mut Cell) -> i32 {
//     let mut max = arr[x1][y1].value;
    
//     for i in x1..=x2 {
//         for j in y1..=y2 {
//             if arr[i][j].is_error {
//                 tgt.is_error = true;
//                 return 0;
//             }
            
//             if max < arr[i][j].value {
//                 max = arr[i][j].value;
//             }
//         }
//     }
    
//     return max;
// }

// fn minimum_range(x1: usize, y1: usize, x2: usize, y2: usize, row: usize, col: usize, arr: &Vec<Vec<Cell>>, tgt: &mut Cell) -> i32 {
//     let mut min = arr[x1][y1].value;
    
//     for i in x1..=x2 {
//         for j in y1..=y2 {
//             if arr[i][j].is_error {
//                 tgt.is_error = true;
//                 return 0;
//             }
            
//             if min > arr[i][j].value {
//                 min = arr[i][j].value;
//             }
//         }
//     }
    
//     return min;
// }

// fn sum_range(x1: usize, y1: usize, x2: usize, y2: usize, row: usize, col: usize, arr: &Vec<Vec<Cell>>, tgt: &mut Cell) -> i32 {
//     let mut sum = 0;
    
//     for i in x1..=x2 {
//         for j in y1..=y2 {
//             if arr[i][j].is_error {
//                 tgt.is_error = true;
//                 return 0;
//             }
            
//             sum += arr[i][j].value;
//         }
//     }
    
//     return sum;
// }

// fn avg_range(x1: usize, y1: usize, x2: usize, y2: usize, row: usize, col: usize, arr: &Vec<Vec<Cell>>, tgt: &mut Cell) -> i32 {
//     let freq = (x2 - x1 + 1) * (y2 - y1 + 1);
//     return sum_range(x1, y1, x2, y2, row, col, arr, tgt) / freq as i32;
// }

// fn stdev(x1: usize, y1: usize, x2: usize, y2: usize, row: usize, col: usize, arr: &Vec<Vec<Cell>>, tgt: &mut Cell) -> i32 {
//     let mean = avg_range(x1, y1, x2, y2, row, col, arr, tgt);
//     let mut sum_of_sq = 0.0;
//     let n = (x2 - x1 + 1) * (y2 - y1 + 1);
    
//     for i in x1..=x2 {
//         for j in y1..=y2 {
//             if arr[i][j].is_error {
//                 tgt.is_error = true;
//                 return 0;
//             }
            
//             sum_of_sq += (arr[i][j].value - mean).pow(2) as f64;
//         }
//     }
    
//     sum_of_sq /= n as f64;
//     return (sum_of_sq.sqrt()).round() as i32;
// }

// fn max(a: i32, b: i32) -> i32 {
//     if a >= b {
//         return a;
//     }
    
//     return b;
// }

// fn min(a: i32, b: i32) -> i32 {
//     if a <= b {
//         return a;
//     }
    
//     return b;
// }

fn main(){
    println!("{}",std::mem::size_of::<Cell>());

}
