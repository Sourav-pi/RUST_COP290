use std::path::PathBuf;

use dioxus::prelude::*;
use super::header::Header;
use super::grid::Grid;
use super::graph_popup::GraphPopup;

// Define explicit types for your contexts
pub type SelectedCellContext = Signal<(i32, i32)>;
pub type FormulaContext = Signal<String>;
pub type CurrentFileContext = Signal<Option<PathBuf>>;
pub type GraphPopupContext = Signal<bool>;
pub type GraphTypeContext = Signal<GraphType>;
pub type ContextMenuContext = Signal<Option<(f64, f64,i32,i32,  MenuType)>>;

#[derive(Clone, Copy, PartialEq)]
pub enum GraphType {
    Line,
    Pie,
    Scatter,
    Bar,   
}

#[derive(Clone, Copy, PartialEq)]
pub enum MenuType {
    RowMenu,
    ColMenu,
    CellMenu,
}

#[component]
pub fn Spreadsheet() -> Element {
    // Create the signals for context
    let selected_cell : SelectedCellContext = use_signal(|| (0, 0));
    let formula : FormulaContext = use_signal(|| String::new());
    let current_file : CurrentFileContext = use_signal(|| None);
    let graph_popup : GraphPopupContext = use_signal(|| false);
    let graph_type : GraphTypeContext = use_signal(|| GraphType::Line);
    let context_menu : ContextMenuContext = use_signal(|| None);
    
    let mut filename = "new_file.xlsx".to_string();
    if let Some(file) = current_file.cloned() {
        filename = file.file_name().unwrap().to_str().unwrap().to_string();
    }


    // Provide the contexts to the components
    provide_context(selected_cell);
    provide_context(formula);
    provide_context(current_file);
    provide_context(graph_popup);
    provide_context(graph_type);
    provide_context(context_menu);

    rsx! {
        Header { 
            filename: filename.clone(),
        }
        Grid {
            num_rows: 30,
            num_cols: 20,
        }
        GraphPopup{},
        ContextMenu {}
    }
}

#[component]
fn ContextMenu() -> Element {
    let mut context_menu = use_context::<ContextMenuContext>();
    
    if let Some((x_cord, y_cord, row_col, _col, menu_type)) = context_menu.cloned() {
        match menu_type {
            MenuType::RowMenu => {
                rsx! {
                    div {
                        style: "position: fixed; left: {x_cord}px; top: {y_cord}px; background-color: white; border: 1px solid #ccc; border-radius: 4px; box-shadow: 0 2px 5px rgba(0,0,0,0.2); z-index: 1000; padding: 5px 0;",
                        div {
                            style: {"padding: 8px 16px; cursor: pointer; &:hover { background-color: #f0f0f0; }"},
                            onclick: move |_| {
                                // Logic to insert row
                                println!("Insert row at {}", row_col);
                                context_menu.set(None);
                            },
                            "Insert Row"
                        }
                        div {
                            style: {"padding: 8px 16px; cursor: pointer; &:hover { background-color: #f0f0f0; }"},
                            onclick: move |_| {
                                // Logic to delete row
                                println!("Delete row at {}", row_col);
                                context_menu.set(None);
                            },
                            "Delete Row"
                        }
                        div {
                            style: {"padding: 8px 16px; cursor: pointer; &:hover { background-color: #f0f0f0; }"},
                            onclick: move |_| {
                                // Logic to hide row
                                println!("Hide row at {}", row_col);
                                context_menu.set(None);
                            },
                            "Copy Row"
                        }
                        div {
                            style: {"padding: 8px 16px; cursor: pointer; &:hover { background-color: #f0f0f0; }"},
                            onclick: move |_| {
                                // Logic to hide row
                                println!("Hide row at {}", row_col);
                                context_menu.set(None);
                            },
                            "Paste Row"
                        }
                    }
                }
            },
            MenuType::ColMenu => {
                rsx! {
                    div {
                        style: "position: fixed; left: {x_cord}px; top: {y_cord}px; background-color: white; border: 1px solid #ccc; border-radius: 4px; box-shadow: 0 2px 5px rgba(0,0,0,0.2); z-index: 1000; padding: 5px 0;",
                        div {
                            style: {"padding: 8px 16px; cursor: pointer; &:hover { background-color: #f0f0f0; }"},
                            onclick: move |_| {
                                // Logic to insert column
                                println!("Insert column at {}", row_col);
                                context_menu.set(None);
                            },
                            "Insert Column"
                        }
                        div {
                            style: {"padding: 8px 16px; cursor: pointer; &:hover { background-color: #f0f0f0; }"},
                            onclick: move |_| {
                                // Logic to delete column
                                println!("Delete column at {}", row_col);
                                context_menu.set(None);
                            },
                            "Delete Column"
                        }
                        div {
                            style: {"padding: 8px 16px; cursor: pointer; &:hover { background-color: #f0f0f0; }"},
                            onclick: move |_| {
                                // Logic to hide column
                                println!("Hide column at {}", row_col);
                                context_menu.set(None);
                            },
                            "Copy Column"
                        }
                        div {
                            style: {"padding: 8px 16px; cursor: pointer; &:hover { background-color: #f0f0f0; }"},
                            onclick: move |_| {
                                // Logic to hide column
                                println!("Hide column at {}", row_col);
                                context_menu.set(None);
                            },
                            "Paste Column"
                        }
                    }
                }
            },
            MenuType::CellMenu => rsx! { div {
                style: "position: fixed; left: {x_cord}px; top: {y_cord}px; background-color: white; border: 1px solid #ccc; border-radius: 4px; box-shadow: 0 2px 5px rgba(0,0,0,0.2); z-index: 1000; padding: 5px 0;",
                div {
                    style: {"padding: 8px 16px; cursor: pointer; &:hover { background-color: #f0f0f0; }"},
                    onclick: move |_| {
                        // Logic to insert cell
                        println!("Copy cell at ({}, {})", row_col, _col);
                        context_menu.set(None);
                    },
                    "Copy Cell"
                }
                div {
                    style: {"padding: 8px 16px; cursor: pointer; &:hover { background-color: #f0f0f0; }"},
                    onclick: move |_| {
                        // Logic to delete cell
                        println!("Paste cell at ({}, {})", row_col, _col);
                        context_menu.set(None);
                    },
                    "Paste Cell"
                }
            } } // Placeholder for future cell context menu
        }
    } else {
        rsx! { div {} }
    }
}
