use crate::API_CLIENT;
use crate::Route;
use dioxus::prelude::*;
use crate::Logout;
use crate::components::Title;
use crate::components::base::error;
use crate::components::base::ButtonHolder;
#[component]
pub fn Home() -> Element {
    let nav = navigator();
    _ = document::eval("document.documentElement.setAttribute('data-theme', 'dark');");
    let spaces = use_resource(|| async move { API_CLIENT.read().list_spaces().await });
    tracing::info!("Opened home");
    match &*spaces.read() {
        Some(Ok(s)) => {
            rsx! {
                Title { title: "Spaces" }
                div { id: "space-list",
                    for space in s.clone().data.unwrap_or_default().clone() {
                        ButtonHolder { key: "{space.clone().id.unwrap_or_default()}",
                            button {
                                class: "button",
                                width: "90vw",
                                height: "8vh",
                                "data-style": "primary",
                                style: "font-size: 1.1rem;",
                                onclick: move |_| {
                                    nav.push(Route::Space {
                                        space_id: space.clone().id.unwrap_or_default(),
                                    });
                                },
                                "{space.clone().name.unwrap_or_default()}"
                            }
                        }
                    }
                }
                Logout {}
            }
        }
        Some(Err(e)) => {
            tracing::debug!("error: {:#?}", e);
            error(e.to_string());
            rsx!()
        }
        _ => rsx!(),
    }
}
