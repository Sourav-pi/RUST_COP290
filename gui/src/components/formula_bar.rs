use dioxus::prelude::*;

const FORMULA_BAR_STYLE: &str = "display: flex;
                                padding: 5px;
                                background-color: #f5f5f5;
                                border-bottom: 1px solid #ddd;
                                margin: 0px;
                                padding: 0px;
                                ";

#[derive(Props, PartialEq, Clone)]
pub struct FormulaBarProps {
    pub cur_cell: String,
}

#[component]
pub fn FormulaBar(props: FormulaBarProps) -> Element {
    rsx! {
        div {
            style: FORMULA_BAR_STYLE,
            input {
                style: "width: 80px; margin-right: 10px; text-align: center;",
                value: "{props.cur_cell}",
                readonly: true,
            }
            input {
                class: "formula-input",
                style: "flex-grow: 1;",
                placeholder: "Enter formula here...",
                oninput: move |e| {
                    let value = e.value();
                    // Handle the input value
                    println!("Input value: {}", value);
                },
            }
        }
    }
}