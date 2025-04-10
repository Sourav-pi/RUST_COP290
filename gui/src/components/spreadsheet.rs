use core::num;
use std::path::PathBuf;
use dioxus::events::Key;
use cores::Sheet;
use std::sync::{Arc, Mutex};

use dioxus::prelude::*;
use super::header::Header;
use super::grid::Grid;
use super::graph_popup::GraphPopup;
use super::context_menu::{ContextMenu,MenuType};

// Define explicit types for your contexts
pub type SelectedCellContext = Signal<(i32, i32)>;
pub type FormulaContext = Signal<String>;
pub type CurrentFileContext = Signal<Option<PathBuf>>;
pub type GraphPopupContext = Signal<bool>;
pub type GraphTypeContext = Signal<GraphType>;
pub type ContextMenuContext = Signal<Option<(f64, f64, i32, i32, MenuType)>>;
pub type SheetContext = Signal<Arc<Mutex<Sheet>>>;
pub type SheetVersionContext = Signal<i32>;

#[derive(Clone, Copy, PartialEq)]
pub enum GraphType {
    Line,
    Pie,
    Scatter,
    Bar,   
}

#[component]
pub fn Spreadsheet() -> Element {
    let mut num_rows = 30;
    let mut num_cols = 30;

    // Create the signals for context
    let mut selected_cell : SelectedCellContext = use_signal(|| (0, 0));
    let formula : FormulaContext = use_signal(|| String::new());
    let current_file : CurrentFileContext = use_signal(|| None);
    let graph_popup : GraphPopupContext = use_signal(|| false);
    let graph_type : GraphTypeContext = use_signal(|| GraphType::Line);
    let context_menu : ContextMenuContext = use_signal(|| None);
    let sheet: SheetContext = use_signal(|| {
        let new_sheet = Sheet::new(num_rows,num_cols); // Create a new Sheet instance
        Arc::new(Mutex::new(new_sheet))
    });
    let sheet_version: SheetVersionContext = use_signal(|| 0);
    
    
    let mut filename = "new_file.xlsx".to_string();
    if let Some(file) = current_file.cloned() {
        filename = file.file_name().unwrap().to_str().unwrap().to_string();
    }

    // Handler for arrow key navigation
    let keydown_handler = move |event: Event<KeyboardData>| {

        // Don't process if modifier keys are held down (for shortcuts)
        if event.modifiers().meta() || event.modifiers().ctrl() || event.modifiers().alt() {
            return;
        }

        let (cur_row, cur_col) = selected_cell.cloned();
        
        let max_rows = num_rows as i32;
        
        // Calculate the new cell based on arrow key pressed
        let new_cell = match event.key() {
            Key::ArrowUp => {
                event.prevent_default();
                if cur_row > 0 {
                    (cur_row - 1, cur_col)
                } else {
                    (cur_row, cur_col) // Stay at current cell if at top edge
                }
            },
            Key::ArrowDown=> {
                event.prevent_default();
                if cur_row < max_rows - 1 {
                    (cur_row + 1, cur_col)
                } else {
                    (cur_row, cur_col) // Stay at current cell if at bottom edge
                }
            },
            Key::Enter => {
                event.prevent_default();
                if cur_row < max_rows - 1 {
                    (cur_row + 1, cur_col) // Move down
                } else {
                    (cur_row, cur_col) // Stay at current cell if at bottom edge
                }
            },
            _ => (cur_row, cur_col) // No change for other keys
        };
        
        // Update selected cell if it changed
        if new_cell != (cur_row, cur_col) {
            selected_cell.set(new_cell);
            println!("Selected cell changed to: ({}, {})", new_cell.0, new_cell.1);
        }
    };

    // Provide the contexts to the components
    provide_context(selected_cell);
    provide_context(formula);
    provide_context(current_file);
    provide_context(graph_popup);
    provide_context(graph_type);
    provide_context(context_menu);
    provide_context(sheet);
    provide_context(sheet_version);

    rsx! {
        div {
            // Global keyboard event listener
            tabindex: 0, // Makes the div focusable
            onkeydown: keydown_handler,
            style: "outline: none; width: 100%; height: 100%;",
            
            Header { 
                filename: filename.clone(),
            }
            Grid {
                num_rows: num_rows as i32,
                num_cols: num_cols as i32,
            }
            GraphPopup {},
            ContextMenu {}
        }
    }
}