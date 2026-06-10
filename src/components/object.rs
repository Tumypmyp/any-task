use crate::Route;
use crate::components::button::Button;
use crate::components::column::Column;
use crate::components::properties::PropertyValue;
use crate::components::row::*;
use crate::helpers::*;
use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Props, PartialEq)]
pub struct ObjectProps {
    pub space_id: ReadSignal<String>,
    pub id: ReadSignal<String>,
    pub positions: Store<TileTree>,
}
#[component]
pub fn Object(props: ObjectProps) -> Element {
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
        Some(Err(e)) => return rsx! { "Error: {e}" },
        None => return rsx! { "Loading..." },
        Some(Ok(props)) => props,
    };
    rsx! {
        ObjectRelations {
            id: NodeId(0),
            positions: props.positions,
            values: property_values.clone(),
        }
    }
}

#[component]
pub fn ObjectRelations(
    id: NodeId,
    positions: Store<TileTree>,
    values: HashMap<String, prost_types::Value>,
) -> Element {
    if !positions().0.contains_key(&id.clone()) {
        return rsx! {};
    }
    match &**&positions().0.get(&id).expect("corrupted tile tree") {
        Node::Split {
            direction,
            ratio,
            first,
            second,
        } => {
            let children = rsx! {
                if positions.read().0.contains_key(&first) {
                    div { style: "flex: {ratio}; min-width: 0; min-height: 0; \
                        box-sizing: border-box; box-shadow: inset 0 0 0 1px var(--secondary-color-6);",
                        ObjectRelations {
                            id: first.clone(),
                            positions,
                            values: values.clone(),
                        }
                    }
                }
                if positions.read().0.contains_key(&second) {
                    div { style: "flex: calc(1 - {ratio}); min-width: 0; min-height: 0; \
                        box-sizing: border-box; box-shadow: inset 0 0 0 1px var(--secondary-color-6);",
                        ObjectRelations {
                            id: second.clone(),
                            positions,
                            values: values.clone(),
                        }
                    }
                }
            };

            match direction {
                SplitDirection::Row => rsx! {
                    Row { {children} }
                },
                SplitDirection::Column => rsx! {
                    Column { {children} }
                },
            }
        }
        Node::Pane { relation_key } => {
            let val = values
                .get(relation_key.as_str())
                .unwrap_or(&prost_types::Value { kind: None })
                .clone();
            rsx! {
                div { style: "display: flex; align-items: center; justify-content: center; \
                                                                  width: 100%; height: 100%; min-width: 0; min-height: 0; \
                                                                  box-sizing: border-box;",
                    PropertyValue {
                        //   space_id: props.space_id,
                        //   object_id: props.id,
                        data: val,
                        // info: property,
                    }
                }
            }
        }
    }
}
