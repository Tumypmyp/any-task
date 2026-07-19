use dioxus_icons::lucide::{Eye, EyeOff};
use dioxus::prelude::*;

#[component]
pub fn ShowHideButton(show: Signal<bool>) -> Element {
    rsx! {
        button {
            r#type: "button",
            style: "position: absolute; right: 0.5rem; top: 50%; transform: translateY(-50%); \
                    background: none; border: none; cursor: pointer; display: flex; \
                    align-items: center; color: inherit; padding: 0;",
            onclick: move |_| show.toggle(),
            aria_label: if show() { "Hide" } else { "Show" },
            if show() {
                EyeOff { size: "1rem" }
            } else {
                Eye { size: "1rem" }
            }
        }
    }
}
