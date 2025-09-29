use dioxus::prelude::*;
use crate::components::base::{ButtonVariant, ButtonWithHolder};
#[component]
pub fn Title(title: String) -> Element {
    rsx! {
        ButtonWithHolder { variant: ButtonVariant::Ghost, "{title}" }
    }
}
#[component]
pub fn Header(children: Element) -> Element {
    rsx! {
        div {
            display: "flex",
            "flex-direction": "row",
            "align-items": "center",
            {children}
        }
    }
}
