// Use main file for testing
// run from home directory : cargo run -p cores

mod parse;
mod sheet;
use parse::{parse_formula, CommandCall};
use sheet::{Sheet, Cell};

fn main(){
    let mut test_sheet = Sheet::create_sheet(6, 6); 
    test_sheet.update_cell_data(1,1, String::from("A2+A3"));
    test_sheet.update_cell_data(1,2, String::from("3"));
    println!("A1: {}", test_sheet.get_value(1, 1));
    
}