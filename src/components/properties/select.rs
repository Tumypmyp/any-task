use dioxus::prelude::*;
use dioxus_primitives::select::{
    Select, SelectGroup, SelectItemIndicator, SelectList, SelectOption, SelectTrigger,
    SelectValue,
};
use crate::helpers::*;
use crate::components::base::ButtonHolder;
use openapi::models::*;
use crate::API_CLIENT;
#[component]
pub fn SelectPropertyValue(
    space_id: String,
    object_id: String,
    prop: Signal<ApimodelPeriodSelectPropertyValue>,
    info: ReadSignal<PropertyViewInfo>,
) -> Element {
    let options = info().options;
    let space_id_clone = use_signal(|| space_id.clone());
    let object_id_clone = use_signal(|| object_id.clone());
    rsx! {
        ButtonHolder { width: "{info().width}vw",
            Select::<Option<String>> {
                "class": "select",
                placeholder: "{prop().select.unwrap_or_default().name.clone().unwrap_or_default()}",
                default_value: prop().select.unwrap_or_default().name.clone(),
                on_value_change: move |v: Option<Option<String>>| {
                    spawn(async move {
                        tracing::debug!("chosen option: {:#?}", v);
                        API_CLIENT
                            .read()
                            .update_select_property(
                                space_id_clone(),
                                object_id_clone(),
                                prop().key.unwrap(),
                                v.unwrap(),
                            )
                            .await;
                    });
                },
                SelectTrigger {
                    "class": "select-trigger",
                    "aria_label": "select",
                    "width": "8rem",
                    SelectValue {}
                    svg {
                        class: "select-expand-icon",
                        view_box: "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        polyline { points: "6 9 12 15 18 9" }
                    }
                }
                SelectPropertySelectList { options }
            }
        }
    }
}
#[component]
pub fn SelectPropertySelectList(options: Vec<ApimodelPeriodTag>) -> Element {
    rsx! {
        SelectList { "class": "select-list",
            SelectGroup { "class": "select-group",
                for (i , option) in options.clone().iter().enumerate() {
                    if let Some(name) = &option.name {
                        SelectOption::<Option<String>> {
                            "class": "select-option",
                            index: i,
                            value: option.id.clone().unwrap(),
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
