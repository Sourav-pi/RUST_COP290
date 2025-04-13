use crate::parse::*;
use std::collections::HashSet;
use std::{thread, time};


#[derive(Clone)]
pub struct Cell {
    pub value: i32,
    pub formula: CommandCall,
    pub depend: Vec<usize>,
}
pub enum Error {
    DivByZero,
    InvalidInput,
    CycleDetected,
    NoError,
}

#[allow(dead_code)]
pub struct CallResult {
    pub time:f64,
    pub error:Error,
}
#[allow(dead_code)]
#[allow(dead_code)]
pub struct Sheet {
    pub grid: Vec<Vec<Cell>>,
    pub row: usize,
    pub col: usize,
}
impl Sheet {
    pub fn new(row: usize, col: usize) -> Self {
        let grid: Vec<Vec<Cell>> = vec![
            vec![
                Cell {
                    value: 0,
                    formula: CommandCall {
                        flag: CommandFlag::new(),
                        param1: 0,
                        param2: 0,
                    },
                    depend: Vec::new(),
                };
                col
            ];
            row
        ];
        let sheet = Self { grid, row, col };
        sheet
    }

    pub fn get_formula(&self, row: usize, col: usize) -> String {
        return unparse(self.grid[row][col].clone());
    }

    pub fn add_row(&mut self ,no_of_row:usize) {
        for _ in 0..no_of_row {
            let mut new_row: Vec<Cell> = Vec::new();
            for _ in 0..self.row {
                new_row.push(Cell {
                    value: 0,
                    formula: CommandCall {
                        flag: CommandFlag::new(),
                        param1: 0,
                        param2: 0,
                    },
                    depend: Vec::new(),
                });
            }
            self.grid.push(new_row);
            
            
        }
        self.row+=no_of_row;
        
    }
    
    pub fn add_col(&mut self,no_of_col:usize) {
        for i in 0..self.row {
            for _ in 0..no_of_col {
                self.grid[i].push(Cell {
                    value: 0,
                    formula: CommandCall {
                        flag: CommandFlag::new(),
                        param1: 0,
                        param2: 0,
                    },
                    depend: Vec::new(),
                });
            }
        }
        
        self.col+=no_of_col;
    }
    
    pub fn copy_row(&mut self, copy_from:usize,copy_to:usize){
        for i in 0..self.col {
            self.grid[copy_to][i].value = self.grid[copy_from][i].value;
            self.grid[copy_to][i].formula = self.grid[copy_from][i].formula.clone();
            self.grid[copy_to][i].depend = self.grid[copy_from][i].depend.clone();
        }
    }
    
    pub fn copy_col(&mut self, copy_from:usize,copy_to:usize){
        for i in 0..self.row {
            self.grid[i][copy_to].value = self.grid[i][copy_from].value;
            self.grid[i][copy_to].formula = self.grid[i][copy_from].formula.clone();
            self.grid[i][copy_to].depend = self.grid[i][copy_from].depend.clone();
        }
    }

