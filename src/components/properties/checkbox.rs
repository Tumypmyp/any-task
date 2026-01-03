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
        match settings {
            PropertySettings::Checkbox(s) => {
                rsx! {
                    CheckboxPropertyValue {
                        space_id: &space_id,
                        object_id: &object_id,
                        prop: self.clone(),
                    }
                }
            }
            _ => rsx! {},
        }
    }
}
#[component]
pub fn CheckboxPropertyValue(
    space_id: String,
    object_id: String,
    prop: ApimodelCheckboxPropertyValue,
) -> Element {
    let mut value = use_signal(|| prop);
    rsx! {
        Checkbox {
            width: "100%",
            height: "100%",
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
