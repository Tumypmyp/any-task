use dioxus::prelude::*;
use dioxus_primitives::select::{
    Select, SelectGroup, SelectItemIndicator, SelectList, SelectOption, SelectTrigger,
    SelectValue,
};
use openapi::models::*;
use crate::API_CLIENT;
#[component]
pub fn SelectPropertyValue(
    space_id: String,
    object_id: String,
    prop: Signal<ApimodelPeriodSelectPropertyValue>,
) -> Element {
    let space_id = use_signal(|| space_id.clone());
    let options = use_resource(move || {
        async move {
            API_CLIENT
                .read()
                .list_select_property_options(&space_id.read(), &prop().id.unwrap())
                .await
        }
    });
    let mut val = use_signal(|| prop().name);
    if let Some(Ok(options)) = &*options.read() {
        rsx! {
            div { "class": "select-holder",
                Select::<Option<String>> {
                    class: "select",
                    placeholder: "{prop().select.unwrap_or_default().name.clone().unwrap_or_default()}",
                    default_value: prop().select.unwrap_or_default().name.clone(),
                    on_value_change: move |v: Option<Option<String>>| {
                        val.set(v.unwrap().clone());
                        println!("{val:#?}changed");
                    },
                    SelectTrigger {
                        class: "select-trigger",
                        aria_label: "select",
                        width: "8rem",
                        SelectValue {}
                        svg {
                            class: "select-expand-icon",
                            view_box: "0 0 24 24",
                            xmlns: "http://www.w3.org/2000/svg",
                            polyline { points: "6 9 12 15 18 9" }
                        }
                    }
                    SelectPropertySelectList { options: options.clone().data.unwrap() }
                }
            }
        }
    } else {
        rsx!()
    }
}
#[component]
pub fn SelectPropertySelectList(options: Vec<ApimodelPeriodTag>) -> Element {
    rsx! {
        SelectList { class: "select-list",
            SelectGroup { class: "select-group",
                for (i , option) in options.clone().iter().enumerate() {
                    if let Some(name) = &option.name {
                        SelectOption::<Option<String>> {
                            class: "select-option",
                            index: i,
                            value: name.clone(),
                            text_value: name.clone(),
                            SelectItemIndicator {
                                svg {
                                    class: "select-check-icon",
                                    view_box: "0 0 24 24",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    path { d: "M5 13l4 4L19 7" }
                                }
                            }
                            "{name}"
                        }
                    }
                }
            }
        }
    }
}
