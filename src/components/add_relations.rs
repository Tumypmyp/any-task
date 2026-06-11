use crate::components::button::*;
use crate::components::row::*;
use crate::helpers::*;
use dioxus::prelude::*;
use std::vec;
#[component]
pub fn CleanRealtions(positions: Store<TileTree>) -> Element {
    rsx! {
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
pub fn ShowProperty(positions: Store<TileTree>, property: RelationInfo) -> Element {
    rsx! {
        Button {
            variant: ButtonVariant::Ghost,
            onclick: move |_| {
                positions
                    .with_mut(|v| {
                        v.root = NodeId(0);
                        v.nodes.clear();
                        v.nodes
                            .insert(
                                NodeId(0),
                                Node::Pane {
                                    parent: None,
                                    relation_key: property.clone().key,
                                },
                            );
                    });
            },
            "Clean"
        }
    }
}