    fn set_dependicies_cell(&mut self, row: usize, col: usize, command: CommandCall) {
        if command.flag.type_() == 0 {
            if command.flag.type1() == 0 {
                self.grid[row][col].value = command.param1;
            } else if command.flag.type1() == 1 {
                let (param1_row, param1_col) = convert_to_index_int(command.param1);
                let mut is_found = false;
                for i in self.grid[param1_row][param1_col].depend.iter() {
                    if *i == row * ENCODE_SHIFT + col {
                        is_found = true;
                        break;
                    }
                }
                if !is_found{
                self.grid[param1_row][param1_col]
                    .depend
                    .push(row * ENCODE_SHIFT + col);}
            }
        } else if command.flag.type_() == 1 {
            if command.flag.type1() == 0 {
                if command.flag.type2() == 0 {
                    if command.flag.cmd() == 0 {
                        self.grid[row][col].value = command.param1 + command.param2;
                    } else if command.flag.cmd() == 1 {
                        self.grid[row][col].value = command.param1 - command.param2;
                    } else if command.flag.cmd() == 2 {
                        self.grid[row][col].value = command.param1 * command.param2;
                    } else {
                        if command.param2 == 0 {
                            self.grid[row][col].formula.flag.set_is_div_by_zero(1);
                        } else {
                            self.grid[row][col].value = command.param1 / command.param2;
                        }
                    }
                } else {
                    let (param2_row, param2_col) = convert_to_index_int(command.param2);
                    let mut is_found = false;
                    for i in self.grid[param2_row][param2_col].depend.iter() {
                        if *i == row * ENCODE_SHIFT + col {
                            is_found = true;
                            break;
                        }
                    }
                    if !is_found {
                        self.grid[param2_row][param2_col]
                            .depend
                            .push(row * ENCODE_SHIFT + col);
                    }
                    // self.grid[param2_row][param2_col]
                    //     .depend
                    //     .push(row * ENCODE_SHIFT + col);
                }
            } else if command.flag.type1() == 1 {
                let (param1_row, param1_col) = convert_to_index_int(command.param1);
                let mut is_found = false;
                for i in self.grid[param1_row][param1_col].depend.iter() {
                    if *i == row * ENCODE_SHIFT + col {
                        is_found = true;
                        break;
                    }
                }
                if !is_found {
                    self.grid[param1_row][param1_col]
                        .depend
                        .push(row * ENCODE_SHIFT + col);
                }
                // self.grid[param1_row][param1_col]
                //     .depend
                //     .push(row * ENCODE_SHIFT + col);
                if command.flag.type2() == 0 {
                } else if command.flag.type2() == 1 {
                    let (param2_row, param2_col) = convert_to_index_int(command.param2);
                    let mut is_found = false;
                    for i in self.grid[param2_row][param2_col].depend.iter() {
                        if *i == row * ENCODE_SHIFT + col {
                            is_found = true;
                            break;
                        }
                    }
                    if !is_found {
                        self.grid[param2_row][param2_col]
                            .depend
                            .push(row * ENCODE_SHIFT + col);
                    }
                    // self.grid[param2_row][param2_col]
                    //     .depend
                    //     .push(row * ENCODE_SHIFT + col);
                }
            }
        } else {
            let t = row * ENCODE_SHIFT + col;
            let (param1_row, param1_col) = convert_to_index_int(command.param1);
            let (param2_row, param2_col) = convert_to_index_int(command.param2);
            for i in param1_row..(param2_row + 1) {
                for j in param1_col..(param2_col + 1) {
                    let mut is_found = false;
                    for k in self.grid[i][j].depend.iter() {
                        if *k == t {
                            is_found = true;
                            break;
                        }
                    }
                    if !is_found {
                        self.grid[i][j].depend.push(t);
                    }
                    // self.grid[i][j].depend.push(t);
                }
            }
        }

        self.grid[row][col].formula = command;
    }

    fn toposort(&self, target_cell: usize) -> Vec<usize> {
        let mut visited: HashSet<usize> = HashSet::new();
        let mut stack: HashSet<usize> = HashSet::new();
        let mut result: Vec<usize> = vec![];
        let is_cycle = self.dfs(target_cell, &mut visited, &mut stack, &mut result);

        if is_cycle {
            println!("Cycle detected in the graph");
            return vec![];
        }
        // while let Some(cell) = stack.pop() {
        //     result.push(cell);
        // }

        result.reverse();
        result
    }
    fn dfs(
        &self,
        cell: usize,
        visited: &mut HashSet<usize>,
        stack: &mut HashSet<usize>,
        result: &mut Vec<usize>,
    ) -> bool {
        if stack.contains(&cell) {
            return true;
        }
        if visited.contains(&cell) {
            return false;
        }

        visited.insert(cell);

        let col = (cell as usize) % ENCODE_SHIFT;
        let row = (cell as usize) / ENCODE_SHIFT;
        let mut is_cycle = false;
        stack.insert(cell);
        for &dep in &self.grid[row][col].depend {
            is_cycle = is_cycle || self.dfs(dep, visited, stack, result);
        }
        stack.remove(&cell);
        result.push(cell);
        return is_cycle;
    }

