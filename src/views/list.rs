use crate::API_CLIENT;
use crate::ObjectRow;
use crate::components::action::{ActionHolder, BaseActions};
use crate::components::base::message;
use crate::components::choose_view::ChooseView;
use crate::components::header::{Header, Title};
use crate::edit_view::*;
use crate::helpers::*;
use dioxus::prelude::*;
use dioxus_sdk_storage::LocalStorage;
use dioxus_sdk_storage::use_synced_storage;
use openapi::models::ApimodelPropertyFormat as Format;
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
        Vec<(PropertyInfo, PropertySettings)>,
    >(
        storage_key,
        || {
            vec![
                (
                    PropertyInfo {
                        id: PropertyID(NAME_PROPERTY_ID_STR.to_string()),
                        name: "Name".to_string(),
                        optional: OptionalInfo::Other,
                    },
                    PropertySettings::General(GeneralPropertySettings {
                        width: 30.0,
                        height: 10.0,
                    }),
                ),
            ]
        },
    );
    let properties_store = use_store(|| properties.read().clone());
    use_effect(move || {
        let store_value = properties_store.read().clone();
        tracing::info!("saved the properties: {:#?}", store_value);
        *properties.write() = store_value;
    });
    let mut all_properties: Store<Vec<PropertyInfo>> = use_store(|| {
        vec![
            PropertyInfo {
                id: PropertyID(NAME_PROPERTY_ID_STR.to_string()),
                name: "Name".to_string(),
                optional: OptionalInfo::Other,
            },
        ]
    });
    use_effect(move || {
        let client = API_CLIENT.read();
        spawn(async move {
            let space_id = space_id();
            let resp = client.list_properties(&space_id).await;
            match resp {
                Ok(props) => {
                    for prop in props.data.unwrap() {
                        let property_id = PropertyID(prop.id.clone().unwrap());
                        let property_name = prop.name.clone().unwrap();
                        let format = prop.format.clone().unwrap();
                        let select_property_options = client
                            .list_select_property_options(
                                &space_id,
                                property_id.clone().as_str(),
                            )
                            .await;
                        let options = match select_property_options {
                            Ok(o) => o.data.unwrap(),
                            _ => vec![],
                        };
                        let optional_info = match format {
                            Format::PropertyFormatSelect => OptionalInfo::Select(options),
                            Format::PropertyFormatDate => OptionalInfo::Date,
                            Format::PropertyFormatCheckbox => OptionalInfo::Checkbox,
                            _ => OptionalInfo::Other,
                        };
                        all_properties
                            .write()
                            .push(PropertyInfo {
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
    space_id: Signal<String>,
    list_id: Signal<String>,
    view_id: Store<String>,
    properties: Store<Vec<(PropertyInfo, PropertySettings)>>,
    all_properties: Store<Vec<PropertyInfo>>,
) -> Element {
    let mut name = use_signal(|| "".to_string());
    let resp = use_resource(move || {
        let client = API_CLIENT.read().clone();
        async move { client.get_object(space_id, list_id).await }
    });
    match &*resp.read() {
        Some(Ok(p)) => {
            name.set(p.clone().object.unwrap().name.unwrap());
        }
        Some(err) => {
            tracing::debug!("error reading header: {:#?}", err);
        }
        _ => {
            tracing::debug!("error reading header");
        }
    }
    rsx! {
        Header {
            Title { title: "{name}" }
            ChooseView { space_id, list_id, view_id }
            EditView { properties, all_properties, space_id }
        }
    }
}
#[component]
pub fn Objects(
    space_id: Signal<String>,
    list_id: Signal<String>,
    view_id: Store<String>,
    properties: Store<Vec<(PropertyInfo, PropertySettings)>>,
) -> Element {
    let api_client_handle = API_CLIENT.cloned();
    let resp = use_resource(move || {
        let view_id = view_id.read().clone();
        let client = api_client_handle.clone();
        async move { client.get_list_objects(space_id, list_id, view_id).await }
    });
    let resp_value = resp.read();
    let objects = match resp_value.as_ref() {
        Some(Ok(objs)) => objs,
        Some(Err(err)) => {
            message::error("Failed to fetch objects", err);
            return rsx! {};
        }
        None => {
            return rsx! { "Loading..." };
        }
    };
    rsx! {
        for obj in objects.data.clone().unwrap_or_default() {
            if let Some(id) = obj.clone().id {
                ObjectRow {
                    key: "{id}",
                    name: obj.clone().name.unwrap(),
                    space_id,
                    object_id: obj.clone().id.unwrap(),
                    properties,
                    data: obj.clone(),
                }
            }
        }
    }
}
