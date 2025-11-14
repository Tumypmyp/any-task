use crate::Route;
use crate::components::base::ButtonHolder;
use crate::helpers::NAME_PROPERTY_ID_STR;
use crate::helpers::*;
use crate::properties::PropertyValue;
use dioxus::prelude::*;
use openapi::models::*;
use std::collections::HashMap;
#[derive(Clone, Props, PartialEq)]
pub struct ListEntryProps {
    pub name: String,
    pub space_id: String,
    pub object_id: String,
    pub data: ApimodelPeriodObject,
    pub show_properties: Store<Vec<PropertyInfo>>,
}
#[component]
pub fn ListEntry(props: ListEntryProps) -> Element {
    let nav = navigator();
    let mut object_properties =
        use_store(|| HashMap::<PropertyID, ApimodelPeriodPropertyWithValue>::new());
    for property in props.data.properties.clone().unwrap().iter() {
        let property_id = get_property_id(property.clone());
        object_properties
            .write()
            .insert(property_id, property.clone());
    }
    let text_property_value = ApimodelPeriodTextPropertyValue {
        format: None,
        text: props.data.name.clone(),
        key: None,
        name: None,
        id: None,
        object: None,
    };
    object_properties.write().insert(
        PropertyID(NAME_PROPERTY_ID_STR.to_string()),
        ApimodelPeriodPropertyWithValue::ApimodelPeriodTextPropertyValue(Box::new(
            text_property_value,
        )),
    );
    rsx! {
        ButtonHolder {
            button {
                class: "button",
                width: "95vw",
                display: "flex",
                "data-style": "ghost",
                "flex-direction": "row",
                onclick: move |_| {
                if let Some(t) = props.clone().data.r#type &&
                    (t.key == Some("set".to_string()) || t.key == Some("collecion".to_string())) {
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
                    for property in props.show_properties.read().clone() {
                        // if property.show {
                        if let Some(prop) = object_properties.get(property.clone().id) {
                            PropertyValue {
                                key: "{property.id.as_str()}",
                                space_id: props.space_id.clone(),
                                object_id: props.object_id.clone(),
                                data: prop.read().clone(),
                                info: property,
                            }
                        } else {
                            PropertyValue {
                                key: "{property.id.as_str()}",
                                space_id: props.space_id.clone(),
                                object_id: props.object_id.clone(),
                                data: None,
                                info: property,
                            }
                        }
                    }
                }
            }
        }
    }
}
fn get_property_id(prop: ApimodelPeriodPropertyWithValue) -> PropertyID {
    return PropertyID(match prop.clone() {
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
        ApimodelPeriodPropertyWithValue::ApimodelPeriodUrlPropertyValue(p) => p.id.clone().unwrap(),
        ApimodelPeriodPropertyWithValue::ApimodelPeriodEmailPropertyValue(p) => {
            p.id.clone().unwrap()
        }
        ApimodelPeriodPropertyWithValue::ApimodelPeriodPhonePropertyValue(p) => {
            p.id.clone().unwrap()
        }
        ApimodelPeriodPropertyWithValue::ApimodelPeriodObjectsPropertyValue(p) => {
            p.id.clone().unwrap()
        }
    });
}
