use crate::API_CLIENT;
use crate::Route;
use crate::components::action::*;
use crate::components::button::Button;
use crate::components::button::ButtonVariant;
use crate::components::column::Column;
use crate::components::header::{Header, Title};
use crate::helpers::*;
use dioxus::prelude::*;
#[component]
pub fn Space(space_id: String) -> Element {
    tracing::info!("Loading space: {space_id}");
    rsx! {
        Collections { space_id: space_id.clone() }
        ActionHolder { BaseActions {} }
    }
}
#[component]
pub fn Collections(space_id: ReadSignal<String>) -> Element {
    use_resource(move || {
        let _reconnect = RECONNECT_COUNT.read();
        let client = API_CLIENT.read().as_ref().cloned();
        let space_id = space_id.clone();
        async move {
            let Some(client) = client else {
                return;
            };
            if let Err(e) = client.subscribe_sets(&space_id()).await {
                tracing::error!("subscribe_sets failed: {e:#}");
            }
        }
    });

    use_drop(move || {
        spawn(async move {
            if let Some(client) = API_CLIENT.read().as_ref().cloned() {
                client.unsubscribe_sets(&space_id()).await.ok();
            }
        });
        *SETS.write() = SetsState::default();
    });

    let items: Vec<(String, String)> = {
        let sets = SETS.read();
        sets.order
            .iter()
            .filter_map(|id| {
                sets.details
                    .get(id)
                    .map(|det| (det.object_id.clone(), det.name.clone()))
            })
            .collect()
    };
    let nav = navigator();
    rsx! {
        Column { style: "align-items: center; gap: 6px;",
            for (object_id, name) in items {
                Button {
                    onclick: move |_| {
                        nav.push(Route::List {
                            space_id: space_id(),
                            list_id: object_id.clone(),
                        });
                    },
                    "{name}"
                }
            }
        }
    }
}
