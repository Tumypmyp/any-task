use crate::API_CLIENT;
use crate::ObjectRow;
use crate::components::list::List;
use crate::helpers::models::DateTimeFormat;
use crate::helpers::*;
use dioxus::prelude::*;
use openapi::models::*;
use std::vec;
struct Object {
    name: String,
    object_id: String,
    data: ApimodelObject,
}
#[component]
pub fn Search(space_id: Signal<String>, types: Vec<String>) -> Element {
    let resp = use_resource(move || {
        let client = API_CLIENT.read().clone();
        let types = types.clone();
        async move { client.get_types(&space_id(), types).await }
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
    let properties: Store<Vec<(PropertyInfo, PropertySettings)>> = use_store(|| {
        vec![(PropertyInfo {
            id: PropertyID(NAME_PROPERTY_ID_STR.to_string()),
            name: "Name".to_string(),
            options: vec![],
        }, PropertySettings{
            date_format: DateTimeFormat::DateTime,
            width: 30.0,
            height: 10.0,
            show: true,
        })]
    });
    rsx! {
        List {
            for obj in objects.iter() {
                ObjectRow {
                    key: "{obj.object_id}",
                    name: obj.name.clone(),
                    space_id,
                    object_id: obj.object_id.clone(),
                    properties,
                    data: obj.data.clone(),
                }
            }
        }
    }
}
