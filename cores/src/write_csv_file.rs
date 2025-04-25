use crate::sheet::Sheet;

fn column_to_letter(col: usize) -> String {
    let mut result = String::new();
    let mut c = col;
    while c > 0 {
        c -= 1;
        let letter = (c % 26) as u8 + b'A';
        result.insert(0, letter as char);
        c /= 26;
    }
    result
}

impl Sheet {
    pub fn write_csv_file(&self, filename: &str) -> Result<(), std::io::Error> {
        let file = std::fs::File::create(filename)?;
        let mut writer = csv::Writer::from_writer(file);
        // Write the header row with column letters
        let mut header: Vec<String> = Vec::new();
        header.push("".to_string()); // Empty cell for the first column
        for i in 0..self.col {
            let col_letter = column_to_letter(i + 1);
            header.push(col_letter);
        }
        writer.write_record(&header)?;
        for i in 1..self.grid.len() {
            // Use the better approach mentioned in the comment
            let mut row: Vec<String> = self.grid[i]
                .iter()
                .map(|cell| cell.value.to_string())
                .collect();
            row[0] = (i).to_string();
            writer.write_record(&row)?;
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
        test_sheet
            .write_csv_file("../temp/test_output.csv")
            .unwrap();
        // Add assertions to verify the contents of the CSV file if needed
    }
}
