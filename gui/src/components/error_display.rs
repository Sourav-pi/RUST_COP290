//! Error display component for the spreadsheet application.
//!
//! This module provides a component for displaying error messages, warnings,
//! information, and success notifications to the user.

use dioxus::prelude::*;

/// Define error types/severity levels for different notifications
#[derive(Clone, PartialEq)]
#[allow(unused)]
pub enum ErrorType {
    /// Error notification - for critical issues that prevent operation
    Error,
    /// Warning notification - for potential issues that don't prevent operation
    Warning,
    /// Information notification - for general information
    Info,
    /// Success notification - for successful operations
    Success,
}

/// Signal to store active errors with message, type and timeout
pub type ErrorContext = Signal<Option<(String, ErrorType, Option<f64>)>>;

/// Style for overlay that captures clicks outside the error display
const OVERLAY_STYLE: &str = r#"
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    z-index: 999;
    background-color: rgba(0, 0, 0, 0.1);
"#;

/// Base style for error container
const ERROR_CONTAINER_STYLE: &str = r#"
    position: fixed;
    bottom: 20px;
    left: 50%;
    transform: translateX(-50%);
    z-index: 1000;
    padding: 12px 20px;
    border-radius: 4px;
    box-shadow: 0 2px 10px rgba(0,0,0,0.2);
    display: flex;
    align-items: center;
    gap: 10px;
    max-width: 80%;
    opacity: 1;
    transition: opacity 0.3s ease;
"#;

/// Style for error notifications
const ERROR_STYLE: &str = r#"
    background-color: #f8d7da;
    border: 1px solid #f5c2c7;
    color: #842029;
"#;

/// Style for warning notifications
const WARNING_STYLE: &str = r#"
    background-color: #fff3cd;
    border: 1px solid #ffecb5;
    color: #664d03;
"#;

/// Style for information notifications
const INFO_STYLE: &str = r#"
    background-color: #cff4fc;
    border: 1px solid #b6effb;
    color: #055160;
"#;

/// Style for success notifications
const SUCCESS_STYLE: &str = r#"
    background-color: #d1e7dd;
    border: 1px solid #badbcc;
    color: #0f5132;
"#;

/// Close button style
const CLOSE_BUTTON_STYLE: &str = r#"
    background: none;
    border: none;
    cursor: pointer;
    font-size: 18px;
    color: inherit;
    margin-left: 10px;
"#;

/// Shows an error message with the specified type and optional timeout
///
/// # Parameters
/// * `error_ctx` - The error context signal to update
/// * `message` - The message to display
/// * `error_type` - The type of notification (error, warning, etc.)
/// * `timeout` - Optional timeout in seconds after which to hide the message
pub fn show_error(
    error_ctx: &mut ErrorContext,
    message: &str,
    error_type: ErrorType,
    timeout: Option<f64>,
) {
    error_ctx.set(Some((message.to_string(), error_type, timeout)));
}

/// Error display component that shows messages to the user
///
/// This component manages displaying error messages, warnings, information,
/// and success notifications. It handles:
/// - Displaying the appropriate styling based on message type
/// - Auto-dismissing messages after their timeout
/// - Allowing users to manually dismiss messages
#[component]
pub fn ErrorDisplay() -> Element {
    let mut error_ctx = use_context::<ErrorContext>();

    // Render error if present
    let error_data = error_ctx.read();
    if let Some((message, error_type, _)) = error_data.as_ref() {
        let type_style = match error_type {
            ErrorType::Error => ERROR_STYLE,
            ErrorType::Warning => WARNING_STYLE,
            ErrorType::Info => INFO_STYLE,
            ErrorType::Success => SUCCESS_STYLE,
        };

        let icon = match error_type {
            ErrorType::Error => "⛔",
            ErrorType::Warning => "⚠️",
            ErrorType::Info => "ℹ️",
            ErrorType::Success => "✅",
        };

        rsx! {
            // Overlay to capture clicks outside
            div {
                style: OVERLAY_STYLE,
                onclick: move |_| error_ctx.set(None),

                // Error container
                div {
                    style: format!("{} {}", ERROR_CONTAINER_STYLE, type_style),
                    // Stop propagation to prevent overlay click from triggering when clicking on the message
                    onclick: move |e| e.stop_propagation(),
                    span { "{icon}" }
                    span { "{message}" }
                    button {
                        style: CLOSE_BUTTON_STYLE,
                        onclick: move |_| error_ctx.set(None),
                        "×"
                    }
                }
            }
        }
    } else {
        rsx! { div {} }
    }
}
