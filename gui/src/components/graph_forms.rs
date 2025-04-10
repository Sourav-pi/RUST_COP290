use dioxus::prelude::*;


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
pub fn LineChartForm() -> Element {
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
                div { style: LABEL_STYLE, "Point Labels (Optional)" }
                input { 
                    style: INPUT_STYLE, 
                    placeholder: "Comma-separated labels",
                    oninput: move |evt| point_labels.set(evt.value().clone())
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
                    println!("Line chart with range: {}, x: {}, y: {}, title: {}, labels: {}", 
                             range.cloned(), x_label.cloned(), y_label.cloned(), title.cloned(), point_labels.cloned());
                },
                "Generate Chart"
            }
        }
    }
}

#[component]
pub fn BarChartForm() -> Element {
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
                div { style: LABEL_STYLE, "Bar Labels (Optional)" }
                input { 
                    style: INPUT_STYLE, 
                    placeholder: "Comma-separated labels",
                    oninput: move |evt| bar_labels.set(evt.value().clone())
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
                    println!("Bar chart with range: {}, x: {}, y: {}, title: {}, labels: {}", 
                             range.cloned(), x_label.cloned(), y_label.cloned(), title.cloned(), bar_labels.cloned());
                },
                "Generate Chart"
            }
        }
    }
}

#[component]
pub fn PieChartForm() -> Element {
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
pub fn ScatterPlotForm() -> Element {
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
