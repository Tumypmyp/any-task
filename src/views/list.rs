use std::vec;
use dioxus::prelude::*;
use crate::components::Title;
use crate::components::Header;
use crate::API_CLIENT;
use crate::Actions;
use crate::ListEntry;
use crate::helpers::*;
use crate::components::add_properties::ShowPropertiesSetting;
use crate::components::edit_properties::PropertiesOrder;
#[component]
pub fn List(space_id: String, list_id: String) -> Element {
    tracing::info!("loading space {space_id}, list {list_id}");
    let space_id = use_signal(|| space_id);
    let list_id = use_signal(|| list_id);
    let view_id = use_signal(|| "".to_string());
    let properties_order: Store<Vec<PropertyViewInfo>> = use_store(|| {
        vec![
            PropertyViewInfo {
                id: PropertyID(NAME_PROPERTY_ID_STR.to_string()),
                name: "Name".to_string(),
                show: true,
                options: vec![],
                width: 30.0,
            },
        ]
    });
    rsx! {
        ListHeader {
            space_id,
            list_id,
            view_id,
            properties_order,
        }
        Objects {
            space_id,
            list_id,
            view_id,
            properties_order,
        }
        Actions {}
    }
}
#[component]
pub fn ListHeader(
    space_id: Signal<String>,
    list_id: Signal<String>,
    view_id: Signal<String>,
    properties_order: Store<Vec<PropertyViewInfo>>,
) -> Element {
    let mut name = use_signal(|| "".to_string());
    let resp = use_resource(move || async move {
        API_CLIENT.read().get_object(space_id, list_id).await
    });
    match &*resp.read() {
        Some(Ok(p)) => {
            name.set(p.clone().object.unwrap().name.unwrap());
        }
        _ => {}
    }
    rsx! {
        Header {
            Title { title: "{name}" }
            ShowPropertiesSetting { space_id, properties_order }
        }
        PropertiesOrder { properties_order }
    }
}
#[component]
pub fn Objects(
    space_id: Signal<String>,
    list_id: Signal<String>,
    view_id: Signal<String>,
    properties_order: Store<Vec<PropertyViewInfo>>,
) -> Element {
    let resp = use_resource(move || async move {
        API_CLIENT.read().get_list_objects(space_id, list_id, view_id).await
    });
    match &*resp.read() {
        Some(Ok(p)) => {
            rsx! {
                for obj in p.clone().data.unwrap() {
                    ListEntry {
                        key: "{obj.clone().id.unwrap()}",
                        name: obj.clone().name.unwrap(),
                        space_id,
                        object_id: obj.clone().id.unwrap(),
                        properties_order,
                        data: obj.clone(),
                    }
                }
            }
        }
        _ => rsx!(),
    }
}
