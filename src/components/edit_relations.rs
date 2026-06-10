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
    positions: Store<TileTree>,

    all_properties: ReadSignal<Vec<RelationInfo>>,
) -> Element {
    let node = positions()
        .0
        .get(&id)
        .context("corrupted tile tree")?
        .clone();

    use_effect(move || {
        let Node::Split { first, second, .. } = positions()
            .0
            .get(&id)
            .expect("got corrupted tile tree")
            .clone()
        else {
            return;
        };
        if id.0 == 0 {
            return;
        }
        let first_exists = positions().0.contains_key(&first);
        let second_exists = positions().0.contains_key(&second);
        match (first_exists, second_exists) {
            (false, true) => {
                let val = positions().0.get(&second.clone()).expect("").clone();
                positions.write().0.insert(id, val);
            }
            (true, false) => {
                let val = positions().0.get(&first.clone()).expect("").clone();
                positions.write().0.insert(id, val);
            }
            (false, false) => {
                positions.write().0.remove(&id);
            }
            _ => {}
        }
    });

    match node {
        Node::Split {
            direction,
            ratio,
            first,
            second,
        } => {
            let child_width = match direction {
                SplitDirection::Row => "50%",
                SplitDirection::Column => "100%",
            };
            let children = rsx! {

                if positions.read().0.contains_key(&first) {
                    div { style: "flex: {ratio}; min-width: 0; min-height: 0; \
                        box-sizing: border-box; box-shadow: inset 0 0 0 1px var(--secondary-color-6);",

                        EditRelations { id: first, positions, all_properties }
                    }
                }
                if positions.read().0.contains_key(&second) {
                    div { style: "flex: calc(1 - {ratio}); min-width: 0; min-height: 0; \
                        box-sizing: border-box; box-shadow: inset 0 0 0 1px var(--secondary-color-6);",
                        EditRelations { id: second, positions, all_properties }
                    }
                }
            };

            match direction {
                SplitDirection::Row => rsx! {
                    Row { style: "box-shadow: inset 0 0 0 1px var(--secondary-color-6);",
                        {children}
                    }
                },
                SplitDirection::Column => rsx! {
                    Column { style: "box-shadow: inset 0 0 0 1px var(--secondary-color-6);",
                        {children}
                    }
                },
            }
        }
        Node::Pane { relation_key } => rsx! {
            div { style: "display: flex; align-items: center; justify-content: center; \
                                                              width: 100%; height: 100%; min-width: 0; min-height: 0; \
                                                              box-sizing: border-box;",

                Property {
                    id,
                    positions,
                    relation_key,
                    all_properties,
                }
            }
        },
    }
}

#[component]
pub fn Property(
    id: NodeId,
    positions: Store<TileTree>,
    relation_key: RelationKey,
    all_properties: ReadSignal<Vec<RelationInfo>>,
) -> Element {
    let mut query = use_signal(String::new);
    let mut value = use_signal(|| Some(relation_key));
    rsx! {
        Column {

            button {
                // variant: ButtonVariant::Primary,
                onclick: move |_| {
                    positions
                        .with_mut(|v| {
                            v.add_up(id);
                        });
                },
                "+"
            }
            Row { position: RowPosition::Middle,

                button {
                    // variant: ButtonVariant::Primary,
                    onclick: move |_| {
                        positions
                            .with_mut(|v| {
                                v.add_left(id);
                            });
                    },
                    "+"
                }
                Combobox::<RelationKey> {
                    // style: "display: flex; align-items: center; justify-content: center; flex: 1 1 auto; min-width: 1;",
                    value: Some(value.into()),
                    query: Some(query()),
                    on_value_change: move |next: Option<RelationKey>| {
                        value.set(next.clone());
                        positions
                            .with_mut(|v| {
                                v.0
                                    .insert(
                                        id,
                                        Node::Pane {
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
                button {
                    // variant: ButtonVariant::Destructive,
                    onclick: move |_| {
                        positions
                            .with_mut(|v| {
                                if id == NodeId(0) {
                                    v.0
                                        .insert(
                                            id,
                                            Node::Pane {
                                                relation_key: RelationKey::default(),
                                            },
                                        );
                                } else {
                                    v.0.remove(&id);
                                }
                            });
                    },
                    "X"
                }
                button {
                    // variant: ButtonVariant::Primary,
                    onclick: move |_| {
                        positions
                            .with_mut(|v| {
                                v.add_right(id);
                            });
                    },
                    "+"
                }
            }
            button {
                onclick: move |_| {
                    positions
                        .with_mut(|v| {
                            v.add_down(id);
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
                style: "max-width: 100%;",
                index: i,
                value: info.key.clone(),
                text_value: info.name.clone(),
                {info.name.clone()}
            }
        }
    }
}
