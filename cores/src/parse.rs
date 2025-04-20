//! This module provides parsing functionality for spreadsheet formulas.
//! 
//! It converts text formulas into structured command representations that can be
//! executed by the spreadsheet engine. The parser handles various formula types:
//! - Simple values (e.g., "42")
//! - Cell references (e.g., "A1")
//! - Arithmetic operations (e.g., "A1+B2")
//! - Range functions (e.g., "SUM(A1:B5)")
//! - Special functions (e.g., "SLEEP(5)")

#![allow(non_snake_case)]
#![allow(unused_braces)]
#![allow(clippy::identity_op)]

use modular_bitfield::prelude::*;
use std::str;

use crate::sheet::Cell;

/// Bitfield representing the type and attributes of a spreadsheet formula command.
///
/// This structure efficiently stores various flags about a command in a single 16-bit value.
#[bitfield]
#[repr(u16)]
#[derive(Clone, Debug, serde::Serialize, Default)]
#[allow(clippy::identity_op)]
pub struct CommandFlag {
    /// Command type: 0 = value/cell, 1 = arithmetic, 2 = range function
    pub type_: B2,          // 2 bits
    /// Operation code (depends on type_):
    /// - For arithmetic: 0 = add, 1 = subtract, 2 = multiply, 3 = divide
    /// - For range functions: 0 = MIN, 1 = MAX, 2 = SUM, 3 = AVG, 4 = STDEV, 5 = SLEEP
    pub cmd: B3,            // 3 bits
    /// Parameter 1 type: 0 = value, 1 = cell reference
    pub type1: B1,          // 1 bit
    /// Parameter 2 type: 0 = value, 1 = cell reference
    pub type2: B1,          // 1 bit
    /// Error code: 0 = no error, 1 = invalid input, 2 = cycle detected
    pub error: B2,          // 2 bits
    /// Division by zero flag: 1 = division by zero occurred
    pub is_div_by_zero: B1, // 1 bit
    /// Reserved bits for future use
    pub is_any: B6,
}

/// A structure representing a parsed formula command.
///
/// This contains all the information needed to execute a formula operation
/// including the operation type, parameters, and any flags.
#[derive(Clone, serde::Serialize)]
#[derive(Debug)]
pub struct CommandCall {
    /// Flag bits indicating the command type and attributes
    pub flag: CommandFlag, // 16 bits
    /// First parameter - either a direct value or an encoded cell reference
    pub param1: i32,       // 4 bytes
    /// Second parameter - either a direct value or an encoded cell reference
    pub param2: i32,       // 4 bytes
}

// Utility functions for character checking

/// Checks if a character is a digit (0-9).
#[inline(always)]
fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

/// Checks if a character is an uppercase letter (A-Z).
#[inline(always)]
fn is_uppercase_letter(c: char) -> bool {
    c.is_ascii_uppercase()
}

/// Checks if a character is an arithmetic operator (+, -, *, /).
#[inline(always)]
fn is_operator(c: char) -> bool {
    c == '+' || c == '-' || c == '*' || c == '/'
}

/// Checks if a character is a sign (+ or -).
#[inline(always)]
fn is_sign(c: char) -> bool {
    c == '+' || c == '-'
}

/// Parses a formula string into a structured CommandCall object.
///
/// This function serves as the main entry point for the formula parser.
/// It analyzes the input string and converts it to the appropriate command structure.
///
/// # Parameters
/// * `input` - A string slice containing the formula to parse
///
/// # Returns
/// A CommandCall structure representing the parsed formula
///
/// # Example
/// ```
/// let command = parse_formula("A1+B2");
/// assert_eq!(command.flag.type_(), 1); // Arithmetic operation
/// assert_eq!(command.flag.cmd(), 0);   // Addition
/// ```
pub fn parse_formula(input: &str) -> CommandCall {
    let mut cell = CommandCall {
        flag: CommandFlag::new(),
        param1: 0,
        param2: 0,
    };

    parse_expression(input, &mut cell);
    cell
}

