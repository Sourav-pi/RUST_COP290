use dioxus::prelude::*;
use super::row::Row;

const GRID_STYLE: &str = "
    overflow-y: auto;
    overflow-x: auto;
    height: calc(100vh - 120px);
    width: 100%;
    display: block;
    position: relative;
    border: 1px solid #e0e0e0;
    box-shadow: inset 0 0 5px rgba(0,0,0,0.1);
    
    /* Improve scrolling performance */
    will-change: transform;
    -webkit-overflow-scrolling: touch;
    
    /* Custom scrollbar styling */
    scrollbar-width: thin;
    scrollbar-color: #888 #f1f1f1;
";

// Ensure table has enough width to trigger horizontal scrolling
const TABLE_STYLE: &str = "
    display: table;
    min-width: max-content; /* Force table to be as wide as its content */
    border-collapse: collapse;
";

#[derive(Props, PartialEq, Clone)]
pub struct GridProps {
    pub num_rows: i32,
    pub num_cols: i32,
}

#[component]
pub fn Grid(props: GridProps) -> Element {
    // Calculate total expected width to ensure scrolling
    let min_cell_width = 80; // Minimum width per cell in pixels
    let expected_width = props.num_cols * min_cell_width;
    
    rsx! {
        div {
            style: GRID_STYLE,
            class: "grid-container",
            onwheel: |e| {
                e.stop_propagation();
            },
            div {
                style: TABLE_STYLE,
                class: "spreadsheet-table",
                for i in 0..=props.num_rows {
                    Row {
                        row: i,
                        num_cols: props.num_cols,
                        is_header: i == 0,
                        min_width: min_cell_width,
                    }
                }
            }
        }
    }
}
