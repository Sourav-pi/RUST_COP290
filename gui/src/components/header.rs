use dioxus::prelude::*;
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
        FormulaBar {}

        }


      }

    }