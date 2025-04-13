use dioxus::prelude::*;
use std::time::Duration;
use dioxus_timer::use_timer;

// Define error types/severity levels
#[derive(Clone, PartialEq)]
pub enum ErrorType {
    Error,
    Warning,
    Info,
}

// Signal to store active errors
pub type ErrorContext = Signal<Option<(String, ErrorType, Option<f64>)>>;

// Style for overlay that captures clicks outside
const OVERLAY_STYLE: &str = r#"
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    z-index: 999;
    background-color: rgba(0, 0, 0, 0.1);
"#;

// Styles for different error types
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

const ERROR_STYLE: &str = r#"
    background-color: #f8d7da;
    border: 1px solid #f5c2c7;
    color: #842029;
"#;

const WARNING_STYLE: &str = r#"
    background-color: #fff3cd;
    border: 1px solid #ffecb5;
    color: #664d03;
"#;

const INFO_STYLE: &str = r#"
    background-color: #cff4fc;
    border: 1px solid #b6effb;
    color: #055160;
"#;

// Close button style
const CLOSE_BUTTON_STYLE: &str = r#"
    background: none;
    border: none;
    cursor: pointer;
    font-size: 18px;
    color: inherit;
    margin-left: 10px;
"#;

// Error display component
#[component]
pub fn ErrorDisplay() -> Element {
    let mut error_ctx = use_context::<ErrorContext>();
    
    // Handle auto-dismissal with timer
    // use_effect(move || {
    //     if let Some((_, _, Some(timeout_seconds))) = error_ctx.read().as_ref() {
    //         let error_ctx_clone = error_ctx.clone();
    //         let duration_ms = (*timeout_seconds * 1000.0) as u64;
            
    //         // Set a one-time timer to clear the error after the timeout
    //         use_timer(move || {
    //             error_ctx_clone.set(None);
    //         }, Duration::from_millis(duration_ms));
    //     }
        
    //     // Cleanup function (not necessary here but required for use_effect)
    //     || {}
    // });
    
    // Render error if present
    let error_data = error_ctx.read();
    if let Some((message, error_type, _)) = error_data.as_ref() {
        let type_style = match error_type {
            ErrorType::Error => ERROR_STYLE,
            ErrorType::Warning => WARNING_STYLE,
            ErrorType::Info => INFO_STYLE,
        };
        
        let icon = match error_type {
            ErrorType::Error => "⛔",
            ErrorType::Warning => "⚠️",
            ErrorType::Info => "ℹ️",
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

// Helper function to show an error
pub fn show_error(
    error_ctx: &mut ErrorContext,
    message: &str, 
    error_type: ErrorType, 
    timeout_seconds: Option<f64>
) {
    error_ctx.set(Some((message.to_string(), error_type, timeout_seconds)));
}