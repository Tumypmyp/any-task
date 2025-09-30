use dioxus::prelude::*;
use crate::components::base::{ButtonVariant, ButtonWithHolder};
#[component]
pub fn Title(title: String) -> Element {
    rsx! {
        ButtonWithHolder { width: "20vw", variant: ButtonVariant::Ghost, "{title}" }
    }
}
#[component]
pub fn Header(children: Element) -> Element {
    rsx! {
        div {
            display: "flex",
            style: "
                flex-direction: row;            
                display: flex;
                align-items: center;
                gap: 10vw;
                // justify-content: space-between;
                padding: 0.5rem 0;
            ",
            {children}
        }
    }
}
