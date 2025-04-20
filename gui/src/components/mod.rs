//! GUI components for the spreadsheet application.
//!
//! This module contains all UI components used in the GUI version of the
//! spreadsheet application. The main component is `Spreadsheet`, which
//! composes the other components to create the complete interface.

mod spreadsheet;
pub use spreadsheet::Spreadsheet;

mod cell;
mod context_menu;
mod error_display;
mod formula_bar;
mod graph_forms;
mod graph_popup;
mod grid;
mod header;
mod row;
mod toolbar;
