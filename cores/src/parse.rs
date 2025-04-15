#![allow(non_snake_case)]
#![allow(unused_braces)]

use regex::Regex;
use modular_bitfield::prelude::*;

use crate::sheet::Cell;

//type_ :0 for conatant, 1 for arithmetic expression, 2 for function 
//cmd : if type==1 then 0 for +, 1 for -, 2 for *, 3 for /
// if type==2 then 0 for MIN, 1 for MAX, 2 for SUM, 3 for AVG, 4 for STDEV , 5 for sleep
// type1 : 0 for value, 1 for cell
//error : 0 for no error, 1 for invalid, 2 for cycle
// is_div_by_zero : 0 for no, 1 for yes

#[bitfield]
#[repr(u16)] // Use a 16-bit underlying storage for all your bitfields
#[derive(Clone)]
#[derive(serde::Serialize)]
#[derive(Debug)]
pub struct CommandFlag{
    pub type_: B2,           // 2 bits
    pub cmd: B3,             // 3 bits
    pub type1: B1,           // 1 bit
    pub type2: B1,           // 1 bit
    pub error: B2,           // 2 bits
    pub is_div_by_zero: B1,  // 1 bit
    pub is_any: B6,              // 6 bits
}
#[derive(Clone)]
#[derive(serde::Serialize)]
pub struct CommandCall {
    pub flag: CommandFlag, // 16 bits
    pub param1: i32,         // 4 bytes
    pub param2: i32,         // 4 bytes
}


pub fn parse_formula(input: &str) -> CommandCall {
    
        let mut cell= CommandCall {
            flag: CommandFlag::new(),
            param1: 0,
            param2: 0,
        };
        
        parse_expression(input, &mut cell);

        cell
    }

