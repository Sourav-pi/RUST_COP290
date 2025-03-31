use dioxus::prelude::*;
use super::header::Header;


#[component]
pub fn Spreadsheet() -> Element {
    let cur_cell = "A1".to_string();
    let filename = "test.xlsx".to_string();
    rsx! {
        Header {filename , cur_cell}


    }
}