/// Parses a SLEEP function and populates the CommandCall structure.
///
/// The SLEEP function can take either a direct value or a cell reference
/// as its parameter.
///
/// # Parameters
/// * `input` - A string slice containing the SLEEP function (e.g., "SLEEP(5)" or "SLEEP(A1)")
/// * `container` - The CommandCall structure to populate
///
/// # Example
/// ```
/// let mut command = CommandCall { flag: CommandFlag::new(), param1: 0, param2: 0 };
/// parse_sleep("SLEEP(5)", &mut command);
/// assert_eq!(command.flag.type_(), 2);
/// assert_eq!(command.flag.cmd(), 5);
/// ```
pub fn parse_sleep(input: &str, container: &mut CommandCall) {
    container.flag.set_type_(2);
    container.flag.set_cmd(5);

    // Check if input starts with "SLEEP(") and ends with ")"
    if input.starts_with("SLEEP(") && input.ends_with(")") {
        // Extract the part between parentheses
        let sleep_time = &input[6..input.len() - 1].trim();

        // Check if it's a cell reference
        let mut is_cell_ref = true;
        let mut has_letter = false;
        let mut has_digit = false;

        for c in sleep_time.chars() {
            if is_uppercase_letter(c) {
                if has_digit {
                    is_cell_ref = false;
                    break;
                }
                has_letter = true;
                if has_digit {
                    is_cell_ref = false;
                    break;
                }
                has_letter = true;
            } else if is_digit(c) {
                has_digit = true;
            } else {
                is_cell_ref = false;
                break;
            }
        }

        if is_cell_ref && has_letter && has_digit {
            container.param1 = encode_cell(sleep_time.to_string());
            container.flag.set_type1(1);
        } else if let Ok(value) = sleep_time.parse::<i32>() {
            container.param1 = value;
            container.flag.set_type1(0);
        } else {
            container.flag.set_error(1);
        }
    } else {
        container.flag.set_error(1);
    }
}

/// Parses an arithmetic expression and populates the CommandCall structure.
///
/// Handles expressions like "A1+B2", "5*C3", etc. The function identifies
/// the operator and operands, then sets the appropriate flags and parameters.
///
/// # Parameters
/// * `input` - A string slice containing the arithmetic expression
/// * `container` - The CommandCall structure to populate
///
/// # Example
/// ```
/// let mut command = CommandCall { flag: CommandFlag::new(), param1: 0, param2: 0 };
/// Arithmatic("5+A1", &mut command);
/// assert_eq!(command.flag.type_(), 1);
/// assert_eq!(command.flag.cmd(), 0);    // Addition
/// assert_eq!(command.flag.type1(), 0);  // param1 is a value
/// assert_eq!(command.flag.type2(), 1);  // param2 is a cell reference
/// ```
pub fn Arithmatic(input: &str, container: &mut CommandCall) {
    container.flag.set_type_(1);

    // Find the operator position
    let mut op_pos = None;
    let mut in_operand = false;
    for (i, c) in input.chars().enumerate() {
        if is_operator(c) && in_operand {
            // Found an operator after seeing an operand
            op_pos = Some(i);
            break;
        } else if is_digit(c) || is_uppercase_letter(c) {
            in_operand = true;
            if is_uppercase_letter(c) {}
        } else if is_sign(c) && !in_operand {
            // Sign at the beginning is ok
            continue;
        } else if c == ' ' {
            // Skip spaces
            continue;
        } else {
            // Invalid character in the operand
            container.flag.set_error(1);
            return;
        }
    }

    if let Some(pos) = op_pos {
        let left = input[..pos].trim();
        let operator = input.chars().nth(pos).unwrap();
        let right = input[pos + 1..].trim();
        // Process left operand
        if left.chars().any(is_uppercase_letter) {
            if !left.chars().next().is_some_and(is_uppercase_letter) {
                container.flag.set_error(1);
                return;
            }
            container.param1 = encode_cell(left.to_string());
            container.flag.set_type1(1);
        } else if let Ok(value) = left.parse::<i32>() {
            container.param1 = value;
            container.flag.set_type1(0);
        } else {
            container.flag.set_error(1);
            return;
        }

        // Process operator
        let cmd = match operator {
            '+' => 0,
            '-' => 1,
            '*' => 2,
            '/' => 3,
            _ => {
                container.flag.set_error(1);
                return;
            }
        };
        container.flag.set_cmd(cmd);

        // Process right operand
        if right.chars().any(is_uppercase_letter) {
            if !right.chars().next().is_some_and(is_uppercase_letter) {
                container.flag.set_error(1);
                return;
            }
            container.param2 = encode_cell(right.to_string());
            container.flag.set_type2(1);
        } else if let Ok(value) = right.parse::<i32>() {
            container.param2 = value;
            container.flag.set_type2(0);
        } else {
            container.flag.set_error(1);
        }
    } else {
        container.flag.set_error(1);
    }
}

