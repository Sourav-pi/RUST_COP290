// Use main file for testing
// run from home directory : cargo run -p cores

mod parse;
mod sheet;
use parse::{parse_formula, CommandCall};
use sheet::{Sheet, Cell};

fn main(){
    let mut test_sheet = Sheet::create_sheet(6, 6); 
    let target_row:usize=1;
    let target_col:usize=1;
    let input= String::from("A2+A3");
    let command = parse_formula(&input);
    test_sheet.set_dependicies_cell(target_row, target_col, command);
    let input1= String::from("A3-A4");
    let command1 = parse_formula(&input1);
    test_sheet.set_dependicies_cell(1, 2, command1);
    let input2= String::from("7");
    let command2 = parse_formula(&input2);
    test_sheet.set_dependicies_cell(1, 3, command2);
    let input3= String::from("A5/B1");
    let command3 = parse_formula(&input3);
    test_sheet.set_dependicies_cell(1, 4, command3);
    let topo_vec=test_sheet.toposort(100001);
    println!("Toposort: {:?}", topo_vec);
    test_sheet.update_cell(topo_vec);
    println!("value {:?}", test_sheet.grid[1][1].value);
    println!("value {:?}", test_sheet.grid[1][1].formula.flag.is_div_by_zero());



}