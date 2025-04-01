use dioxus::prelude::*;
use dioxus_elements::form;
use super::formula_bar::*;
use super::toolbar::*;

const SPREADSHEET_ICON: Asset = asset!( "assets/spreadsheet.png");
const HEAD_STYLE: &str = "
   width: 100vw;
    padding: 0px;
    margin: 0px;
   ";

   #[derive(Props, PartialEq, Clone)]
   pub struct HeaderProps {
       pub filename: String,
   }



#[component]
pub fn Header(props : HeaderProps) -> Element {
    let mut cur_cell = use_signal(|| "A1".to_string());
    let mut formula = use_signal(|| "".to_string());
    rsx! {   
        div {style : HEAD_STYLE,
        div {  style: "height: 50px; display: flex; flex-direction: row; align-items: center; justify-content: space-between;margin: 10px 15px;",

            div {
                style: "display: flex; gap: 20px; flex-direction: row; align-items: center; justify-content: start; ",
                img {
                    src: "{SPREADSHEET_ICON}",
                    alt: "Logo",
                    style: "width: 50px; height: 50px;"
                }
                h1 {
                    style: "margin: 0; font-family: Arial, Helvetica, sans-serif",
                    "{props.filename}"
                }
            }
            Toolbar {}
        }
        FormulaBar {cur_cell: cur_cell.cloned(), formula: formula.cloned()}

        }


      }

    }