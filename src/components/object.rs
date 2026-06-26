use crate::components::column::Column;
use crate::components::properties::PropertyValue;
use crate::components::row::*;
use crate::helpers::*;
use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Props, PartialEq)]
pub struct ObjectProps {
    pub positions: Store<TileTree>,
    pub details: ObjectDetails,
    pub all_properties: ReadSignal<HashMap<RelationKey, RelationInfo>>,
}
#[component]
pub fn Object(props: ObjectProps) -> Element {
    let values: HashMap<String, prost_types::Value> = props
        .details
        .fields
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();

    rsx! {
        ObjectRelations {
            id: NodeId(0),
            positions: props.positions,
            values,
            all_properties: props.all_properties,
        }
    }
}

#[component]
pub fn ObjectRelations(
    id: NodeId,
    positions: Store<TileTree>,
    values: HashMap<String, prost_types::Value>,
    all_properties: ReadSignal<HashMap<RelationKey, RelationInfo>>,
) -> Element {
    if !positions().nodes.contains_key(&id.clone()) {
        return rsx! {};
    }
    match &**&positions().nodes.get(&id).expect("corrupted tile tree") {
        Node::Split {
            direction,
            ratio,
            first,
            second,
            ..
        } => {
            let children = rsx! {
                if positions.read().nodes.contains_key(&first) {
                    div { style: "flex: {ratio}; min-width: 0; min-height: 0;",
                        // box-sizing: border-box; box-shadow: inset 0 0 0 1px var(--secondary-color-6);",
                        ObjectRelations {
                            id: first.clone(),
                            positions,
                            values: values.clone(),
                            all_properties,
                        }
                    }
                }
                if positions.read().nodes.contains_key(&second) {
                    div { style: "flex: calc(1 - {ratio}); min-width: 0; min-height: 0;",
                        // box-sizing: border-box; box-shadow: inset 0 0 0 1px var(--secondary-color-6);",
                        ObjectRelations {
                            id: second.clone(),
                            positions,
                            values: values.clone(),
                            all_properties,
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
        Node::Pane { relation_key, .. } => {
            let val = values
                .get(relation_key.as_str())
                .unwrap_or(&prost_types::Value { kind: None })
                .clone();
            rsx! {
                div { style: "display: flex; justify-content: center; \
                                 width: 100%; height: 100%; min-width: 0; min-height: 0; \
                             align-items: center; box-sizing: border-box;",
                    PropertyValue {
                        relation_key: relation_key.clone(),
                        data: val,
                        all_properties,
                    }
                }
            }
        }
    }
}
