use crate::API_CLIENT;
use crate::Logout;
use crate::Route;
use crate::components::Title;
use crate::components::base::message;
use crate::components::button::ButtonHolder;
use crate::components::list::List;
use dioxus::prelude::*;
#[component]
pub fn Home() -> Element {
    let nav = navigator();
    let client = API_CLIENT.read().cloned();
    let spaces = use_resource(move || {
        let client = client.clone();
        async move { client.list_spaces().await }
    });
    tracing::info!("Opened home");
    match &*spaces.read() {
        Some(Ok(s)) => {
            rsx! {
                Title { title: "Spaces" }
                List {
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
        Some(Err(err)) => {
            tracing::debug!("error: {:#?}", err);
            message::error("Failed to load spaces", err);
            nav.push(Route::Login {});
            rsx! {
                Logout {}
            }
        }
        None => {
            rsx!()
        }
    }
}
