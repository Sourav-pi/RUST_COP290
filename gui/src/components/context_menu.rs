use super::error_display::{show_error, ErrorContext, ErrorType};
use super::spreadsheet::*;
use cores::Error;
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum MenuType {
    Row,
    Col,
    Cell,
}

#[component]
#[allow(unused_braces)]
pub fn ContextMenu() -> Element {
    let mut context_menu = use_context::<ContextMenuContext>();
    let sheet = use_context::<SheetContext>();
    let mut sheet_version = use_context::<SheetVersionContext>();
    let mut error_ctx = use_context::<ErrorContext>();

    // Get clipboard contexts
    let mut copied_cell = use_context::<CopiedCellContext>();
    let mut copied_row = use_context::<CopiedRowContext>();
    let mut copied_col = use_context::<CopiedColContext>();

    if let Some((x_cord, y_cord, row, col, menu_type)) = context_menu.cloned() {
        match menu_type {
            MenuType::Row => {
                // Check if we have a copied row
                let has_copied_row = copied_row.read().is_some();

                rsx! {
                    div {
                        style: "position: fixed; left: {x_cord}px; top: {y_cord}px; background-color: white; border: 1px solid #ccc; border-radius: 4px; box-shadow: 0 2px 5px rgba(0,0,0,0.2); z-index: 1000; padding: 5px 0;",
                        div {
                            style: {"padding: 8px 16px; cursor: pointer; &:hover { background-color: #f0f0f0; }"},
                            onclick: move |_| {
                                copied_row.set(Some(row));
                                context_menu.set(None);
                            },
                            "Copy Row"
                        }
                        div {
                            style: if has_copied_row {
                                {"padding: 8px 16px; cursor: pointer; &:hover { background-color: #f0f0f0; }"}
                            } else {
                                "padding: 8px 16px; cursor: not-allowed; color: #ccc;"
                            },
                            onclick: move |_| {
                                if let Some(source_row) = *copied_row.read() {
                                    if let Ok(mut sheet_locked) = sheet.cloned().lock() {
                                        // Use the Sheet's copy_row method with error handling
                                        match sheet_locked.copy_row(source_row as usize, row as usize) {
                                            Ok(_) => {
                                                // Update sheet version to trigger rerender
                                                sheet_version.set(sheet_version.cloned() + 1);
                                            },
                                            Err(Error::CycleDetected) => {
                                                show_error(&mut error_ctx, "Cannot paste: would create circular reference",
                                                          ErrorType::Error, Some(3.0));
                                            },
                                            Err(_) => {
                                                show_error(&mut error_ctx, "Error pasting row", ErrorType::Error, Some(3.0));
                                            }
                                        }
                                    }
                                }
                                context_menu.set(None);
                            },
                            "Paste Row"
                        }
                    }
                }
            }
            MenuType::Col => {
                // Check if we have a copied column
                let has_copied_col = copied_col.read().is_some();

                rsx! {
                    div {
                        style: "position: fixed; left: {x_cord}px; top: {y_cord}px; background-color: white; border: 1px solid #ccc; border-radius: 4px; box-shadow: 0 2px 5px rgba(0,0,0,0.2); z-index: 1000; padding: 5px 0;",
                        div {
                            style: {"padding: 8px 16px; cursor: pointer; &:hover { background-color: #f0f0f0; }"},
                            onclick: move |_| {
                                // Store which column was copied
                                copied_col.set(Some(col));
                                context_menu.set(None);
                            },
                            "Copy Column"
                        }
                        div {
                            style: if has_copied_col {
                                {"padding: 8px 16px; cursor: pointer; &:hover { background-color: #f0f0f0; }"}
                            } else {
                                "padding: 8px 16px; cursor: not-allowed; color: #ccc;"
                            },
                            onclick: move |_| {
                                if let Some(source_col) = *copied_col.read() {
                                    if let Ok(mut sheet_locked) = sheet.cloned().lock() {
                                        // Use the Sheet's copy_col method with error handling
                                        match sheet_locked.copy_col(source_col as usize, col as usize) {
                                            Ok(_) => {
                                                // Update sheet version to trigger rerender
                                                sheet_version.set(sheet_version.cloned() + 1);
                                            },
                                            Err(Error::CycleDetected) => {
                                                show_error(&mut error_ctx, "Cannot paste: would create circular reference",
                                                          ErrorType::Error, Some(3.0));
                                            },
                                            Err(_) => {
                                                show_error(&mut error_ctx, "Error pasting column", ErrorType::Error, Some(3.0));
                                            }
                                        }
                                    }
                                }
                                context_menu.set(None);
                            },
                            "Paste Column"
                        }
                    }
                }
            }
            MenuType::Cell => {
                // Check if we have a copied cell
                let has_copied_cell = copied_cell.read().is_some();

                rsx! {
                    div {
                        style: "position: fixed; left: {x_cord}px; top: {y_cord}px; background-color: white; border: 1px solid #ccc; border-radius: 4px; box-shadow: 0 2px 5px rgba(0,0,0,0.2); z-index: 1000; padding: 5px 0;",
                        div {
                            style: {"padding: 8px 16px; cursor: pointer; &:hover { background-color: #f0f0f0; }"},
                            onclick: move |_| {
                                // Store which cell was copied
                                copied_cell.set(Some((row, col)));
                                context_menu.set(None);
                            },
                            "Copy Cell"
                        }
                        div {
                            style: if has_copied_cell {
                               { "padding: 8px 16px; cursor: pointer; &:hover { background-color: #f0f0f0; }"}
                            } else {
                                "padding: 8px 16px; cursor: not-allowed; color: #ccc;"
                            },
                            onclick: move |_| {
                                if let Some((source_row, source_col)) = *copied_cell.read() {
                                    if let Ok(mut sheet_locked) = sheet.cloned().lock() {
                                        // Use the Sheet's copy_cell method with error handling
                                        match sheet_locked.copy_cell(
                                            source_row as usize,
                                            source_col as usize,
                                            row as usize,
                                            col as usize
                                        ) {
                                            Ok(_) => {
                                                sheet_version.set(sheet_version.cloned() + 1);
                                            },
                                            Err(Error::CycleDetected) => {
                                                show_error(&mut error_ctx, "Cannot paste: would create circular reference",
                                                          ErrorType::Error, Some(3.0));
                                            },
                                            Err(_) => {
                                                show_error(&mut error_ctx, "Error pasting cell", ErrorType::Error, Some(3.0));
                                            }
                                        }
                                    }
                                }
                                context_menu.set(None);
                            },
                            "Paste Cell"
                        }
                    }
                }
            }
        }
    } else {
        rsx! { div {} }
    }
}