pub fn parse_sleep(input: &str, container: &mut CommandCall) {
    container.flag.set_type_(2);
    container.flag.set_cmd(5);
    let re = Regex::new(r"SLEEP\([A-Z]*\d+\)").unwrap();
    if re.is_match(input) {
        let sleep_time = input[6..input.len()-1].to_string();
        if Regex::new(r"[A-Z]").unwrap().is_match(&sleep_time) {
            container.param1 = encode_cell(sleep_time);
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

pub fn Arithmatic(input: &str, container: &mut CommandCall) {
    container.flag.set_type_(1);
    let re = Regex::new(r"^([-+]?[A-Z]*\d+)([-+*/])([-+]?[A-Z]*\d+)$").unwrap();
    if let Some(caps) = re.captures(input) {
        let left = caps.get(1).unwrap().as_str();
        if Regex::new(r"[A-Z]").unwrap().is_match(left) {
            container.param1 = encode_cell(left.to_string());
            container.flag.set_type1(1);
        } else if let Ok(value) = left.parse::<i32>() {
            container.param1 = value;
            container.flag.set_type1(0);
        } else {
            container.flag.set_error(1);
        }

        let operator = caps.get(2).unwrap().as_str().chars().next().unwrap();
        let temp = match operator {
            '+' => 0,
            '-' => 1,
            '*' => 2,
            '/' => 3,
            _ => {
                container.flag.set_error(1);
                return;
            }
        };
        if container.flag.error() == 0 {
            container.flag.set_cmd(temp);
        }

        let right = caps.get(3).unwrap().as_str();
        if Regex::new(r"[A-Z]").unwrap().is_match(right) {
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

pub fn rangeoper(input: &str, container: &mut CommandCall) {
    container.flag.set_type_(2);
    let re = Regex::new(r"^(MIN|MAX|AVG|STDEV|SUM)\(([A-Z]+\d+):([A-Z]+\d+)\)$").unwrap();
    if let Some(caps) = re.captures(input) {
        let start = caps.get(2).unwrap().as_str().to_string();
        container.param1 = encode_cell(start.clone());
        container.param2 = encode_cell(caps.get(3).unwrap().as_str().to_string());
        let (row1,col1) = convert_to_index_int(container.param1);
        let (row2,col2) = convert_to_index_int(container.param2);
        if (row1>row2) || (col1>col2){
            container.flag.set_error(1);
            return ;
        }
        container.flag.set_type1(1);
        container.flag.set_type2(1);
        let temp = match caps.get(1).unwrap().as_str() {
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
        if container.flag.error() == 0 {
            container.flag.set_cmd(temp);
        }
    } else {
        container.flag.set_error(1);
    }
}

pub fn parse_expression(input: &str, container: &mut CommandCall){
    if Regex::new(r"^[-+]?\d+$").unwrap().is_match(input) {
        if let Ok(value) = input.parse::<i32>() {
            container.param1 = value;
            container.param2 = 0;
            container.flag.set_type_(0);
            container.flag.set_cmd(0);
            container.flag.set_type1(0);
        } else {
            container.flag.set_error(1);
        }
    }
    else if input.starts_with("SLEEP") {
        parse_sleep(input,container)
    } else if input.contains('+') || input.contains('-') || input.contains('*') || input.contains('/') {
        Arithmatic(input,container)
    } else if input.contains(':') {
        rangeoper(input,container)
    } else if Regex::new(r"^[A-Z]+\d+$").unwrap().is_match(input) {
        container.param1= encode_cell(input.to_string());
        container.flag.set_type_(0);
        container.flag.set_cmd(0);
        container.flag.set_type1(1);
    }
    else{
        container.flag.set_error(1);
    }
}
pub fn convert_to_index(cell:String) -> (usize, usize) {
    let re = Regex::new(r"([A-Z]+)(\d+)").unwrap();
    if let Some(caps) = re.captures(&cell) {
        let col_str = caps.get(1).unwrap().as_str();
        let row_str = caps.get(2).unwrap().as_str();
        let mut col = 0;
        for i in col_str.chars() {
            col = col * 26 + (i as usize - 'A' as usize + 1);
        }
        let row = row_str.parse::<usize>().unwrap();
        return (row,col);
    }
    (0, 0)
}

pub const ENCODE_SHIFT: usize = 100000;

pub fn encode_cell(cell:String) -> i32{
    let (row, col) = convert_to_index(cell);
    let encoded= row*(ENCODE_SHIFT as usize)+col;
    encoded as i32
}

pub fn decode_cell(encoded:i32) -> String{
    let mut col = (encoded%(ENCODE_SHIFT as i32)) as usize;
    let row = (encoded/(ENCODE_SHIFT as i32)) as usize;
    let mut cell=String::new();
    while col>0{
        let mut temp= col%26;
        if temp==0{
            temp=26;
        }
        cell.insert(0,(temp as u8 + 'A' as u8 -1) as char);
        col=(col-temp)/26;
    }
    cell.push_str(&row.to_string());
    cell
}

pub fn convert_to_index_int(encode:i32) -> (usize,usize){
    let inp= decode_cell(encode);
    convert_to_index(inp)
}

#[allow(dead_code)]
pub fn unparse(cell: Cell) -> String {
    match cell.formula.flag.type_(){
        0 =>{ // Constant
            if cell.formula.flag.type1() == 0 {
                return cell.formula.param1.to_string();
            } else {
                return decode_cell(cell.formula.param1);
            }
        }
        1 =>{
            let sym;
            match cell.formula.flag.cmd(){
                0 => sym = "+",
                1 => sym = "-",
                2 => sym = "*",
                3 => sym = "/",
                _ => sym = "",
            }
            let left;
            if cell.formula.flag.type1() == 0 {
                left = cell.formula.param1.to_string();
            } else {
                left = decode_cell(cell.formula.param1);
            }
            let right;
            if cell.formula.flag.type2() == 0 {
                right = cell.formula.param2.to_string();
            } else {
                right = decode_cell(cell.formula.param2);
            }
            return format!("{}{}{}", left, sym, right);
        }
        2 =>{
            let func;
            match cell.formula.flag.cmd(){
                0 => func = "MIN",
                1 => func = "MAX",
                2 => func = "SUM",
                3 => func = "AVG",
                4 => func = "STDEV",
                5 => func = "SLEEP",
                _ => func = "",
            }
            let start;
            if cell.formula.flag.type1() == 0 {
                start = cell.formula.param1.to_string();
            } else {
                start = decode_cell(cell.formula.param1);
            }
            let end;
            if cell.formula.flag.type2() == 0 {
                end = cell.formula.param2.to_string();
            } else {
                end = decode_cell(cell.formula.param2);
            }
            return format!("{}({}:{})", func, start, end);
        }
        _ => return "".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_formula_add() {
        let input = "A1+B2";
        let result = parse_formula(input);
        assert_eq!(result.param1, 100001);
        assert_eq!(result.param2, 200002);
        assert_eq!(result.flag.type_(), 1);
        assert_eq!(result.flag.cmd(), 0);
        assert_eq!(result.flag.type1(), 1);
        assert_eq!(result.flag.type2(), 1);
    }

    #[test]
    fn test_parse_formula_subtract() {
        let input = "A1-B2";
        let result = parse_formula(input);
        assert_eq!(result.param1, 100001);
        assert_eq!(result.param2, 200002);
        assert_eq!(result.flag.type_(), 1);
        assert_eq!(result.flag.cmd(), 1);
        assert_eq!(result.flag.type1(), 1);
        assert_eq!(result.flag.type2(), 1);
    }

    #[test]
    fn test_parse_formula_multiply() {
        let input = "A1*B2";
        let result = parse_formula(input);
        assert_eq!(result.param1, 100001);
        assert_eq!(result.param2, 200002);
        assert_eq!(result.flag.type_(), 1);
        assert_eq!(result.flag.cmd(), 2);
        assert_eq!(result.flag.type1(), 1);
        assert_eq!(result.flag.type2(), 1);
    }

    #[test]
    fn test_parse_formula_divide() {
        let input = "A1/B2";
        let result = parse_formula(input);
        assert_eq!(result.param1, 100001);
        assert_eq!(result.param2, 200002);
        assert_eq!(result.flag.type_(), 1);
        assert_eq!(result.flag.cmd(), 3);
        assert_eq!(result.flag.type1(), 1);
        assert_eq!(result.flag.type2(), 1);
    }

    #[test]
    fn test_parse_formula_constant() {
        let input = "5";
        let result = parse_formula(input);
        assert_eq!(result.param1, 5);
        assert_eq!(result.param2, 0);
        assert_eq!(result.flag.type_(), 0);
        assert_eq!(result.flag.cmd(), 0);
        assert_eq!(result.flag.type1(), 0);
    }

    #[test]
    fn test_parse_formula_neg_constant(){
        let input = "-5";
        let result = parse_formula(input);
        assert_eq!(result.param1, -5);
        assert_eq!(result.param2, 0);
        assert_eq!(result.flag.type_(), 0);
        assert_eq!(result.flag.cmd(), 0);
        assert_eq!(result.flag.type1(), 0);
    }

    #[test]
    fn test_parse_formula_cell() {
        let input = "ZZZ999";
        let result = parse_formula(input);
        assert_eq!(result.param1, 99918278);
        assert_eq!(result.param2, 0);
        assert_eq!(result.flag.type_(), 0);
        assert_eq!(result.flag.cmd(), 0);
        assert_eq!(result.flag.type1(), 1);
        assert_eq!(result.flag.type2(), 0);
        assert_eq!(result.flag.error(), 0);
    }



    #[test]
    fn test_parse_formula_invalid() {
        let input = "A1+B2*";
        let result = parse_formula(input);
        assert_eq!(result.flag.error(), 1);
    }



    #[test]
    fn test_parse_sleep_val() {
        let input = "SLEEP(5)";
        let mut result = CommandCall {
            flag: CommandFlag::new(),
            param1: 0,
            param2: 0,
        };

        parse_sleep(input, &mut result);
        assert_eq!(result.param1, 5);
        assert_eq!(result.param2, 0);
        assert_eq!(result.flag.type_(), 2);
        assert_eq!(result.flag.cmd(), 5);
        assert_eq!(result.flag.type1(), 0);
        assert_eq!(result.flag.type2(), 0);
        assert_eq!(result.flag.error(), 0);
        assert_eq!(result.flag.is_div_by_zero(), 0);

    }

    #[test]
    fn test_parse_sleep_cell() {
        let input = "SLEEP(A1)";
        let mut result = CommandCall {
            flag: CommandFlag::new(),
            param1: 0,
            param2: 0,
        };

        parse_sleep(input, &mut result);
        assert_eq!(result.param1, 100001);
        assert_eq!(result.param2, 0);
        assert_eq!(result.flag.type_(), 2);
        assert_eq!(result.flag.cmd(), 5);
        assert_eq!(result.flag.type1(), 1);
        assert_eq!(result.flag.type2(), 0);
        assert_eq!(result.flag.error(), 0);
        assert_eq!(result.flag.is_div_by_zero(), 0);

    }

    #[test]
    fn test_parse_arithmetic() {
        let input = "ZZZ999+B2";
        let mut result = CommandCall {
            flag: CommandFlag::new(),
            param1: 0,
            param2: 0,
        };

        Arithmatic(input, &mut result);
        assert_eq!(result.param1, 99918278);
        assert_eq!(result.param2, 200002);
        assert_eq!(result.flag.type_(), 1);
        assert_eq!(result.flag.cmd(), 0);
        assert_eq!(result.flag.type1(), 1);
        assert_eq!(result.flag.type2(), 1);
        assert_eq!(result.flag.error(), 0);
    }

    #[test]
    fn test_parse_range_sum() {
        let input = "SUM(A1:ZZZ999)";
        let mut result = CommandCall {
            flag: CommandFlag::new(),
            param1: 0,
            param2: 0,
        };

        rangeoper(input, &mut result);
        assert_eq!(result.param1, 100001);
        assert_eq!(result.param2, 99918278);
        assert_eq!(result.flag.type_(), 2);
        assert_eq!(result.flag.cmd(), 2);
        assert_eq!(result.flag.type1(), 1);
        assert_eq!(result.flag.type2(), 1);
        assert_eq!(result.flag.error(), 0);
    }

    #[test]
    fn test_parse_range_max() {
        let input = "MAX(A1:ZZZ999)";
        let mut result = CommandCall {
            flag: CommandFlag::new(),
            param1: 0,
            param2: 0,
        };

        rangeoper(input, &mut result);
        assert_eq!(result.param1, 100001);
        assert_eq!(result.param2, 99918278);
        assert_eq!(result.flag.type_(), 2);
        assert_eq!(result.flag.cmd(), 1);
        assert_eq!(result.flag.type1(), 1);
        assert_eq!(result.flag.type2(), 1);
        assert_eq!(result.flag.error(), 0);
    }

    #[test]
    fn tets_parse_range_min() {
        let input = "MIN(A1:ZZZ999)";
        let mut result = CommandCall {
            flag: CommandFlag::new(),
            param1: 0,
            param2: 0,
        };

        rangeoper(input, &mut result);
        assert_eq!(result.param1, 100001);
        assert_eq!(result.param2, 99918278);
        assert_eq!(result.flag.type_(), 2);
        assert_eq!(result.flag.cmd(), 0);
        assert_eq!(result.flag.type1(), 1);
        assert_eq!(result.flag.type2(), 1);
        assert_eq!(result.flag.error(), 0);
    }

    #[test]
    fn test_parse_range_avg() {
        let input = "AVG(A1:ZZZ999)";
        let mut result = CommandCall {  
            flag: CommandFlag::new(),
            param1: 0,
            param2: 0,
        };

        rangeoper(input, &mut result);
        assert_eq!(result.param1, 100001);
        assert_eq!(result.param2, 99918278);
        assert_eq!(result.flag.type_(), 2);
        assert_eq!(result.flag.cmd(), 3);
        assert_eq!(result.flag.type1(), 1);
        assert_eq!(result.flag.type2(), 1);
        assert_eq!(result.flag.error(), 0);
    }

    #[test]
    fn test_parse_range_stdev() {
        let input = "STDEV(A1:ZZZ999)";
        let mut result = CommandCall {
            flag: CommandFlag::new(),
            param1: 0,
            param2: 0,
        };

        rangeoper(input, &mut result);
        assert_eq!(result.param1, 100001);
        assert_eq!(result.param2, 99918278);
        assert_eq!(result.flag.type_(), 2);
        assert_eq!(result.flag.cmd(), 4);
        assert_eq!(result.flag.type1(), 1);
        assert_eq!(result.flag.type2(), 1);
        assert_eq!(result.flag.error(), 0);
    }

    #[test]
    fn test_parse_valid_func_with_invalid_range(){
        let input = "SUM(ZZZ999:BB22)";
        let mut result = CommandCall {
            flag: CommandFlag::new(),
            param1: 0,
            param2: 0,
        };

        rangeoper(input, &mut result);
        assert_eq!(result.param1, 99918278);
        assert_eq!(result.param2, 2200054);
        assert_eq!(result.flag.error(), 1);
    }

    #[test]
    fn test_parse_invalid_func_with_valid_range(){
        let input = "INVALID(A1:ZZZ999)";
        let mut result = CommandCall {
            flag: CommandFlag::new(),
            param1: 0,
            param2: 0,
        };

        rangeoper(input, &mut result);
        assert_eq!(result.flag.error(), 1);
    }

    #[test]
    fn test_convert_to_index() {
        let input = "ZZ29";
        let (col, row) = convert_to_index(input.to_string());
        assert_eq!(col, 29);
        assert_eq!(row, 702);
    }

    #[test]
    fn test_encode_cell() {
        let input = "ZZ29";
        let encoded = encode_cell(input.to_string());
        assert_eq!(encoded, 2900702);

        let input = "C7";
        let encoded = encode_cell(input.to_string());
        assert_eq!(encoded, 700003);
    }

    #[test]
    fn test_decode_cell() {
        let input = 2900702;
        let decoded = decode_cell(input);
        assert_eq!(decoded, "ZZ29");

        let input = 700005;
        let decoded = decode_cell(input);
        assert_eq!(decoded, "E7");
    }

    #[test]
    fn test_unparse_constant() {
        let cell = Cell {
            formula: CommandCall {
                flag: CommandFlag::new(),
                param1: 5,
                param2: 0,
            },
            value: 5,
            depend: vec![],
        };
        let result = unparse(cell);
        assert_eq!(result, "5");
    }

    #[test]
    fn test_unparse_sum_range(){
        let mut cell = Cell {
            formula: CommandCall {
                flag: CommandFlag::new(),
                param1: 100001,
                param2: 99918278,
            },
            value: 0,
            depend: vec![],
        };
        cell.formula.flag.set_type_(2);
        cell.formula.flag.set_cmd(2);
        cell.formula.flag.set_type1(1);
        cell.formula.flag.set_type2(1);
        let result = unparse(cell);
        assert_eq!(result, "SUM(A1:ZZZ999)");
    }




}