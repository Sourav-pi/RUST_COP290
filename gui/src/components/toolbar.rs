use dioxus::prelude::*;
use super::spreadsheet::*;
use cores::convert_to_index;
use super::error_display::{ErrorContext, ErrorType, show_error};


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
    pub num_rows: usize,
    pub num_cols: usize,
}

#[component]
pub fn Toolbar(props: ToolbarProps) -> Element {
    let mut cur_file = use_context::<CurrentFileContext>();
    let mut is_open = use_context::<GraphPopupContext>();
    let mut start_row_ctx = use_context::<StartRowContext>();
    let mut start_col_ctx = use_context::<StartColContext>();
    let mut search_term = use_signal(|| String::new());
    let mut selected_cell = use_context::<SelectedCellContext>();
    let mut error_ctx = use_context::<ErrorContext>();
    let sheet = use_context::<SheetContext>();
    let mut sheetversion = use_context::<SheetVersionContext>();
    
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
                    
                    let (a ,b) = convert_to_index(search_term.cloned());
                    let (a,b) = (a as i32,b as i32);
                    // println!("ans {a} {b}");
                    if !(a==0 && b==0) && a <= props.num_rows as i32 && b <= props.num_cols as i32 {
                        start_row_ctx.set(a-1);
                        start_col_ctx.set(b-1);
                        selected_cell.set((a,b));

                        let _ =document::eval(&format!("document.getElementById('row-{}-col-{}').focus()",a,b));
                    } else {
                        show_error(&mut error_ctx, "Invalid Cell !!", ErrorType::Error, Some(5.0));
                        search_term.set(String::new());
                    }
                },
                "GO"
            }
        }
        
        // Existing buttons
        button { style: BUTTON_STYLE,
            onclick: move |_| {
                let path = std::env::current_dir().unwrap();

                let res = rfd::FileDialog::new()
                    .add_filter("spreadsheet", &["csv"])
                    .set_directory(&path)
                    .pick_file();
            
                if let Some(file) = &res {
                    cur_file.set(Some(file.clone()));
                    let file_path = file.as_path().to_string_lossy().to_string();
                    // Save the current sheet to the selected file
                    if let Ok(mut sheet_locked) = sheet.cloned().lock() {
                        // Update the cell value in the Sheet object
                        let write_result = sheet_locked.read_file(&file_path);
                        sheetversion.set(sheetversion.cloned() + 1);
                        if let Err(e) = write_result {
                            show_error(&mut error_ctx, &format!("Error reading from file: {}", e), ErrorType::Error, Some(5.0));
                        } else {
                            show_error(&mut error_ctx, "File loaded successfully", ErrorType::Success, Some(5.0));
                            sheetversion.set(sheetversion.cloned() + 1);
                        }
                    }
                }

                
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
                let res = rfd::FileDialog::new().add_filter("spreadsheet", &["csv"])
                .set_file_name("new_sheet.csv")
                .set_directory(&path)
                .save_file();
                // println!("The user choose: {:#?}", res);
                if let Some(file) = &res {
                    let file_path = file.as_path().to_string_lossy().to_string();
                    // Save the current sheet to the selected file
                    if let Ok(sheet_locked) = sheet.cloned().lock() {
                        // Update the cell value in the Sheet object
                        let write_result = sheet_locked.write_csv(&file_path);
                        sheetversion.set(sheetversion.cloned() + 1);
                        if let Err(e) = write_result {
                            show_error(&mut error_ctx, &format!("Error writing to file: {}", e), ErrorType::Error, Some(5.0));
                        } else {
                            show_error(&mut error_ctx, "File saved successfully", ErrorType::Success, Some(5.0));
                            sheetversion.set(sheetversion.cloned() + 1);
                        }
                    }
                }



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