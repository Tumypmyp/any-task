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
    info: ReadSignal<(PropertyInfo, PropertySettings)>,
) -> Element {
    rsx! {
        Checkbox {

            // height: "{info().1.height}vh",
            // width: "{info().1.height}vh",
            // style: "max-width: 100%; max-height: 100%; aspect-ratio: 1/1;",
            // style: "width: auto; height: 100%; aspect-ratio: 1 / 1; padding: 0;", //display: flex; align-items: center; justify-content: center;",
            style: "width: 100%; height: 100%; max-width: 100%; max-height: 100%; aspect-ratio: 1 / 1; padding: 0; box-sizing: border-box;",
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
