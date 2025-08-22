use dioxus::prelude::*;
use openapi::models::*;
use crate::PropertyValue;
#[derive(Clone, Props, PartialEq)]
pub struct ListEntryProps {
    pub name: String,
    pub space_id: String,
    pub object_id: String,
    pub properties: Vec<Option<ApimodelPeriodPropertyWithValue>>,
}
#[component]
pub fn ListEntry(props: ListEntryProps) -> Element {
    rsx! {
        div { "class": "button-holder", id: props.object_id,
            button {
                class: "button",
                width: "90vw",
                display: "flex",
                "flex-direction": "row",
                "data-style": "outline",
                "{props.name}"
                for prop in props.properties {
                    if let Some(p) = prop {
                        PropertyValue {
                            space_id: &props.space_id,
                            object_id: &props.object_id,
                            data: p.clone(),
                        }
                    }
                }
            }
        }
    }
}
