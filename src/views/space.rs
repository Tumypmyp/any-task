use crate::API_CLIENT;
use crate::Route;
use crate::components::action::*;
use crate::components::button::Button;
use crate::components::button::ButtonHolder;
use crate::components::button::ButtonVariant;
use crate::components::column::Column;
use crate::components::header::{Header, Title};
use dioxus::prelude::*;
#[component]
pub fn Space(space_id: String) -> Element {
    tracing::info!("loading space {space_id}");
    rsx! {
        Collections { space_id: space_id.clone() }
        ActionHolder { BaseActions {} }
    }
}
#[component]
pub fn Collections(space_id: ReadSignal<String>) -> Element {
    let resp = use_resource({
        let space_id = space_id.clone();
        move || {
            let space_id = space_id.clone();
            async move {
                let client_guard = API_CLIENT.read();
                let client = client_guard.as_ref().ok_or_else(|| {
                    anyhow::anyhow!("No API client available, try reloading the app")
                })?;
                client.fetch_sets(&space_id()).await
            }
        }
    });
    let collections = match &*resp.read() {
        None => {
            return rsx! { "Loading..." };
        }
        Some(Err(err)) => {
            tracing::debug!("Got error loading collections: {:#?}", err);
            return rsx! { "Error: {err}" };
        }
        Some(Ok(collections)) => collections.clone(),
    };
    let nav = navigator();
    rsx! {
        Column {
            for (id, name, _) in collections {
                Button {
                    onclick: move |_| {
                        nav.push(Route::List {
                            space_id: space_id(),
                            list_id: id.clone(),
                        });
                    },
                    "{name}"
                }
            }
        }
    }
}
