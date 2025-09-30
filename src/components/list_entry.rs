use dioxus::prelude::*;
use openapi::models::*;
use crate::Route;
use crate::properties::PropertyValue;
use std::collections::HashMap;
use crate::components::base::ButtonWithHolder;
use crate::components::base::ButtonHolder;
use crate::helpers::*;
#[derive(Clone, Props, PartialEq)]
pub struct ListEntryProps {
    pub name: String,
    pub space_id: String,
    pub object_id: String,
    pub data: ApimodelPeriodObject,
    pub properties_order: Store<Vec<PropertyViewInfo>>,
}
#[component]
pub fn ListEntry(props: ListEntryProps) -> Element {
    let nav = navigator();
    let mut properties = use_store(|| HashMap::<
        PropertyID,
        ApimodelPeriodPropertyWithValue,
    >::new());
    for property in props.data.properties.clone().unwrap().iter() {
        let property_id = get_property_id(property.clone());
        properties.write().insert(property_id, property.clone());
    }
    rsx! {
        ButtonHolder {
            button {
                class: "button",
                width: "95vw",
                display: "flex",
                "data-style": "outline",
                "flex-direction": "row",
                onclick: move |_| {
                    if let Some(t) = props.clone().data.r#type && t.key == Some("set".to_string()) {
                        nav.push(Route::List {
                            space_id: props.clone().space_id.clone(),
                            list_id: props.clone().object_id.clone(),
                        });
                    }
                },
                div {
                    style: "
                        display: flex;
                        flex-direction: row;   
                        align-items: center; 
                    ",
                    width: "95vw",
                    ButtonWithHolder { width: "25vw", "{props.name.clone()}" }
                    for property in props.properties_order.read().clone() {
                        if property.show && let Some(prop) = properties.get(property.clone().id) {
                            PropertyValue {
                                key: "{property.id.as_str()}",
                                space_id: props.space_id.clone(),
                                object_id: props.object_id.clone(),
                                data: prop.read().clone(),
                                options: property.options,
                            }
                        }
                    }
                }
            }
        }
    }
}
fn get_property_id(prop: ApimodelPeriodPropertyWithValue) -> PropertyID {
    return PropertyID(
        match prop.clone() {
            ApimodelPeriodPropertyWithValue::ApimodelPeriodTextPropertyValue(p) => {
                p.id.clone().unwrap()
            }
            ApimodelPeriodPropertyWithValue::ApimodelPeriodNumberPropertyValue(p) => {
                p.id.clone().unwrap()
            }
            ApimodelPeriodPropertyWithValue::ApimodelPeriodSelectPropertyValue(p) => {
                p.id.clone().unwrap()
            }
            ApimodelPeriodPropertyWithValue::ApimodelPeriodMultiSelectPropertyValue(
                p,
            ) => p.id.clone().unwrap(),
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
        },
    );
}
