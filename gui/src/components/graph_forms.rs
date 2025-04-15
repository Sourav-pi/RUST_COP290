use dioxus::prelude::*;
use super::spreadsheet::SheetContext;

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

    let mut show_graph = use_signal(|| false);
    let mut range = use_signal(|| String::new());
    let mut x_label = use_signal(|| String::new());
    let mut y_label = use_signal(|| String::new());
    let mut title = use_signal(|| String::new());
    let mut point_labels = use_signal(|| String::new());
    let sheet = use_context::<SheetContext>();
    let mut chart_json = use_signal(|| String::new());

    if show_graph.cloned(){
        if let Ok(sheet_locked) = sheet.cloned().lock() {
            // Update the cell value in the Sheet object
            let x = sheet_locked.line_graph(
                &range.cloned(),
                &x_label.cloned(),
                &y_label.cloned(),
                &title.cloned(),
            );
            if let Ok(json) = x {
                chart_json.set(json);
            } else {
                println!("Error generating chart: {}", x.err().unwrap());
                return rsx! { div { "Error generating chart" } };
            }
        }
        let mount_code = format!(
            r#"
                var millis = 500;
                setTimeout(function() {{
                    var chart = echarts.init(document.getElementById('chart'), null, {{renderer: 'canvas'}});
                    window.addEventListener('resize', function() {{
                        chart.resize();
                    }});
                    chart.setOption({});
                }}, millis)
            "#, chart_json.cloned()
        );
        return rsx! {
            document::Script { src: asset!("/assets/echarts-5.5.1.min.js") }
            div { style: "width: 100%; text-align: center; display: flex; justify-content: center; align-items: center;",
            div { id: "chart", style: "display: inline-block; height: 80%; width:80% ; background-color: white; border-radius: 10px;", onmounted: move |_| {
                document::eval(&mount_code);
            },}
        }

        }
    } else {


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
                    show_graph.set(true);
                },
                "Generate Chart"
            }
        }
    }
}
}

#[component]
pub fn BarChartForm() -> Element {
    let mut show_graph = use_signal(|| false);
    let mut range = use_signal(|| String::new());
    let mut x_label = use_signal(|| String::new());
    let mut y_label = use_signal(|| String::new());
    let mut title = use_signal(|| String::new());
    let mut bar_labels = use_signal(|| String::new());
    let sheet = use_context::<SheetContext>();
    let mut chart_json = use_signal(|| String::new());

    if show_graph.cloned() {
        if let Ok(sheet_locked) = sheet.cloned().lock() {
            // Generate bar chart
            let x = sheet_locked.bar_graph(
                &range.cloned(),
                &bar_labels.cloned(),
                &y_label.cloned(),
                &title.cloned(),
            );
            if let Ok(json) = x {
                chart_json.set(json);
            } else {
                println!("Error generating bar chart: {}", x.err().unwrap());
                return rsx! { div { "Error generating bar chart" } };
            }
        }
        
        let mount_code = format!(
            r#"
                var millis = 500;
                setTimeout(function() {{
                    var chart = echarts.init(document.getElementById('chart'), null, {{renderer: 'canvas'}});
                    window.addEventListener('resize', function() {{
                        chart.resize();
                    }});
                    chart.setOption({});
                }}, millis)
            "#, chart_json.cloned()
        );
        
        return rsx! {
            document::Script { src: asset!("/assets/echarts-5.5.1.min.js") }
            div { style: "width: 100%; text-align: center; display: flex; justify-content: center; align-items: center;",
                div { 
                    id: "chart", 
                    style: "display: inline-block; height: 80%; width:80%; background-color: white; border-radius: 10px;", 
                    onmounted: move |_| {
                        document::eval(&mount_code);
                    }
                }
            }
        }
    } else {
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
                    div { style: LABEL_STYLE, "Bar Labels (Comma-separated)" }
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
                        println!("Bar chart with range: {}, labels: {}, y: {}, title: {}", 
                                range.cloned(), bar_labels.cloned(), y_label.cloned(), title.cloned());
                        show_graph.set(true);
                    },
                    "Generate Chart"
                }
            }
        }
    }
}

