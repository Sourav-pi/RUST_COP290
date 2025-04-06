
mod parse;
use parse::{parse_formula, CommandCall};
struct Cell{
    value:i32,
    formula:CommandCall,
    depend:Vec<i32>,
    is_error:bool,
    
}

struct Sheet{
    grid:Vec<Vec<Cell>>,
}
impl Sheet ( &mut self, row:usize, col:usize, command :CommandCall){
    fn set_dependicies_cell(&mut self, row: usize, col: usize, command: CommandCall) {
        self.grid[row][col].formula=command;
    if(command.type_==0){
        if(command.type1==0){
            self.grid[row][col].value=command.param1;
        }else if(command.type1==1){
            self.grid[row][col].depend.push(command.param1);
           
        }
    }else if(command.type_==1){
        if(command.type1==0){
            if(command.type2==0){
                if(command.cmd==0){
                    self.grid[row][col].value=command.param1+command.param2;
                }else if(command.cmd==1){
                    self.grid[row][col].value=command.param1-command.param2;
                }else if(command.cmd==2){
                    self.grid[row][col].value=command.param1*command.param2;
                }else (command.cmd==3){
                    self.grid[row][col].value=command.param1/command.param2;
                }
            }else{
                self.grid[row][col].depend.push(command.param2);
            }
        }else if(command.type1==1){
            self.grid[row][col].depend.push(command.param1);
            if(command.type2==0){
                
            }else if(command.type2==1){
                self.grid[row][col].depend.push(command.param2);
            }
        }   
}else{
    self.grid[row][col].depend.push(command.param1);
    self.grid[row][col].depend.push(command.param2);
}
    }
    fn update_cell (&mut self, list_fpr_update:Vec<i32>){
        for i in list_fpr_update{
            row=i%1000;
            col=i/1000;
            if (self.grid[row][col].formula.type_==0){
                if(self.grid[row][col].formula.type1==0){
                    self.grid[row][col].value=self.grid[row][col].formula.param1;
                }else if(self.grid[row][col].formula.type1==1){
                    param1_row=self.grid[row][col].formula.param1%1000;
                    param1_col=self.grid[row][col].formula.param1/1000;
                    self.grid[row][col].value=self.grid[param1_row][param1_col].value;
                }
            }
            else if (self.grid[row][col].formula.type_==1){
                if(self.grid[row][col].formula.type1==0){
                    if(self.grid[row][col].formula.type2==0){
                        if(self.grid[row][col].formula.cmd==0){
                            self.grid[row][col].value=self.grid[row][col].formula.param1+self.grid[row][col].formula.param2;
                        }else if(self.grid[row][col].formula.cmd==1){
                            self.grid[row][col].value=self.grid[row][col].formula.param1-self.grid[row][col].formula.param2;
                        }else if(self.grid[row][col].formula.cmd==2){
                            self.grid[row][col].value=self.grid[row][col].formula.param1*self.grid[row][col].formula.param2;
                        }else {
                            self.grid[row][col].value=self.grid[row][col].formula.param1/self.grid[row][col].formula.param2;
                        }
                    }else{
                        param2_row=self.grid[row][col].formula.param2%1000;
                        param2_col=self.grid[row][col].formula.param2/1000;
                        if(self.grid[row][col].formula.cmd==0){
                            self.grid[row][col].value=self.grid[row][col].formula.param1+self.grid[param2_row][param2_col].value;
                        }else if(self.grid[row][col].formula.cmd==1){
                            self.grid[row][col].value=self.grid[row][col].formula.param1-self.grid[param2_row][param2_col].value;
                        }else if(self.grid[row][col].formula.cmd==2){
                            self.grid[row][col].value=self.grid[row][col].formula.param1*self.grid[param2_row][param2_col].value;
                        }else{
                            self.grid[row][col].value=self.grid[row][col].formula.param1/self.grid[param2_row][param2_col].value;
                        }
                        
                    }
                }else if (self.grid[row][col].formula.type1==1){
                    param1_row=self.grid[row][col].formula.param1%1000;
                    param1_col=self.grid[row][col].formula.param1/1000;
                    
                    if(self.grid[row][col].formula.type2==0){
                        if(self.grid[row][col].formula.cmd==0){
                            self.grid[row][col].value=self.grid[param1_row][param1_col].value+self.grid[row][col].formula.param2;
                        }else if(self.grid[row][col].formula.cmd==1){
                            self.grid[row][col].value=self.grid[param1_row][param1_col].value-self.grid[row][col].formula.param2;
                        }else if(self.grid[row][col].formula.cmd==2){
                            self.grid[row][col].value=self.grid[param1_row][param1_col].value*self.grid[row][col].formula.param2;
                        }else{
                            self.grid[row][col].value=self.grid[param1_row][param1_col].value/self.grid[row][col].formula.param2;
                        }
                    }else if (self.grid[row][col].formula.type2==1){
                        param2_row=self.grid[row][col].formula.param2%1000;
                        param2_col=self.grid[row][col].formula.param2/1000;
                        if(self.grid[row][col].formula.cmd==0){
                            self.grid[row][col].value=self.grid[param1_row][param1_col].value+self.grid[param2_row][param2_col].value;
                        }else if(self.grid[row][col].formula.cmd==1){
                            self.grid[row][col].value=self.grid[param1_row][param1_col].value-self.grid[param2_row][param2_col].value;
                        }else if(self.grid[row][col].formula.cmd==2){
                            self.grid[row][col].value=self.grid[param1_row][param1_col].value*self.grid[param2_row][param2_col].value;
                        }else{
                            self.grid[row][col].value=self.grid[param1_row][param1_col].value/self.grid[param2_row][param2_col].value;
                        }

                    }
                }
        }else{

        }
    }
}

}

fn main (){
    grid = vec![vec![Cell{value:0,formula:CommandCall::new(),is_formula:false};10];10];

}