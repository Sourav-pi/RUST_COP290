use serde::Serialize;

use crate::sheet::Sheet;

pub fn write_csv(data: Sheet, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut wtr = csv::Writer::from_path(file_path)?;
    for row in data.grid{
        for cell in row{
            let serialized_data = cell.serialize(); // Replace with your actual method name
            wtr.serialize(serialized_data)?;
        }   
    }
    wtr.flush()?;
    Ok(())  
}