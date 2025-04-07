// Use main file for testing
// run from home directory : cargo run -p cores

mod parse;
mod sheet;
use sheet::Sheet;

fn main(){
    let mut test_sheet = Sheet::new(6, 6); 
    test_sheet.update_cell_data(1,1, String::from("MAX(A2:D5)"));
    test_sheet.update_cell_data(2,2, String::from("100"));
    println!("{}", test_sheet.get_value(1, 1));
    
}