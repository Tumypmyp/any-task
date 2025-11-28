use crate::API_CLIENT;
use crate::components::checkbox::Checkbox;
use crate::helpers::*;
use dioxus::prelude::*;
use dioxus_primitives::checkbox::CheckboxState;
use openapi::models::ApimodelCheckboxPropertyValue;
#[component]
pub fn CheckboxPropertyValue(
    space_id: String,
    object_id: String,
    prop: Signal<ApimodelCheckboxPropertyValue>,
    info: ReadSignal<PropertyInfo>,
) -> Element {
    rsx! {
        div { class: "checkbox-holder", width: "{info().width}vw",
            Checkbox {
                on_checked_change: move |e| {
                    let sp = space_id.clone();
                    let ob = object_id.clone();
                    prop.write().checkbox = if e == CheckboxState::Checked {
                        Some(true)
                    } else {
                        Some(false)
                    };
                    API_CLIENT.read().update_done_property(sp, ob, prop.read().checkbox);
                },

                default_checked: if prop().checkbox.unwrap_or_default() { CheckboxState::Checked } else { CheckboxState::Unchecked },
            }
        }
    }
}
