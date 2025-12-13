use crate::components::button::{Button, ButtonVariant};
use dioxus::prelude::*;
#[component]
pub fn Title(title: String) -> Element {
    rsx! {
        Button { width: "50vw", variant: ButtonVariant::Ghost, "{title}" }
    }
}
#[component]
pub fn Header(children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div { class: "header", {children} }
    }
}
