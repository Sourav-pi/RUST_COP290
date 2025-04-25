use crate::parse::CommandFlag;
use crate::{parse::CommandCall, sheet::*};
impl Sheet {
    pub fn read_csv_file(&mut self, filename: &str) -> Result<(), std::io::Error> {
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
        let mut row = 1;
        // Read the header row and skip it
        let _ = reader.headers()?;
        for result in reader.deserialize() {
            let cell_data: Vec<String> = result?;
            // Skip the first row if it contains headers
            for (i, item) in cell_data.iter().enumerate().skip(1) {
                if item.is_empty() {
                    continue;
                }
                let col = i;
                if col >= self.col {
                    break;
                }
                self.grid[row][col].value = item.parse::<i32>().unwrap();
            }
            row += 1;
        }
        Ok(())
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
