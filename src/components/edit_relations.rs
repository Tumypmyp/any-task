use crate::components::button::*;
use crate::components::column::*;
use crate::components::combobox::*;
use crate::components::label::*;
use std::collections::HashMap;
// use crate::components::properties::*;
use crate::components::row::*;
use crate::components::separator::*;
use crate::components::slider::*;
use crate::helpers::*;
use dioxus::prelude::*;
use std::vec;

#[component]
pub fn EditRelations(
    id: NodeId,
    positions: Store<HashMap<NodeId, ViewTree>>,
    all_properties: ReadSignal<Vec<RelationInfo>>,
) -> Element {
    let node = positions
        .get(id)
        .context("corrupted tile tree")?
        .read()
        .clone();

    use_effect(move || {
        let ViewTree::Split { first, second, .. } = positions
            .get(id)
            .expect("got corrupted tile tree")
            .read()
            .clone()
        else {
            return;
        };
        if id.0 == 0 {
            return;
        }
        let first_exists = positions.contains_key(&first);
        let second_exists = positions.contains_key(&second);
        match (first_exists, second_exists) {
            (false, true) => {
                let val = positions.get(second.clone()).expect("").read().clone();
                positions.write().insert(id, val);
            }
            (true, false) => {
                let val = positions.get(first.clone()).expect("").read().clone();
                positions.write().insert(id, val);
            }
            (false, false) => {
                positions.remove(&id);
            }
            _ => {}
        }
    });

    match node {
        ViewTree::Split {
            first,
            second,
            direction,
            ..
        } => {
            let children = rsx! {
                if positions.read().contains_key(&first) {
                    EditRelations { id: first, positions, all_properties }
                }
                if positions.read().contains_key(&second) {
                    EditRelations { id: second, positions, all_properties }
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
        ViewTree::Pane { relation_key } => rsx! {
            Property {
                id,
                positions,
                relation_key,
                all_properties,
            }
        },
    }
}

#[component]
pub fn Property(
    id: NodeId,
    positions: Store<HashMap<NodeId, ViewTree>>,
    relation_key: RelationKey,
    all_properties: ReadSignal<Vec<RelationInfo>>,
) -> Element {
    let mut query = use_signal(String::new);
    let mut value = use_signal(|| Some(relation_key));
    rsx! {
        Column {
            Row { position: Position::Middle,
                Combobox::<RelationKey> {
                    value: Some(value.into()),
                    query: Some(query()),
                    on_value_change: move |next: Option<RelationKey>| {
                        value.set(next.clone());
                        positions
                            .with_mut(|v| {
                                v.insert(
                                    id,
                                    ViewTree::Pane {
                                        relation_key: next.unwrap_or_default(),
                                    },
                                );
                            });
                    },
                    on_query_change: move |next| query.set(next),
                    placeholder: "Search relations...",
                    aria_label: "Switch relation",
                    list_aria_label: "Relations",
                    ComboboxEmpty { "No relations match." }
                    PropertyOptions { all_properties }

                }
                Button {
                    variant: ButtonVariant::Destructive,
                    onclick: move |_| {
                        positions
                            .with_mut(|v| {
                                if id == NodeId(0) {
                                    v.insert(
                                        id,
                                        ViewTree::Pane {
                                            relation_key: RelationKey::default(),
                                        },
                                    );
                                } else {
                                    v.remove(&id);
                                }
                            });
                    },
                    "X"
                }
                Button {
                    variant: ButtonVariant::Primary,
                    onclick: move |_| {
                        positions
                            .with_mut(|v| {
                                let id_first = v
                                    .keys()
                                    .map(|node_id| node_id.0)
                                    .max()
                                    .map(|max_val| NodeId(max_val + 1))
                                    .unwrap();
                                let id_second = NodeId(id_first.0 + 1);
                                v.insert(
                                    id_second,
                                    ViewTree::Pane {
                                        relation_key: RelationKey::default(),
                                    },
                                );
                                let val = v.get(&id).expect("node dissapered from tree").clone();
                                v.insert(id_first, val);
                                v.insert(
                                    id,
                                    ViewTree::Split {
                                        direction: SplitDirection::Row,
                                        ratio: 0.5,
                                        first: id_first,
                                        second: id_second,
                                    },
                                );
                            });
                    },
                    "+"
                }
            }
            Button {
                variant: ButtonVariant::Primary,
                onclick: move |_| {
                    positions
                        .with_mut(|v| {
                            let id_first = v
                                .keys()
                                .map(|node_id| node_id.0)
                                .max()
                                .map(|max_val| NodeId(max_val + 1))
                                .unwrap();
                            let id_second = NodeId(id_first.0 + 1);
                            v.insert(
                                id_second,
                                ViewTree::Pane {
                                    relation_key: RelationKey::default(),
                                },
                            );
                            let val = v.get(&id).expect("node dissapered from tree").clone();
                            v.insert(id_first, val);
                            v.insert(
                                id,
                                ViewTree::Split {
                                    direction: SplitDirection::Column,
                                    ratio: 0.5,
                                    first: id_first,
                                    second: id_second,
                                },
                            );
                        });
                },
                "+"
            }
        }
    }
}

#[component]
fn PropertyOptions(all_properties: ReadSignal<Vec<RelationInfo>>) -> Element {
    rsx! {
        for (i, info) in all_properties.read().iter().enumerate() {
            ComboboxOption::<RelationKey> {
                index: i,
                value: info.key.clone(),
                text_value: info.name.clone(),
                {info.name.clone()}
            }
        }
    }
}