/// Parses a range operation (e.g., "SUM(A1:B5)") and populates the CommandCall structure.
///
/// This handles functions that operate on a range of cells, such as MIN, MAX, SUM, AVG, and STDEV.
///
/// # Parameters
/// * `input` - A string slice containing the range function expression
/// * `container` - The CommandCall structure to populate
///
/// # Example
/// ```
/// let mut command = CommandCall { flag: CommandFlag::new(), param1: 0, param2: 0 };
/// rangeoper("SUM(A1:B5)", &mut command);
/// assert_eq!(command.flag.type_(), 2);
/// assert_eq!(command.flag.cmd(), 2);    // SUM function
/// ```
pub fn rangeoper(input: &str, container: &mut CommandCall) {
    container.flag.set_type_(2);

    // Check for function pattern: FUNC(START:END)
    if !input.contains('(') || !input.contains(')') || !input.contains(':') {
        container.flag.set_error(1);
        return;
    }

    // Extract function name
    let func_end = input.find('(').unwrap();
    let func_name = &input[0..func_end];

    // Extract range
    let range_start = func_end + 1;
    let range_end = input.rfind(')').unwrap();
    let range = &input[range_start..range_end];

    // Split range into start and end cells
    let parts: Vec<&str> = range.split(':').collect();
    if parts.len() != 2 {
        container.flag.set_error(1);
        return;
    }

    let start_cell = parts[0].trim();
    let end_cell = parts[1].trim();

    // Encode cell references
    container.param1 = encode_cell(start_cell.to_string());
    container.param2 = encode_cell(end_cell.to_string());
    if container.param1 == 0 || container.param2 == 0 {
        container.flag.set_error(1);
        container.param1 = 0;
        container.param2 = 0;
        container.flag.set_type_(0);
        container.flag.set_type1(0);
        container.flag.set_type2(0);
        container.flag.set_cmd(0);
        return;
    }
    // Check if range is valid
    let (row1, col1) = convert_to_index_int(container.param1);
    let (row2, col2) = convert_to_index_int(container.param2);
    if row1 > row2 || col1 > col2 {
        container.flag.set_error(1);
        container.param1 = 0;
        container.param2 = 0;
        container.flag.set_type_(0);
        container.flag.set_type1(0);
        container.flag.set_type2(0);
        container.flag.set_cmd(0);
        return;
    }

    container.flag.set_type1(1);
    container.flag.set_type2(1);
    // Set function type
    let cmd = match func_name {
        "MIN" => 0,
        "MAX" => 1,
        "SUM" => 2,
        "AVG" => 3,
        "STDEV" => 4,
        _ => {
            container.flag.set_error(1);
            return;
        }
    };
    container.flag.set_cmd(cmd);
}

