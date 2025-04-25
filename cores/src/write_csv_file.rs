
use crate::sheet::Sheet;

impl Sheet {
    pub fn write_csv_file(&self, filename: &str) -> Result<(), std::io::Error> {
        let file = std::fs::File::create(filename)?;
        let mut writer = csv::Writer::from_writer(file);

        for i in 0..self.grid.len() {
            let mut row_data = String::new();
            for j in 0..self.grid[i].len() {
                let csv_data = self.grid[i][j].value.clone();
                row_data = format!("{},{}", row_data, csv_data);
            }
            // Since we're manually building a row, trim the leading comma and write it
            if !row_data.is_empty() {
                row_data = row_data[1..].to_string();
            }
            writer.write_record(&[row_data])?;

            // A better approach would be:
            // let row: Vec<String> = self.grid[i].iter().map(|cell| cell.value.clone()).collect();
            // writer.write_record(&row)?;
        }
        writer.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::sheet::Sheet;

    #[test]
    fn test_write_csv_file() {
        let mut test_sheet = Sheet::new(6, 6);
        test_sheet.update_cell_data(1, 1, String::from("10"));
        test_sheet.write_csv_file("../temp/test_output.csv").unwrap();
        // Add assertions to verify the contents of the CSV file if needed
    }
}