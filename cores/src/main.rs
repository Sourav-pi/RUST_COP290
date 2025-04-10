// Use main file for testing
// run from home directory : cargo run -p cores

mod parse;
mod sheet;
use sheet::Sheet;
mod write_csv;
// use write_csv::write_csv;

fn main(){
    let mut test_sheet = Sheet::new(6, 6); 
    test_sheet.update_cell_data(1,1, String::from("AVG(A2:D5)"));
    test_sheet.update_cell_data(2,2, String::from("500"));
    test_sheet.update_cell_data(2,1,String::from("B1+B5"));
    println!("{}", test_sheet.get_value(1, 1));
    let res=write_csv(test_sheet, "/Users/aditya/Downloads/sem4/cop290/temp.csv");
    match res {
        Ok(_) => println!("CSV file written successfully."),
        Err(e) => println!("Error writing CSV file: {}", e),
    } 
    
}