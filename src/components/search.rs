use crate::API_CLIENT;
use crate::ListEntry;
use crate::helpers::models::DateTimeFormat;
use crate::helpers::*;
use dioxus::prelude::*;
use openapi::models::*;
use std::vec;
struct Object {
    name: String,
    object_id: String,
    data: ApimodelPeriodObject,
}
#[component]
pub fn Search(space_id: Signal<String>, types: Vec<String>) -> Element {
    let resp = use_resource(move || {
        let types = types.clone();
        async move { API_CLIENT.read().get_types(&space_id(), types).await }
    });
    let mut objects = Vec::<Object>::new();
    match &*resp.read() {
        Some(Ok(s)) => {
            for object in s.data.clone().unwrap() {
                let obj = Object {
                    name: object.clone().name.unwrap(),
                    object_id: object.clone().id.unwrap(),
                    data: object.clone(),
                };
                objects.push(obj);
            }
        }
        Some(Err(e)) => {
            tracing::error!("error: {:#?}", e);
        }
        _ => {}
    }
    let show_properties: Store<Vec<PropertyViewInfo>> = use_store(|| {
        vec![PropertyViewInfo {
            id: PropertyID(NAME_PROPERTY_ID_STR.to_string()),
            name: "Name".to_string(),
            options: vec![],
            date_format: DateTimeFormat::DateTime,
            width: 30.0,
        }]
    });
    rsx! {
        div { id: "object-list",
            for obj in objects.iter() {
                ListEntry {
                    key: "{obj.object_id}",
                    name: obj.name.clone(),
                    space_id,
                    object_id: obj.object_id.clone(),
                    show_properties,
                    data: obj.data.clone(),
                }
            }
        }
    }
}
