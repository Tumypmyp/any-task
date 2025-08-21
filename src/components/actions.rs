use crate::Route;
use dioxus::prelude::*;
#[component]
pub fn Actions() -> Element {
    rsx! {
        div { id: "actions-holder",
            Link {
                class: "button",
                "data-style": "outline",
                to: Route::Home {},
                "Home"
            }
        }
    }
}
