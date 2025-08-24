use crate::Route;
use dioxus::prelude::*;
use dioxus_primitives::toast::{ToastOptions, use_toast};
use std::time::Duration;
pub fn error(text: String) -> () {
    let nav = navigator();
    nav.push(Route::Login {});
    let toast_api = use_toast();
    toast_api
        .error(
            text,
            ToastOptions::new()
                .description("Some info you need")
                .duration(Duration::from_secs(5))
                .permanent(false),
        );
}
pub fn info(text: String) -> () {
    let toast_api = use_toast();
    toast_api
        .info(
            text,
            ToastOptions::new()
                .description("Some info you need")
                .duration(Duration::from_secs(5))
                .permanent(false),
        );
}
