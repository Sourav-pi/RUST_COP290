use cores::Sheet;
use cores::convert_to_index;
use std::io;
use std::io::Write;
// use std::io::stdin;
use std::cmp;
use std::env;
use cores::Error;
// use cores::SheetError;

const DEBUG:bool = false;

pub fn column_to_letter(col: usize) -> String {
    let mut result = String::new();
    let mut col_num = col;

    while col_num > 0 {
        let remainder = (col_num - 1) % 26;
        result.insert(0, (b'A' + remainder as u8) as char);
        col_num = (col_num - 1) / 26;
    }

    result
}
fn display_sheet(sheet: &Sheet, row: usize, col: usize,rowi: usize, coli: usize) {
    let mut i=coli;
    print!(" \t ");
    while i<coli+10 && i< col {
        // print!(" ");
        // print!(" ");
        print!("{}\t ", column_to_letter(i));
        i += 1;
    }println!();
    i = rowi;
    while i<rowi+10 && i< row {
        print!("{}\t ", i);
        let mut j=coli;
        while j<coli+10 && j< col {
            let value = sheet.get_value(i as i32, j as i32);
            if sheet.grid[i][j].formula.flag.is_div_by_zero()==1 {
                print!("ERR\t ");
            }else{
                print!("{}\t ", value);
            }
            //print!("{}\t ", value);
            j += 1;
        }println!();
        i += 1;
    }
     
}
fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <int1> <int2>", args[0]);
        std::process::exit(1);
    }
    let int1: i32 = args[1].parse().expect("Invalid integer for int1");
    let int2: i32 = args[2].parse().expect("Invalid integer for int2");
    let int1= int1+1;
    let int2= int2+1;
    
    let mut test_sheet = Sheet::new(int1 as usize, int2 as usize);
    let mut rowi = 1;
    let mut coli = 1;
    let mut input = String::new();
    let mut display_button=true;
    let mut massage="ok";
    let mut time=0.0;
    let mut trimmed:&str;
    while {
        if display_button {
            display_sheet(&test_sheet, int1 as usize, int2 as usize,rowi as usize, coli as usize);
        }
        print!("[{time}] ({}) > ", massage);
        massage="ok";
        io::stdout().flush().unwrap();

        input.clear(); // Clear previous input
        io::stdin().read_line(&mut input).expect("Failed to read input");
        trimmed =input.trim();
        //print!("{}\n", trimmed);
        trimmed != "q"
        
    }{
        if DEBUG {println!("{}", trimmed);}
        if trimmed.contains("=") {
            if DEBUG { println!("This is an assignment: {}", trimmed);}
            // Split the assignment into left-hand side (lhs) and right-hand side (rhs)
            let parts: Vec<&str> = trimmed.split('=').collect();
            if parts.len() == 2 {
                let lhs = parts[0].trim(); // e.g., A1
                let rhs = parts[1].trim(); // e.g., A2+A3
                if DEBUG {println!("Left: {}, Right: {}", lhs, rhs);}
                // Convert the cell reference to indices
                let (cell_index_row,cell_index_col) = convert_to_index(lhs.to_string());
                let result=test_sheet.update_cell_data( cell_index_row, cell_index_col, rhs.to_string());
                time =result.time;
                match  result.error {
                    Error::InvalidInput=> {massage="invalid input"}
                    Error::None=> {massage="ok"}
                    Error::CycleDetected=> {massage="cycle detected"}
                    Error::DivByZero=> {massage="ok"}
                }
            } else {
                // println!("Invalid assignment format");
                massage="invalid input";
            }
        } else if trimmed=="w"{
            rowi=cmp::max(1,rowi-10);
            
        }else if trimmed=="s" {
            rowi=cmp::min(int1-10,rowi+10);
            
        }else if trimmed=="a" {
            coli=cmp::max(1,coli-10);

        }else if trimmed=="d" {
            coli=cmp::min(int2-10,coli+10);
        }else if trimmed =="disable_output" {
            display_button=false
        }else if trimmed =="enable_output"{
            display_button=true
        }else if trimmed.len()>9&& &trimmed[0..9]=="scroll_to"   {
            let parts: Vec<&str> = trimmed.split(' ').collect();
            if parts.len() == 2 {
                
            //     println!("scroll_to: {}", parts[1]);
            //     println!("scroll_to: {}", parts[0]);

            // println!("This is a normal input: {}this", trimmed);
            let (scroll_row,scroll_col) = convert_to_index(parts[1].to_string());
            
            if scroll_row as i32<=int1 && scroll_col as i32<=int2 && scroll_row as i32>=1 && scroll_col as i32>=1{
                rowi=scroll_row as i32;
                coli=scroll_col as i32;
                rowi=cmp::min(rowi,int1);
                coli=cmp::min(coli,int2);
            }else{
                massage="invalid input";
            }

            }else{
                massage="invalid input";
            }
        }else{
            // println!("this is invalid input: {}", trimmed);
            massage="invalid input";
        }
    
}

}