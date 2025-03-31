use dioxus::prelude::*;
use super::formula_bar::*;
use super::toolbar::*;


const HEAD_STYLE: &str = "
   width: 100%;
    height: 50px;
    background-color:rgb(159, 150, 150);
    display: flex;
    flex-direction: column;
    gap: 0px;
    padding: 0px;
    margin: 0px;
   ";

   #[derive(Props, PartialEq, Clone)]
   pub struct HeaderProps {
       pub cur_cell: String,
       pub filename: String,
   }

#[component]
pub fn Header(props : HeaderProps) -> Element {
    rsx! {
        div {style : HEAD_STYLE,
        h1 {
            style: "margin: 0; padding-top: 20px; padding-left : 20px; padding-right : 20px ; font-size: 24px;",
            "Spreadsheet - {props.filename}"
        }
        Toolbar {}
        FormulaBar {cur_cell: props.cur_cell.clone() }

        }


      }

    }