#[component]
pub fn PieChartForm() -> Element {
    let mut show_graph = use_signal(|| false);
    let mut range = use_signal(|| String::new());
    let mut title = use_signal(|| String::new());
    let mut slice_labels = use_signal(|| String::new());
    let sheet = use_context::<SheetContext>();
    let mut chart_json = use_signal(|| String::new());

    if show_graph.cloned() {
        if let Ok(sheet_locked) = sheet.cloned().lock() {
            // Generate pie chart
            let x = sheet_locked.pie_graph(
                &range.cloned(),
                &slice_labels.cloned(),
                &title.cloned(),
            );
            if let Ok(json) = x {
                chart_json.set(json);
            } else {
                println!("Error generating pie chart: {}", x.err().unwrap());
                return rsx! { div { "Error generating pie chart" } };
            }
        }
        
        let mount_code = format!(
            r#"
                var millis = 500;
                setTimeout(function() {{
                    var chart = echarts.init(document.getElementById('pie-chart'), null, {{renderer: 'canvas'}});
                    window.addEventListener('resize', function() {{
                        chart.resize();
                    }});
                    chart.setOption({});
                }}, millis)
            "#, chart_json.cloned()
        );
        
        return rsx! {
            document::Script { src: asset!("/assets/echarts-5.5.1.min.js") }
            div { style: "width: 100%; text-align: center; display: flex; justify-content: center; align-items: center;",
                div { 
                    id: "pie-chart", 
                    style: "display: inline-block; height: 80%; width:80%; background-color: white; border-radius: 10px;", 
                    onmounted: move |_| {
                        document::eval(&mount_code);
                    }
                }
            }
        }
    } else {
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
                    div { style: LABEL_STYLE, "Slice Labels (Comma-separated)" }
                    input { 
                        style: INPUT_STYLE, 
                        placeholder: "Comma-separated labels",
                        oninput: move |evt| slice_labels.set(evt.value().clone())
                    }
                }
                // Keep the placeholder divs for consistent layout
                div { style : "visibility: hidden ",
                    div { style: LABEL_STYLE, "placeholder" }
                    input { 
                        style: INPUT_STYLE, 
                        placeholder: "placeholder"
                    }
                }
                div { style : "visibility: hidden ",
                    div { style: LABEL_STYLE, "placeholder" }
                    input { 
                        style: INPUT_STYLE, 
                        placeholder: "placeholder"
                    }
                }
                button { 
                    style: SUBMIT_BUTTON_STYLE,
                    onclick: move |_| {
                        println!("Pie chart with range: {}, title: {}, labels: {}", 
                                range.cloned(), title.cloned(), slice_labels.cloned());
                        show_graph.set(true);
                    },
                    "Generate Chart"
                }
            }
        }
    }
}

#[component]
pub fn ScatterPlotForm() -> Element {
    let mut show_graph = use_signal(|| false);
    let mut x_range = use_signal(|| String::new());
    let mut y_range = use_signal(|| String::new());
    let mut x_label = use_signal(|| String::new());
    let mut y_label = use_signal(|| String::new());
    let mut title = use_signal(|| String::new());
    let sheet = use_context::<SheetContext>();
    let mut chart_json = use_signal(|| String::new());

    if show_graph.cloned() {
        if let Ok(sheet_locked) = sheet.cloned().lock() {
            // Generate scatter plot
            let x = sheet_locked.scatter_graph(
                &x_range.cloned(),
                &y_range.cloned(),
                &title.cloned(),
                &x_label.cloned(),
                &y_label.cloned()
            );
            if let Ok(json) = x {
                chart_json.set(json);
            } else {
                println!("Error generating scatter plot: {}", x.err().unwrap());
                return rsx! { div { "Error generating scatter plot" } };
            }
        }
        
        let mount_code = format!(
            r#"
                var millis = 500;
                setTimeout(function() {{
                    var chart = echarts.init(document.getElementById('scatter-chart'), null, {{renderer: 'canvas'}});
                    window.addEventListener('resize', function() {{
                        chart.resize();
                    }});
                    chart.setOption({});
                }}, millis)
            "#, chart_json.cloned()
        );
        
        return rsx! {
            document::Script { src: asset!("/assets/echarts-5.5.1.min.js") }
            div { style: "width: 100%; text-align: center; display: flex; justify-content: center; align-items: center;",
                div { 
                    id: "scatter-chart", 
                    style: "display: inline-block; height: 80%; width:80%; background-color: white; border-radius: 10px;", 
                    onmounted: move |_| {
                        document::eval(&mount_code);
                    }
                }
            }
        }
    } else {
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
                        println!("Scatter plot with x-range: {}, y-range: {}, x-label: {}, y-label: {}, title: {}", 
                                x_range.cloned(), y_range.cloned(), x_label.cloned(), y_label.cloned(), title.cloned());
                        show_graph.set(true);
                    },
                    "Generate Chart"
                }
            }
        }
    }
}
