// Use main file for testing
// run from home directory : cargo run -p cores

mod parse;
use parse::{parse_formula, CommandCall};

fn main() {
    let formula_examples = vec![
        "-42++3",
        "-4",
        "C1",
        "A1+B2",
        "E5*F6",
        "SUM(A1:A20)",
        "AVG(B1:D10)",
        "SLEEP(5)",
        "MIN(A1:F1)",
    ];
    
    for example in formula_examples {
        println!("{}", parse_formula(example).pr());
    }
}
