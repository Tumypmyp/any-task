use crate::components::action::{ActionHolder, BaseActions};
use crate::components::base::message;
use crate::components::column::*;
use crate::components::edit_view::*;
use crate::components::header::{Header, Title};
use crate::components::object::*;
use crate::components::separator::Separator;
use crate::helpers::*;
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

    let all_properties: Memo<Vec<RelationInfo>> = use_memo(move || {
        all_properties_res
            .read()
            .as_ref()
            .and_then(|r| r.as_ref().ok())
            .map(|props| {
                let mut sorted_props: Vec<RelationInfo> = props.to_vec();
                sorted_props.sort_by_cached_key(|prop| prop.name.to_lowercase());
                sorted_props
            })
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
    all_properties: ReadSignal<Vec<RelationInfo>>,
) -> Element {
    let name = SET_META.read().name.clone();
    rsx! {
        Header {
            Title { title: "{name}" }
            EditView {
                space_id,
                list_id,
                view_id,
                positions,
                all_properties,
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
) -> Element {
    use_resource(move || {
        let _reconnect = RECONNECT_COUNT.read();
        let sid = space_id.read().clone();
        let lid = list_id.read().clone();
        let client = API_CLIENT.read().as_ref().cloned();
        async move {
            let Some(client) = client else {
                tracing::warn!("subscribe_set_meta: no client");
                return;
            };
            match client.subscribe_set_meta(&sid, &lid).await {
                Ok(resp) => {
                    if let Some(record) = resp.records.first() {
                        let mut state = SET_META.write();
                        state.name = extract_string(record.fields.get("name"));
                        state.set_of_ids = extract_list_strings(record.fields.get("setOf"));
                    }
                }
                Err(e) => tracing::error!("subscribe_set_meta: {e:#}"),
            }
        }
    });
    use_resource(move || {
        let _reconnect = RECONNECT_COUNT.read();
        let sid = space_id.read().clone();
        let lid = list_id.read().clone();
        let set_of_ids = SET_META.read().set_of_ids.clone(); // ← tracked dependency
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
                .subscribe_list_objects(&sid, &lid, set_of_ids, keys)
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
                client.unsubscribe_set_meta(&lid).await.ok();
                client.unsubscribe_list_objects(&lid).await.ok();
            }
        });
    });

    // use_resource(move || {
    //     let _reconnect = RECONNECT_COUNT.read();
    //     let client = API_CLIENT.read().as_ref().cloned();
    //     let sid = space_id();
    //     let lid = list_id();
    //     let sets = SETS.read().details.clone();
    //     tracing::debug!("sets: {:#?}", sets);

    //     let set_of = SETS
    //         .read()
    //         .details
    //         .get(&list_id())
    //         .unwrap_or(&SetDetails {
    //             object_id: "".to_string(),
    //             name: "".to_string(),
    //             layout: 0,
    //             set_of: vec![],
    //         })
    //         .set_of
    //         .clone();

    //     // Synchronous read — positions is tracked as a reactive dependency.
    //     // When any Pane's relation_key changes or nodes are added/removed,
    //     // use_resource cancels the old task and re-runs with the new keys.
    //     let keys = pane_keys(&positions.read());
    //     async move {
    //         let Some(client) = client else {
    //             tracing::warn!("subscribe_list_objects: no client yet");
    //             return;
    //         };
    //         match client
    //             .subscribe_list_objects(&sid, &lid, set_of, keys)
    //             .await
    //         {
    //             Ok(resp) => {
    //                 let mut state = LIST_OBJECTS.write();
    //                 state.order.clear();
    //                 state.details.clear();
    //                 for record in resp.records {
    //                     let id = extract_string(record.fields.get("id"));
    //                     let det = parse_object_details(&id, &record.fields);
    //                     state.order.push(id.clone());
    //                     state.details.insert(id, det);
    //                 }
    //             }
    //             Err(e) => tracing::error!("subscribe_list_objects: {e:#}"),
    //         }
    //     }
    // });

    // use_drop(move || {
    //     *LIST_OBJECTS.write() = ListObjectsState::default();
    //     spawn(async move {
    //         if let Some(client) = API_CLIENT.read().as_ref().cloned() {
    //             client.unsubscribe_list_objects(list_id()).await.ok();
    //         }
    //     });
    // });

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
                Object { key: "{det.id}", positions, details: det }
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
