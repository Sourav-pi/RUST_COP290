#![allow(non_snake_case)]
#![allow(unused_braces)]

use regex::Regex;
use modular_bitfield::prelude::*;

//type_ :0 for conatant, 1 for arithmetic expression, 2 for function 
//cmd : if type==1 then 0 for +, 1 for -, 2 for *, 3 for /
// if type==2 then 0 for MIN, 1 for MAX, 2 for SUM, 3 for AVG, 4 for STDEV , 5 for sleep
// type1 : 0 for value, 1 for cell
//error : 0 for no error, 1 for invalid, 2 for cycle
// is_div_by_zero : 0 for no, 1 for yes

#[bitfield]
#[repr(u16)] // Use a 16-bit underlying storage for all your bitfields
#[derive(Clone)]
pub struct CommandFlag{
    pub type_: B2,           // 2 bits
    pub cmd: B3,             // 3 bits
    pub type1: B1,           // 1 bit
    pub type2: B1,           // 1 bit
    pub error: B2,           // 2 bits
    pub is_div_by_zero: B1,  // 1 bit
    #[skip]              // Skip the remaining bits to round up to 16 bits
    __: B6,              // 6 bits
}
#[derive(Clone)]
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
    } else {
        container.param1= encode_cell(input.to_string());
        container.flag.set_type_(0);
        container.flag.set_cmd(0);
        container.flag.set_type1(1);

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
        return (col, row);
    }
    (0, 0)
}

pub fn encode_cell(cell:String) -> i32{
    let (col, row) = convert_to_index(cell);
    let encoded= col*(100000)+row;
    encoded as i32
}

pub fn decode_cell(encoded:i32) -> String{
    let col = (encoded%100000) as usize;
    let mut row = (encoded/100000) as usize;
    let mut cell=String::new();
    while row>0{
        let mut temp= row%26;
        if temp==0{
            temp=26;
        }
        cell.insert(0,(temp as u8 + 'A' as u8 -1) as char);
        row=(row-temp)/26;
    }
    cell.push_str(&col.to_string());
    cell
}

pub fn convert_to_index_int(encode:i32) -> (usize,usize){
    let inp= decode_cell(encode);
    convert_to_index(inp)
}

