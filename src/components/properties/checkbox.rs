use crate::API_CLIENT;
use crate::components::checkbox::Checkbox;
use crate::helpers::*;
use dioxus::prelude::*;
use dioxus_primitives::checkbox::CheckboxState;
use openapi::models::ApimodelCheckboxPropertyValue;

impl PropertyRenderer for ApimodelCheckboxPropertyValue {
    fn render(
        &self,
        space_id: String,
        object_id: String,
        _info: PropertyInfo,
        settings: PropertySettings,
    ) -> Element {
        rsx! {
            CheckboxPropertyValue {
                space_id: &space_id,
                object_id: &object_id,
                prop: self.clone(),
                size: settings.height(),
            }
        }
    }
}

#[component]
pub fn CheckboxPropertyValue(
    space_id: String,
    object_id: String,
    prop: ApimodelCheckboxPropertyValue,
    size: f64,
) -> Element {
    let mut value = use_signal(|| prop);
    rsx! {
        Checkbox {
            height: "{size}vh",
            width: "{size}vh",
            // style: "max-width: 100%; max-height: 100%; aspect-ratio: 1/1;",
            // style: "width: auto; height: 100%; aspect-ratio: 1 / 1; padding: 0;", //display: flex; align-items: center; justify-content: center;",
            // style: "width: 100%; height: 100%; max-width: 100%; max-height: 100%; aspect-ratio: 1 / 1; padding: 0; box-sizing: border-box;",
            // width: "100%",
            // height: "100%",
            on_checked_change: move |e| {
                let sp = space_id.clone();
                let ob = object_id.clone();
                value.write().checkbox = if e == CheckboxState::Checked {
                    Some(true)
                } else {
                    Some(false)
                };
                API_CLIENT.read().update_done_property(sp, ob, value.read().checkbox);
            },

            default_checked: if value().checkbox.unwrap_or_default() { CheckboxState::Checked } else { CheckboxState::Unchecked },
        }
    }
}
