use crate::components::column::*;
use crate::components::object::*;
use crate::helpers::*;
use dioxus::prelude::*;
use std::collections::HashMap;
use std::vec;

#[component]
pub fn ObjectsList(
    space_id: ReadSignal<String>,
    list_id: ReadSignal<String>,
    view_id: ReadSignal<String>,
    positions: ReadSignal<TileTree>,
    all_properties: ReadSignal<HashMap<RelationKey, RelationInfo>>,
) -> Element {
    let keys = use_memo(move || pane_keys(&positions.read()));
    let mut view_type = use_signal(|| 0);
    use_resource(move || {
        let _reconnect = RECONNECT_COUNT.read();
        let sid = space_id.read().clone();
        let lid = list_id.read().clone();

        let meta = SET_META.read();
        let set_of_ids = meta.set_of.clone();
        let active_view_id = meta.active_view_id.clone();
        let (filters, sorts, vtype) = meta
            .views
            .iter()
            .find(|v| v.id == active_view_id)
            .map(|v| (v.filters.clone(), v.sorts.clone(), v.r#type.clone()))
            .unwrap_or_default();
        view_type.set(vtype);
        drop(meta);

        let client = API_CLIENT.read().as_ref().cloned();
        async move {
            let Some(client) = client else { return };
            if set_of_ids.is_empty() {
                return;
            }
            // preload first objects of a set
            let phase1 = tokio::spawn({
                let client = client.clone();
                let sid = sid.clone();
                let lid = lid.clone();
                let set_of_ids = set_of_ids.clone();
                let keys = keys();
                let filters = filters.clone();
                let sorts = sorts.clone();
                async move {
                    client
                        .subscribe_list_objects(&sid, &lid, set_of_ids, keys, filters, sorts, 15)
                        .await
                }
            });

            match phase1.await {
                Ok(Ok(resp)) => {
                    let mut new_order = Vec::new();
                    let mut new_details = HashMap::new();
                    for record in resp.records {
                        let id = extract_string(record.fields.get("id"));
                        let det = parse_object_details(&id, &record.fields);
                        new_order.push(id.clone());
                        new_details.insert(id, det);
                    }
                    let mut state = LIST_OBJECTS.write();
                    state.order = new_order;
                    state.details = new_details;
                }
                Ok(Err(e)) => tracing::error!("subscribe_list_objects phase1: {e:#}"),
                Err(e) => tracing::error!("subscribe_list_objects phase1 panicked: {e}"),
            }

            // load 100 objects of a set (with all objects tile edit is still slow)
            // todo: load all objects
            let phase2 = tokio::spawn({
                let client = client.clone();
                let sid = sid.clone();
                let lid = lid.clone();
                let keys = keys();
                async move {
                    client
                        .subscribe_list_objects(&sid, &lid, set_of_ids, keys, filters, sorts, 100)
                        .await
                }
            });

            match phase2.await {
                Ok(Ok(resp)) => {
                    let mut new_order = Vec::new();
                    let mut new_details = HashMap::new();
                    for record in resp.records {
                        let id = extract_string(record.fields.get("id"));
                        let det = parse_object_details(&id, &record.fields);
                        new_order.push(id.clone());
                        new_details.insert(id, det);
                    }
                    let mut state = LIST_OBJECTS.write();
                    state.order = new_order;
                    state.details = new_details;
                }
                Ok(Err(e)) => tracing::error!("subscribe_list_objects phase2: {e:#}"),
                Err(e) => tracing::error!("subscribe_list_objects phase2 panicked: {e}"),
            }
        }
    });

    use_drop(move || {
        *LIST_OBJECTS.write() = ListObjectsState::default();
        let lid = list_id.peek().clone();
        spawn(async move {
            if let Some(client) = API_CLIENT.read().as_ref().cloned() {
                client.unsubscribe_list_objects(&lid).await.ok();
            }
        });
    });
    let items: Vec<ObjectDetails> = {
        let state = LIST_OBJECTS.read();
        state
            .order
            .iter()
            .filter_map(|id| state.details.get(id).cloned())
            .collect()
    };
    rsx! {
        Column { style: "width: 98vw;",
            for det in items {
                Object {
                    key: "{det.id}",
                    positions,
                    details: det,
                    all_properties,
                }
            }
        }
    }
}

fn pane_keys(tree: &TileTree) -> Vec<String> {
    let mut keys = vec!["id".to_string(), "name".to_string()];
    for node in tree.nodes.values() {
        if let Node::Pane { relation_key, .. } = node {
            let k = relation_key.as_str().to_string();
            if !keys.contains(&k.clone()) {
                keys.push(k.to_string());
            }
        }
    }
    keys
}
