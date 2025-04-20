use crate::parse::CommandFlag;
use crate::{parse::CommandCall, sheet::*};
use serde::{self, Deserialize};

#[derive(Debug, Deserialize)]
struct TempRecord {
    row: i32,
    col: i32,
    value: i32,
    flag: String,
    param1: i32,
    param2: i32,
    depend: String,
}

impl Sheet {
    pub fn read_file(&mut self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut rdr = csv::Reader::from_path(file_path)?;
        for i in 0..self.row {
            for j in 0..self.col {
                self.grid[i][j] = Cell {
                    value: 0,
                    formula: CommandCall {
                        flag: CommandFlag::new(),
                        param1: 0,
                        param2: 0,
                    },
                    depend: Vec::new(),
                };
            }
        }
        for result in rdr.deserialize() {
            let record: TempRecord = result?;
            let mut new_cell = Cell {
                value: record.value,
                formula: CommandCall {
                    flag: CommandFlag::new(),
                    param1: record.param1,
                    param2: record.param2,
                },
                depend: Vec::new(),
            };
            new_cell.value = record.value;
            new_cell.formula.param1 = record.param1;
            new_cell.formula.param2 = record.param2;

            let flag_parts: Vec<&str> = record.flag.split(",").collect();
            for i in flag_parts {
                if let [flag_type, value_of_flag] = i.split(":").collect::<Vec<&str>>().as_slice() {
                    match *flag_type {
                        "type" => new_cell
                            .formula
                            .flag
                            .set_type_(value_of_flag.parse::<u8>().unwrap()),
                        "cmd" => new_cell
                            .formula
                            .flag
                            .set_cmd(value_of_flag.parse::<u8>().unwrap()),
                        "type1" => new_cell
                            .formula
                            .flag
                            .set_type1(value_of_flag.parse::<u8>().unwrap()),
                        "type2" => new_cell
                            .formula
                            .flag
                            .set_type2(value_of_flag.parse::<u8>().unwrap()),
                        "error" => new_cell
                            .formula
                            .flag
                            .set_error(value_of_flag.parse::<u8>().unwrap()),
                        "div_by_zero" => new_cell
                            .formula
                            .flag
                            .set_is_div_by_zero(value_of_flag.parse::<u8>().unwrap()),
                        _ => {}
                    }
                }
            }
            if record.depend.is_empty() {
                new_cell.depend = Vec::new();
            }
            let depend_parts: Vec<&str> = record.depend.split(",").collect();
            for i in depend_parts {
                if let Ok(index) = i.parse::<usize>() {
                    new_cell.depend.push(index);
                }
            }
            self.grid[record.row as usize][record.col as usize] = new_cell;
        }
        Ok(())
    }
}

// #[test]
// fn test_read_csv() {
//     let mut new_sheet = Sheet::new(6, 6);
//     let result = new_sheet.read_file("/Users/aditya/Downloads/sem4/cop290/temp.csv");
//     match result {
//         Ok(()) => println!("CSV file read successfully."),
//         Err(e) => println!("Error reading CSV file: {}", e),
//     }
//     assert_eq!(new_sheet.get_value(2, 2), 500);
//     assert_eq!(new_sheet.get_value(1, 1), 31);
// }
