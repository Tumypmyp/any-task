use crate::API_CLIENT;
use crate::helpers::*;
use dioxus::prelude::*;
use dioxus_primitives::checkbox::{Checkbox, CheckboxIndicator, CheckboxState};
use openapi::models::ApimodelPeriodCheckboxPropertyValue;
#[component]
pub fn CheckboxPropertyValue(
    space_id: String,
    object_id: String,
    prop: Signal<ApimodelPeriodCheckboxPropertyValue>,
    info: ReadSignal<PropertyInfo>,
) -> Element {
    rsx! {
        div { class: "checkbox-holder", width: "{info().width}vw",
            Checkbox {
                "class": "checkbox",
                name: "tos-check",
                "aria_label": "checkbox",
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
                CheckboxIndicator { "class": "checkbox-indicator",
                    svg {
                        class: "checkbox-check-icon",
                        view_box: "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        path { d: "M5 13l4 4L19 7" }
                    }
                }
            }
        }
    }
}
