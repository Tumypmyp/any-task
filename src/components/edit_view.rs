use crate::components::button::*;

use crate::components::drop_zone::Dropzone;

use crate::components::column::*;
use crate::components::relation_positions::*;
use crate::components::row::*;
use crate::helpers::*;
use dioxus::prelude::*;
use dioxus_icons::lucide::Plus;
use dioxus_icons::lucide::SquarePlus;
use dioxus_icons::lucide::Trash2;
use std::collections::HashMap;
use std::vec;
#[component]
pub fn EditView(
    open: ReadSignal<bool>,
    space_id: String,
    list_id: String,
    positions: Store<TileTree>,
    all_properties: ReadSignal<HashMap<RelationKey, RelationInfo>>,
) -> Element {
    let mut delete_mode = use_signal(|| false);
    let mut new_pane_drag: Signal<NewPaneDrag> = use_signal(NewPaneDrag::default);
    use_context_provider(|| new_pane_drag);
    let on_move = move |e: PointerEvent| {
        if new_pane_drag.read().is_dragging {
            let pos = e.client_coordinates();
            spawn(async move {
                document::eval(&format!(
                    r#"const g=document.querySelector('[data-drag-ghost]');if(g){{g.style.left='{}px';g.style.top='{}px';}}"#,
                    pos.x, pos.y
                )).await.ok();
            });
        }
    };
    rsx! {
        if new_pane_drag().is_dragging {
            div {
                "data-drag-ghost": "",
                style: "position: fixed; width: 45px; height: 32px; \
                            background: var(--secondary-color); opacity: 0.5; \
                            border-radius: 6px; pointer-events: none; z-index: 9999; \
                            transform: translate(-50%, -50%);",
                Plus {}
            }
        }
        if open() {
            div {
                onpointermove: on_move,
                onpointerup: move |_| {
                    let state = new_pane_drag.read().clone();
                    if state.is_dragging {
                        match (
                            state.hover_delete,
                            state.hover_node,
                            state.drop_zone,
                            state.dragging_key,
                        ) {
                            (true, _, _, _) => {
                                tracing::debug!("pane dropped on delete button, discarding");
                            }
                            (false, Some(node_id), Some(zone), key) => {
                                positions
                                    .with_mut(|p| match (key, zone) {
                                        (Some(key), z) => p.add_pane_at(node_id, z, key),
                                        (None, z) => p.add_pane_at(node_id, z, RelationKey::empty()),
                                    });
                            }
                            (false, _, _, Some(key)) => {
                                positions.with_mut(|p| p.add_pane_at(p.root, DropZone::Right, key));
                            }
                            _ => {}
                        }
                        *new_pane_drag.write() = NewPaneDrag::default();
                    }
                },

                Column {
                    Row { position: RowPosition::Middle,
                        Button {
                            style: "touch-action: none;", // prevents browser scroll on touch before handler runs
                            "data-drag-btn": "",
                            disabled: new_pane_drag().is_dragging,
                            onpointerdown: move |e: PointerEvent| {
                                e.prevent_default(); // prevents scroll on pointer drag
                                e.stop_propagation();
                                delete_mode.set(false);
                                new_pane_drag.write().is_dragging = true;

                                let pointer_id = e.pointer_id();
                                spawn(async move {
                                    document::eval(
                                            &format!(
                                                r#"document.querySelector('[data-drag-btn]')?.releasePointerCapture({pointer_id});"#,
                                            ),
                                        )
                                        .await
                                        .ok();
                                });
                            },

                            aria_label: "Add new relation",
                            SquarePlus {}
                        }
                        Button {
                            variant: if new_pane_drag().hover_delete && new_pane_drag().is_dragging { ButtonVariant::Destructive } else { ButtonVariant::Primary },
                            onclick: move |_| {
                                if !new_pane_drag.read().is_dragging {
                                    delete_mode.toggle();
                                }
                            },
                            onpointerenter: move |_| {
                                if new_pane_drag.read().is_dragging {
                                    new_pane_drag.write().hover_delete = true;
                                }
                            },
                            onpointerleave: move |_| {
                                new_pane_drag.write().hover_delete = false;
                            },
                            aria_label: "Delete mode",
                            Trash2 {}
                        }

                        RelationPositionsCleaner { positions }
                    }

                    if positions().nodes.len() > 1 {
                        RootDropZone {
                            positions,
                            new_pane_drag,
                            zone: DropZone::Top,
                        }
                    }
                    RelationPositionsEditor {
                        id: positions().root,
                        delete_mode,
                        positions,
                        all_properties,
                    }
                    if positions().nodes.len() > 1 {
                        RootDropZone {
                            positions,
                            new_pane_drag,
                            zone: DropZone::Bottom,
                        }
                    }
                }
            }
        }
    }
}
#[component]
fn RootDropZone(
    positions: Store<TileTree>,
    mut new_pane_drag: Signal<NewPaneDrag>,
    zone: DropZone,
) -> Element {
    let root_id = positions().root;
    let dragging = new_pane_drag().is_dragging;
    let is_active = dragging
        && new_pane_drag().hover_node == Some(root_id)
        && new_pane_drag().drop_zone == Some(zone);

    rsx! {
        Dropzone {
            style: "position: relative; height: 15px; width: 100%; flex-shrink: 0;",
            active: is_active,
            dragging,
            onpointerenter: move |_| {
                if new_pane_drag.read().is_dragging {
                    let mut s = new_pane_drag.write();
                    s.hover_node = Some(root_id);
                    s.drop_zone = Some(zone);
                }
            },
            onpointerleave: move |_| {
                let mut s = new_pane_drag.write();
                if s.hover_node == Some(root_id) && s.drop_zone == Some(zone) {
                    s.hover_node = None;
                    s.drop_zone = None;
                }
            },
        }
    }
}
