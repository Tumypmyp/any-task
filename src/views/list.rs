use crate::API_CLIENT;
use crate::components::action::{ActionHolder, BaseActions};
use crate::components::base::message;
use crate::components::edit_view::*;
use crate::components::header::{Header, Title};
use crate::components::object_row::*;
use crate::components::separator::Separator;
use crate::helpers::*;
use crate::protos::anytype_model::*;
use dioxus::prelude::*;
use dioxus_sdk_storage::LocalStorage;
use dioxus_sdk_storage::use_synced_storage;
use std::vec;
#[component]
pub fn List(space_id: String, list_id: String) -> Element {
    tracing::info!("loading space {space_id}, list {list_id}");
    let space_id = use_signal(|| space_id);
    let list_id = use_signal(|| list_id);
    let view_id = use_store(|| "".to_string());
    let storage_key = format!("properties-list-view-{}", list_id());
    let mut properties = use_synced_storage::<
        LocalStorage,
        Vec<Vec<(PropertyInfo, PropertySettings)>>,
    >(storage_key, || {
        vec![vec![(
            PropertyInfo {
                id: PropertyID(NAME_PROPERTY_ID_STR.to_string()),
                name: "Name".to_string(),
                optional: OptionalInfo::Other,
            },
            NAME_PROPERTY_SETTINGS,
        )]]
    });
    let properties_store = use_store(|| properties.read().clone());
    use_effect(move || {
        let store_value = properties_store.read().clone();
        tracing::info!("saved the properties: {:#?}", store_value);
        *properties.write() = store_value;
    });
    let mut all_properties: Store<Vec<PropertyInfo>> = use_store(|| {
        vec![PropertyInfo {
            id: PropertyID(NAME_PROPERTY_ID_STR.to_string()),
            name: "Name".to_string(),
            optional: OptionalInfo::Other,
        }]
    });
    use_effect(move || {
        spawn(async move {
            let client_guard = API_CLIENT.read().clone();
            let Some(client) = client_guard.as_ref() else {
                tracing::warn!("No API client available");
                return;
            };
            let space_id = space_id();
            let resp = client.fetch_properties(&space_id).await;
            match resp {
                Ok(props) => {
                    for prop in props {
                        let property_id = PropertyID(prop.0.clone());
                        let property_name = prop.1.clone();
                        let format = prop.3.clone();
                        let optional_info = match format {
                            RelationFormat::Date => OptionalInfo::Date,
                            RelationFormat::Checkbox => OptionalInfo::Checkbox,
                            _ => OptionalInfo::Other,
                        };
                        all_properties.write().push(PropertyInfo {
                            id: property_id.clone(),
                            name: property_name,
                            optional: optional_info,
                        });
                    }
                }
                Err(e) => {
                    tracing::error!("error loading property list: {:#?}", e);
                }
            }
        });
    });
    rsx! {
        ListHeader {
            space_id,
            list_id,
            view_id,
            properties: properties_store,
            all_properties,
        }
        Objects {
            space_id,
            list_id,
            view_id,
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
    properties: Store<Vec<Vec<(PropertyInfo, PropertySettings)>>>,
    all_properties: Store<Vec<PropertyInfo>>,
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
        None => {
            return rsx! { "Loading..." };
        }
        Some(Err(err)) => {
            return rsx! { "Error: {err}" };
        }
        Some(Ok(name)) => name.clone(),
    };
    rsx! {
        Header {
            Title { title: "{name}" }
            EditView {
                space_id,
                list_id,
                view_id,
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
    view_id: Store<String>,
    properties: Store<Vec<Vec<(PropertyInfo, PropertySettings)>>>,
) -> Element {
    let resp = use_resource({
        move || async move {
            let client_guard = API_CLIENT.read();
            let client = client_guard
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("No API client available, try reloading the app"))?;
            client
                .get_list_objects(&space_id(), &list_id()) //, view_id())
                .await
        }
    });
    let resp_value = resp.read();
    let objects = match resp_value.as_ref() {
        Some(Ok(objs)) => objs,
        Some(Err(err)) => {
            // message::error("Failed to fetch objects", err);
            return rsx! {};
        }
        None => {
            return rsx! { "Loading..." };
        }
    };
    rsx! {
        for obj in objects {
                Separator {
                    style: "margin: 2px 0; width: 95vw;",
                    horizontal: true,
                    decorative: true,
                }
                ObjectRow {
                    key: "{obj.clone().0}",
                    name: obj.clone().1,
                    space_id,
                    id: obj.clone().0,
                    properties,
                    // data: obj
                            // .clone(),
                }
        }
    }
}
