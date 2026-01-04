use crate::API_CLIENT;
use crate::components::select::*;
use crate::helpers::*;
use dioxus::prelude::*;
use std::vec;
#[component]
pub fn ChooseView(
    space_id: Signal<String>,
    list_id: Signal<String>,
    view_id: Store<String>,
) -> Element {
    let mut all_views: Store<Vec<ViewInfo>> = use_store(|| vec![]);
    use_effect(move || {
        let client = API_CLIENT.read().clone();
        spawn(async move {
            let views = client.get_views(&space_id(), &list_id()).await;
            match views {
                Ok(view) => {
                    for v in view.data.unwrap() {
                        all_views.write().push(ViewInfo {
                            id: v.id.clone().unwrap(),
                            name: v.name.unwrap(),
                        });
                        if view_id.read().is_empty() {
                            view_id.set(v.id.clone().unwrap());
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("error loading views list: {:#?}", e);
                }
            }
        });
    });
    let views = all_views.iter().enumerate().map(|(i, f)| {
        rsx! {
            SelectOption::<String> { index: i, value: f().id, text_value: f().name,
                "{f().name}"
                SelectItemIndicator {}
            }
        }
    });
    let mut view_id_setter = use_signal(|| view_id().clone());
    let select_value = use_memo(move || {
        let current_view_id = view_id_setter.read();
        if current_view_id.is_empty() {
            None
        } else {
            view_id.set(current_view_id.clone());
            Some(Some(current_view_id.clone()))
        }
    });
    rsx! {
        Select::<String> {
            placeholder: "Select a view",
            on_value_change: move |v: Option<String>| {
                view_id_setter.set(v.unwrap());
            },
            SelectTrigger { aria_label: "Select View", width: "20vw", SelectValue {} }
            SelectList { aria_label: "Views",
                SelectGroup {
                    SelectGroupLabel { "Views" }
                    {views}
                }
            }
        }
    }
}
