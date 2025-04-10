
use crate::parse;
use crate::parse::parse_formula;
use crate::parse::CommandCall;
use crate::parse::CommandFlag;
use crate:: parse::convert_to_index_int;
use crate::parse::ENCODE_SHIFT;
use std::collections::HashSet;
#[derive(Clone)]
        pub struct Cell{
            pub value:i32,
            pub formula:CommandCall,
            pub depend:Vec<usize>,
            
            
        }

        pub struct Sheet{
            pub grid:Vec<Vec<Cell>>,
        }
        impl Sheet {
            pub fn new(row:usize, col:usize) -> Self {
                let grid:Vec<Vec<Cell>>= vec![vec![Cell{
                    value:0,
                    formula:CommandCall{
                        flag:CommandFlag::new(),
                        param1:0,
                        param2:0,
                    },
                    depend:Vec::new(),
                    
                };col];row];
                let sheet=Self {
                    grid,
                };
                sheet
            }
            pub fn set_dependicies_cell(&mut self, row: usize, col: usize, command: CommandCall) {

            if command.flag.type_()==0 {
                if command.flag.type1()==0 {
                    self.grid[row][col].value=command.param1;
                }else if command.flag.type1()==1 {
                    let (param1_row,param1_col)=convert_to_index_int(command.param1);
                    self.grid[param1_row][param1_col].depend.push(row*ENCODE_SHIFT+col);
                   
                }
            }else if command.flag.type_()==1 {
                if command.flag.type1()==0 {
                    if command.flag.type2()==0 {
                        if command.flag.cmd()==0 {
                            self.grid[row][col].value=command.param1+command.param2;
                        }else if command.flag.cmd()==1 {
                            self.grid[row][col].value=command.param1-command.param2;
                        }else if command.flag.cmd()==2 {
                            self.grid[row][col].value=command.param1*command.param2;
                        }else{
                            
                            self.grid[row][col].value=command.param1/command.param2;
                        }
                    }else{
                        let (param2_row,param2_col)=convert_to_index_int(command.param2);
                        self.grid[param2_row][param2_col].depend.push(row*ENCODE_SHIFT+col);
                    }
                }else if command.flag.type1()==1 {
                    let (param1_row,param1_col)=convert_to_index_int(command.param1);
                    self.grid[param1_row][param1_col].depend.push(row*ENCODE_SHIFT+col);
                    if command.flag.type2()==0 {
                        
                    }else if command.flag.type2()==1 {
                        let (param2_row,param2_col)=convert_to_index_int(command.param2);
                        self.grid[param2_row][param2_col].depend.push(row*ENCODE_SHIFT+col);
                    }
                }   
        }else{
            let t=row*ENCODE_SHIFT+col;
            let (param1_row,param1_col)=convert_to_index_int(command.param1);
            let (param2_row,param2_col)=convert_to_index_int(command.param2);
            for i in param1_row..(param2_row+1){
                for j in param1_col..(param2_col+1){
                    self.grid[i][j].depend.push(t);
                }
            }

        }

            self.grid[row][col].formula=command;
            }

            pub fn toposort(& self, target_cell :usize) -> Vec<usize> {
                let mut visited:HashSet<usize> = HashSet::new();
                let mut stack:HashSet<usize> = HashSet::new();
                let mut result:Vec<usize> = vec![];
                let is_cycle=self.dfs(target_cell, &mut visited, &mut stack,&mut result);
                
                if is_cycle {
                    println!("Cycle detected in the graph");
                    return vec![];
                }
                // while let Some(cell) = stack.pop() {
                //     result.push(cell);
                // }

                // result.reverse();
                result}
            pub fn dfs(&self, cell: usize, visited: &mut HashSet<usize>, stack: &mut  HashSet<usize>,result :&mut Vec<usize>)-> bool  {
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
                for &dep in & self.grid[row][col].depend {
                    is_cycle = is_cycle || self.dfs(dep, visited, stack,result);
                }
                stack.remove(&cell);
                result.push(cell);
                return is_cycle;
            }

            pub fn minimum(&self,row1: usize ,row2: usize      ,col1 : usize , col2 : usize) -> i32 {
                let mut min = i32::MAX;
                for i in row1..(row2+1) {
                    for j in col1..(col2+1) {
                        // No need to check for negative values as `i` and `j` are of type `usize`
                        if self.grid[i as usize][j as usize].value < min {
                            min = self.grid[i as usize][j as usize].value;
                        }
                    }
                }
                min
            }
            pub fn maximum(&self,row1: usize, row2: usize, col1: usize, col2: usize) -> i32 {
                let mut max = i32::MIN;
                for i in row1..(row2+1) {
                    for j in col1..(col2+1) {
                        // println!("{}",self.grid[i as usize][j as usize].value);
                        if self.grid[i as usize][j as usize].value > max {
                            max = self.grid[i as usize][j as usize].value;
                        }
                    }
                }
                max
            }
            pub fn average(&self,row1: usize, row2: usize, col1: usize, col2: usize) -> i32 {
                let mut sum = 0;
                let mut count = 0;
                for i in row1..(row2+1) {
                    for j in col1..(col2+1) {
                        sum += self.grid[i as usize][j as usize].value;
                        count += 1;
                    }
                }
                if count == 0 {
                    return 0;
                }
                (sum as f32 / count as f32) as i32
            }
            pub fn sum(&self,row1: usize, row2: usize, col1: usize, col2: usize) -> i32 {
                let mut sum = 0;
                for i in row1..(row2+1){
                    for j in col1..(col2+1) {
                        sum += self.grid[i as usize][j as usize].value;
                    }
                }
                sum
            }
            pub fn stddev(&self, row1: usize, row2: usize, col1: usize, col2: usize) -> i32 {
                let mean = self.average(row1, row2, col1, col2);
                let mut sum = 0;
                let mut count = 0;
                for i in row1..(row2+1) {
                    for j in col1..(col2+1) {
                        let value = self.grid[i as usize][j as usize].value ;
                        sum += (value - mean).pow(2);
                        count += 1;
                    }
                }
                if count == 0 {
                    return 0;
                }
                ((sum as f64 / count as f64).sqrt()) as i32
            }

            pub fn update_cell (&mut self, list_fpr_update:Vec<usize>){

                for i in list_fpr_update{
                    let col = (i as usize) % ENCODE_SHIFT;
                let row = (i as usize) / ENCODE_SHIFT;
                    if self.grid[row][col].formula.flag.type_() == 0 { // value
                        
                        if self.grid[row][col].formula.flag.type1() ==0 {
                            self.grid[row][col].value=self.grid[row][col].formula.param1;
                        }else if self.grid[row][col].formula.flag.type1()==1 {

                            let (param1_row,param1_col)=convert_to_index_int(self.grid[row][col].formula.param1);
                            if self.grid[param1_row][param1_col].formula.flag.is_div_by_zero()==1 {
                                self.grid[row][col].formula.flag.set_is_div_by_zero(1);
                            }
                            self.grid[row][col].value=self.grid[param1_row][param1_col].value;
                        }
                    }
                    else if self.grid[row][col].formula.flag.type_()==1 { // arithmatic
                        if self.grid[row][col].formula.flag.type1()==0 {
                            if self.grid[row][col].formula.flag.type2()==0 {
                                if self.grid[row][col].formula.flag.cmd()==0 {
                                    self.grid[row][col].value=self.grid[row][col].formula.param1+self.grid[row][col].formula.param2;
                                }else if self.grid[row][col].formula.flag.cmd()==1 {
                                    self.grid[row][col].value=self.grid[row][col].formula.param1-self.grid[row][col].formula.param2;
                                }else if self.grid[row][col].formula.flag.cmd()==2 {
                                    self.grid[row][col].value=self.grid[row][col].formula.param1*self.grid[row][col].formula.param2;
                                }else {
                                    if self.grid[row][col].formula.param2==0 {
                                        self.grid[row][col].formula.flag.set_is_div_by_zero(1);
                                    }else{
                                        self.grid[row][col].value=self.grid[row][col].formula.param1/self.grid[row][col].formula.param2;

                                    }
                                }
                            }else{
                                // let param2_row=(self.grid[row][col].formula.param2%1000) as usize;
                                // let param2_col=(self.grid[row][col].formula.param2/1000) as usize;
                                let (param2_row,param2_col)=convert_to_index_int(self.grid[row][col].formula.param2);
                                if self.grid[param2_row][param2_col].formula.flag.is_div_by_zero()==1 {
                                    self.grid[row][col].formula.flag.set_is_div_by_zero(1);
                                }
                                if self.grid[row][col].formula.flag.cmd()==0 {
                                    self.grid[row][col].value=self.grid[row][col].formula.param1+self.grid[param2_row][param2_col].value;
                                }else if self.grid[row][col].formula.flag.cmd()==1 {
                                    self.grid[row][col].value=self.grid[row][col].formula.param1-self.grid[param2_row][param2_col].value;
                                }else if self.grid[row][col].formula.flag.cmd()==2 {
                                    self.grid[row][col].value=self.grid[row][col].formula.param1*self.grid[param2_row][param2_col].value;
                                }else{
                                    if self.grid[param2_row][param2_col].value==0 {
                                        self.grid[row][col].formula.flag.set_is_div_by_zero(1);
                                    }else{
                                        self.grid[row][col].value=self.grid[row][col].formula.param1/self.grid[param2_row][param2_col].value;
                                    }
                                    // self.grid[row][col].value=self.grid[row][col].formula.param1/self.grid[param2_row][param2_col].value;
                                }
                                
                            }
                        }else if self.grid[row][col].formula.flag.type1()==1 {
                            // let param1_row=(self.grid[row][col].formula.param1%1000) as usize;
                            // let param1_col: usize=(self.grid[row][col].formula.param1/1000) as usize;
                            let (param1_row,param1_col)=convert_to_index_int(self.grid[row][col].formula.param1);
                            if self.grid[param1_row][param1_col].formula.flag.is_div_by_zero()==1 {
                                self.grid[row][col].formula.flag.set_is_div_by_zero(1);
                            }
                            if self.grid[row][col].formula.flag.type2()==0 {
                                if self.grid[row][col].formula.flag.cmd()==0 {
                                    self.grid[row][col].value=self.grid[param1_row][param1_col].value+self.grid[row][col].formula.param2;
                                }else if self.grid[row][col].formula.flag.cmd()==1 {
                                    self.grid[row][col].value=self.grid[param1_row][param1_col].value-self.grid[row][col].formula.param2;
                                }else if self.grid[row][col].formula.flag.cmd()==2 {
                                    self.grid[row][col].value=self.grid[param1_row][param1_col].value*self.grid[row][col].formula.param2;
                                }else{
                                    if self.grid[row][col].formula.param2==0 {
                                        self.grid[row][col].formula.flag.set_is_div_by_zero(1);
                                    }else{
                                        self.grid[row][col].value=self.grid[param1_row][param1_col].value/self.grid[row][col].formula.param2;
                                    }
                                    // self.grid[row][col].value=self.grid[param1_row][param1_col].value/self.grid[row][col].formula.param2;
                                }
                            }else if self.grid[row][col].formula.flag.type2()==1 {
                                // let param2_row=(self.grid[row][col].formula.param2%1000) as usize;
                                // let param2_col=(self.grid[row][col].formula.param2/1000) as usize;

                                let (param2_row,param2_col)=convert_to_index_int(self.grid[row][col].formula.param2);
                                if self.grid[param2_row][param2_col].formula.flag.is_div_by_zero()==1 {
                                    self.grid[row][col].formula.flag.set_is_div_by_zero(1);
                                }
                                if self.grid[row][col].formula.flag.cmd()==0 {
                                    self.grid[row][col].value=self.grid[param1_row][param1_col].value+self.grid[param2_row][param2_col].value;
                                }else if self.grid[row][col].formula.flag.cmd()==1 {
                                    self.grid[row][col].value=self.grid[param1_row][param1_col].value-self.grid[param2_row][param2_col].value;
                                }else if self.grid[row][col].formula.flag.cmd()==2 {
                                    self.grid[row][col].value=self.grid[param1_row][param1_col].value*self.grid[param2_row][param2_col].value;
                                }else{
                                    if self.grid[param2_row][param2_col].value==0 {
                                        self.grid[row][col].formula.flag.set_is_div_by_zero(1);
                                    }else{
                                        self.grid[row][col].value=self.grid[param1_row][param1_col].value/self.grid[param2_row][param2_col].value;
                                    }
                                    // self.grid[row][col].value=self.grid[param1_row][param1_col].value/self.grid[param2_row][param2_col].value;
                                }
                                

                            }
                        }
                }else{
                    let (param1_row,param1_col)=convert_to_index_int(self.grid[row][col].formula.param1);
                    if self.grid[param1_row][param1_col].formula.flag.is_div_by_zero()==1 {
                        self.grid[row][col].formula.flag.set_is_div_by_zero(1);
                    }
                    let (param2_row, param2_col) = convert_to_index_int(self.grid[row][col].formula.param2);
                    if self.grid[param2_row][param2_col].formula.flag.is_div_by_zero() == 1 {
                        self.grid[row][col].formula.flag.set_is_div_by_zero(1);
                    }
                    if self.grid[row][col].formula.flag.cmd()==0 {
                        self.grid[row][col].value= self.minimum(param1_row ,param2_row, param1_col, param2_col) ;
                    }
                    else if self.grid[row][col].formula.flag.cmd()==1 {
                        self.grid[row][col].value= self.maximum(param1_row ,param2_row, param1_col, param2_col) ;
                    }
                    else if self.grid[row][col].formula.flag.cmd()==2 {
                        self.grid[row][col].value= self.sum(param1_row ,param2_row, param1_col, param2_col) ;
                    }
                    else if self.grid[row][col].formula.flag.cmd()==3 {
                        self.grid[row][col].value= self.average(param1_row ,param2_row, param1_col, param2_col) ;
                    }
                    else if self.grid[row][col].formula.flag.cmd()==4 {
                        self.grid[row][col].value= self.stddev(param1_row ,param2_row, param1_col, param2_col) ;
                    }
            }
        }

        }
        
            pub fn update_cell_data(&mut self, row :usize , col :usize, new_formula: String ) {
                let mut command = parse_formula(&new_formula);
                self.set_dependicies_cell(row as usize, col as usize, command.clone());
                let topo_vec = self.toposort(row*ENCODE_SHIFT+col);   
                if topo_vec==vec![] {
                    command.flag.set_error(1);
                }
                else{
                self.update_cell(topo_vec);}
            }
            pub fn get_value(&self, row : i32, col : i32) -> i32{
                self.grid[row as usize][col as usize].value
            }  
}