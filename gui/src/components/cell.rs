use dioxus::prelude::*;
use super::spreadsheet::*;

const CELL_STYLE: &str = "
    width: 81px;
    height: 31px;
    border: 1px solid #ccc;
    outline: none;
    text-align: center;
";

const CELL_HEADER_STYLE: &str = "
    width: 81px;
    height: 31px;
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
    width: 81px;
    height: 31px;
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


#[component]
pub fn Cell(props: CellProps) -> Element {
    // Consume the contexts
    let mut selected_cell = use_context::<SelectedCellContext>();
    let sheet = use_context::<SheetContext>();
    let mut sheetversion = use_context::<SheetVersionContext>();
    
    let mut is_editing = use_signal(|| false);
    let mut formula = use_signal(|| String::new());
    let mut value = use_signal(|| String::new());
    

    // Check if this cell is selected based on context
    let is_this_cell_selected = {
        let (sel_row, sel_col) = *selected_cell.read();
        props.row == sel_row && props.col == sel_col
    };

    // Handler for when user starts editing
    let on_focus = {
        let row = props.row; 
        let col = props.col;
        move |_| {
            is_editing.set(true);
            selected_cell.set((row, col));
            println!("Selected cell ({}, {}) for editing", row, col);
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
                    sheetversion.set(sheetversion.cloned() + 1);
                    // Update the displayed value
                    println!("Updated cell ({}, {}) to: {}", row, col, formula_text);
                }
            }
        }
    };
    
    
    // Handler for input changes
    let on_input = move |e: Event<FormData>| {
        let new_value = e.value().clone();
        formula.set(new_value);
    };

    use_effect(move || {
        let _ = sheetversion.cloned();
        if let Ok(sheet_locked) = sheet.cloned().lock() {
            // Update the cell value in the Sheet object
            formula.set(sheet_locked.get_formula(props.row as usize, props.col as usize).to_string());
        }

    });
    

    // use_effect(move ||{
        let _ = sheetversion.cloned();

        if let Ok(sheet_locked) = sheet.cloned().lock() {
            // Update the cell value in the Sheet object
            value.set(sheet_locked.get_value(props.row, props.col).to_string());
        }
    // });
    
    
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
                oninput: on_input,
                oncontextmenu: props.oncontextmenu,
                // Show the formula when editing, otherwise show the result
                value: if *is_editing.read() {
                    formula.cloned()
                } else {
                    value.cloned()
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