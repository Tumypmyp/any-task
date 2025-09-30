use crate::Route;
use dioxus::prelude::*;
use crate::components::base::ButtonWithHolder;
#[component]
pub fn Actions() -> Element {
    let nav = navigator();
    rsx! {
        div { id: "actions-holder",
            ButtonWithHolder {
                onclick: move |_| {
                    nav.push(Route::Home {});
                },
                "Home"
            }
            ButtonWithHolder {
                onclick: move |_| {
                    nav.go_back();
                },
                "Back"
            }
        }
    }
}