/// Parses any formula expression and determines its type.
///
/// This function serves as the dispatcher for different types of expressions:
/// - Simple values
/// - Cell references
/// - Arithmetic operations
/// - Range functions
/// - Special functions
///
/// # Parameters
/// * `input` - A string slice containing the expression to parse
/// * `container` - The CommandCall structure to populate
///
/// # Example
/// ```
/// let mut command = CommandCall { flag: CommandFlag::new(), param1: 0, param2: 0 };
/// parse_expression("42", &mut command);
/// assert_eq!(command.param1, 42);
/// assert_eq!(command.flag.type_(), 0);  // Value type
/// ```
pub fn parse_expression(input: &str, container: &mut CommandCall) {
    let trimmed = input.trim();

    // Check if input is just a number
    if !trimmed.is_empty()
        && (is_digit(trimmed.chars().next().unwrap())
            || (is_sign(trimmed.chars().next().unwrap())
                && trimmed.len() > 1
                && is_digit(trimmed.chars().nth(1).unwrap())))
        && trimmed
            .chars()
            .all(|c| is_digit(c) || (c == '-' || c == '+') && trimmed.starts_with(c))
    {
        if let Ok(value) = trimmed.parse::<i32>() {
            container.param1 = value;
            container.param2 = 0;
            container.flag.set_type_(0);
            container.flag.set_cmd(0);
            container.flag.set_type1(0);
            return;
        }
    }

    // Check for arithmetic operations
    if trimmed.contains('+')
        || trimmed.contains('-')
        || trimmed.contains('*')
        || trimmed.contains('/')
    {
        Arithmatic(trimmed, container);
        return;
    }

    // Check for range functions
    if trimmed.contains(':') {
        rangeoper(trimmed, container);
        return;
    }
    // Check for special functions
    if trimmed.starts_with("SLEEP") {
        parse_sleep(trimmed, container);
        return;
    }

    // Check if it's a cell reference
    let mut is_cell_ref = true;
    let mut has_letter = false;
    let mut has_digit = false;

    for c in trimmed.chars() {
        if is_uppercase_letter(c) {
            if has_digit {
                is_cell_ref = false;
                break;
            }
            has_letter = true;
        } else if is_digit(c) {
            has_digit = true;
        } else {
            is_cell_ref = false;
            break;
        }
    }

    if is_cell_ref && has_letter && has_digit {
        container.param1 = encode_cell(trimmed.to_string());
        container.flag.set_type_(0);
        container.flag.set_cmd(0);
        container.flag.set_type1(1);
        return;
    }

    container.flag.set_error(1);
}

/// Converts a cell reference string (e.g., "A1") to row and column indices.
///
/// # Parameters
/// * `cell` - The cell reference string to convert
///
/// # Returns
/// A tuple of (row, column) indices, where both are 1-based
///
/// # Example
/// ```
/// let (row, col) = convert_to_index("B3".to_string());
/// assert_eq!(row, 3);
/// assert_eq!(col, 2);
/// ```
pub fn convert_to_index(cell: String) -> (usize, usize) {
    let mut col_str = String::new();
    let mut row_str = String::new();
    let mut processing_col = true;

    for c in cell.chars() {
        if is_uppercase_letter(c) && processing_col {
            col_str.push(c);
        } else if is_digit(c) {
            if processing_col {
                processing_col = false;
            }
            row_str.push(c);
        } else {
            return (0, 0); // Invalid cell reference
        }
    }

    if col_str.is_empty() || row_str.is_empty() {
        return (0, 0);
    }

    // Convert column letters to number
    let mut col = 0;
    for c in col_str.chars() {
        col = col * 26 + (c as usize - 'A' as usize + 1);
    }

    // Parse row number
    match row_str.parse::<usize>() {
        Ok(row) => (row, col),
        Err(_) => (0, 0),
    }
}

/// Shift value used for encoding cell references into a single integer.
///
/// This constant determines how many columns can be represented in a sheet.
pub const ENCODE_SHIFT: usize = 100000;

/// Encodes a cell reference (e.g., "A1") into a single integer value.
///
/// # Parameters
/// * `cell` - The cell reference string to encode
///
/// # Returns
/// An integer encoding of the cell reference
///
/// # Example
/// ```
/// let encoded = encode_cell("A1".to_string());
/// assert_eq!(encoded, 100001); // 1 * ENCODE_SHIFT + 1
/// ```
pub fn encode_cell(cell: String) -> i32 {
    let (row, col) = convert_to_index(cell);
    let encoded = row * (ENCODE_SHIFT) + col;
    encoded as i32
}

