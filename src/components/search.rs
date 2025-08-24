use dioxus::prelude::*;
use openapi::models::*;
use std::vec;
use crate::API_CLIENT;
use crate::ListEntry;
struct Object {
    name: String,
    object_id: String,
    properties: Vec<Option<ApimodelPeriodPropertyWithValue>>,
    options: Vec<Option<Vec<ApimodelPeriodTag>>>,
}
#[component]
pub fn Search(space_id: String) -> Element {
    let space_id = use_signal(|| space_id.clone());
    let resp = use_resource(move || async move {
        API_CLIENT.read().get_tasks(&space_id.read()).await
    });
    let keys: Signal<Vec<String>> = use_signal(|| {
        vec![
            "title".to_string(),
            "status".to_string(),
            "date".to_string(),
            "done".to_string(),
        ]
    });
    let mut ids: Signal<Vec<String>> = use_signal(|| vec!["".to_string(); 4]);
    let mut default_values: Signal<Vec<Option<ApimodelPeriodPropertyWithValue>>> = use_signal(||
    vec![None; 4]);
    let mut options: Signal<Vec<Option<Vec<ApimodelPeriodTag>>>> = use_signal(|| {
        vec![None; 4]
    });
    use_effect(move || {
        spawn(async move {
            for (i, key) in keys().clone().iter().enumerate() {
                let space_id = space_id().clone();
                let properties = API_CLIENT.read().list_properties(&space_id).await;
                match properties {
                    Ok(props) => {
                        for prop in props.data.clone().unwrap() {
                            if prop.key.unwrap() == *key {
                                ids.write()[i] = prop.id.unwrap();
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("error: {:#?}", e);
                    }
                }
                let property_id = ids.read()[i].clone();
                let property = API_CLIENT
                    .read()
                    .get_property(&space_id, property_id.clone())
                    .await;
                match property {
                    Ok(r) => {
                        let prop = r.clone().property.unwrap_or_default();
                        default_values.write()[i] = match prop.format.unwrap_or_default()
                        {
                            ApimodelPeriodPropertyFormat::PropertyFormatSelect => {
                                Some(
                                    ApimodelPeriodPropertyWithValue::ApimodelPeriodSelectPropertyValue(
                                        Box::new(ApimodelPeriodSelectPropertyValue {
                                            select: None,
                                            format: prop.format,
                                            id: prop.id,
                                            key: prop.key,
                                            name: prop.name,
                                            object: prop.object,
                                        }),
                                    ),
                                )
                            }
                            ApimodelPeriodPropertyFormat::PropertyFormatCheckbox => {
                                Some(
                                    ApimodelPeriodPropertyWithValue::ApimodelPeriodCheckboxPropertyValue(
                                        Box::new(ApimodelPeriodCheckboxPropertyValue {
                                            checkbox: None,
                                            format: prop.format,
                                            id: prop.id,
                                            key: prop.key,
                                            name: prop.name,
                                            object: prop.object,
                                        }),
                                    ),
                                )
                            }
                            _ => None,
                        };
                    }
                    _ => {}
                }
                let options_res = API_CLIENT
                    .read()
                    .list_select_property_options(&space_id, &property_id)
                    .await;
                if let Ok(o) = options_res {
                    options.write()[i] = o.data;
                }
            }
        });
    });
    let mut objects = Vec::<Object>::new();
    match &*resp.read() {
        Some(Ok(s)) => {
            for object in s.data.clone().unwrap() {
                let mut obj = Object {
                    name: object.clone().name.unwrap(),
                    object_id: object.clone().id.unwrap(),
                    properties: vec![],
                    options: vec![],
                };
                for (i, key) in keys().clone().iter().enumerate() {
                    obj.options.push(options.read()[i].clone());
                    let p = get_object_property_by_key(
                            &object.properties.clone().unwrap(),
                            key,
                        )
                        .cloned();
                    match p {
                        Some(prop) => {
                            obj.properties.push(Some(prop));
                        }
                        _ => {
                            let prop = &default_values.read()[i];
                            obj.properties.push(prop.clone());
                        }
                    }
                }
                objects.push(obj);
            }
        }
        Some(Err(e)) => {
            tracing::error!("error: {:#?}", e);
        }
        _ => {}
    }
    rsx! {
        div { id: "object-list",
            for obj in objects.iter() {
                ListEntry {
                    key: "{obj.object_id}",
                    name: obj.name.clone(),
                    space_id,
                    object_id: obj.object_id.clone(),
                    properties: obj.properties.clone(),
                    options: obj.options.clone(),
                }
            }
        }
    }
}
fn get_object_property_by_key<'a>(
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
