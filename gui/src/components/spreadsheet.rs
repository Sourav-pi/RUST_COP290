use dioxus::prelude::*;
use super::header::Header;
use super::grid::Grid;

// Define explicit types for your contexts
pub type SelectedCellContext = Signal<(i32, i32)>;
pub type FormulaContext = Signal<String>;

#[component]
pub fn Spreadsheet() -> Element {
    let filename = "test.xlsx".to_string();

    // Create the signals for context
    let selected_cell = use_signal(|| (0, 0));
    let formula = use_signal(|| String::new());

    // Provide context with explicit types
    provide_context(selected_cell);
    provide_context(formula);

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