    fn minimum(&self, row1: usize, row2: usize, col1: usize, col2: usize) -> i32 {
        let mut min = i32::MAX;
        for i in row1..(row2 + 1) {
            for j in col1..(col2 + 1) {
                // No need to check for negative values as `i` and `j` are of type `usize`
                if self.grid[i as usize][j as usize].value < min {
                    min = self.grid[i as usize][j as usize].value;
                }
            }
        }
        min
    }
    fn maximum(&self, row1: usize, row2: usize, col1: usize, col2: usize) -> i32 {
        let mut max = i32::MIN;
        for i in row1..(row2 + 1) {
            for j in col1..(col2 + 1) {
                // println!("{}",self.grid[i as usize][j as usize].value);
                if self.grid[i as usize][j as usize].value > max {
                    max = self.grid[i as usize][j as usize].value;
                }
            }
        }
        max
    }
    fn average(&self, row1: usize, row2: usize, col1: usize, col2: usize) -> i32 {
        let mut sum = 0;
        let mut count = 0;
        for i in row1..(row2 + 1) {
            for j in col1..(col2 + 1) {
                sum += self.grid[i as usize][j as usize].value;
                count += 1;
            }
        }
        if count == 0 {
            return 0;
        }
        (sum as f32 / count as f32) as i32
    }
    fn sum(&self, row1: usize, row2: usize, col1: usize, col2: usize) -> i32 {
        let mut sum = 0;
        for i in row1..(row2 + 1) {
            for j in col1..(col2 + 1) {
                sum += self.grid[i as usize][j as usize].value;
            }
        }
        sum
    }
    fn stddev(&self, row1: usize, row2: usize, col1: usize, col2: usize) -> i32 {
        let mean = self.average(row1, row2, col1, col2);
        let mut sum = 0;
        let mut count = 0;
        for i in row1..(row2 + 1) {
            for j in col1..(col2 + 1) {
                let value = self.grid[i as usize][j as usize].value;
                sum += (value - mean).pow(2);
                count += 1;
            }
        }
        if count == 0 {
            return 0;
        }
        ((sum as f64 / count as f64).sqrt()) as i32
    }

