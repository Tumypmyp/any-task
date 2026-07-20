use crate::API_CLIENT;
use crate::Route;
use crate::components::action::*;
use crate::components::button::Button;
use crate::components::column::Column;
use crate::components::header::{Header, Title};
use crate::helpers::*;
use dioxus::prelude::*;
#[component]
pub fn SpaceLayout(space_id: String) -> Element {
    let space_id = use_signal(|| space_id.clone());
    use_resource(move || {
        let _reconnect = RECONNECT_COUNT.read();
        let client = API_CLIENT.read().as_ref().cloned();
        let space_id = space_id();
        async move {
            let Some(client) = client else {
                return;
            };
            match client.subscribe_relation_options(&space_id).await {
                Ok(resp) => {
                    let mut state = RELATION_OPTIONS.write();
                    state.by_relation.clear();
                    state.details.clear();
                    for record in resp.records {
                        let id = extract_string(record.fields.get("id"));
                        let relation_key = extract_string(record.fields.get("relationKey"));
                        let opt = RelationOptionDetails {
                            id: id.clone(),
                            name: extract_string(record.fields.get("name")),
                            color: extract_string(record.fields.get("relationOptionColor")),
                            relation_key: relation_key.clone(),
                        };
                        state
                            .by_relation
                            .entry(relation_key)
                            .or_default()
                            .push(id.clone());
                        state.details.insert(id, opt);
                    }
                }
                Err(e) => tracing::error!("subscribe_relation_options failed: {e:#}"),
            }
        }
    });
    use_drop(move || {
        spawn(async move {
            if let Some(client) = API_CLIENT.read().as_ref().cloned() {
                client.unsubscribe_relation_options(&space_id()).await.ok();
            }
        });
        *RELATION_OPTIONS.write() = RelationOptionsState::default();
    });

    rsx! {
        Outlet::<Route> {}
        BaseActions {}
    }
}
