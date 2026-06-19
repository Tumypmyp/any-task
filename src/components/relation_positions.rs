use crate::components::button::*;
use crate::components::column::*;
use crate::components::combobox::*;
use crate::components::label::*;
use dioxus_html::MountedData;
use dioxus_html::geometry::PixelsRect;
use std::collections::HashMap;
use std::rc::Rc;
// use crate::components::properties::*;
use crate::components::row::*;
use crate::helpers::*;
use dioxus::prelude::*;
use std::vec;

#[component]
pub fn RelationPositionsCleaner(positions: Store<TileTree>) -> Element {
    rsx! {
        Button {
            variant: ButtonVariant::Destructive,
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
                                    relation_key: RelationKey("name".to_string()),
                                },
                            );
                    });
            },
            "Clean"
        }
    }
}
#[component]
pub fn RelationPositionsEditor(
    id: NodeId,
    positions: Store<TileTree>,
    all_properties: ReadSignal<HashMap<RelationKey, RelationInfo>>,
) -> Element {
    let node = positions()
        .nodes
        .get(&id)
        .context("corrupted tile tree")?
        .clone();

    let mut dragging = use_signal(|| false);
    let mut container_mounted = use_signal(|| None::<Rc<MountedData>>);
    let mut cached_rect = use_signal(|| None::<PixelsRect>);

    match node {
        Node::Split {
            direction,
            ratio,
            first,
            second,
            ..
        } => {
            let divider_style = match direction {
                SplitDirection::Row => {
                    "width: 5px; cursor: col-resize; background: var(--secondary-color-6); \
                   flex-shrink: 0; z-index: 1; touch-action: none;"
                }
                SplitDirection::Column => {
                    "height: 7px; cursor: row-resize; background: var(--secondary-color-6); \
                   flex-shrink: 0; z-index: 1; touch-action: none;"
                }
            };

            let children = rsx! {
                if positions.read().nodes.contains_key(&first) {
                    div { style: "flex: {ratio}; min-width: 0; min-height: 0;",
                        // box-sizing: border-box; box-shadow: inset 0 0 0 1px var(--secondary-color-6);",
                        RelationPositionsEditor { id: first, positions, all_properties }
                    }
                }

                div {
                    style: "{divider_style}",
                    prevent_default: "onpointerdown",
                    onpointerdown: move |e: PointerEvent| {
                        e.stop_propagation();
                        // tracing::info!("[drag] pointerdown (type={})", e.pointer_type());
                        // Set dragging=true IMMEDIATELY before the async rect fetch.
                        // Without this, onpointermove fires while dragging() is still
                        // false and every move event is silently dropped.
                        dragging.set(true);

                        let Some(mounted) = container_mounted.read().clone() else {
                            tracing::warn!(
                                "[drag] pointerdown: container_mounted is None — onmounted never fired!"
                            );
                            dragging.set(false);
                            return;
                        };
                        spawn(async move {
                            match mounted.get_client_rect().await {
                                Ok(rect) => {
                                    // tracing::info!(
                                    //     "[drag] rect cached: origin=({:.1},{:.1}) size={:.1}x{:.1}", rect
                                    //     .origin.x, rect.origin.y, rect.size.width, rect.size.height
                                    // );
                                    cached_rect.set(Some(rect));
                                }
                                Err(e) => {
                                    tracing::error!("[drag] get_client_rect failed: {}", e);
                                    dragging.set(false);
                                }
                            }
                        });
                    },
                }

                if positions.read().nodes.contains_key(&second) {
                    div { style: "flex: calc(1 - {ratio}); min-width: 0; min-height: 0;",
                        // box-sizing: border-box; box-shadow: inset 0 0 0 1px var(--secondary-color-6);",
                        RelationPositionsEditor { id: second, positions, all_properties }
                    }
                }
            };

            let on_pointer_move = move |e: PointerEvent| {
                if !dragging() {
                    return;
                }
                // Prevent the browser from panning/scrolling the page
                e.prevent_default();
                e.stop_propagation();
                let Some(rect) = cached_rect() else {
                    // dragging=true but rect not yet fetched — skip this frame
                    // tracing::info!("[drag] pointermove: waiting for rect...");
                    return;
                };

                let client_pos = e.client_coordinates();
                let direction = direction; // copy
                let new_ratio = match direction {
                    SplitDirection::Row => (client_pos.x - rect.origin.x) / rect.size.width,
                    SplitDirection::Column => (client_pos.y - rect.origin.y) / rect.size.height,
                };
                let new_ratio = new_ratio.clamp(0.05, 0.95);
                // tracing::info!(
                //     "[drag] pointermove: pos=({:.1},{:.1}) → ratio={:.3}",
                //     client_pos.x,
                //     client_pos.y,
                //     new_ratio
                // );
                positions.with_mut(|tree| {
                    if let Some(node) = tree.nodes.get_mut(&id) {
                        if let Node::Split { ratio, .. } = node {
                            *ratio = new_ratio as f32;
                        }
                    }
                });
            };

            let on_pointer_up = move |e: PointerEvent| {
                if dragging() {
                    e.stop_propagation();
                    // tracing::info!("[drag] pointerup: drag ended");
                    dragging.set(false);
                    cached_rect.set(None);
                }
            };

            let flex_direction = match direction {
                SplitDirection::Row => "row",
                SplitDirection::Column => "column",
            };

            rsx! {
                div {
                    style: "display: flex; flex-direction: {flex_direction}; \
                            width: 100%; height: 100%; \
                            box-shadow: inset 0 0 0 1px var(--secondary-color-6); \
                            user-select: none; touch-action: none;",
                    onmounted: move |e: MountedEvent| {
                        // tracing::info!("[drag] container mounted");
                        container_mounted.set(Some(e.data()));
                    },
                    onpointermove: on_pointer_move,
                    onpointerup: on_pointer_up,
                    {children}
                }
            }
        }
        Node::Pane { .. } => rsx! {
            div { style: "display: flex; align-items: center; justify-content: center; \
                        width: 100%; height: 100%; min-width: 0; min-height: 0; \
                        box-sizing: border-box;",
                Property {
                    key: "{id:#?}",
                    id,
                    positions,
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
    all_properties: ReadSignal<HashMap<RelationKey, RelationInfo>>,
) -> Element {
    let mut query = use_signal(String::new);
    let current_relation = use_memo(move || {
        positions().nodes.get(&id).and_then(|n| match n {
            Node::Pane { relation_key, .. } => Some(relation_key.clone()),
            _ => None,
        })
    });
    rsx! {
        Column {
            Button {
                size: ButtonSize::Xs,
                onclick: move |_| {
                    positions
                        .with_mut(|v| {
                            v.add_up(id);
                        });
                },
                "+"
            }
            Row { position: RowPosition::Middle,
                Button {
                    size: ButtonSize::Xs,
                    style: "align-self: stretch; height: auto;",
                    onclick: move |_| {
                        positions
                            .with_mut(|v| {
                                v.add_left(id);
                            });
                    },
                    "+"
                }
                Combobox::<RelationKey> {
                    value: Some(current_relation.into()),
                    query: Some(query()),
                    on_value_change: move |next: Option<RelationKey>| {
                        positions
                            .with_mut(|v| {
                                if let Some(Node::Pane { relation_key,.. }) = v.nodes
                                    .get_mut(
                                        &id){
                                    *relation_key = next.unwrap_or_default();
                                }
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
                    size: ButtonSize::Xs,
                    style: "align-self: center;",
                    variant: ButtonVariant::Destructive,
                    onclick: move |_| {
                        positions
                            .with_mut(|v| {
                                if v.root != id {
                                    v.remove_node(id);
                                }
                                tracing::debug!("map: {:#?}", v);
                            });
                    },
                    "X"
                }
                Button {
                    size: ButtonSize::Xs,
                    style: "align-self: stretch; height: auto;",
                    onclick: move |_| {
                        positions
                            .with_mut(|v| {
                                v.add_right(id);
                            });
                    },
                    "+"
                }
            }
            Button {
                size: ButtonSize::Xs,
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
fn PropertyOptions(all_properties: ReadSignal<HashMap<RelationKey, RelationInfo>>) -> Element {
    rsx! {
        for (i, info) in all_properties.read().values().enumerate() {
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
