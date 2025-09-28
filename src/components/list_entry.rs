use dioxus::prelude::*;
use openapi::models::*;
use crate::Route;
use crate::properties::PropertyValue;
use crate::views::PropertyID;
use std::collections::HashMap;
#[derive(Clone, Props, PartialEq)]
pub struct ListEntryProps {
    pub name: String,
    pub space_id: String,
    pub object_id: String,
    pub prop_ids_to_options_map: Store<HashMap<PropertyID, Vec<ApimodelPeriodTag>>>,
    pub data: ApimodelPeriodObject,
    pub properties_order: Store<Vec<PropertyID>>,
    pub prop_ids_to_show: Store<HashMap<PropertyID, bool>>,
}
#[component]
pub fn ListEntry(props: ListEntryProps) -> Element {
    let nav = navigator();
    let mut properties = use_store(|| HashMap::<
        PropertyID,
        ApimodelPeriodPropertyWithValue,
    >::new());
    for prop in props.data.properties.clone().unwrap().iter() {
        let prop_id = get_object_property_id(prop.clone());
        properties.write().insert(PropertyID(prop_id), prop.clone());
    }
    rsx! {
        div { class: "button-holder", key: "{props.object_id}",
            button {
                class: "button",
                width: "90vw",
                display: "flex",
                "flex-direction": "row",
                "data-style": "outline",
                onclick: move |_| {
                    tracing::debug!("object is {:#?}", props.data.clone());
                    if let Some(t) = &props.data.r#type && t.key == Some("set".to_string()) {
                        nav.push(Route::List {
                            space_id: props.space_id.clone(),
                            list_id: props.object_id.clone(),
                        });
                    }
                },
                div { class: "properties-holder",
                    div { class: "button-holder",
                        button { class: "button", "data-style": "outline", "{props.name}" }
                    }
                    for prop_id in props.properties_order.read().clone() {
                        if let Some(show) = props.prop_ids_to_show.get(prop_id.clone()) && show()
                            && let Some(prop) = properties.get(prop_id.clone())
                            && let options = props
                                .prop_ids_to_options_map
                                .read()
                                .clone()
                                .get(&prop_id)
                                .unwrap_or(&vec![])
                                .clone()
                        {
                            PropertyValue {
                                space_id: &props.space_id,
                                object_id: &props.object_id,
                                data: prop.read().clone(),
                                options,
                            }
                        }
                    }
                }
            }
        }
    }
}
fn get_object_property_id(prop: ApimodelPeriodPropertyWithValue) -> String {
    return match prop.clone() {
        ApimodelPeriodPropertyWithValue::ApimodelPeriodTextPropertyValue(p) => {
            p.id.clone().unwrap()
        }
        ApimodelPeriodPropertyWithValue::ApimodelPeriodNumberPropertyValue(p) => {
            p.id.clone().unwrap()
        }
        ApimodelPeriodPropertyWithValue::ApimodelPeriodSelectPropertyValue(p) => {
            p.id.clone().unwrap()
        }
        ApimodelPeriodPropertyWithValue::ApimodelPeriodMultiSelectPropertyValue(p) => {
            p.id.clone().unwrap()
        }
        ApimodelPeriodPropertyWithValue::ApimodelPeriodDatePropertyValue(p) => {
            p.id.clone().unwrap()
        }
        ApimodelPeriodPropertyWithValue::ApimodelPeriodFilesPropertyValue(p) => {
            p.id.clone().unwrap()
        }
        ApimodelPeriodPropertyWithValue::ApimodelPeriodCheckboxPropertyValue(p) => {
            p.id.clone().unwrap()
        }
        ApimodelPeriodPropertyWithValue::ApimodelPeriodUrlPropertyValue(p) => {
            p.id.clone().unwrap()
        }
        ApimodelPeriodPropertyWithValue::ApimodelPeriodEmailPropertyValue(p) => {
            p.id.clone().unwrap()
        }
        ApimodelPeriodPropertyWithValue::ApimodelPeriodPhonePropertyValue(p) => {
            p.id.clone().unwrap()
        }
        ApimodelPeriodPropertyWithValue::ApimodelPeriodObjectsPropertyValue(p) => {
            p.id.clone().unwrap()
        }
    };
}