/// Decodes an encoded cell reference back to a string (e.g., "A1").
///
/// # Parameters
/// * `encoded` - The encoded cell value to decode
///
/// # Returns
/// The string representation of the cell reference
///
/// # Example
/// ```
/// let decoded = decode_cell(100001);
/// assert_eq!(decoded, "A1");
/// ```
pub fn decode_cell(encoded: i32) -> String {
    let mut col = (encoded % (ENCODE_SHIFT as i32)) as usize;
    let row = (encoded / (ENCODE_SHIFT as i32)) as usize;
    let mut cell = String::new();

    if col == 0 {
        return String::new();
    }

    while col > 0 {
        let mut temp = col % 26;
        if temp == 0 {
            temp = 26;
        }
        cell.insert(0, (temp as u8 + b'A' - 1) as char);
        col = (col - temp) / 26;
    }

    cell.push_str(&row.to_string());
    cell
}

/// Converts an encoded cell reference to row and column indices.
///
/// # Parameters
/// * `encode` - The encoded cell value
///
/// # Returns
/// A tuple of (row, column) indices
///
/// # Example
/// ```
/// let (row, col) = convert_to_index_int(100001);
/// assert_eq!(row, 1);
/// assert_eq!(col, 1);
/// ```
pub fn convert_to_index_int(encode: i32) -> (usize, usize) {
    let inp = decode_cell(encode);
    convert_to_index(inp)
}

