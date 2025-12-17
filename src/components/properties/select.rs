use crate::API_CLIENT;
use crate::components::button::ButtonHolder;
use crate::components::select::*;
use crate::helpers::*;
use dioxus::prelude::*;
use openapi::models::*;
#[component]
pub fn SelectPropertyValue(
    space_id: String,
    object_id: String,
    prop: Signal<ApimodelSelectPropertyValue>,
    info: ReadSignal<PropertyInfo>,
) -> Element {
    let options = info().options;
    let space_id_clone = use_signal(|| space_id.clone());
    let object_id_clone = use_signal(|| object_id.clone());
    rsx! {
        Select::<Option<String>> {
            width: "100%",
            height: "100%",
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
            SelectTrigger { width: "100%", height: "100%",
                SelectValue { width: "100%", height: "100%" }
            }
            SelectPropertySelectList { options }
        }
    }
}
#[component]
pub fn SelectPropertySelectList(options: Vec<ApimodelTag>) -> Element {
    rsx! {
        SelectList {
            SelectGroup {
                for (i , option) in options.clone().iter().enumerate() {
                    if let Some(name) = &option.name {
                        SelectOption::<Option<String>> {
                            index: i,
                            value: option.id.clone().unwrap(),
                            text_value: name.clone(),
                            "{name}"
                        }
                    }
                }
            }
        }
    }
}
