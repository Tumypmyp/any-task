use crate::API_CLIENT;
use crate::components::action::{ActionHolder, BaseActions};
use crate::components::base::message;
use crate::components::edit_view::*;
use crate::components::header::{Header, Title};
use crate::components::object::*;
use crate::components::separator::Separator;
use crate::helpers::*;
use crate::protos::anytype_model::*;
use dioxus::prelude::*;
use dioxus_sdk_storage::LocalStorage;
use dioxus_sdk_storage::use_synced_storage;
use std::collections::HashMap;
use std::vec;

#[component]
pub fn List(space_id: ReadSignal<String>, list_id: ReadSignal<String>) -> Element {
    tracing::info!("loading space {space_id}, list {list_id}");
    let view_id = use_store(|| "".to_string());
    let storage_relations_key = format!("list-view-relations-{}", list_id());
    let storage_view_tree_key = format!("list-view-relations-tree-{}", list_id());

    let mut properties = use_synced_storage::<
        LocalStorage,
        HashMap<RelationKey, (RelationInfo, PropertySettings)>,
    >(storage_relations_key.clone(), || {
        HashMap::from([(
            RelationKey(NAME_RELATION_KEY.to_string()),
            (
                RelationInfo {
                    name: NAME_RELATION_KEY.to_string(),
                    key: RelationKey("name".to_string()),
                    // optional: OptionalInfo::Other,
                },
                NAME_PROPERTY_SETTINGS,
            ),
        )])
    });
    let properties_store = use_store(|| properties.read().clone());
    use_effect(move || {
        let store_value = properties_store.read().clone();
        tracing::info!("saved the properties: {:#?}", store_value);
        *properties.write() = store_value;
    });

    let mut positions = use_synced_storage::<LocalStorage, HashMap<NodeId, ViewTree>>(
        storage_view_tree_key.clone(),
        || {
            HashMap::from([
                (
                    NodeId(0),
                    ViewTree::Split {
                        direction: SplitDirection::Row,
                        ratio: 0.5,
                        first: NodeId(1),
                        second: NodeId(2),
                    },
                ),
                (
                    NodeId(1),
                    ViewTree::Pane {
                        relation_key: RelationKey("name".to_string()),
                    },
                ),
                (
                    NodeId(2),
                    ViewTree::Pane {
                        relation_key: RelationKey("description".to_string()),
                    },
                ),
            ])
        },
    );
    let positions_store = use_store(|| positions.read().clone());

    use_effect(move || {
        let store_value = positions_store.read().clone();
        tracing::info!("saved the properties: {:#?}", store_value);
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
            properties: properties_store,
            positions: positions_store,
            all_properties,
        }
        Objects {
            space_id,
            list_id,
            view_id,
            positions: positions_store,
            properties: properties_store,
        }
        ActionHolder { BaseActions {} }
    }
}
#[component]
pub fn ListHeader(
    space_id: ReadSignal<String>,
    list_id: ReadSignal<String>,
    view_id: Store<String>,
    properties: Store<HashMap<RelationKey, (RelationInfo, PropertySettings)>>,
    positions: Store<HashMap<NodeId, ViewTree>>,
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
                properties,
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
    positions: Store<HashMap<NodeId, ViewTree>>,
    properties: ReadSignal<HashMap<RelationKey, (RelationInfo, PropertySettings)>>,
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
        for id in objects {
            Separator {
                style: "margin: 2px 0; width: 95vw;",
                horizontal: true,
                decorative: true,
            }
            Object {
                positions,
                space_id,
                id,
                properties,
            }
        }
    }
}
