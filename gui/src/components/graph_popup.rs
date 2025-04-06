use super::spreadsheet::{GraphPopupContext, GraphType, GraphTypeContext};
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

// Common style for input containers
const INPUT_CONTAINER_STYLE: &str = r#"
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    padding: 16px;
    gap: 12px;
    width: 90%;
"#;

const INPUT_STYLE: &str = r#"
    padding: 8px 12px;
    border: 1px solid #ccc;
    border-radius: 4px;
    font-size: 14px;
    width: 100%;
"#;

const LABEL_STYLE: &str = r#"
    font-weight: bold;
    margin-bottom: 4px;
"#;

const SUBMIT_BUTTON_STYLE: &str = r#"
    background-color: rgb(36, 117, 247);
    border: none;
    color: white;
    padding: 10px 20px;
    text-align: center;
    text-decoration: none;
    display: inline-block;
    font-size: 16px;
    margin-top: 16px;
    cursor: pointer;
    border-radius: 4px;
    transition-duration: 0.2s;
    &:hover {
        background-color:rgb(92, 141, 255);
    }
"#;

// Helper component for each type of graph input
#[component]
fn LineChartForm() -> Element {
    let mut range = use_signal(|| String::new());
    let mut x_label = use_signal(|| String::new());
    let mut y_label = use_signal(|| String::new());
    let mut title = use_signal(|| String::new());
    let mut point_labels = use_signal(|| String::new());

    rsx! {
        div { style: INPUT_CONTAINER_STYLE,
            div {
                div { style: LABEL_STYLE, "Cell Range (e.g., A1:C10)" }
                input { 
                    style: INPUT_STYLE, 
                    placeholder: "Enter cell range",
                    oninput: move |evt| range.set(evt.value().clone())
                }
            }
            div {
                div { style: LABEL_STYLE, "X-Axis Label" }
                input { 
                    style: INPUT_STYLE, 
                    placeholder: "X-Axis",
                    oninput: move |evt| x_label.set(evt.value().clone())
                }
            }
            div {
                div { style: LABEL_STYLE, "Y-Axis Label" }
                input { 
                    style: INPUT_STYLE, 
                    placeholder: "Y-Axis",
                    oninput: move |evt| y_label.set(evt.value().clone())
                }
            }
            div {
                div { style: LABEL_STYLE, "Chart Title (Optional)" }
                input { 
                    style: INPUT_STYLE, 
                    placeholder: "Enter title",
                    oninput: move |evt| title.set(evt.value().clone())
                }
            }
            div {
                div { style: LABEL_STYLE, "Point Labels (Optional)" }
                input { 
                    style: INPUT_STYLE, 
                    placeholder: "Comma-separated labels",
                    oninput: move |evt| point_labels.set(evt.value().clone())
                }
            }
            button { 
                style: SUBMIT_BUTTON_STYLE,
                onclick: move |_| {
                    // Handle submission
                    println!("Line chart with range: {}, x: {}, y: {}, title: {}, labels: {}", 
                             range.cloned(), x_label.cloned(), y_label.cloned(), title.cloned(), point_labels.cloned());
                },
                "Generate Chart"
            }
        }
    }
}

#[component]
fn BarChartForm() -> Element {
    let mut range = use_signal(|| String::new());
    let mut x_label = use_signal(|| String::new());
    let mut y_label = use_signal(|| String::new());
    let mut title = use_signal(|| String::new());
    let mut bar_labels = use_signal(|| String::new());

    rsx! {
        div { style: INPUT_CONTAINER_STYLE,
            div {
                div { style: LABEL_STYLE, "Cell Range (e.g., A1:C10)" }
                input { 
                    style: INPUT_STYLE, 
                    placeholder: "Enter cell range",
                    oninput: move |evt| range.set(evt.value().clone())
                }
            }
            div {
                div { style: LABEL_STYLE, "X-Axis Label" }
                input { 
                    style: INPUT_STYLE, 
                    placeholder: "X-Axis",
                    oninput: move |evt| x_label.set(evt.value().clone())
                }
            }
            div {
                div { style: LABEL_STYLE, "Y-Axis Label" }
                input { 
                    style: INPUT_STYLE, 
                    placeholder: "Y-Axis",
                    oninput: move |evt| y_label.set(evt.value().clone())
                }
            }
            div {
                div { style: LABEL_STYLE, "Chart Title (Optional)" }
                input { 
                    style: INPUT_STYLE, 
                    placeholder: "Enter title",
                    oninput: move |evt| title.set(evt.value().clone())
                }
            }
            div {
                div { style: LABEL_STYLE, "Bar Labels (Optional)" }
                input { 
                    style: INPUT_STYLE, 
                    placeholder: "Comma-separated labels",
                    oninput: move |evt| bar_labels.set(evt.value().clone())
                }
            }
            button { 
                style: SUBMIT_BUTTON_STYLE,
                onclick: move |_| {
                    // Handle submission
                    println!("Bar chart with range: {}, x: {}, y: {}, title: {}, labels: {}", 
                             range.cloned(), x_label.cloned(), y_label.cloned(), title.cloned(), bar_labels.cloned());
                },
                "Generate Chart"
            }
        }
    }
}

