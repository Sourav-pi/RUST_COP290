use dioxus::prelude::*;
use dioxus::events::Key;

const FORMULA_BAR_STYLE: &str = "display: flex;
                                height: 30px;
                                background-color: rgb(42, 42, 42);
                                margin: 0px;
                                padding: 0px;
                                width: 100%;";

#[derive(Props, PartialEq, Clone)]
pub struct FormulaBarProps {
    pub cur_cell: String,
}

#[component]
pub fn FormulaBar(props: FormulaBarProps) -> Element {
    let mut formula = use_signal(|| "".to_string());
    
    rsx! {
        div {
            style: FORMULA_BAR_STYLE,
            input {
                style: "color: white; width: 7%; text-align: center; background-color: rgb(42, 42, 42); font-size: 20px;",
                value: "{props.cur_cell}",
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
                onkeydown: move |e| {
                    if e.key() == Key::Enter {
                        // Handle the Enter key event
                        println!("Enter key pressed");
                        // You can also update the formula signal here if needed
                        println!("Formula: {}{}", props.cur_cell,formula); 
                        formula.set("".to_string()); // Clear the input after pressing Enter
                    }
                },
            }
        }
    }
}