// this function will be called from the app to run the gui

use dioxus::desktop::{Config, WindowBuilder};

mod components;
use components::Spreadsheet;
use dioxus::prelude::*;
pub fn run() {
    let window_builder = WindowBuilder::new()
        .with_title("Spreadsheet")
        .with_inner_size(dioxus::desktop::LogicalSize::new(1024, 768))
        .with_min_inner_size(dioxus::desktop::LogicalSize::new(400, 300))
        .with_resizable(true)
        .with_maximizable(true)
        .with_decorations(true)
        .with_always_on_top(false);
    let config = Config::new().with_window(window_builder);
    dioxus::LaunchBuilder::desktop()
        .with_cfg(config)
        .launch(App);
}

const STYLE: &str = "html {
    height: 100vh;
}

body {
    height: 100%;
    margin: 0;
}";

#[component]
fn App() -> Element {
    rsx! {
         style { "{STYLE}" },
        Spreadsheet {}

    }
}
