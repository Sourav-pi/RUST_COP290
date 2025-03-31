use dioxus::prelude::*;
use super::cell::Cell;

const ROW_STYLE: &str ="
    height: 30px;
    display: flex;
    flex-direction: row;
    gap: 0px;
    align-items: center;
    justify-content: center;
";

const CELL_HEADER_STYLE : &str = "
    width: 80px;
    height: 30px;
    border: 1px solid #ccc;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: #f3f3f3;
    font-weight: bold;
    position: sticky;
    left: 0px;
    z-index: 20;
";

#[derive(Props, PartialEq, Clone)]
pub struct RowProps {
    pub row: i32,
    pub num_cols: i32,
    #[props(default = false)]
    pub is_header: bool,
    #[props(default = 80)]
    pub min_width: i32,
}

#[component]
pub fn Row(props: RowProps) -> Element {
    let row_style = if props.is_header {
        "position: sticky; top: 0; background-color: #f3f3f3; z-index: 10;"
    } else {
        ""
    };
    
    rsx! {
        div {
            style: "display: flex; flex-direction: row; {row_style}",
            class: "spreadsheet-row",
            // Add column header (A, B, C...) cell if this is the first column
            if props.is_header {
                div {
                    style: CELL_HEADER_STYLE,
                    ""  // Empty corner cell
                }
            } else {
                div {
                    style: CELL_HEADER_STYLE,
                    "{props.row}"  // Row header (1, 2, 3...)
                }
            }
            
            // Generate cells for this row
            for col in 1..=props.num_cols {
                Cell {
                    value: 0,
                    row: props.row,
                    col: col,
                    is_header: props.is_header,
                    min_width: props.min_width,
                }
            }
        }
    }
}