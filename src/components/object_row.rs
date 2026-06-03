use crate::Route;
use crate::components::button::Button;
use crate::components::column::Column;
use crate::components::properties::PropertyValue;
use crate::components::row::*;
use crate::helpers::NAME_PROPERTY_ID_STR;
use crate::helpers::*;
use dioxus::prelude::*;
use std::collections::HashMap;
#[derive(Clone, Props, PartialEq)]
pub struct ObjectProps {
    pub space_id: ReadSignal<String>,
    pub id: ReadSignal<String>,
    pub properties: Store<HashMap<RelationKey, (RelationInfo, PropertySettings)>>,
}
#[component]
pub fn ObjectView(props: ObjectProps) -> Element {
    let nav = navigator();
    let resp = use_resource({
        move || async move {
            let client_guard = API_CLIENT.read();
            let client = client_guard
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("No API client available, try reloading the app"))?;
            client
                .get_object_properties(&(props.space_id)(), &(props.id)())
                .await
        }
    });
    let resp_value = resp.read();
    let property_values = match resp_value.as_ref() {
        Some(Err(err)) => {
            return rsx! {};
        }
        None => {
            return rsx! { "Loading..." };
        }
        Some(Ok(props)) => props,
    };
    tracing::debug!("got object: {:#?}", property_values);
    let rows: Vec<Element> = (props.properties)()
        .into_iter()
        .filter_map(|(key, property)| {
            let val = property_values.get(property.0.key.as_str())?.clone();
            Some(rsx! {
                Column {
                    PropertyValue {
                        space_id: props.space_id,
                        object_id: props.id,
                        data: val,
                        info: property,
                    }
                }
            })
        })
        .collect();

    rsx! {
        Row { {rows.into_iter()} }
    }
}
