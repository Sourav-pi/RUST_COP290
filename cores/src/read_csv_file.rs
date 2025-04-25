use crate::parse::CommandFlag;
use crate::{parse::CommandCall, sheet::*};
impl Sheet{
    pub fn read_csv_file(& mut self, filename: &str) -> Result<(), std::io::Error> {
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
        let file = std::fs::File::open(filename)?;
        let mut reader = csv::Reader::from_reader(file);
        let mut row=1;
        for result in reader.deserialize(){
            let record:String = result?;
            let cell_data: Vec<String> = record.split(',').map(|s| s.to_string()).collect();
            // Skip the first row if it contains headers
            for i in 0..cell_data.len(){
                if cell_data[i].len() == 0{
                    continue;
                }
                let col = i;
                if col >= self.col{
                    break;
                }
                self.grid[row][col].value = cell_data[i].parse::<i32>().unwrap();
            }
            row += 1;
            
        }
        return Ok(());

}
}

#[cfg(test)]
mod tests {
    use crate::sheet::Sheet;

    #[test]
    fn test_read_csv_file() {
        let mut test_sheet = Sheet::new(6, 6);
        test_sheet.read_csv_file("../temp/test_output.csv").unwrap();
        assert!(test_sheet.grid[1][1].value == 10);
    }
}