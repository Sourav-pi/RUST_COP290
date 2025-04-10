use serde::{self,Serialize};
use crate::sheet::{Sheet, Cell};
use serde::ser::{ Serializer, SerializeStruct};
struct csv_store{
    row: i32,
    col: i32,
    data:Cell
}
impl Serialize for csv_store {
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
            "type: {}, cmd: {}, type1: {}, type2: {}, error: {}, div_by_zero: {}",
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
        let depend_str = self
            .data.depend
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<_>>()
            .join(",");
        state.serialize_field("depend", &depend_str)?;
    
        state.end()
    }
}    

pub fn write_csv(data: Sheet, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Get the dimensions
    let num_rows = data.grid.len();
    let num_cols = if num_rows > 0 { data.grid[0].len() } else { 0 };
    let mut wtr = csv::Writer::from_path(file_path)?;
    for row in 0..num_rows {
        for col in 0..num_cols {
            let cell = &data.grid[row][col];
            let csv_data = csv_store {
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