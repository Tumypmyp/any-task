use dioxus::html::switch;
use dioxus::prelude::*;
use openapi::models::*;
use std::collections::HashMap;
use std::vec;
use crate::components::property;
use crate::API_CLIENT;
use crate::get_property_id_by_key;
use crate::ListObject;
struct Object {
    name: String,
    object_id: String,
    properties: HashMap<String, Option<ApimodelPeriodPropertyWithValue>>,
}
#[component]
pub fn Search(space_id: String) -> Element {
    let space_id = use_signal(|| space_id.clone());
    let resp = use_resource(move || async move {
        API_CLIENT.read().get_tasks(&space_id.read()).await
    });
    let keys: Vec<String> = vec!["done".to_string(), "status".to_string()];
    let mut ids: Vec<String> = vec!["".to_string(); 2];
    for (i, key) in keys.iter().enumerate() {
        if let Some(id) = get_property_id_by_key(space_id.to_string(), key) {
            ids[i] = id;
        }
    }
    let mut objects = Vec::<Object>::new();
    match &*resp.read() {
        Some(Ok(s)) => {
            for object in s.data.clone().unwrap() {
                let mut obj = Object {
                    name: object.clone().name.unwrap(),
                    object_id: object.clone().id.unwrap(),
                    properties: HashMap::new(),
                };
                for (i, key) in keys.iter().enumerate() {
                    let property_id = use_signal(|| ids[i].clone());
                    let p = get_property_by_key(&object.properties.clone().unwrap(), key)
                        .cloned();
                    match p {
                        Some(prop) => {
                            obj.properties.insert(property_id(), Some(prop));
                        }
                        _ => {
                            let prop = default_property_value(space_id, property_id);
                            obj.properties.insert(property_id(), prop);
                        }
                    }
                }
                objects.push(obj);
            }
        }
        Some(Err(e)) => {
            println!("error: {:#?}", e);
        }
        _ => {}
    }
    rsx! {
        div { id: "object-list",
            for obj in objects.iter() {
                ListObject {
                    key: "{obj.object_id}",
                    name: obj.name.clone(),
                    space_id,
                    object_id: obj.object_id.clone(),
                    properties: obj.properties.clone(),
                    ids_order: ids.clone(),
                }
            }
        }
    }
}
fn default_property_value(
    space_id: Signal<String>,
    property_id: Signal<String>,
) -> Option<ApimodelPeriodPropertyWithValue> {
    let resp = API_CLIENT.read().get_property(space_id, property_id);
    match resp {
        Some(prop) => {
            match prop.property {
                Some(p) => {
                    match p.format {
                        Some(ApimodelPeriodPropertyFormat::PropertyFormatSelect) => {
                            return Some(
                                ApimodelPeriodPropertyWithValue::ApimodelPeriodSelectPropertyValue(
                                    Box::new(ApimodelPeriodSelectPropertyValue::new()),
                                ),
                            );
                        }
                        Some(ApimodelPeriodPropertyFormat::PropertyFormatCheckbox) => {
                            return Some(
                                ApimodelPeriodPropertyWithValue::ApimodelPeriodCheckboxPropertyValue(
                                    Box::new(ApimodelPeriodCheckboxPropertyValue::new()),
                                ),
                            );
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }
    None
}
fn get_property_by_key<'a>(
    properties: &'a Vec<ApimodelPeriodPropertyWithValue>,
    target_name: &str,
) -> Option<&'a ApimodelPeriodPropertyWithValue> {
    for prop in properties {
        let name_match = match prop {
            ApimodelPeriodPropertyWithValue::ApimodelPeriodTextPropertyValue(p) => {
                p.key.as_deref()
            }
            ApimodelPeriodPropertyWithValue::ApimodelPeriodNumberPropertyValue(p) => {
                p.key.as_deref()
            }
            ApimodelPeriodPropertyWithValue::ApimodelPeriodSelectPropertyValue(p) => {
                p.key.as_deref()
            }
            ApimodelPeriodPropertyWithValue::ApimodelPeriodMultiSelectPropertyValue(
                p,
            ) => p.key.as_deref(),
            ApimodelPeriodPropertyWithValue::ApimodelPeriodDatePropertyValue(p) => {
                p.key.as_deref()
            }
            ApimodelPeriodPropertyWithValue::ApimodelPeriodFilesPropertyValue(p) => {
                p.key.as_deref()
            }
            ApimodelPeriodPropertyWithValue::ApimodelPeriodCheckboxPropertyValue(p) => {
                p.key.as_deref()
            }
            ApimodelPeriodPropertyWithValue::ApimodelPeriodUrlPropertyValue(p) => {
                p.key.as_deref()
            }
            ApimodelPeriodPropertyWithValue::ApimodelPeriodEmailPropertyValue(p) => {
                p.key.as_deref()
            }
            ApimodelPeriodPropertyWithValue::ApimodelPeriodPhonePropertyValue(p) => {
                p.key.as_deref()
            }
            ApimodelPeriodPropertyWithValue::ApimodelPeriodObjectsPropertyValue(p) => {
                p.key.as_deref()
            }
        };
        if let Some(name) = name_match {
            if name == target_name {
                return Some(prop);
            }
        }
    }
    None
}
