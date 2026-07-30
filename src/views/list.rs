use crate::components::button::*;
use crate::components::column::*;
use crate::components::edit_view::*;
use crate::components::header::{Header, Title};
use crate::components::object::*;
use crate::components::row::*;
use crate::components::select::*;
use crate::helpers::*;
use dioxus::prelude::*;
use dioxus_icons::lucide::Settings2;
use dioxus_sdk_storage::LocalStorage;
use dioxus_sdk_storage::use_synced_storage;
use std::collections::HashMap;
use std::vec;

#[component]
pub fn List(space_id: ReadSignal<String>, list_id: ReadSignal<String>) -> Element {
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
    let all_properties: Memo<HashMap<RelationKey, RelationInfo>> = use_memo(move || {
        all_properties_res
            .read()
            .as_ref()
            .and_then(|r| r.as_ref().ok())
            .cloned()
            .unwrap_or_default()
    });

    match &*all_properties_res.read_unchecked() {
        None => return rsx! { "Loading..." },
        Some(Err(e)) => return rsx! { "Error: {e}" },
        Some(Ok(_)) => {}
    }

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
    let mut open = use_signal(|| false);
    rsx! {
        Column {
            Row {
                Row { position: RowPosition::Middle,
                    Title { title: "{name}" }
                }
                Row { position: RowPosition::Right,
                    Views { list_id, space_id }
                    Button {
                        variant: ButtonVariant::Secondary,
                        onclick: move |_| open.toggle(),
                        aria_label: "Edit view",
                        Settings2 {}
                    }
                }
            }
            EditView {
                open,
                space_id,
                list_id,
                positions,
                all_properties,
            }
        }
    }
}

#[component]
pub fn Views(list_id: ReadSignal<String>, space_id: ReadSignal<String>) -> Element {
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
                    SET_META.write().active_view_id = id.clone();
                    spawn(async move {
                        if let Some(client) = API_CLIENT.read().as_ref().cloned() {
                            client.set_active_view(&space_id(), &list_id(), &id).await.ok();
                        }
                    });
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
    let keys = use_memo(move || pane_keys(&positions.read()));
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
