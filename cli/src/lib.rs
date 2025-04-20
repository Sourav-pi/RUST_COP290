//! Command-line interface for the spreadsheet application.
//!
//! This module provides a terminal-based interface for interacting with the spreadsheet.
//! It supports operations such as:
//! - Viewing spreadsheet data
//! - Entering formulas and values into cells
//! - Navigating through the spreadsheet using keyboard commands
//! - Scrolling to specific cells

use cores::Sheet;
use cores::convert_to_index;
use std::io;
use std::io::Write;
use cores::Error;
use std::cmp;
use std::env;

/// Controls debug output throughout the application
const DEBUG: bool = false;

/// Converts a numeric column index to an alphabetic column name.
///
/// This function transforms column indices (1-based) into Excel-style
/// column identifiers (A, B, ..., Z, AA, AB, etc.).
///
/// # Parameters
/// * `col` - The column index to convert (1-based)
///
/// # Returns
/// A string representing the column name
pub fn column_to_letter(col: usize) -> String {
    let mut result = String::new();
    let mut col_num = col;

    while col_num > 0 {
        let remainder = (col_num - 1) % 26;
        result.insert(0, (b'A' + remainder as u8) as char);
        col_num = (col_num - 1) / 26;
    }

    result
}

/// Displays a portion of the spreadsheet in the terminal.
///
/// This function prints a 10x10 section of the spreadsheet starting from
/// the specified row and column indices. It formats the output as a table
/// with row and column headers.
///
/// # Parameters
/// * `sheet` - Reference to the spreadsheet to display
/// * `row` - Total number of rows in the spreadsheet
/// * `col` - Total number of columns in the spreadsheet
/// * `rowi` - Starting row index for display
/// * `coli` - Starting column index for display
fn display_sheet(sheet: &Sheet, row: usize, col: usize, rowi: usize, coli: usize) {
    let mut i = coli;
    print!(" \t ");
    while i < coli + 10 && i < col {
        print!("{}\t ", column_to_letter(i));
        i += 1;
    }
    println!();
    i = rowi;
    while i < rowi + 10 && i < row {
        print!("{}\t ", i);
        let mut j = coli;
        while j < coli + 10 && j < col {
            let value = sheet.get_value(i as i32, j as i32);
            if sheet.grid[i][j].formula.flag.is_div_by_zero() == 1 {
                print!("ERR\t ");
            } else {
                print!("{}\t ", value);
            }
            j += 1;
        }
        println!();
        i += 1;
    }
}

