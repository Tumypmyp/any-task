use crate::Route;
use dioxus::prelude::*;
#[component]
pub fn Actions() -> Element {
    let nav = navigator();
    rsx! {
        div { id: "actions-holder",
            div { class: "button-holder",
                button {
                    class: "button",
                    "data-style": "outline",
                    onclick: move |_| {
                        nav.push(Route::Home {});
                    },
                    "Home"
                }
            }
        }
    }
}
