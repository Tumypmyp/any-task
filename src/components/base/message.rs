use dioxus::prelude::*;
use dioxus_primitives::toast::{ToastOptions, use_toast};
use std::time::Duration;
pub fn error(text: &str, description: String) -> () {
    let toast_api = use_toast();
    toast_api.error(
        text.to_string(),
        ToastOptions::new()
            .description(description)
            .duration(Duration::from_secs(5))
            .permanent(false),
    );
}
pub fn info(text: String) -> () {
    let toast_api = use_toast();
    toast_api.info(
        text,
        ToastOptions::new()
            .duration(Duration::from_secs(3))
            .permanent(false),
    );
}
