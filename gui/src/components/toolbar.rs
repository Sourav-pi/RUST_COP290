use dioxus::prelude::*;
use super::formula_bar::*;


const TOOLBAR_STYLE: &str = "
    height: 50px;
    background-color:rgb(159, 150, 150);
    display: flex;
    gap: 30px;
    align-items: center;
    padding: 10px;
   ";

   const BUTTON_STYLE: &str = "
   background-color: #4CAF50; /* Green */
    border: none;
    color: white;
    padding: 5px 20px;
    text-align: center;
    text-decoration: none;
    display: inline-block;
    font-size: 16px;
    margin: 4px 2px;
    cursor: pointer;
    border-radius: 2px;
    transition-duration: 0.4s;";


   #[derive(Props, PartialEq, Clone)]
   pub struct ToolbarProps {
       pub cur_cell: String,
       pub filename: String,
   }

#[component]
pub fn Toolbar() -> Element {
    rsx! {
        div {style : TOOLBAR_STYLE,
        button { style: BUTTON_STYLE,
            "Open"
        },
        button { style: BUTTON_STYLE,
            "Save"
        },
        button { style: BUTTON_STYLE,
            "Data Visualization"
        },
        }


      }

    }