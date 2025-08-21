use dioxus::prelude::*;
use openapi::models::*;
use std::collections::HashMap;
use crate::PropertyValue;
#[derive(Clone, Props, PartialEq)]
pub struct ListObjectProps {
    pub name: String,
    pub space_id: String,
    pub object_id: String,
    pub properties: HashMap<String, Option<ApimodelPeriodPropertyWithValue>>,
    pub ids_order: Vec<String>,
}
#[component]
pub fn ListObject(props: ListObjectProps) -> Element {
    rsx! {
        div { "class": "button-holder", id: props.object_id,
            button {
                class: "button",
                width: "90vw",
                display: "flex",
                "flex-direction": "row",
                "data-style": "outline",
                "{props.name}"
                for id in &props.ids_order {
                    if let Some(p) = props.properties.get(id) {
                        PropertyValue {
                            space_id: &props.space_id,
                            object_id: &props.object_id,
                            property_id: id,
                            data: p.clone(),
                        }
                    }
                }
            }
        }
    }
}
