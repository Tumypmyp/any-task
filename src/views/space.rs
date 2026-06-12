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
    let sub_id = sets_sub_id(space_id());
    let sub_id_drop = sub_id.clone();

    use_resource(move || {
        let _reconnect = RECONNECT_COUNT.read();
        let client = API_CLIENT.read().as_ref().cloned();
        let space_id = space_id.clone();
        let sub_id = sub_id.clone();
        async move {
            let Some(client) = client else {
                return;
            };
            let resp = match client.subscribe_sets(space_id(), &sub_id).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!("subscribe_sets failed: {e:#}");
                    return;
                }
            };
            let mut state = SETS.write();
            state.order.clear();
            state.details.clear();
            for record in resp.records {
                let id = extract_string(record.fields.get("id"));
                let det = SetDetails {
                    object_id: id.clone(),
                    name: extract_string(record.fields.get("name")),
                    layout: extract_number(record.fields.get("resolvedLayout")),
                };
                state.order.push(id.clone());
                state.details.insert(id, det);
            }
        }
    });

    use_drop(move || {
        let sub_id = sub_id_drop.clone();
        spawn(async move {
            if let Some(client) = API_CLIENT.read().as_ref().cloned() {
                client.unsubscribe_sets(sub_id).await.ok();
            }
        });
        // Clear state so stale data doesn't show on next mount
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
