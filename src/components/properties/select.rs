use crate::API_CLIENT;
use crate::components::button::ButtonHolder;
use crate::components::select::*;
use crate::helpers::*;
use dioxus::prelude::*;
use openapi::models::SelectPropertyValue;
use openapi::models::Tag;
impl PropertyRenderer for SelectPropertyValue {
    fn render(
        &self,
        space_id: String,
        object_id: String,
        info: PropertyInfo,
        _settings: PropertySettings,
    ) -> Element {
        rsx! {
            if let OptionalInfo::Select(options) = info.optional {
                SelectPValue {
                    space_id: &space_id,
                    object_id: &object_id,
                    prop: self.clone(),
                    options,
                }
            }
        }
    }
}
#[component]
pub fn SelectPValue(
    space_id: String,
    object_id: String,
    prop: SelectPropertyValue,
    options: Vec<Tag>,
) -> Element {
    let space_id_clone = use_signal(|| space_id.clone());
    let object_id_clone = use_signal(|| object_id.clone());
    rsx! {
        Select::<Option<String>> {
            width: "100%",
            height: "100%",
            placeholder: "{prop.clone().select.unwrap_or_default().name.clone().unwrap_or_default()}",
            default_value: prop.clone().select.unwrap_or_default().name.clone(),
            on_value_change: move |v: Option<Option<String>>| {
                let prop = prop.clone();
                spawn(async move {
                    tracing::debug!("chosen option: {:#?}", v);
                    API_CLIENT
                        .read()
                        .update_select_property(
                            space_id_clone(),
                            object_id_clone(),
                            prop.key.unwrap(),
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
pub fn SelectPropertySelectList(options: Vec<Tag>) -> Element {
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
