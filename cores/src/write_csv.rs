use crate::sheet::{Cell, Sheet};
use serde::ser::{SerializeStruct, Serializer};
use serde::{self, Serialize};
struct CsvStore {
    row: i32,
    col: i32,
    data: Cell,
}
impl Serialize for CsvStore {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // We'll output a flat record with five fields:
        // "value", "flag", "param1", "param2", and "depend".
        let mut state = serializer.serialize_struct("Cell", 5)?;
        // Serialize the simple scalar value.
        state.serialize_field("row", &self.row)?;
        state.serialize_field("col", &self.col)?;
        state.serialize_field("value", &self.data.value)?;
        // Format the nested CommandFlag into a string.
        let flag_str = format!(
            "type:{},cmd:{},type1:{},type2:{},error:{},div_by_zero:{}",
            self.data.formula.flag.type_(),
            self.data.formula.flag.cmd(),
            self.data.formula.flag.type1(),
            self.data.formula.flag.type2(),
            self.data.formula.flag.error(),
            self.data.formula.flag.is_div_by_zero(),
        );
        state.serialize_field("flag", &flag_str)?;

        // Serialize CommandCall's parameters.
        state.serialize_field("param1", &self.data.formula.param1)?;
        state.serialize_field("param2", &self.data.formula.param2)?;

        // Convert the dependency vector into a comma-separated string.
        // let depend_str = self
        //     .data
        //     .depend
        //     .iter()
        //     .map(|d| d.to_string())
        //     .collect::<Vec<_>>()
        //     .join(",");
        // state.serialize_field("depend", &depend_str)?;

        state.end()
    }
}

impl Sheet {
    pub fn write_csv(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Get the dimensions
        let num_rows = self.grid.len();
        let num_cols = if num_rows > 0 { self.grid[0].len() } else { 0 };
        let mut wtr = csv::Writer::from_path(file_path)?;
        for row in 0..num_rows {
            for col in 0..num_cols {
                if self.grid[row][col].formula.flag.is_any() == 0 {
                    continue;
                }
                let cell = &self.grid[row][col];
                let csv_data = CsvStore {
                    row: row as i32,
                    col: col as i32,
                    data: cell.clone(),
                };
                wtr.serialize(csv_data)?;
            }
        }
        wtr.flush()?;
        Ok(())
    }
}



// #[test]
// fn test_write_csv() {
//     let mut test_sheet = Sheet::new(6, 6);
//     test_sheet.update_cell_data(1, 1, String::from("AVG(A2:D5)"));
//     test_sheet.update_cell_data(2, 2, String::from("500"));
//     test_sheet.update_cell_data(2, 1, String::from("B1+B5"));
//     println!("{}", test_sheet.get_value(1, 1));
//     let res = test_sheet.write_csv("/Users/aditya/Downloads/sem4/cop290/temp.csv");
//     match res {
//         Ok(_) => println!("CSV file written successfully."),
//         Err(e) => println!("Error writing CSV file: {}", e),
//     }
// }

