use std::fmt::Display;
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
pub struct CommandFlag{
    type_: B2,           // 2 bits
    cmd: B3,             // 3 bits
    type1: B1,           // 1 bit
    type2: B1,           // 1 bit
    error: B2,           // 2 bits
    is_div_by_zero: B1,  // 1 bit
    #[skip]              // Skip the remaining bits to round up to 16 bits
    __: B6,              // 6 bits
}
impl CommandFlag {
    pub fn pr(&self) -> String {
        format!("type_: {}, cmd: {}, type1: {}, type2: {}, error: {}, is_div_by_zero: {}",
            self.type_(), self.cmd(), self.type1(), self.type2(), self.error(), self.is_div_by_zero())

    }
}
pub struct CommandCall {
    flag: CommandFlag, // 16 bits
    param1: i32,         // 4 bytes
    param2: i32,         // 4 bytes
}

impl CommandCall {
    pub fn pr(&self) -> String {
        format!("flag: {}, param1: {}, param2: {}", self.flag.pr(), self.param1, self.param2)
    }
    
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

pub fn Arithmatic(input: &str, container: &mut CommandCall){
    container.flag.set_type_(1);
    let re = Regex::new(r"([A-Z]*\d+)([+\-*/])([A-Z]*\d+)").unwrap();
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
            }};
        if (container.flag.error() == 0) {
            container.flag.set_cmd(temp);
        };
        let right = caps.get(3).unwrap().as_str().to_string();
        if Regex::new(r"[A-Z]").unwrap().is_match(&right) {
            container.param2 = encode_cell(right.to_string());
            container.flag.set_type2(1);
        } else if let Ok(value) = right.parse::<i32>() {
            container.param2 = value;
            container.flag.set_type2(0);
        } else {
            container.flag.set_error(1);
        }
    }
    else {
        container.flag.set_error(1);
    }
}

pub fn rangeoper(input: &str, container: &mut CommandCall){
    container.flag.set_type_(2);
    let re = Regex::new(r"([A-Z]+[0-9]+):([A-Z]+[0-9]+)").unwrap();
    if re.is_match(input) {
        let caps = re.captures(input).unwrap();
        let start = caps.get(1).unwrap().as_str().to_string();
        container.param1 = encode_cell(start.clone());
        container.param2 = encode_cell(caps.get(2).unwrap().as_str().to_string());
        container.flag.set_type1(1);
        container.flag.set_type2(1);
        let temp = match input.split('(').next().unwrap() {
            "MIN" => 0.into(),
            "MAX" => 1.into(),
            "SUM" => 2.into(),
            "AVG" => 3.into(),
            "STDEV" => 4.into(),
            _ => {
                container.flag.set_error(1);
                return;
            }
        };
        if (container.flag.error() == 0) {
            container.flag.set_cmd(temp);
        };
    }
    else{
        container.flag.set_error(1);
    }
}

pub fn parse_expression(input: &str, container: &mut CommandCall){
    if input.starts_with("SLEEP") {
        parse_sleep(input,container)
    } else if input.contains('+') || input.contains('-') || input.contains('*') || input.contains('/') {
        Arithmatic(input,container)
    } else if input.contains(':') {
        rangeoper(input,container)
    } else if let Ok(value) = input.parse::<i32>() {
        container.param1 = value;
        container.param2 = 0;
        container.flag.set_type_(0);
        container.flag.set_cmd(0);
        container.flag.set_type1(0);
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
        
        let col = col_str.chars().map(|c| c as usize - 'A' as usize + 1).sum::<usize>();
        let row = row_str.parse::<usize>().unwrap();
        
        return (col, row);
    }
    (0, 0)
}

pub fn encode_cell(cell:String) -> i32{
    let (col, row) = convert_to_index(cell);
    let mut encoded = 0;
    encoded |= (col as i32) << 16; // Store column in the upper 16 bits
    encoded |= (row as i32);       // Store row in the lower 16 bits
    encoded
}

pub fn decode_cell(encoded:i32) -> String{
    let col = (encoded >> 16) as usize;
    let row = (encoded & 0xFFFF) as usize;
    let col_str = (col + 'A' as usize - 1) as u8 as char;
    format!("{}{}", col_str, row)
}
