// this function will be called from the app to run the gui

mod components;
use components::Spreadsheet;
use dioxus::prelude::*;
pub fn run() {
    dioxus::launch(App);
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