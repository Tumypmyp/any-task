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
        Checkbox {
            height: "{info().height}vh",
            width: "{info().height}vh", // make it square
            // style: "height: 100%; aspect-ratio: 1 / 1; display: flex;",
            // width: "100%",
            // height: "100%",
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
