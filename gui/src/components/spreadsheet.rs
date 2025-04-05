use std::path::PathBuf;

use dioxus::prelude::*;
use super::header::Header;
use super::grid::Grid;

// Define explicit types for your contexts
pub type SelectedCellContext = Signal<(i32, i32)>;
pub type FormulaContext = Signal<String>;
pub type CurrentFileContext = Signal<Option<PathBuf>>;

#[component]
pub fn Spreadsheet() -> Element {
    // Create the signals for context
    let selected_cell : SelectedCellContext = use_signal(|| (0, 0));
    let formula : FormulaContext = use_signal(|| String::new());
    let current_file : CurrentFileContext = use_signal(|| None);
    
    let mut filename = "new_file.xlsx".to_string();
    if let Some(file) = current_file.cloned() {
        filename = file.file_name().unwrap().to_str().unwrap().to_string();
    }


    // Provide the contexts to the components
    provide_context(selected_cell);
    provide_context(formula);
    provide_context(current_file);

    rsx! {
        Header { 
            filename: filename.clone(),
        }
        Grid {
            num_rows: 30,
            num_cols: 20,
        }
    }
}
