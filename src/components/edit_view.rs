use crate::components::button::*;
use crate::components::column::*;
use crate::components::relation_positions::*;
use crate::components::row::*;
use crate::components::scroll_area::*;
use crate::components::sheet::*;
use crate::helpers::*;
use dioxus::prelude::*;
use dioxus_icons::lucide::Settings2;
use dioxus_icons::lucide::SquarePlus;
use dioxus_icons::lucide::Trash2;
use dioxus_primitives::scroll_area::ScrollDirection;
use std::collections::HashMap;
use std::vec;
#[component]
pub fn EditView(
    space_id: String,
    list_id: String,
    positions: Store<TileTree>,
    all_properties: ReadSignal<HashMap<RelationKey, RelationInfo>>,
) -> Element {
    let mut open = use_signal(|| false);
    let mut delete_mode = use_signal(|| false);
    let mut new_pane_drag: Signal<NewPaneDrag> = use_signal(NewPaneDrag::default);
    use_context_provider(|| new_pane_drag);
    rsx! {
        if new_pane_drag().is_dragging {
            div {
                "data-drag-ghost": "",
                style: "position: fixed; width: 32px; height: 32px; \
                            background: var(--primary-color); opacity: 0.8; \
                            border-radius: 6px; pointer-events: none; z-index: 9999; \
                            transform: translate(-50%, -50%);",
            }
        }
        Button {
            variant: ButtonVariant::Secondary,
            onclick: move |_| open.set(true),
            aria_label: "Edit view",
            Settings2 {}
        }
        Sheet {
            open: open(),
            on_open_change: move |v| {
                open.set(v);
                if !v {
                    delete_mode.set(false);
                }
            },
            div {
                onpointermove: move |e: PointerEvent| {
                    if new_pane_drag.read().is_dragging {
                        let x = e.client_coordinates().x;
                        let y = e.client_coordinates().y;
                        spawn(async move {
                            document::eval(
                                    &format!(
                                        r#"const g = document.querySelector('[data-drag-ghost]');
                                                                                                                           if (g) {{ g.style.left = '{x}px'; g.style.top = '{y}px'; }}"#,
                                    ),
                                )
                                .await
                                .ok();
                        });
                    }
                },
                onpointerup: move |_| {
                    let state = new_pane_drag.read().clone();
                    if state.is_dragging {
                        if let (Some(node_id), Some(zone)) = (state.hover_node, state.drop_zone) {
                            positions
                                .with_mut(|p| match zone {
                                    DropZone::Top => p.add_up(node_id),
                                    DropZone::Bottom => p.add_down(node_id),
                                    DropZone::Left => p.add_left(node_id),
                                    DropZone::Right => p.add_right(node_id),
                                });
                        }
                        *new_pane_drag.write() = NewPaneDrag::default();
                    }
                },

                SheetContent {
                    side: SheetSide::Bottom,
                    style: "min-height: 50vh; max-height: 80vh;",
                    ScrollArea {
                        direction: ScrollDirection::Vertical,
                        style: "overflow: hidden auto; overscroll-behavior: contain; \
                                    min-height: 50vh; max-height: 70vh;",
                        Column {
                            Row { position: RowPosition::Middle,
                                Button {
                                    style: "touch-action: none;", // prevents browser scroll on touch before handler runs
                                    "data-drag-btn": "",
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
                                    onclick: move |_| delete_mode.toggle(),
                                    aria_label: "Delete mode",
                                    Trash2 {}
                                }
                                RelationPositionsCleaner { positions }
                            }
                            RelationPositionsEditor {
                                id: positions().root,
                                delete_mode,
                                positions,
                                all_properties,
                            }
                        }
                    }
                }
            }
        }
    }
}
