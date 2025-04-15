use super::spreadsheet::{GraphPopupContext, GraphType, GraphTypeContext};
use super::graph_forms::{BarChartForm, LineChartForm, PieChartForm, ScatterPlotForm};
use ::dioxus::prelude::*;
const CLOSE_BUTTON_STYLE: &str = r#"
    background-color:rgb(155, 155, 155);
    height: 30px;
    width: 30px;
    border-radius: 50%;
    color: white;
    text-align: center;
    cursor: pointer;
    border: none;
    transition-duration: 0.1s;
    &:hover {
        background-color:  #f0f0f0;
        color: black;
    }
"#;

const HEADER_STYLE: &str = r#"
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    margin: 0px;
    padding: 0px;
    align-items: space-between;
"#;

const TABS_GROUP_STYLE: &str = r#"
    flex-grow: 0;
    flex-shrink: 0;
    display: flex;
    flex-direction: row;
    gap: 10px;
    align-items: center;
    margin: 0px;
    padding: 0px;
"#;

const BODY_STYLE: &str = r#"
    flex-grow: 1;
    flex-shrink: 1;
    display: flex;
    margin: 0px;
    padding: 0px;
    background-color:#f0f0f0;
    min-height: 80%;
    width: 100%;
    border-radius: 0px 12px 12px 12px;
"#;

const OVERLAY_STYLE: &str = r#"
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 1000;
"#;

const POPUP_STYLE: &str = r#"
    display: flex;
    flex-direction: column;
    background-color: white;
    padding: 20px;
    border-radius: 10px;
    justify-content: center;
    width: 80%;
    height: 80%;
"#;

const BUTTON_STYLE_UNSELECT: &str = r#"
    background-color:rgb(155, 155, 155); 
    border: none;
    color: white;
    padding: 15px 32px;
    text-align: center;
    text-decoration: none;
    display: inline-block;
    font-size: 16px;
    cursor: pointer;
    border-radius: 12px 12px 0px 0px;
    transition-duration: 0.1s;
    &:hover {
        background-color: white;
        color: black;
        border: 2px solid rgb(255, 0, 25);
    }
"#;
const BUTTON_STYLE_SELECT: &str = r#"
    background-color: #f0f0f0; 
    border: none;
    color: black;
    padding: 15px 32px;
    text-align: center;
    text-decoration: none;
    display: inline-block;
    font-size: 16px;
    cursor: pointer;
    border-radius: 12px 12px 0px 0px;
    transition-duration: 0.1s;
    &:hover {
        background-color: white;
        color: black;
        border: 2px solid rgb(255, 0, 25);
    }
"#;

#[component]
pub fn GraphPopup() -> Element {
    let mut is_open = use_context::<GraphPopupContext>();
    let mut graph_type = use_context::<GraphTypeContext>();

    let options = vec![
        ("Line Chart".to_string(), GraphType::Line),
        ("Bar Chart".to_string(), GraphType::Bar),
        ("Pie Chart".to_string(), GraphType::Pie),
        ("Scatter Plot".to_string(), GraphType::Scatter),
    ];
    if is_open.cloned() {
        rsx! {
            div {style: OVERLAY_STYLE,
                div {
                    style: POPUP_STYLE,
                    div{ style: HEADER_STYLE,
                        
                        div { style: TABS_GROUP_STYLE,
                            {options.into_iter().map(|graph| {
                            rsx! {button {
                                style: format!("{} margin: 5px;", (if graph_type.cloned()== graph.1 { BUTTON_STYLE_SELECT} else {BUTTON_STYLE_UNSELECT})),
                                onclick: move |_| {
                                    graph_type.set(graph.1);
                                },
                                "{graph.0}"
                            }}
                    })
                        }}
                        button {
                            style: CLOSE_BUTTON_STYLE,
                            onclick: move |_| {
                                is_open.set(false);
                            },
                            "X"
                        }
                    }
                    div { 
                        style: BODY_STYLE,
                        match graph_type.cloned() {
                            GraphType::Line => rsx! { LineChartForm {} },
                            GraphType::Bar => rsx! { BarChartForm {} },
                            GraphType::Pie => rsx! { PieChartForm {} },
                            GraphType::Scatter => rsx! { ScatterPlotForm {} },
                        }
                     }

                }

            }
        }
    } else {
        rsx! {
            div {
                style: "display: none;"
            }
        }
    }
}
