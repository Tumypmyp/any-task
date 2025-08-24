use crate::API_CLIENT;
use crate::Route;
use dioxus::prelude::*;
#[component]
pub fn Home() -> Element {
    let nav = navigator();
    
    _ = document::eval("document.documentElement.setAttribute('data-theme', 'dark');");
    let spaces = use_resource(|| async move { API_CLIENT.read().list_spaces().await });
    match &*spaces.read() {
        Some(Ok(s)) => {
            rsx! {
                div { id: "space-list",
                    for space in s.clone().data.unwrap_or_default().clone() {
                        div { class: "button-holder", key: "{space.clone().id.unwrap_or_default()}",
                            button {
                                class: "button",
                                width: "90vw",
                                height: "10vh",
                                "data-style": "primary",
                                onclick: move |_| {
                                    nav.push(Route::Space {
                                        id: space.clone().id.unwrap_or_default(),
                                    });
                                },
                                "{space.clone().name.unwrap_or_default()}"
                            }
                        }
                    }
                }
            }
        }
        Some(Err(e)) => {
            tracing::debug!("error: {:#?}", e);
            crate::error(e.to_string());
            rsx!()
        }
        _ => rsx!(),
    }
}
