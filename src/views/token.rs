use dioxus::prelude::*;
use crate::Route;
use crate::API_CLIENT;

const STYLE_CSS: Asset = asset!("/assets/styling/style.css");

#[component]
pub fn Token() -> Element {
    let mut name = use_signal(|| "".to_string());
    rsx! {
        document::Link { rel: "stylesheet", href: STYLE_CSS }
        div {
            id: "token-holder",
            input {
                class: "input",
                placeholder: "Paste your Anytype token",
                value: "{name.read()}",
                oninput: move |event| {
                    *name.write() = event.value();
                    API_CLIENT.write().set_token(event.value());

                    // todo: check before redirect
                    let nav = navigator();
                    nav.push(Route::Home {});
                },
            }
        }
    }
}