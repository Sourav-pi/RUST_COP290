use dioxus::prelude::*;


const OPEN_ICON : Asset = asset!( "assets/open.png");
const SAVE_ICON : Asset = asset!( "assets/save.png");
const VISUALIZE_ICON : Asset = asset!( "assets/visualize.png");

const TOOLBAR_STYLE: &str = "
    height: 50px;
    display: flex;
    gap: 10px;
    align-items: center;
    padding-left: 25px;
   ";

   const BUTTON_STYLE: &str = "
    border: none;
    color: black;
    padding: 5px ;
    text-align: center;
    text-decoration: none;
    display: inline-block;
    cursor: pointer;
    border-radius: 4px;
    transition-duration: 0.4s;
    background-color: #f0f0f0;
    ";


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
            img {
                src: "{OPEN_ICON}",
                alt: "Open",
                style: "width: 30px; height: 30px;"
            }
        },
        button { style: BUTTON_STYLE,
            img {
                src: "{SAVE_ICON}",
                alt: "Save",
                style: "width: 30px; height: 30px;"
            }
        },
        button { style: BUTTON_STYLE,
            img {
                src: "{VISUALIZE_ICON}",
                alt: "Visualize",
                style: "width: 30px; height: 30px;"
            }
        },
        }


      }

    }