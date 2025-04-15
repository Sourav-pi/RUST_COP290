use std::path::PathBuf;
use cores::Sheet;
use std::sync::{Arc, Mutex};
use std::vec::Vec;

use dioxus::prelude::*;
use super::header::Header;
use super::grid::Grid;
use super::graph_popup::GraphPopup;
use super::context_menu::{ContextMenu,MenuType};
use super::error_display::ErrorDisplay;

// Define explicit types for your contexts
pub type SelectedCellContext = Signal<(i32, i32)>;
pub type FormulaContext = Signal<String>;
pub type CurrentFileContext = Signal<Option<PathBuf>>;
pub type GraphPopupContext = Signal<bool>;
pub type GraphTypeContext = Signal<GraphType>;
pub type ContextMenuContext = Signal<Option<(f64, f64, i32, i32, MenuType)>>;
pub type SheetContext = Signal<Arc<Mutex<Sheet>>>;
pub type SheetVersionContext = Signal<i32>;
pub type StartRowContext = Signal<i32>;
pub type StartColContext = Signal<i32>;
pub type MaxStartRowContext = Signal<i32>;
pub type MaxStartColContext = Signal<i32>;
pub type ErrorContext = super::error_display::ErrorContext;

#[derive(Clone, Copy, PartialEq)]
pub enum GraphType {
    Line,
    Pie,
    Scatter,
    Bar,   
}

#[component]
pub fn Spreadsheet() -> Element {
    let num_rows = 999;
    let num_cols = 18278;

    // Create the signals for context
    let selected_cell : SelectedCellContext = use_signal(|| (1, 1));
    let formula : FormulaContext = use_signal(|| String::new());
    let current_file : CurrentFileContext = use_signal(|| None);
    let graph_popup : GraphPopupContext = use_signal(|| false);
    let graph_type : GraphTypeContext = use_signal(|| GraphType::Line);
    let context_menu : ContextMenuContext = use_signal(|| None);
    let sheet: SheetContext = use_signal(|| {
        let new_sheet = Sheet::new(num_rows,num_cols); // Create a new Sheet instance
        Arc::new(Mutex::new(new_sheet))
    });
    let start_row: StartRowContext = use_signal(|| 1);
    let start_col: StartColContext = use_signal(|| 1);
    let max_start_row: MaxStartRowContext = use_signal(|| 1);
    let max_start_col: MaxStartColContext = use_signal(|| 1);
    let sheet_version: SheetVersionContext = use_signal(|| 0);
    let error_ctx: ErrorContext = use_signal(|| None);
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
    provide_context(context_menu);
    provide_context(sheet);
    provide_context(sheet_version);
    provide_context(start_row);
    provide_context(start_col);
    provide_context(max_start_row);
    provide_context(max_start_col);
    provide_context(error_ctx);

    use_effect(move||{
        let _ = document::eval("
            document.getElementById('row-1-col-1').focus();
        
        ");
    });

    rsx! {
        div {
            // Global keyboard event listener
            tabindex: 0, // Makes the div focusable
            style: "outline: none; width: 100%; height: 100%; overflow: hidden;",
            
            Header { 
                filename: filename.clone(),
                num_rows: num_rows,
                num_cols: num_cols,
            }
            Grid {
                num_rows: num_rows as i32,
                num_cols: num_cols as i32,
            }
            GraphPopup {},
            ContextMenu {},
            ErrorDisplay {}
        }
    }
}