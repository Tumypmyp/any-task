use crate::API_CLIENT;
use crate::ObjectRow;
use crate::components::action::{ActionHolder, BaseActions};
use crate::components::base::message;
use crate::components::choose_view::ChooseView;
use crate::components::header::{Header, Title};
use crate::components::properties_row::PropertiesRow;
use crate::edit_view::*;
use crate::helpers::models::DateTimeFormat;
use crate::helpers::*;
use dioxus::prelude::*;
use std::vec;
#[component]
pub fn List(space_id: String, list_id: String) -> Element {
    tracing::info!("loading space {space_id}, list {list_id}");
    let space_id = use_signal(|| space_id);
    let list_id = use_signal(|| list_id);
    let view_id = use_store(|| "".to_string());

    let properties: Store<Vec<PropertyInfo>> = use_store(|| {
        vec![PropertyInfo {
            id: PropertyID(NAME_PROPERTY_ID_STR.to_string()),
            name: "Name".to_string(),
            date_format: DateTimeFormat::DateTime,
            options: vec![],
            width: 30.0,
            show: true,
        }]
    });
    rsx! {
        ListHeader {
            space_id,
            list_id,
            view_id,
            properties,
        }
        Objects {
            space_id,
            list_id,
            view_id,
            properties,
        }
        ActionHolder { BaseActions {} }
    }
}
#[component]
pub fn ListHeader(
    space_id: Signal<String>,
    list_id: Signal<String>,
    view_id: Store<String>,
    properties: Store<Vec<PropertyInfo>>,
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
            EditView { properties, space_id }
        }
    }
}
#[component]
pub fn Objects(
    space_id: Signal<String>,
    list_id: Signal<String>,
    view_id: Store<String>,
    properties: Store<Vec<PropertyInfo>>,
) -> Element {
    let api_client_handle = API_CLIENT.cloned();
    let resp = use_resource(move || {
        let view_id = view_id.read().clone();
        let client = api_client_handle.clone();
        async move { client.get_list_objects(space_id, list_id, view_id).await }
    });

    // let objects = match resp.result() {
    let resp_value = resp.read();
    let objects = match resp_value.as_ref() {
        Some(Ok(objs)) => objs,
        Some(Err(err)) => {
            message::error("Failed to fetch objects", err);
            return rsx! {};
        }
        None => return rsx! { "Loading..." },
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
