use crate::components::button::{Button, ButtonVariant};
use dioxus::prelude::*;
#[component]
pub fn Title(title: String) -> Element {
    rsx! {
        Button {
            style: "position: absolute; left: 50%; transform: translateX(-50%); width: auto; max-width: 50vw; z-index: 10;",
            variant: ButtonVariant::Ghost,
            "{title}"
        }
    }
}
#[component]
pub fn Header(children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div { class: "header", {children} }
    }
}
