use dioxus::prelude::*;
use openapi::models::*;
use std::vec;
use crate::API_CLIENT;
use crate::ListEntry;
use crate::helpers::models::PropertyID;
use std::collections::HashMap;
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
    let prop_ids_to_options_map = use_store(|| HashMap::<
        PropertyID,
        Vec<ApimodelPeriodTag>,
    >::new());
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
    let prop_ids_to_show = use_store(|| HashMap::<PropertyID, bool>::new());
    let properties_order: Store<Vec<PropertyID>> = use_store(|| vec![]);
    rsx! {
        div { id: "object-list",
            for obj in objects.iter() {
                ListEntry {
                    key: "{obj.object_id}",
                    name: obj.name.clone(),
                    space_id,
                    object_id: obj.object_id.clone(),
                    properties_order,
                    prop_ids_to_show,
                    prop_ids_to_options_map,
                    data: obj.data.clone(),
                }
            }
        }
    }
}
