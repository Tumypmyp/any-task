use dioxus::{html::details::open, prelude::*};
use crate::Route;

const STYLE_CSS: Asset = asset!("/assets/styling/style.css");

#[component]
pub fn Home() -> Element {
    
    _ = document::eval("document.documentElement.setAttribute('data-theme', 'dark');");
    
    let nav = navigator();
    let spaces = use_resource(|| async move {
        let mut config = openapi::apis::configuration::Configuration::new();
        config.bearer_access_token = Some(TOKEN.read().clone());

        openapi::apis::spaces_api::list_spaces(&config, "2025-05-20", None, None)
            .await
    });
    match &*spaces.read() {
        Some(Ok(s)) => {
            rsx!{
                div { id: "space-list",
                    for space in s.clone().data.unwrap().clone() {
                        Link {
                            class: "button",
                            "data-style": "primary",
                            to: Route::Space {
                                id: space.clone().id.unwrap(),
                            },
                            "{space.clone().name.unwrap()}"
                        }
                    }
                }
                Token {}
            }
        },
        _ => rsx!{
            Token {}
        }   
    }
}

pub static TOKEN: GlobalSignal<String> = Global::new(|| "".to_string());
#[component]
pub fn Token() -> Element {
    let mut name = use_signal(|| "".to_string());

    rsx! {
        document::Link { rel: "stylesheet", href: STYLE_CSS }
        input {
            class: "input",
            placeholder: "Enter your Anytype token",
            // we tell the component what to render
            value: "{*TOKEN.read()}",
            // and what to do when the value changes
            oninput: move |event| *TOKEN.write() = event.value(),
        }
    }
}