#[component]
fn PieChartForm() -> Element {
    let mut range = use_signal(|| String::new());
    let mut title = use_signal(|| String::new());
    let mut slice_labels = use_signal(|| String::new());

    rsx! {
        div { style: INPUT_CONTAINER_STYLE,
            div {
                div { style: LABEL_STYLE, "Cell Range (e.g., A1:A10,B1:B10)" }
                input { 
                    style: INPUT_STYLE, 
                    placeholder: "Enter cell range",
                    oninput: move |evt| range.set(evt.value().clone())
                }
            }
            div {
                div { style: LABEL_STYLE, "Chart Title (Optional)" }
                input { 
                    style: INPUT_STYLE, 
                    placeholder: "Enter title",
                    oninput: move |evt| title.set(evt.value().clone())
                }
            }
            div {
                div { style: LABEL_STYLE, "Slice Labels (Optional)" }
                input { 
                    style: INPUT_STYLE, 
                    placeholder: "Comma-separated labels",
                    oninput: move |evt| slice_labels.set(evt.value().clone())
                }
            }
            div { style : "visibility: hidden ",
                div { style: LABEL_STYLE, "placeholder" }
                input { 
                    style: INPUT_STYLE, 
                    placeholder: "placeholder",
                    oninput: move |evt| slice_labels.set(evt.value().clone())
                }
            }
            div { style : "visibility: hidden ",
                div { style: LABEL_STYLE, "placeholder" }
                input { 
                    style: INPUT_STYLE, 
                    placeholder: "placeholder",
                    oninput: move |evt| slice_labels.set(evt.value().clone())
                }
            }
            button { 
                style: SUBMIT_BUTTON_STYLE,
                onclick: move |_| {
                    // Handle submission
                    println!("Pie chart with range: {}, title: {}, labels: {}", 
                             range.cloned(), title.cloned(), slice_labels.cloned());
                },
                "Generate Chart"
            }
        }
    }
}

#[component]
fn ScatterPlotForm() -> Element {
    let mut x_range = use_signal(|| String::new());
    let mut y_range = use_signal(|| String::new());
    let mut x_label = use_signal(|| String::new());
    let mut y_label = use_signal(|| String::new());
    let mut title = use_signal(|| String::new());

    rsx! {
        div { style: INPUT_CONTAINER_STYLE,
            div {
                div { style: LABEL_STYLE, "X-Axis Cell Range (e.g., A1:A10)" }
                input { 
                    style: INPUT_STYLE, 
                    placeholder: "Enter X-axis cell range",
                    oninput: move |evt| x_range.set(evt.value().clone())
                }
            }
            div {
                div { style: LABEL_STYLE, "Y-Axis Cell Range (e.g., B1:B10)" }
                input { 
                    style: INPUT_STYLE, 
                    placeholder: "Enter Y-axis cell range",
                    oninput: move |evt| y_range.set(evt.value().clone())
                }
            }
            div {
                div { style: LABEL_STYLE, "X-Axis Label" }
                input { 
                    style: INPUT_STYLE, 
                    placeholder: "X-Axis",
                    oninput: move |evt| x_label.set(evt.value().clone())
                }
            }
            div {
                div { style: LABEL_STYLE, "Y-Axis Label" }
                input { 
                    style: INPUT_STYLE, 
                    placeholder: "Y-Axis",
                    oninput: move |evt| y_label.set(evt.value().clone())
                }
            }
            div {
                div { style: LABEL_STYLE, "Chart Title (Optional)" }
                input { 
                    style: INPUT_STYLE, 
                    placeholder: "Enter title",
                    oninput: move |evt| title.set(evt.value().clone())
                }
            }
            button { 
                style: SUBMIT_BUTTON_STYLE,
                onclick: move |_| {
                    // Handle submission
                    println!("Scatter plot with x-range: {}, y-range: {}, x-label: {}, y-label: {}, title: {}", 
                             x_range.cloned(), y_range.cloned(), x_label.cloned(), y_label.cloned(), title.cloned());
                },
                "Generate Chart"
            }
        }
    }
}

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
