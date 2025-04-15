use dioxus::prelude::*;
use super::spreadsheet::ContextMenuContext;
#[derive(Clone, Copy, PartialEq)]
pub enum MenuType {
    RowMenu,
    ColMenu,
    CellMenu,
}

#[component]
#[allow(unused_braces)]
pub fn ContextMenu() -> Element {
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
