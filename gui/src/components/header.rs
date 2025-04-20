//! Header component for the spreadsheet application.
//!
//! This module provides the header component that displays the application
//! title, current filename, and contains the toolbar and formula bar.

use super::formula_bar::*;
use super::toolbar::*;
use dioxus::prelude::*;

/// Icon for the spreadsheet application
const SPREADSHEET_ICON: Asset = asset!("assets/spreadsheet.png");

/// CSS styles for the header container
const HEAD_STYLE: &str = "
   width: 100vw;
   padding: 0px;
   margin: 0px;
";

/// Properties for the Header component
#[derive(Props, PartialEq, Clone)]
pub struct HeaderProps {
    /// The current filename being edited
    pub filename: String,
    /// Number of rows in the spreadsheet
    pub num_rows: usize,
    /// Number of columns in the spreadsheet
    pub num_cols: usize,
}

/// The Header component for the spreadsheet application.
///
/// Displays the application title, current filename, toolbar, and formula bar.
///
/// # Properties
/// * `filename` - The name of the current file being edited
/// * `num_rows` - Total number of rows in the spreadsheet
/// * `num_cols` - Total number of columns in the spreadsheet
#[component]
pub fn Header(props: HeaderProps) -> Element {
    rsx! {
      div {style : HEAD_STYLE,
        div {
          style: "height: 50px; display: flex; flex-direction: row; align-items: center; justify-content: space-between;margin: 10px 15px;",
          div {
              style: "display: flex; gap: 20px; flex-direction: row; align-items: center; justify-content: start; ",
              img {
                  src: "{SPREADSHEET_ICON}",
                  alt: "Logo",
                  style: "width: 50px; height: 50px;"
              }
              h1 {
                  style: "margin: 0; font-family: Arial, Helvetica, sans-serif",
                  "{props.filename}"
              }
          }
          Toolbar {
              num_rows: props.num_rows,
              num_cols: props.num_cols,
          }
        }
        FormulaBar {}
      }
    }
}
