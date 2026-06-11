use crate::API_CLIENT;
use crate::components::action::{ActionHolder, BaseActions};
use crate::components::base::message;
use crate::components::column::*;
use crate::components::edit_view::*;
use crate::components::header::{Header, Title};
use crate::components::object::*;
use crate::components::separator::Separator;
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
    let resp = use_resource({
        move || async move {
            let client_guard = API_CLIENT.read();
            let client = client_guard
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("No API client available, try reloading the app"))?;
            client.get_list_name(&space_id(), &list_id()).await
        }
    });
    let name = match &*resp.read() {
        None => return rsx! { "Loading..." },
        Some(Err(e)) => return rsx! { "Error: {e}" },
        Some(Ok(name)) => name.clone(),
    };
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
    let resp = use_resource({
        move || async move {
            let client_guard = API_CLIENT.read();
            let client = client_guard
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("No API client available, try reloading the app"))?;
            client.get_list_objects(&space_id(), &list_id()).await
        }
    });
    let resp_value = resp.read();
    let objects = match resp_value.as_ref() {
        None => return rsx! { "Loading..." },
        Some(Err(e)) => return rsx! { "Error: {e}" },
        Some(Ok(objs)) => objs.clone(),
    };
    rsx! {
        Column { style: "width: 98vw;",
            for id in objects {
                Separator {
                    style: "margin: 2px 5px; width: 95vw;",
                    horizontal: true,
                    decorative: true,
                }
                Object { positions, space_id, id }
            }
        }
    }
}
