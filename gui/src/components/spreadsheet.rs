use std::path::PathBuf;

use dioxus::prelude::*;
use super::header::Header;
use super::grid::Grid;
use super::graph_popup::GraphPopup;

// Define explicit types for your contexts
pub type SelectedCellContext = Signal<(i32, i32)>;
pub type FormulaContext = Signal<String>;
pub type CurrentFileContext = Signal<Option<PathBuf>>;
pub type GraphPopupContext = Signal<bool>;
pub type GraphTypeContext = Signal<GraphType>;

#[derive(Clone, Copy, PartialEq)]
pub enum GraphType {
    Line,
    Pie,
    Scatter,
    Bar,   
}

#[component]
pub fn Spreadsheet() -> Element {
    // Create the signals for context
    let selected_cell : SelectedCellContext = use_signal(|| (0, 0));
    let formula : FormulaContext = use_signal(|| String::new());
    let current_file : CurrentFileContext = use_signal(|| None);
    let graph_popup : GraphPopupContext = use_signal(|| false);
    let graph_type : GraphTypeContext = use_signal(|| GraphType::Line);
    
    let mut filename = "new_file.xlsx".to_string();
    if let Some(file) = current_file.cloned() {
        filename = file.file_name().unwrap().to_str().unwrap().to_string();
    }


    // Provide the contexts to the components
    provide_context(selected_cell);
    provide_context(formula);
    provide_context(current_file);
    provide_context(graph_popup);
    provide_context(graph_type);

    rsx! {
        Header { 
            filename: filename.clone(),
        }
        Grid {
            num_rows: 30,
            num_cols: 20,
        }
        GraphPopup{},
    }
}