/// Main function implementing the command-line interface for the spreadsheet.
///
/// The CLI supports the following commands:
/// - `w`: Move up (display previous 10 rows)
/// - `a`: Move left (display previous 10 columns)
/// - `s`: Move down (display next 10 rows)
/// - `d`: Move right (display next 10 columns)
/// - `scroll_to <cell>`: Jump to the specified cell location
/// - `disable_output`: Stop displaying the spreadsheet after each command
/// - `enable_output`: Resume displaying the spreadsheet after each command
/// - `q`: Quit the application
/// - `<cell>=<formula>`: Set a formula for the specified cell
///
/// # Command-line Arguments
/// * First argument: Number of rows in the spreadsheet
/// * Second argument: Number of columns in the spreadsheet
pub fn run() {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <rows> <columns>", args[0]);
        std::process::exit(1);
    }
    let int1: i32 = args[1].parse().expect("Invalid integer for rows");
    let int2: i32 = args[2].parse().expect("Invalid integer for columns");
    let int1 = int1 + 1;
    let int2 = int2 + 1;

    // Initialize spreadsheet and UI state
    let mut test_sheet = Sheet::new(int1 as usize, int2 as usize);
    let mut rowi = 1;
    let mut coli = 1;
    let mut input = String::new();
    let mut display_button = true;
    let mut massage = "ok";
    let mut time = 0.0;
    
    // Main input loop
    while {
        if display_button {
            display_sheet(
                &test_sheet,
                int1 as usize,
                int2 as usize,
                rowi as usize,
                coli as usize,
            );
        }
        print!("[{time}] ({}) > ", massage);
        massage = "ok";
        io::stdout().flush().unwrap();

        input.clear(); // Clear previous input
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input_trimmed = input.trim();
        input_trimmed != "q" // Continue until 'q' is entered
    } {
        let trimmed = input.trim();
        if DEBUG {
            println!("{}", trimmed);
        }
        
        // Handle cell assignment (e.g., "A1=42" or "B2=A1+C3")
        if trimmed.contains("=") {
            if DEBUG {
                println!("This is an assignment: {}", trimmed);
            }
            // Split the assignment into left-hand side (lhs) and right-hand side (rhs)
            let parts: Vec<&str> = trimmed.split('=').collect();
            if parts.len() == 2 {
                let lhs = parts[0].trim(); // e.g., A1
                let rhs = parts[1].trim(); // e.g., A2+A3
                if DEBUG {
                    println!("Left: {}, Right: {}", lhs, rhs);
                }
                // Convert the cell reference to indices
                let (cell_index_row, cell_index_col) = convert_to_index(lhs.to_string());
                let result =
                    test_sheet.update_cell_data(cell_index_row, cell_index_col, rhs.to_string());
                time = result.time;
                match result.error {
                    Error::InvalidInput => massage = "invalid input",
                    Error::None => massage = "ok",
                    Error::CycleDetected => massage = "cycle detected",
                    Error::DivByZero => massage = "ok",
                }
            } else {
                massage = "invalid input";
            }
        } 
        // Handle navigation commands
        else if trimmed == "w" {
            rowi = cmp::max(1, rowi - 10);
        } else if trimmed == "s" {
            rowi = cmp::min(int1 - 10, rowi + 10);
        } else if trimmed == "a" {
            coli = cmp::max(1, coli - 10);
        } else if trimmed == "d" {
            coli = cmp::min(int2 - 10, coli + 10);
        } 
        // Handle display toggle commands
        else if trimmed == "disable_output" {
            display_button = false
        } else if trimmed == "enable_output" {
            display_button = true
        } 
        // Handle scroll_to command
        else if trimmed.len() > 9 && &trimmed[0..9] == "scroll_to" {
            let parts: Vec<&str> = trimmed.split(' ').collect();
            if parts.len() == 2 {
                let (scroll_row, scroll_col) = convert_to_index(parts[1].to_string());
                if ((scroll_row as i32) < int1)
                    && ((scroll_col as i32) < int2)
                    && (scroll_row as i32 >= 1)
                    && (scroll_col as i32 >= 1)
                {
                    rowi = scroll_row as i32;
                    coli = scroll_col as i32;
                    rowi = cmp::min(rowi, int1);
                    coli = cmp::min(coli, int2);
                } else {
                    massage = "invalid input";
                }
            } else {
                massage = "invalid input";
            }
        } else {
            massage = "invalid input";
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cores::Sheet;
    use cores::convert_to_index;

    #[test]
    fn test_column_to_letter() {
        assert_eq!(column_to_letter(1), "A");
        assert_eq!(column_to_letter(26), "Z");
        assert_eq!(column_to_letter(27), "AA");
        assert_eq!(column_to_letter(28), "AB");
        assert_eq!(column_to_letter(52), "AZ");
        assert_eq!(column_to_letter(53), "BA");
    }

    #[test]
    fn test_convert_to_index() {
        let (row, col) = convert_to_index("A1".to_string());
        assert_eq!(row, 1);
        assert_eq!(col, 1);
    }
    #[test]
    fn test_display_sheet() {
        let mut sheet = Sheet::new(20, 20);
        sheet.grid[1][1].value = 9;
        sheet.grid[1][2].value = 8;
        sheet.grid[1][3].value = 7;
        sheet.grid[1][4].value = 6;
        sheet.grid[1][5].value = 5;
        display_sheet(&sheet, 20, 20, 1, 1);
    }
}