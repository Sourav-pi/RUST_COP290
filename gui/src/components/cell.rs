use dioxus::prelude::*;
use dioxus::events::Key;
// Import context types from spreadsheet module
use super::spreadsheet::{SelectedCellContext, FormulaContext, SheetContext};

const CELL_STYLE: &str = "
    width: 80px;
    height: 30px;
    border: 1px solid #ccc;
    outline: none;
    text-align: center;
";

const CELL_HEADER_STYLE: &str = "
    width: 80px;
    height: 30px;
    border: 1px solid #ccc;
    background-color: #f3f3f3;
    font-weight: bold;
    position: sticky;
    top: 90px;
    left: 0px;
    z-index: 10;
    outline: none;
    text-align: center;
    font-size: 16px;
";
const CELL_SELECTED_STYLE: &str = "
    width: 80px;
    height: 30px;
    border: 1px solid blue;
    outline: none;
    text-align: center;
";

#[derive(Props, PartialEq, Clone)]
pub struct CellProps {
    pub row: i32,
    pub col: i32,
    #[props(default = false)]
    pub is_header: bool,
    #[props(default = false)]
    pub is_selected: bool,
    #[props(default = 80)]
    pub min_width: i32,
    #[props(into)]
    pub oncontextmenu: Callback<dioxus::prelude::Event<dioxus::events::MouseData>>,
}

// Dummy function to simulate formula evaluation
fn evaluate_formula(formula: &str, row: i32, col: i32) -> String {
   return format!("{} hi", formula);
}

#[component]
pub fn Cell(props: CellProps) -> Element {
    // Consume the contexts
    let mut selected_cell = use_context::<SelectedCellContext>();
    let mut formula = use_context::<FormulaContext>();
    let mut sheet = use_context::<SheetContext>();
    
    let mut is_editing = use_signal(|| false);
    let mut formula_local = use_signal(|| String::new());
    let mut displayed_local = use_signal(|| String::new());
    
    // Track previous selection state
    let mut was_selected_before = use_signal(|| false);

    // Check if this cell is selected based on context
    let is_this_cell_selected = {
        let (sel_row, sel_col) = *selected_cell.read();
        props.row == sel_row && props.col == sel_col
    };

    // Move the selection handling to a use_effect to avoid infinite loops
    use_effect(move || {
        if is_this_cell_selected && !*was_selected_before.read() {
            was_selected_before.set(true);
            
            // Focus this cell
            let script = format!(
                "setTimeout(function() {{
                    const el = document.getElementById('row-{}-col-{}');
                    if (el) {{
                        el.focus();
                    }}
                }}, 10);",
                props.row, props.col
            );
            document::eval(&script);
        } else if !is_this_cell_selected && *was_selected_before.read() {
            was_selected_before.set(false);
        }
    });

    // Handler for when user starts editing
    let on_focus = {
        let row = props.row; 
        let col = props.col;
        move |_| {
            is_editing.set(true);
            
            // Update the selected cell in the context
            selected_cell.set((row, col));
            
            // Update formula context with this cell's formula
            formula.set(formula_local.cloned());
        }
    };
    
    // Handler for when the cell loses focus
    let on_blur = {
        let row = props.row;
        let col = props.col;
        move |_| {
            is_editing.set(false);
            
            // Get the formula from context in case it was updated elsewhere
            let formula_text = formula.read().clone();
            
            // Evaluate the formula and update the displayed value
            if !formula_text.is_empty() {
                if let Ok(mut sheet_locked) = sheet.cloned().lock() {
                    // Update the cell value in the Sheet object
                    sheet_locked.update_cell_data(row as usize, col as usize, formula_text.clone());
                    // Update the displayed value
                    displayed_local.set(sheet_locked.get_value(row, col).to_string());
                    println!("Updated cell ({}, {}) to: {}", row, col, formula_text);
                }
            }
        }
    };
    
    // Handler for keyboard events
    let on_keydown = {
        let row = props.row;
        let col = props.col;
        move |e: Event<KeyboardData>| {
            if e.key() == Key::Enter {
                is_editing.set(false);
            
            // Get the formula from context in case it was updated elsewhere
            let formula_text = formula.read().clone();
            
            // Evaluate the formula and update the displayed value
            if !formula_text.is_empty() {
                if let Ok(mut sheet_locked) = sheet.cloned().lock() {
                    // Update the cell value in the Sheet object
                    sheet_locked.update_cell_data(row as usize, col as usize, formula_text.clone());
                    // Update the displayed value
                    displayed_local.set(sheet_locked.get_value(row, col).to_string());
                    
                    println!("Updated cell ({}, {}) to: {}", row, col, formula_text);
                }
            }
            }
        }
    };
    
    // Handler for input changes
    let on_input = move |e: Event<FormData>| {
        let new_value = e.value().clone();
        formula_local.set(new_value.clone());
        
        // Also update the global formula context
        formula.set(new_value);
    };
    
    // Handle cell click to select it
    let on_click = {
        let row = props.row;
        let col = props.col;
        move |_| {
            // Update selected cell in context
            selected_cell.set((row, col));
            
            // Update formula context with this cell's formula
            formula.set(formula_local.cloned());
            
            // Directly focus this cell
            let script = format!(
                "setTimeout(function() {{
                    const el = document.getElementById('row-{}-col-{}');
                    console.log('Focusing on cell: row-{}-col-{}');
                    if (el) {{ el.focus(); }}
                }}, 10);",
                row, col,row,col
            );
            document::eval(&script);
        }
    };
    
    if props.is_header {
        rsx!{
            input {
                id: "row-{props.row}-col-{props.col}",
                readonly: true,
                value: props.col.to_string(),
                style: CELL_HEADER_STYLE,
                class: "cell"
            }
        }
    } else {
        rsx! {
            input {
                id: "row-{props.row}-col-{props.col}",
                onfocus: on_focus,
                onblur: on_blur,
                onkeydown: on_keydown,
                oninput: on_input,
                onclick: on_click,
                oncontextmenu: props.oncontextmenu,
                // Show the formula when editing, otherwise show the result
                value: if *is_editing.read() {
                    formula_local.cloned()
                } else {
                    displayed_local.cloned()
                },
                style: if props.is_header {
                    CELL_HEADER_STYLE
                } else if is_editing.cloned() || is_this_cell_selected {
                    CELL_SELECTED_STYLE
                } else {
                    CELL_STYLE
                },
                class: "cell"
            }
        }
    }
}