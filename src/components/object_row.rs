use crate::Route;
use crate::components::row::*;
use crate::helpers::NAME_PROPERTY_ID_STR;
use crate::helpers::*;
use crate::properties::PropertyValue;
use crate::separator::Separator;
use dioxus::prelude::*;
use openapi::models::*;
use std::collections::HashMap;
#[derive(Clone, Props, PartialEq)]
pub struct ObjectProps {
    pub name: String,
    pub space_id: String,
    pub object_id: String,
    pub data: ApimodelObject,
    pub properties: Store<Vec<(PropertyInfo, PropertySettings)>>,
}
#[component]
pub fn ObjectRow(props: ObjectProps) -> Element {
    let nav = navigator();
    let mut object_properties =
        use_store(|| HashMap::<PropertyID, ApimodelPropertyWithValue>::new());
    for property in props.data.properties.clone().unwrap().iter() {
        let property_id = get_property_id(property.clone());
        object_properties
            .write()
            .insert(property_id, property.clone());
    }
    let text_property_value = ApimodelTextPropertyValue {
        format: None,
        text: props.data.name.clone(),
        key: None,
        name: None,
        id: None,
        object: None,
    };
    object_properties.write().insert(
        PropertyID(NAME_PROPERTY_ID_STR.to_string()),
        ApimodelPropertyWithValue::Text(Box::new(text_property_value)),
    );
    let p = props.clone();
    let p2 = props.clone();
    rsx! {
        Separator {
            style: "margin: 2px 0; width: 95vw;",
            horizontal: true,
            decorative: true,
        }
        Row {
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
            for property in p2.clone().properties.read().clone() {
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
fn get_property_id(prop: ApimodelPropertyWithValue) -> PropertyID {
    return PropertyID(match prop.clone() {
        ApimodelPropertyWithValue::Text(p) => p.id.clone().unwrap(),
        ApimodelPropertyWithValue::Number(p) => p.id.clone().unwrap(),
        ApimodelPropertyWithValue::Select(p) => p.id.clone().unwrap(),
        ApimodelPropertyWithValue::MultiSelect(p) => p.id.clone().unwrap(),
        ApimodelPropertyWithValue::Date(p) => p.id.clone().unwrap(),
        ApimodelPropertyWithValue::Files(p) => p.id.clone().unwrap(),
        ApimodelPropertyWithValue::Checkbox(p) => p.id.clone().unwrap(),
        ApimodelPropertyWithValue::Url(p) => p.id.clone().unwrap(),
        ApimodelPropertyWithValue::Email(p) => p.id.clone().unwrap(),
        ApimodelPropertyWithValue::Phone(p) => p.id.clone().unwrap(),
        ApimodelPropertyWithValue::Objects(p) => p.id.clone().unwrap(),
    });
}
