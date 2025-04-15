use dioxus::prelude::*;
use dioxus::events::Key;
use super::spreadsheet::{SelectedCellContext, SheetContext, SheetVersionContext};
use crate::components::row::column_to_letter;

const FORMULA_BAR_STYLE: &str = "display: flex;
                                height: 30px;
                                background-color: rgb(42, 42, 42);
                                margin: 0px;
                                padding: 0px;
                                width: 100%;";


#[component]
pub fn FormulaBar() -> Element {
    // Consume the contexts

    let sheet = use_context::<SheetContext>();
    let mut sheetversion = use_context::<SheetVersionContext>();
    let selected_cell = use_context::<SelectedCellContext>();
    let mut formula = use_signal(|| String::new());

    use_effect(move || {
        let _ = sheetversion.cloned();

        if let Ok(sheet_locked) = sheet.cloned().lock() {
        // Update the cell value in the Sheet object
            let formula_in_sheet = sheet_locked.get_formula(selected_cell.cloned().0 as usize, selected_cell.cloned().1 as usize).to_string();
            if formula_in_sheet != "0" {
                formula.set(formula_in_sheet);
            } else {
                formula.set("".to_string());
            }
        }
    });

  
    let on_submit = move |e: Event<KeyboardData>| {
        if e.key()==Key::Enter {
            if let Ok(mut sheet_locked) = sheet.cloned().lock() {
                // Update the cell value in the Sheet object
                sheet_locked.update_cell_data(selected_cell.cloned().0 as usize, selected_cell.cloned().1 as usize, formula.cloned());
                sheetversion.set(sheetversion.cloned() + 1);
            }
        };
    };

    let on_blur = move |_| {
        if let Ok(mut sheet_locked) = sheet.cloned().lock() {
            // Update the cell value in the Sheet object
            sheet_locked.update_cell_data(selected_cell.cloned().0 as usize, selected_cell.cloned().1 as usize, formula.cloned());
            sheetversion.set(sheetversion.cloned() + 1);
        }
    };

    rsx! {
        div {
            style: FORMULA_BAR_STYLE,
            input {
                style: "color: white; width: 7%; text-align: center; background-color: rgb(42, 42, 42); font-size: 20px;",
                value: "{column_to_letter(selected_cell.cloned().1)}{selected_cell.cloned().0}",
                readonly: true,
            }
            input {
                class: "formula-input",
                style: "height: 24px; font-size: 20px; border: none; margin: 0px; width: 93%; outline: none; box-shadow: none; margin: 2px 0px;",
                placeholder: "Enter formula here...",
                value: "{formula}",
                oninput: move |e| {
                    formula.set(e.value().clone());
                },
                onkeydown: on_submit,
                onblur : on_blur,
                // onkeydown: move |e| {
                //     if e.key() == Key::Enter {
                        
                //     }
                // },
            }
        }
    }
}