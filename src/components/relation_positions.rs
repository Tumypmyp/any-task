use crate::components::button::*;
use crate::components::column::*;
use crate::components::combobox::*;
use crate::components::label::*;
use crate::components::row::*;
use crate::helpers::*;
use dioxus::prelude::*;
use dioxus_html::MountedData;
use dioxus_html::geometry::PixelsRect;
use dioxus_icons::lucide::X;
use std::collections::HashMap;
use std::rc::Rc;
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
    delete_mode: ReadSignal<bool>,
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
    let initial_ratio = match &node {
        Node::Split { ratio, .. } => *ratio as f32,
        _ => 0.5 as f32,
    };
    let mut live_ratio = use_signal(|| initial_ratio);
    let mut new_pane_drag = use_context::<Signal<NewPaneDrag>>();

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
                    "width: 8px; cursor: col-resize; background: var(--secondary-color-6); \
                   flex-shrink: 0; z-index: 1; touch-action: none;"
                }
                SplitDirection::Column => {
                    "height: 8px; cursor: row-resize; background: var(--secondary-color-6); \
                   flex-shrink: 0; z-index: 1; touch-action: none;"
                }
            };

            let children = rsx! {
                if positions.read().nodes.contains_key(&first) {
                    div { style: "flex: {live_ratio}; min-width: 0; min-height: 0;",
                        // box-sizing: border-box; box-shadow: inset 0 0 0 1px var(--secondary-color-6);",
                        RelationPositionsEditor {
                            id: first,
                            delete_mode,
                            positions,
                            all_properties,
                        }
                    }
                }

                div {
                    style: "{divider_style}",
                    prevent_default: "onpointerdown",
                    onpointerdown: move |e: PointerEvent| {
                        e.stop_propagation();
                        // Set dragging=true IMMEDIATELY before the async rect fetch.
                        // tracing::info!("[drag] pointerdown (type={})", e.pointer_type());
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
                    div { style: "flex: calc(1 - {live_ratio}); min-width: 0; min-height: 0;",
                        // box-sizing: border-box; box-shadow: inset 0 0 0 1px var(--secondary-color-6);",
                        RelationPositionsEditor {
                            id: second,
                            delete_mode,
                            positions,
                            all_properties,
                        }
                    }
                }
            };

            let on_pointer_move = move |e: PointerEvent| {
                if dragging() {
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
                    let new_ratio = new_ratio.clamp(0.05, 0.95) as f32;
                    live_ratio.set(new_ratio);
                    // tracing::info!(
                    //     "[drag] pointermove: pos=({:.1},{:.1}) → ratio={:.3}",
                    //     client_pos.x,
                    //     client_pos.y,
                    //     new_ratio
                    // );
                    // positions.with_mut(|tree| {
                    //     if let Some(node) = tree.nodes.get_mut(&id) {
                    //         if let Node::Split { ratio, .. } = node {
                    //             *ratio = new_ratio as f32;
                    //         }
                    //     }
                    // });
                }

                // --- new pane drag: update ghost position ---
                if new_pane_drag.read().is_dragging {
                    let pos = e.client_coordinates();
                    let mut state = new_pane_drag.write();
                    state.cursor_x = pos.x;
                    state.cursor_y = pos.y;
                }
            };

            let on_pointer_up = move |e: PointerEvent| {
                if dragging() {
                    e.stop_propagation();
                    let final_ratio = live_ratio();
                    positions.with_mut(|tree| {
                        if let Some(Node::Split { ratio, .. }) = tree.nodes.get_mut(&id) {
                            *ratio = final_ratio;
                        }
                    });
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
                            width: 100%; \
                            box-shadow: inset 0 0 0 1px var(--secondary-color-6); \
                            user-select: none;",
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
            div { style: "position: relative; display: flex; align-items: center; justify-content: center; \
                        width: 100%; height: 100%; min-width: 0; min-height: 0; \
                        box-sizing: border-box;",
                Property {
                    key: "{id:#?}",
                    id,
                    delete_mode,
                    positions,
                    all_properties,
                }
                if new_pane_drag().is_dragging {
                    PaneDropZones { id, new_pane_drag }
                }
            }
        },
    }
}
#[component]
pub fn Property(
    id: NodeId,
    positions: Store<TileTree>,
    delete_mode: ReadSignal<bool>,
    all_properties: ReadSignal<HashMap<RelationKey, RelationInfo>>,
) -> Element {
    let mut query = use_signal(String::new);
    let current_relation = use_memo(move || {
        positions().nodes.get(&id).and_then(|n| match n {
            Node::Pane { relation_key, .. } => Some(relation_key.clone()),
            _ => None,
        })
    });

    if delete_mode() {
        return rsx! {
            Row { position: RowPosition::Middle,
                Button {
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
                    aria_label: "Remove",
                    X {}
                }
            }
        };
    }
    rsx! {
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
#[component]
fn PaneDropZones(id: NodeId, mut new_pane_drag: Signal<NewPaneDrag>) -> Element {
    const ZONES: &[(DropZone, &str, &str)] = &[
        (
            DropZone::Top,
            "polygon(0% 0%, 100% 0%, 50% 50%)",
            "polygon(0% 0%, 100% 0%, 100% 50%, 0% 50%)",
        ),
        (
            DropZone::Bottom,
            "polygon(0% 100%, 100% 100%, 50% 50%)",
            "polygon(0% 50%, 100% 50%, 100% 100%, 0% 100%)",
        ),
        (
            DropZone::Left,
            "polygon(0% 0%, 0% 100%, 50% 50%)",
            "polygon(0% 0%, 50% 0%, 50% 100%, 0% 100%)",
        ),
        (
            DropZone::Right,
            "polygon(100% 0%, 100% 100%, 50% 50%)",
            "polygon(50% 0%, 100% 0%, 100% 100%, 50% 100%)",
        ),
    ];
    let is_active = move |zone: DropZone| {
        let s = new_pane_drag();
        s.hover_node == Some(id) && s.drop_zone == Some(zone)
    };

    rsx! {
        for (zone, tri_clip, _rect_clip) in ZONES {
            div {
                key: "hit-{zone:?}",
                style: format!(
                    "position: absolute; inset: 0; z-index: 51; clip-path: {}; pointer-events: all;",
                    tri_clip,
                ),
                onpointerenter: move |_| {
                    let mut s = new_pane_drag.write();
                    s.hover_node = Some(id);
                    s.drop_zone = Some(zone.clone());
                },
                onpointerleave: move |_| {
                    let mut s = new_pane_drag.write();
                    if s.hover_node == Some(id) && s.drop_zone == Some(zone.clone()) {
                        s.hover_node = None;
                        s.drop_zone = None;
                    }
                },
            }
        }
        for (zone, _tri_clip, rect_clip) in ZONES {
            div {
                key: "vis-{zone:?}",
                style: format!(
                    "position: absolute; inset: 0; z-index: 50; clip-path: {}; pointer-events: none; background-color: var(--secondary-color-5); opacity: {};",
                    rect_clip,
                    if is_active(zone.clone()) { "0.5" } else { "0" },
                ),
            }
        }
    }
}
