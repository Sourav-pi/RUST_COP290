use dioxus::prelude::*;

const CELL_STYLE : &str = "
    width: 80px;
    height: 30px;
    border: 1px solid #ccc;
    display: flex;
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
    top: 90px;
    left: 0px;
    z-index: 10;
";
const CELL_SELECTED_STYLE : &str = "
    width: 80px;
    height: 30px;
    border: 1px solid #ccc;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: #d3d3d3;
    border: 2px solid #000;
    box-shadow: 0 0 5px rgba(0, 0, 0, 0.5);
";

#[derive(Props, PartialEq, Clone)]
pub struct CellProps {
    pub value: i32,
    pub row: i32,
    pub col: i32,
    #[props(default = false)]
    pub is_header: bool,
    #[props(default = false)]
    pub is_selected: bool,
    #[props(default = 80)]
    pub min_width: i32,
}

#[component]
pub fn Cell(props: CellProps) -> Element {
    rsx! {
        div {
            id: "row-{props.row}-col-{props.col}",
            style: {if props.is_header {
                CELL_HEADER_STYLE
            } else if props.is_selected {
                CELL_SELECTED_STYLE
            } else {
                CELL_STYLE
            }},
            class: "cell",
            "{props.value}"
        }
        
    }
}