use crate::API_CLIENT;
use crate::components::base::*;
use crate::helpers::*;
use dioxus::prelude::*;
use dioxus_primitives::scroll_area::ScrollDirection;
use std::vec;
#[component]
pub fn ChooseView(
    space_id: Signal<String>,
    list_id: Signal<String>,
    view_id: Store<String>,
    all_views: Store<Vec<ViewInfo>>,
) -> Element {
    use_effect(move || {
        spawn(async move {
            let space_id = space_id();
            let list_id = list_id();
            let views = API_CLIENT.read().get_views(&space_id, &list_id).await;
            match views {
                Ok(view) => {
                    for v in view.data.unwrap() {
                        all_views.write().push(ViewInfo {
                            id: v.id.unwrap(),
                            name: v.name.unwrap(),
                        });
                    }
                }
                Err(e) => {
                    tracing::error!("error loading property list: {:#?}", e);
                }
            }
        });
    });
    let mut open = use_signal(|| false);
    rsx! {
        ButtonHolder { "flex-shrink": "0", width: "20vw",
            PopoverRoot {
                open: open(),
                on_open_change: move |v| {
                    open.set(v);
                },
                PopoverTrigger { "Views" }
                PopoverContent {
                    ScrollArea {
                        width: "13em",
                        height: "15em",
                        border: "1px solid var(--primary-color-6)",
                        border_radius: "0.5em",
                        padding: "0 1em 1em 1em",
                        direction: ScrollDirection::Vertical,
                        tabindex: "0",
                        for (i , view) in all_views.read().clone().iter().enumerate() {
                            ViewButton { index: i, view_id, view: view.clone() }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ViewButton(index: usize, view_id: Store<String>, view: ViewInfo) -> Element {
    let value = view.id.clone();
    rsx! {
        ButtonWithHolder {
            variant: ButtonVariant::Ghost,
            onclick: move |_| {
                view_id
                    .with_mut(|v| {
                        *v = value.clone();
                    });
            },
            "{view.name}"
        }
    }
}