    fn update_cell(&mut self, list_fpr_update: Vec<usize>){
        for i in list_fpr_update {
            let col = (i as usize) % ENCODE_SHIFT;
            let row = (i as usize) / ENCODE_SHIFT;
            self.grid[row][col].formula.flag.set_is_div_by_zero(0);
            if self.grid[row][col].formula.flag.type_() == 0 {
                // value

                if self.grid[row][col].formula.flag.type1() == 0 {
                    self.grid[row][col].value = self.grid[row][col].formula.param1;
                } else if self.grid[row][col].formula.flag.type1() == 1 {
                    let (param1_row, param1_col) =
                        convert_to_index_int(self.grid[row][col].formula.param1);
                    if self.grid[param1_row][param1_col]
                        .formula
                        .flag
                        .is_div_by_zero()
                        == 1
                    {
                        self.grid[row][col].formula.flag.set_is_div_by_zero(1);
                    }
                    self.grid[row][col].value = self.grid[param1_row][param1_col].value;
                }
            } else if self.grid[row][col].formula.flag.type_() == 1 {
                // arithmatic
                if self.grid[row][col].formula.flag.type1() == 0 {
                    if self.grid[row][col].formula.flag.type2() == 0 {
                        if self.grid[row][col].formula.flag.cmd() == 0 {
                            self.grid[row][col].value = self.grid[row][col].formula.param1
                                + self.grid[row][col].formula.param2;
                        } else if self.grid[row][col].formula.flag.cmd() == 1 {
                            self.grid[row][col].value = self.grid[row][col].formula.param1
                                - self.grid[row][col].formula.param2;
                        } else if self.grid[row][col].formula.flag.cmd() == 2 {
                            self.grid[row][col].value = self.grid[row][col].formula.param1
                                * self.grid[row][col].formula.param2;
                        } else {
                            if self.grid[row][col].formula.param2 == 0 {
                                self.grid[row][col].formula.flag.set_is_div_by_zero(1);
                                
                            } else {
                                self.grid[row][col].value = self.grid[row][col].formula.param1
                                    / self.grid[row][col].formula.param2;
                            }
                        }
                    } else {
                        // let param2_row=(self.grid[row][col].formula.param2%1000) as usize;
                        // let param2_col=(self.grid[row][col].formula.param2/1000) as usize;
                        let (param2_row, param2_col) =
                            convert_to_index_int(self.grid[row][col].formula.param2);
                        if self.grid[param2_row][param2_col]
                            .formula
                            .flag
                            .is_div_by_zero()
                            == 1
                        {
                            self.grid[row][col].formula.flag.set_is_div_by_zero(1);
                        }
                        if self.grid[row][col].formula.flag.cmd() == 0 {
                            self.grid[row][col].value = self.grid[row][col].formula.param1
                                + self.grid[param2_row][param2_col].value;
                        } else if self.grid[row][col].formula.flag.cmd() == 1 {
                            self.grid[row][col].value = self.grid[row][col].formula.param1
                                - self.grid[param2_row][param2_col].value;
                        } else if self.grid[row][col].formula.flag.cmd() == 2 {
                            self.grid[row][col].value = self.grid[row][col].formula.param1
                                * self.grid[param2_row][param2_col].value;
                        } else {
                            if self.grid[param2_row][param2_col].value == 0 {
                                self.grid[row][col].formula.flag.set_is_div_by_zero(1);
                            } else {
                                self.grid[row][col].value = self.grid[row][col].formula.param1
                                    / self.grid[param2_row][param2_col].value;
                            }
                            // self.grid[row][col].value=self.grid[row][col].formula.param1/self.grid[param2_row][param2_col].value;
                        }
                    }
                } else if self.grid[row][col].formula.flag.type1() == 1 {
                    // let param1_row=(self.grid[row][col].formula.param1%1000) as usize;
                    // let param1_col: usize=(self.grid[row][col].formula.param1/1000) as usize;
                    let (param1_row, param1_col) =
                        convert_to_index_int(self.grid[row][col].formula.param1);
                    if self.grid[param1_row][param1_col]
                        .formula
                        .flag
                        .is_div_by_zero()
                        == 1
                    {
                        self.grid[row][col].formula.flag.set_is_div_by_zero(1);
                    }
                    if self.grid[row][col].formula.flag.type2() == 0 {
                        if self.grid[row][col].formula.flag.cmd() == 0 {
                            self.grid[row][col].value = self.grid[param1_row][param1_col].value
                                + self.grid[row][col].formula.param2;
                        } else if self.grid[row][col].formula.flag.cmd() == 1 {
                            self.grid[row][col].value = self.grid[param1_row][param1_col].value
                                - self.grid[row][col].formula.param2;
                        } else if self.grid[row][col].formula.flag.cmd() == 2 {
                            self.grid[row][col].value = self.grid[param1_row][param1_col].value
                                * self.grid[row][col].formula.param2;
                        } else {
                            if self.grid[row][col].formula.param2 == 0 {
                                self.grid[row][col].formula.flag.set_is_div_by_zero(1);
                            } else {
                                self.grid[row][col].value = self.grid[param1_row][param1_col].value
                                    / self.grid[row][col].formula.param2;
                            }
                        }
                    } else if self.grid[row][col].formula.flag.type2() == 1 {

                        let (param2_row, param2_col) =
                            convert_to_index_int(self.grid[row][col].formula.param2);
                        if self.grid[param2_row][param2_col]
                            .formula
                            .flag
                            .is_div_by_zero()
                            == 1
                        {
                            self.grid[row][col].formula.flag.set_is_div_by_zero(1);
                        }
                        if self.grid[row][col].formula.flag.cmd() == 0 {
                            self.grid[row][col].value = self.grid[param1_row][param1_col].value
                                + self.grid[param2_row][param2_col].value;
                        } else if self.grid[row][col].formula.flag.cmd() == 1 {
                            self.grid[row][col].value = self.grid[param1_row][param1_col].value
                                - self.grid[param2_row][param2_col].value;
                        } else if self.grid[row][col].formula.flag.cmd() == 2 {
                            self.grid[row][col].value = self.grid[param1_row][param1_col].value
                                * self.grid[param2_row][param2_col].value;
                        } else {
                            if self.grid[param2_row][param2_col].value == 0 {
                                self.grid[row][col].formula.flag.set_is_div_by_zero(1);
                            } else {
                                self.grid[row][col].value = self.grid[param1_row][param1_col].value
                                    / self.grid[param2_row][param2_col].value;
                            }
                            // self.grid[row][col].value=self.grid[param1_row][param1_col].value/self.grid[param2_row][param2_col].value;
                        }
                    }
                }
            } else {
                let (param1_row, param1_col) =
                    convert_to_index_int(self.grid[row][col].formula.param1);
                if self.grid[param1_row][param1_col]
                    .formula
                    .flag
                    .is_div_by_zero()
                    == 1
                {
                    self.grid[row][col].formula.flag.set_is_div_by_zero(1);
                }
                let (param2_row, param2_col) =
                    convert_to_index_int(self.grid[row][col].formula.param2);
                if self.grid[param2_row][param2_col]
                    .formula
                    .flag
                    .is_div_by_zero()
                    == 1
                {
                    self.grid[row][col].formula.flag.set_is_div_by_zero(1);
                }
                if self.grid[row][col].formula.flag.cmd() == 0 {
                    self.grid[row][col].value =
                        self.minimum(param1_row, param2_row, param1_col, param2_col);
                } else if self.grid[row][col].formula.flag.cmd() == 1 {
                    self.grid[row][col].value =
                        self.maximum(param1_row, param2_row, param1_col, param2_col);
                } else if self.grid[row][col].formula.flag.cmd() == 2 {
                    self.grid[row][col].value =
                        self.sum(param1_row, param2_row, param1_col, param2_col);
                } else if self.grid[row][col].formula.flag.cmd() == 3 {
                    self.grid[row][col].value =
                        self.average(param1_row, param2_row, param1_col, param2_col);
                } else if self.grid[row][col].formula.flag.cmd() == 4 {
                    self.grid[row][col].value =
                        self.stddev(param1_row, param2_row, param1_col, param2_col);
                } else if self.grid[row][col].formula.flag.cmd() == 5 {
                    let time_millis =
                        time::Duration::from_millis(self.grid[param1_row][param1_col].value as u64);
                    thread::sleep(time_millis);
                    self.grid[row][col].value = self.grid[param1_row][param1_col].value;
                }
            }
        }
    }
    fn remove_old_dependicies(&mut self, row: usize, col: usize,restore_command: CommandCall) {
        // Remove all dependencies from previous formula
        let curr_index = row * ENCODE_SHIFT + col;
        println!("curr index {}", curr_index);
        println!("curr index {}", curr_index);
        let current_command = &self.grid[row][col].formula.clone();
        println!("{} {}", current_command.param1, current_command.param2);
        println!("{} {}", current_command.param1, current_command.param2);
        // Remove dependencies based on command type
        if current_command.flag.type_() == 0 && current_command.flag.type1() == 1 {
            // Cell reference dependency
           
           
            let (param1_row, param1_col) = convert_to_index_int(current_command.param1);
            let depend_vec = &mut self.grid[param1_row][param1_col].depend;
            depend_vec.retain(|&x| x != curr_index);
        } 
        else if current_command.flag.type_() == 1 {
            // Arithmetic operation dependencies
            if current_command.flag.type1() == 1 {
                // First parameter is a cell reference
                let (param1_row, param1_col) = convert_to_index_int(current_command.param1);
                let depend_vec = &mut self.grid[param1_row][param1_col].depend;
                depend_vec.retain(|&x| x != curr_index);
                // let mut new_depend_vec= Vec::new();
                // for i in self.grid[param1_row][param1_col].depend.iter() {
                //     if *i != curr_index {
                //         new_depend_vec.push(*i);
                //     }
                // }
                // self.grid[param1_row][param1_col].depend=new_depend_vec;
            }
            if current_command.flag.type2() == 1 {
                // Second parameter is a cell reference
                let (param2_row, param2_col) = convert_to_index_int(current_command.param2);
                let depend_vec = &mut self.grid[param2_row][param2_col].depend;
                depend_vec.retain(|&x| x != curr_index);
            }
        }
        else if current_command.flag.type_() == 2 {
            // Range function dependencies
            let (param1_row, param1_col) = convert_to_index_int(current_command.param1);
            let (param2_row, param2_col) = convert_to_index_int(current_command.param2);
            for i in param1_row..(param2_row + 1) {
                for j in param1_col..(param2_col + 1) {
                    let depend_vec = &mut self.grid[i][j].depend;
                    depend_vec.retain(|&x| x != curr_index);
                }
            }
        }

        // // Set the cell's formula to the restore command
        // self.grid[row][col].formula = restore_command;
        // Restore the cell's value to the original value
        self.set_dependicies_cell(row, col, restore_command.clone());

        
    }
    pub fn update_cell_data(&mut self, row: usize, col: usize, new_formula: String) -> CallResult {
        let start = time::Instant::now();
        let mut command = parse_formula(&new_formula);
        command.flag.set_is_any(1);
        command.flag.set_is_any(1);
        let old_command=self.grid[row][col].formula.clone();
        self.set_dependicies_cell(row as usize, col as usize, command.clone());
        let topo_vec = self.toposort(row * ENCODE_SHIFT + col);
        if topo_vec.is_empty() {
            self.grid[row][col].formula.flag.set_error(2);

        } else {
            self.update_cell(topo_vec);
        }
        let end= start.elapsed();
        let mut ans=CallResult{
            time:end.as_millis() as f64,
            error:Error::NoError,
        };
        if self.grid[row][col].formula.flag.is_div_by_zero() == 1 {
            ans.error=Error::DivByZero;
            return ans;
        } else if self.grid[row][col].formula.flag.error()==1 {
            ans.error=Error::InvalidInput;
            return ans;
        } else if self.grid[row][col].formula.flag.error()==2 {
            ans.error=Error::CycleDetected;
            self.remove_old_dependicies(row,col,old_command);
            return ans;
        } 
        else {
            return ans;
        }
    }

    pub fn get_value(&self, row: i32, col: i32) -> i32 {
        self.grid[row as usize][col as usize].value
    }
}
