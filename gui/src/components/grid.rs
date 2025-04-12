use dioxus::prelude::*;
use dioxus_elements::{ol::start, option::selected};
use crate::components::spreadsheet::{SheetVersionContext, StartRowContext, StartColContext,SelectedCellContext};
use super::row::Row;
use std::rc::Rc;

const GRID_STYLE: &str = "
    overflow: hidden;
    height: calc(100vh - 120px);
    width: 100%;
    display: block;
    position: relative;
    border: 1px solid #e0e0e0;
    box-shadow: inset 0 0 5px rgba(0,0,0,0.1);
";

const TABLE_STYLE: &str = "
    display: table;
    min-width: max-content;
    border-collapse: collapse;
";

const NAVIGATION_CONTROLS_STYLE: &str = "
    position: absolute;
    bottom: 16px;
    right: 16px;
    display: flex;
    gap: 8px;
    z-index: 100;
";

const NAV_BUTTON_STYLE: &str = "
    width: 40px;
    height: 40px;
    background-color: rgba(255, 255, 255, 0.8);
    border: 1px solid #ccc;
    border-radius: 50%;
    display: flex;
    justify-content: center;
    align-items: center;
    font-size: 20px;
    cursor: pointer;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    &:hover {
        background-color: rgba(240, 240, 240, 0.9);
    }
";

const PAGE_INFO_STYLE: &str = "
    background-color: rgba(255, 255, 255, 0.8);
    padding: 5px 10px;
    border-radius: 12px;
    border: 1px solid #ccc;
    font-size: 14px;
    box-shadow: 0 1px 3px rgba(0,0,0,0.1);
";

#[derive(Props, PartialEq, Clone)]
pub struct GridProps {
    pub num_rows: i32,
    pub num_cols: i32,
}

#[component]
pub fn Grid(props: GridProps) -> Element {
    let mut start_row_ctx = use_context::<StartRowContext>();
    let mut start_col_ctx = use_context::<StartColContext>();
    let mut selected_cell = use_context::<SelectedCellContext>();

    let min_cell_width = 80; // Minimum width per cell in pixels
    
    // Visible rows per page
    let rows_per_page = 23; 
    let cols_per_page = 18;
    
    
    // Calculate max pages
    let max_start_row = (props.num_rows - rows_per_page).max(0);
    let max_start_col = (props.num_cols - cols_per_page).max(0);
    
    let mut move_up_help = move || {
        if start_row_ctx.cloned() > 0 {
            start_row_ctx.set(start_row_ctx.cloned() - 1);
        }
    };
    
    let mut move_down_help = move || {
        if start_row_ctx.cloned() < max_start_row {
            start_row_ctx.set(start_row_ctx.cloned() + 1);
        }
    };
    
    let mut move_left_help = move || {
        if start_col_ctx.cloned() > 0 {
            start_col_ctx.set(start_col_ctx.cloned() - 1);
        }
    };
    
    let mut move_right_help = move || {
        if start_col_ctx.cloned() < max_start_col {
            start_col_ctx.set(start_col_ctx.cloned() + 1);
        }
    };


    // Navigation handlers
    let move_up = move |_| {
            move_up_help();
    };
    
    let move_down = move |_| {
            move_down_help();
    };
    
    let move_left = move |_| {
            move_left_help();
    };
    
    let move_right = move |_| {
            move_right_help();
    };
    
    // Calculate visible range

    let on_keydown = {
        let row = selected_cell.cloned().0;
        let col = selected_cell.cloned().1;
        move |e: Event<KeyboardData>| {
            let key = e.key();
            if key == Key::Enter || key == Key::ArrowDown || key == Key::ArrowUp || key == Key::Tab || key == Key::ArrowLeft || key == Key::ArrowRight {
                e.prevent_default();
                let to_row = (if e.key() == Key::ArrowDown || e.key() == Key::Enter {
                    if selected_cell.cloned().0 == start_row_ctx.cloned()+rows_per_page{
                        move_down_help();
                    };
                    row + 1
                } else if e.key() == Key::ArrowUp {
                    if selected_cell.cloned().0 == start_row_ctx.cloned()+1 {
                        move_up_help();
                    };
                    row - 1
                } else {
                    row
                }).max(1);
                
                let to_col = 
                (if (e.key() == Key::Tab && e.modifiers().shift()) || e.key() == Key::ArrowLeft {
                    if selected_cell.cloned().1 == start_col_ctx.cloned()+1 {
                        move_left_help();
                    };
                    col-1
                } else if e.key() == Key::Tab || e.key() == Key::ArrowRight {
                    if selected_cell.cloned().1 == start_col_ctx.cloned()+cols_per_page {
                        move_right_help();
                    };
                    col+1
                    
                } else {
                    col
                }).max(1);
                selected_cell.set((to_row, to_col));

                let script =  format!(r#"
                    let x = document.getElementById('row-{}-col-{}');
                    if (x) {{
                        x.focus();
                    }}

                "#,to_row,to_col);
                document::eval(&script);
            
            }
        }
    };

    let end_row = (start_row_ctx.cloned() + rows_per_page).min(props.num_rows);
    let end_col = (start_col_ctx.cloned() + cols_per_page).min(props.num_cols);
    
    rsx! {
        div {
            style: GRID_STYLE,
            class: "grid-container",
            onkeydown: on_keydown,
            div {
                style: TABLE_STYLE,
                class: "spreadsheet-table",
                // Header row (always visible)
                Row {
                    row: 0,
                    num_cols: cols_per_page,
                    is_header: true,
                    min_width: min_cell_width,
                    start_col: start_col_ctx.cloned(),
                    end_col: end_col,
                }
                
                // Visible rows
                for i in (start_row_ctx.cloned()+1)..=(end_row) {
                    Row {
                        row: i,
                        num_cols: cols_per_page,
                        is_header: false,
                        min_width: min_cell_width,
                        start_col: start_col_ctx.cloned(),
                        end_col: end_col,
                    }
                }
            }
            
            // Navigation controls
            div {
                style: NAVIGATION_CONTROLS_STYLE,
                
                // Page info display
                div {
                    style: PAGE_INFO_STYLE,
                    "Rows: {start_row_ctx}-{end_row} / {props.num_rows}, Cols: {start_col_ctx}-{end_col} / {props.num_cols}"
                }
                
                // Navigation buttons
                button {
                    style: NAV_BUTTON_STYLE,
                    onclick: move_up,
                    disabled: start_row_ctx.cloned() == 0,
                    id: "up-button",
                    "↑"
                }
                button {
                    style: NAV_BUTTON_STYLE,
                    onclick: move_down,
                    disabled: start_row_ctx.cloned() >= max_start_row-1,
                    id: "down-button",
                    "↓"
                }
                button {
                    style: NAV_BUTTON_STYLE,
                    onclick: move_left,
                    disabled: start_col_ctx.cloned() == 0,
                    id: "left-button",
                    "←"
                }
                button {
                    style: NAV_BUTTON_STYLE,
                    onclick: move_right,
                    disabled: start_col_ctx.cloned() >= max_start_col-1,
                    id: "right-button",
                    "→"
                }
            }
        }
    }
}
