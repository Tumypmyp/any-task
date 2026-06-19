use crate::components::action::{ActionHolder, BaseActions};
use crate::components::base::message;
use crate::components::column::*;
use crate::components::edit_view::*;
use crate::components::header::{Header, Title};
use crate::components::object::*;
use crate::components::select::*;
use crate::helpers::*;
use dioxus::prelude::*;
use dioxus_sdk_storage::LocalStorage;
use dioxus_sdk_storage::use_synced_storage;
use std::collections::HashMap;
use std::vec;

#[component]
pub fn List(space_id: ReadSignal<String>, list_id: ReadSignal<String>) -> Element {
    tracing::info!("loading space {space_id}, list {list_id}");
    let view_id = use_store(|| "".to_string());
    let storage_view_tree_key = format!("list-view-relations-tree-{}", list_id());

    let mut positions =
        use_synced_storage::<LocalStorage, TileTree>(storage_view_tree_key.clone(), || TileTree {
            root: NodeId(0),
            nodes: HashMap::from([
                (
                    NodeId(0),
                    Node::Split {
                        parent: None,
                        direction: SplitDirection::Row,
                        ratio: 0.5,
                        first: NodeId(1),
                        second: NodeId(2),
                    },
                ),
                (
                    NodeId(1),
                    Node::Pane {
                        parent: Some(NodeId(0)),
                        relation_key: RelationKey("name".to_string()),
                    },
                ),
                (
                    NodeId(2),
                    Node::Pane {
                        parent: Some(NodeId(0)),
                        relation_key: RelationKey("description".to_string()),
                    },
                ),
            ]),
        });
    let positions_store = use_store(|| positions.read().clone());

    use_effect(move || {
        let store_value = positions_store.read().clone();
        *positions.write() = store_value;
    });

    let all_properties_res = use_resource(move || async move {
        let client_guard = API_CLIENT.read();
        let client = client_guard
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("No API client available"))?;
        client.fetch_properties(&space_id()).await
    });

    match &*all_properties_res.read_unchecked() {
        None => return rsx! { "Loading..." },
        Some(Err(e)) => return rsx! { "Error: {e}" },
        Some(Ok(_)) => {}
    }

    let all_properties: Memo<HashMap<RelationKey, RelationInfo>> = use_memo(move || {
        all_properties_res
            .read()
            .as_ref()
            .and_then(|r| r.as_ref().ok())
            .cloned()
            .unwrap_or_default()
    });

    rsx! {
        ListHeader {
            space_id,
            list_id,
            view_id,
            positions: positions_store,
            all_properties,
        }
        Objects {
            space_id,
            list_id,
            view_id,
            all_properties,
            positions: positions_store,
        }
        ActionHolder { BaseActions {} }
    }
}
#[component]
pub fn ListHeader(
    space_id: ReadSignal<String>,
    list_id: ReadSignal<String>,
    view_id: Store<String>,
    positions: Store<TileTree>,
    all_properties: ReadSignal<HashMap<RelationKey, RelationInfo>>,
) -> Element {
    let name = SET_META.read().name.clone();
    rsx! {
        Header {
            Title { title: "{name}" }
            Views {}
            EditView {
                space_id,
                list_id,
                positions,
                all_properties,
            }
        }
    }
}

#[component]
pub fn Views() -> Element {
    let selected = use_memo(move || Some(SET_META.read().active_view_id.clone()));
    let selected_signal: ReadSignal<Option<String>> = selected.into();

    let views: Vec<(String, String)> = SET_META
        .read()
        .views
        .iter()
        .map(|view| (view.id.clone(), view.name.clone()))
        .collect();

    rsx! {
        Select::<String> {
            value: Some(selected_signal),
            on_value_change: move |new_id: Option<String>| {
                if let Some(id) = new_id {
                    SET_META.write().active_view_id = id;
                }
            },
            SelectTrigger { SelectValue {} }
            SelectList {
                SelectGroup {
                    for (i, (view_id, view_name)) in views.into_iter().enumerate() {
                        SelectOption::<String> {
                            key: "{view_id}",
                            index: i,
                            value: view_id.clone(),
                            text_value: view_name.clone(),
                            "{view_name}"
                            SelectItemIndicator {}
                        }
                    }
                }
            }
        }
    }
}
#[component]
pub fn Objects(
    space_id: ReadSignal<String>,
    list_id: ReadSignal<String>,
    view_id: ReadSignal<String>,
    positions: Store<TileTree>,
    all_properties: ReadSignal<HashMap<RelationKey, RelationInfo>>,
) -> Element {
    use_resource(move || {
        let _reconnect = RECONNECT_COUNT.read();
        let client = API_CLIENT.read().as_ref().cloned();
        async move {
            let Some(client) = client else {
                tracing::warn!("subscribe_set_meta: no client");
                return;
            };
            if let Err(e) = client.object_open(&space_id(), &list_id()).await {
                tracing::error!("subscribe_spaces failed: {e:#}");
            }
        }
    });
    use_resource(move || {
        let _reconnect = RECONNECT_COUNT.read();
        let sid = space_id.read().clone();
        let lid = list_id.read().clone();

        let meta = SET_META.read();
        let set_of_ids = meta.set_of.clone();
        let active_view_id = meta.active_view_id.clone();
        let (filters, sorts) = meta
            .views
            .iter()
            .find(|v| v.id == active_view_id)
            .map(|v| (v.filters.clone(), v.sorts.clone()))
            .unwrap_or_default();
        drop(meta);

        let keys = pane_keys(&positions.read());
        let client = API_CLIENT.read().as_ref().cloned();
        async move {
            let Some(client) = client else {
                return;
            };
            if set_of_ids.is_empty() {
                return; // wait until meta arrives
            }
            let mut state = LIST_OBJECTS.write();
            state.order.clear();
            state.details.clear();
            drop(state);
            match client
                .subscribe_list_objects(&sid, &lid, set_of_ids, keys, filters, sorts)
                .await
            {
                Ok(resp) => {
                    let mut state = LIST_OBJECTS.write();
                    for record in resp.records {
                        let id = extract_string(record.fields.get("id"));
                        let det = parse_object_details(&id, &record.fields);
                        state.order.push(id.clone());
                        state.details.insert(id, det);
                    }
                }
                Err(e) => tracing::error!("subscribe_list_objects: {e:#}"),
            }
        }
    });

    use_drop(move || {
        *SET_META.write() = SetMetaState::default();
        *LIST_OBJECTS.write() = ListObjectsState::default();
        let lid = list_id.peek().clone();
        spawn(async move {
            if let Some(client) = API_CLIENT.read().as_ref().cloned() {
                client.object_close(&space_id(), &list_id()).await.ok();
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
