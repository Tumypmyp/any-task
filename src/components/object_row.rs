use crate::Route;
use crate::column::Column;
use crate::components::row::*;
use crate::helpers::NAME_PROPERTY_ID_STR;
use crate::helpers::*;
use crate::properties::PropertyValue;
use dioxus::prelude::*;
use openapi::models::*;
use std::collections::HashMap;
#[derive(Clone, Props, PartialEq)]
pub struct ObjectProps {
    pub name: String,
    pub space_id: String,
    pub object_id: String,
    pub data: Object,
    pub properties: Store<Vec<Vec<(PropertyInfo, PropertySettings)>>>,
}
#[component]
pub fn ObjectRow(props: ObjectProps) -> Element {
    let nav = navigator();
    let mut object_properties = use_store(|| HashMap::<PropertyID, PropertyWithValue>::new());
    for property in props.data.properties.clone().unwrap().iter() {
        let property_id = get_property_id(property.clone());
        object_properties
            .write()
            .insert(property_id, property.clone());
    }
    let text_property_value = TextPropertyValue {
        format: None,
        text: props.data.name.clone(),
        key: None,
        name: None,
        id: None,
        object: None,
    };
    object_properties.write().insert(
        PropertyID(NAME_PROPERTY_ID_STR.to_string()),
        PropertyWithValue::Text(Box::new(text_property_value)),
    );
    let p = props.clone();
    let p2 = props.clone();
    rsx! {
        Column {
            onclick: move |_| {
                if let Some(t) = p.clone().data.r#type
                    && (t.key == Some("set".to_string())
                        || t.key == Some("collecion".to_string()))
                {
                    nav.push(Route::List {
                        space_id: p.clone().space_id.clone(),
                        list_id: p.clone().object_id.clone(),
                    });
                }
            },

            for property_vec in p2.clone().properties.read().clone() {
                Row {
                    // onclick: move |_| {
                    //     if let Some(t) = p.clone().data.r#type
                    //         && (t.key == Some("set".to_string())
                    //             || t.key == Some("collecion".to_string()))
                    //     {
                    //         nav.push(Route::List {
                    //             space_id: p.clone().space_id.clone(),
                    //             list_id: p.clone().object_id.clone(),
                    //         });
                    //     }
                    // },
                    for property in property_vec.clone() {
                        if let Some(prop) = object_properties.get(property.clone().0.id) {
                            PropertyValue {
                                key: "{property.0.id.as_str()}",
                                space_id: props.space_id.clone(),
                                object_id: props.object_id.clone(),
                                data: prop.read().clone(),
                                info: property,
                            }
                        } else {
                            PropertyValue {
                                key: "{property.0.id.as_str()}",
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
fn get_property_id(prop: PropertyWithValue) -> PropertyID {
    return PropertyID(match prop.clone() {
        PropertyWithValue::Text(p) => p.id.clone().unwrap(),
        PropertyWithValue::Number(p) => p.id.clone().unwrap(),
        PropertyWithValue::Select(p) => p.id.clone().unwrap(),
        PropertyWithValue::MultiSelect(p) => p.id.clone().unwrap(),
        PropertyWithValue::Date(p) => p.id.clone().unwrap(),
        PropertyWithValue::Files(p) => p.id.clone().unwrap(),
        PropertyWithValue::Checkbox(p) => p.id.clone().unwrap(),
        PropertyWithValue::Url(p) => p.id.clone().unwrap(),
        PropertyWithValue::Email(p) => p.id.clone().unwrap(),
        PropertyWithValue::Phone(p) => p.id.clone().unwrap(),
        PropertyWithValue::Objects(p) => p.id.clone().unwrap(),
    });
}
