use crate::components::button::*;
use crate::components::row::*;
use crate::helpers::*;
use dioxus::prelude::*;
use std::collections::HashMap;
use std::vec;
#[component]
pub fn AddRelations(
    positions: Store<HashMap<NodeId, ViewTree>>,
    all_properties: ReadSignal<Vec<RelationInfo>>,
) -> Element {
    rsx! {
        Row { position: Position::Middle,
            Button { variant: ButtonVariant::Secondary, "Add Properties" }
        }
        ShowProperty {
            positions,
            property: RelationInfo {
                key: RelationKey("name".to_string()),
                name: "Name".to_string(),
            },
        }

    }
}
#[component]
pub fn ShowProperty(
    positions: Store<HashMap<NodeId, ViewTree>>,
    property: RelationInfo,
) -> Element {
    let name = property.clone().name;
    rsx! {
        Button {
            variant: ButtonVariant::Ghost,
            onclick: move |_| {
                positions
                    .with_mut(|v| {
                        v.insert(
                            NodeId(0),
                            ViewTree::Pane {
                                relation_key: property.clone().key,
                            },
                        );
                    });
            },
            "{name}"
        }
    }
}