/// Converts a Cell structure back to its formula string representation.
///
/// This is essentially the inverse of the parse_formula function.
///
/// # Parameters
/// * `cell` - The Cell structure to convert
///
/// # Returns
/// A string representation of the cell's formula
///
/// # Example
/// ```
/// let formula_str = unparse(cell);
/// assert_eq!(formula_str, "A1+B2");
/// ```
#[allow(dead_code)]
pub fn unparse(cell: Cell) -> String {
    match cell.formula.flag.type_() {
        0 => {
            // Constant
            if cell.formula.flag.type1() == 0 {
                cell.formula.param1.to_string()
            } else {
                decode_cell(cell.formula.param1)
            }
        }
        1 => {
            let sym = match cell.formula.flag.cmd() {
                0 => "+",
                1 => "-",
                2 => "*",
                3 => "/",
                _ => "",
            };
            let left = if cell.formula.flag.type1() == 0 {
                cell.formula.param1.to_string()
            } else {
                decode_cell(cell.formula.param1)
            };
            let right = if cell.formula.flag.type2() == 0 {
                cell.formula.param2.to_string()
            } else {
                decode_cell(cell.formula.param2)
            };
            format!("{}{}{}", left, sym, right)
        }
        2 => {
            let func = match cell.formula.flag.cmd() {
                0 => "MIN",
                1 => "MAX",
                2 => "SUM",
                3 => "AVG",
                4 => "STDEV",
                5 => "SLEEP",
                _ => "",
            };

            if cell.formula.flag.cmd() == 5 {
                // SLEEP function has different format
                let param = if cell.formula.flag.type1() == 0 {
                    cell.formula.param1.to_string()
                } else {
                    decode_cell(cell.formula.param1)
                };
                format!("{}({})", func, param)
            } else {
                // Range functions
                let start = decode_cell(cell.formula.param1);
                let end = decode_cell(cell.formula.param2);
                format!("{}({}:{})", func, start, end)
            }
        }
        _ => "".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_formula() {
        let input = "A1 + B2";
        let result = parse_formula(input);
        assert_eq!(result.flag.type_(), 1);
        assert_eq!(result.flag.cmd(), 0);
        assert_eq!(result.param1, encode_cell("A1".to_string()));
        assert_eq!(result.param2, encode_cell("B2".to_string()));
    }

    #[test]
    fn test_parse_sleep() {
        let input = "SLEEP(5)";
        let mut container = CommandCall {
            flag: CommandFlag::new(),
            param1: 0,
            param2: 0,
        };
        parse_sleep(input, &mut container);
        assert_eq!(container.flag.type_(), 2);
        assert_eq!(container.flag.cmd(), 5);
        assert_eq!(container.param1, 5);
    }
    #[test]
    fn test_parse_sleep_cell_ref() {
        let input = "SLEEP(A1)";
        let mut container = CommandCall {
            flag: CommandFlag::new(),
            param1: 0,
            param2: 0,
        };
        parse_sleep(input, &mut container);
        assert_eq!(container.flag.type_(), 2);
        assert_eq!(container.flag.cmd(), 5);
        assert_eq!(container.param1, encode_cell("A1".to_string()));
    }
    #[test]
    fn test_parse_sleep_invalid() {
        let input = "SLEEP(5A)";
        let mut container = CommandCall {
            flag: CommandFlag::new(),
            param1: 0,
            param2: 0,
        };
        parse_sleep(input, &mut container);
        assert_eq!(container.flag.error(), 1);
    }
    #[test]
    fn test_parse_arithmetic() {
        let input = "A1 + B2";
        let mut container = CommandCall {
            flag: CommandFlag::new(),
            param1: 0,
            param2: 0,
        };
        Arithmatic(input, &mut container);
        assert_eq!(container.flag.type_(), 1);
        assert_eq!(container.flag.cmd(), 0);
        assert_eq!(container.param1, encode_cell("A1".to_string()));
        assert_eq!(container.param2, encode_cell("B2".to_string()));
    }
    #[test]
    fn test_parse_range() {
        let input = "SUM(A1:B2)";
        let mut container = CommandCall {
            flag: CommandFlag::new(),
            param1: 0,
            param2: 0,
        };
        rangeoper(input, &mut container);
        assert_eq!(container.flag.type_(), 2);
        assert_eq!(container.flag.cmd(), 2);
        assert_eq!(container.param1, encode_cell("A1".to_string()));
        assert_eq!(container.param2, encode_cell("B2".to_string()));
    }
    #[test]
    fn test_parse_range_invalid() {
        let input = "SUM(A1:B2:C3)";
        let mut container = CommandCall {
            flag: CommandFlag::new(),
            param1: 0,
            param2: 0,
        };
        rangeoper(input, &mut container);
        assert_eq!(container.flag.error(), 1);
    }
    #[test]
    fn test_parse_expression() {
        let input = "A1 + B2";
        let mut container = CommandCall {
            flag: CommandFlag::new(),
            param1: 0,
            param2: 0,
        };
        parse_expression(input, &mut container);
        assert_eq!(container.flag.type_(), 1);
        assert_eq!(container.flag.cmd(), 0);
        assert_eq!(container.param1, encode_cell("A1".to_string()));
        assert_eq!(container.param2, encode_cell("B2".to_string()));
    }
    #[test]
    fn test_convert_to_index() {
        let cell = "A1".to_string();
        let (row, col) = convert_to_index(cell);
        assert_eq!(row, 1);
        assert_eq!(col, 1);
    }
    #[test]
    fn test_encode_cell() {
        let cell = "A1".to_string();
        let encoded = encode_cell(cell);
        assert_eq!(encoded, 100001);
    }
    #[test]
    fn test_decode_cell() {
        let encoded = 100001;
        let decoded = decode_cell(encoded);
        assert_eq!(decoded, "A1");
    }
    #[test]
    fn test_convert_to_index_int() {
        let encoded = 100001;
        let (row, col) = convert_to_index_int(encoded);
        assert_eq!(row, 1);
        assert_eq!(col, 1);
    }
    #[test]
    fn test_unparse() {
        let mut flag = CommandFlag::new();
        flag.set_type_(1);
        flag.set_cmd(0);
        flag.set_type1(1);
        flag.set_type2(1);
        flag.set_error(0);
        flag.set_is_div_by_zero(0);
        flag.set_is_any(0);

        let cell = Cell {
            formula: CommandCall {
                flag,
                param1: encode_cell("A1".to_string()),
                param2: encode_cell("B2".to_string()),
            },
            value: 0,
            depend: Vec::new(),
        };
        let result = unparse(cell);
        assert_eq!(result, "A1+B2");
    }
    #[test]
    fn test_unparse_constant() {
        let cell = Cell {
            formula: CommandCall {
                flag: CommandFlag::new(),
                param1: 42,
                param2: 0,
            },
            value: 42,
            depend: Vec::new(),
        };
        let result = unparse(cell);
        assert_eq!(result, "42");
    }
}
