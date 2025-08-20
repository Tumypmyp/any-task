use dioxus::prelude::*;
use crate::API_CLIENT;
use crate::Task;
use openapi::models::*;

const STYLE_CSS: Asset = asset!("/assets/styling/style.css");
#[component]
pub fn Search(space_id: String) -> Element {
    let space_id = use_signal(|| space_id.clone());
    let objects = use_resource(move || async move {
            API_CLIENT.read().get_tasks(&space_id.read()).await
        }
    );

    let properties_names = vec!["Done", "Status"];
    
    match &*objects.read() {
        Some(Ok(s)) => {
            rsx! {
                document::Link { rel: "stylesheet", href: STYLE_CSS }
                div { id: "object-list",
                    for object in s.data.clone().unwrap() {
                        if let Some(p) = get_property_by_name(
                            &object.properties.clone().unwrap(),
                            properties_names[0],
                        )
                        {
                            match p {
                                ApimodelPeriodPropertyWithValue::ApimodelPeriodCheckboxPropertyValue(
                                    checkbox,
                                ) => {
                                    rsx! {
                                        Task {
                                            done: checkbox.checkbox.unwrap_or_default(),
                                            name: object.clone().name.unwrap(),
                                            space_id,
                                            object_id: object.clone().id.unwrap(),
                                        }
                                    }
                                }
                                _ => rsx! {},
                            }
                        }
                    }
                }
            }
        },
        Some(Err(e)) => {
            println!("error: {:#?}", e);
            crate::Error()
        },
        _ => rsx! ()
        
    }
}

fn get_property_by_name<'a>(
    properties: &'a Vec<ApimodelPeriodPropertyWithValue>,
    target_name: &str,
) -> Option<&'a ApimodelPeriodPropertyWithValue> {
    for prop in properties {
        let name_match = match prop {
            ApimodelPeriodPropertyWithValue::ApimodelPeriodTextPropertyValue(p) => p.name.as_deref(),
            ApimodelPeriodPropertyWithValue::ApimodelPeriodNumberPropertyValue(p) => p.name.as_deref(),
            ApimodelPeriodPropertyWithValue::ApimodelPeriodSelectPropertyValue(p) => p.name.as_deref(),
            ApimodelPeriodPropertyWithValue::ApimodelPeriodMultiSelectPropertyValue(p) => p.name.as_deref(),
            ApimodelPeriodPropertyWithValue::ApimodelPeriodDatePropertyValue(p) => p.name.as_deref(),
            ApimodelPeriodPropertyWithValue::ApimodelPeriodFilesPropertyValue(p) => p.name.as_deref(),
            ApimodelPeriodPropertyWithValue::ApimodelPeriodCheckboxPropertyValue(p) => p.name.as_deref(),
            ApimodelPeriodPropertyWithValue::ApimodelPeriodUrlPropertyValue(p) => p.name.as_deref(),
            ApimodelPeriodPropertyWithValue::ApimodelPeriodEmailPropertyValue(p) => p.name.as_deref(),
            ApimodelPeriodPropertyWithValue::ApimodelPeriodPhonePropertyValue(p) => p.name.as_deref(),
            ApimodelPeriodPropertyWithValue::ApimodelPeriodObjectsPropertyValue(p) => p.name.as_deref(),
        };

        if let Some(name) = name_match {
            if name == target_name {
                return Some(prop);
            }
        }
    }
    None
}