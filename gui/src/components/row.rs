use super::cell::Cell;
use super::spreadsheet::ContextMenuContext;
use super::context_menu::MenuType;
use dioxus::prelude::*;

const ROW_STYLE: &str = "
    position: sticky; 
    top: 0; background-color: #f3f3f3; 
    z-index: 10;
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

// Helper function to convert column number to letter
pub fn column_to_letter(col: i32) -> String {
    if col <= 0 {
        return String::new();
    }

    let mut result = String::new();
    let mut col_num = col;

    while col_num > 0 {
        let remainder = (col_num - 1) % 26;
        result.insert(0, (b'A' + remainder as u8) as char);
        col_num = (col_num - 1) / 26;
    }

    result
}

#[derive(Props, PartialEq, Clone)]
pub struct RowProps {
    pub row: i32,
    pub num_cols: i32,
    #[props(default = false)]
    pub is_header: bool,
    #[props(default = 80)]
    pub min_width: i32,
    #[props(default = 0)]
    pub start_col: i32,
    #[props(default = 0)]
    pub end_col: i32,

}

#[component]
pub fn Row(props: RowProps) -> Element {
    let mut context_menu = use_context::<ContextMenuContext>();

    let row_style = if props.is_header { ROW_STYLE } else { "" };

    // Handler for row header right-click
    let on_row_contextmenu = {
        let row = props.row;
        move |e: Event<MouseData>| {
            e.stop_propagation();
            e.prevent_default();
            // Get the mouse position
            let mouse_y = e.client_coordinates().y;
            let mouse_x = e.client_coordinates().x;

            // Set the context menu data with mouse position and row number
            context_menu.set(Some((mouse_x, mouse_y, row, 0, MenuType::RowMenu)));

            println!("Right-clicked on row {}", row);
        }
    };

    let generate_on_col_contextmenu = |col: i32| {
        let col_num = col;
        let z =
        move |e: Event<MouseData>| {
            e.stop_propagation();
            e.prevent_default();
            // Get the mouse position
            let mouse_y = e.client_coordinates().y;
            let mouse_x = e.client_coordinates().x;

            // Set the context menu data with mouse position and column number
            context_menu.set(Some((mouse_x, mouse_y, col_num, 0, MenuType::ColMenu)));

            println!("Right-clicked on column {}", col_num);
        };
        z
    };

    let generate_on_cell_contextmenu = |row: i32, col: i32| {
        move |e: Event<MouseData>| {
            e.stop_propagation();
            e.prevent_default();
            // Get the mouse position
            let mouse_y = e.client_coordinates().y;
            let mouse_x = e.client_coordinates().x;

            // Set the context menu data with mouse position and cell coordinates
            context_menu.set(Some((mouse_x, mouse_y, row, col, MenuType::CellMenu)));

            println!("Right-clicked on cell ({}, {})", row, col);
        }
    };

    // Close context menu when clicking elsewhere
    let on_click = move |_| {
        context_menu.set(None);
    };

    rsx! {

        div {
            style: "display: flex; flex-direction: row; {row_style}",
            class: "spreadsheet-row",
            onclick: on_click,
            // Add column header (A, B, C...) cell if this is the first column
            if props.is_header {
                input { style: CELL_HEADER_STYLE, readonly: true, "" }
                for col in (props.start_col)..=props.end_col {
                    {
                        let col_letter = column_to_letter(col);
                        rsx! {
                            input {
                                style: CELL_HEADER_STYLE,
                                readonly: true,
                                value: "{col_letter}",
                                oncontextmenu: generate_on_col_contextmenu(col),
                            }
                        }
                    }
                }
            } else {
                input {
                    style: CELL_HEADER_STYLE,
                    readonly: true,
                    value: props.row.to_string(),
                    oncontextmenu: on_row_contextmenu,
                }

                // Generate cells for this row
                for col in (props.start_col)..=props.end_col {
                    Cell {
                        row: props.row,
                        col,
                        is_header: props.is_header,
                        min_width: props.min_width,
                        oncontextmenu: generate_on_cell_contextmenu(props.row, col),
                    }
                }
            }
        }

    }
}
