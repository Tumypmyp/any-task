use crate::components::button::{ButtonVariant, ButtonWithHolder};
use dioxus::prelude::*;
#[component]
pub fn Title(title: String) -> Element {
    rsx! {
        ButtonWithHolder { width: "50vw", variant: ButtonVariant::Ghost, "{title}" }
    }
}
#[component]
pub fn Header(children: Element) -> Element {
    rsx! {
        div { style: "
                // align-items: center;
                flex-direction: row;
                display: flex;
                gap: 5px;
                // justify-content: space-between;
                // padding: 0.5rem 0;
            ",
            {children}
        }
    }
}
