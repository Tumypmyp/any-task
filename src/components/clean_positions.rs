use crate::components::button::*;
use crate::components::row::*;
use crate::helpers::*;
use dioxus::prelude::*;
use std::vec;
#[component]
pub fn CleanPositions(positions: Store<TileTree>) -> Element {
    let property = RelationInfo {
        key: RelationKey("name".to_string()),
        name: "Name".to_string(),
    };
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
