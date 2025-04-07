use dioxus::prelude::*;
use super::spreadsheet::{CurrentFileContext, GraphPopupContext};


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
    padding: 5px;
    text-align: center;
    text-decoration: none;
    display: inline-block;
    cursor: pointer;
    border-radius: 4px;
    transition-duration: 0.4s;
    background-color: #f0f0f0;
    ";

const SEARCH_CONTAINER_STYLE: &str = "
    display: flex;
    align-items: center;
    margin-right: 20px;
    ";

const SEARCH_INPUT_STYLE: &str = "
    padding: 6px 12px;
    border: 1px solid #ccc;
    border-radius: 4px 0px 0px 4px;
    font-size: 14px;
    width: 80px;
    ";

const SEARCH_BUTTON_STYLE: &str = "
    background-color: #f0f0f0;
    border: 1px solid #ccc;
    border-radius: 0 4px 4px 0;
    padding: 6px 12px;
    margin-left: -1px;
    cursor: pointer;
    height: 31px;
    ";


#[derive(Props, PartialEq, Clone)]
pub struct ToolbarProps {
    pub cur_cell: String,
    pub filename: String,
}

#[component]
pub fn Toolbar() -> Element {
    let mut cur_file = use_context::<CurrentFileContext>();
    let mut is_open = use_context::<GraphPopupContext>();
    let mut cur_cell = use_context::<CurrentFileContext>();
    let mut search_term = use_signal(|| String::new());
    
    rsx! {
        div {style : TOOLBAR_STYLE,
        // Search bar
        div { style: SEARCH_CONTAINER_STYLE,
            input {
                style: SEARCH_INPUT_STYLE,
                placeholder: "Cell Number",
                value: "{search_term}",
                oninput: move |evt| search_term.set(evt.value().clone())
            }
            button {
                style: SEARCH_BUTTON_STYLE,
                onclick: move |_| {
                    // Handle search
                    // CONVERT string to cell number
                    // set cur_cell to the cell number
                    println!("Searching for: {}", search_term.cloned());
                },
                "GO"
            }
        }
        
        // Existing buttons
        button { style: BUTTON_STYLE,
            onclick: move |_| {
                let path = std::env::current_dir().unwrap();

                let res = rfd::FileDialog::new()
                    .add_filter("spreadsheet", &["csv", "xlsx"])
                    .set_directory(&path)
                    .pick_file();
            
                if let Some(file) = &res {
                    cur_file.set(Some(file.clone()));
                } 
                // println!("The user choose: {:#?}", res);
            },
            img {
                src: "{OPEN_ICON}",
                alt: "Open",
                style: "width: 30px; height: 30px;"
            }
        },
        button { style: BUTTON_STYLE,
            onclick: move |_| {
                let path = std::env::current_dir().unwrap();
                let res = rfd::FileDialog::new()
                .set_file_name("new_sheet.txt")
                .set_directory(&path)
                .save_file();
                // println!("The user choose: {:#?}", res);

            },
            img {
                src: "{SAVE_ICON}",
                alt: "Save",
                style: "width: 30px; height: 30px;"
            }
        },
        button { style: BUTTON_STYLE,
            
            img {
                onclick: move |_| {
                    is_open.set(true);
                },
                src: "{VISUALIZE_ICON}",
                alt: "Visualize",
                style: "width: 30px; height: 30px;"
            }
        },
        }
      }
    
}