use dioxus::prelude::*;
use dioxus_primitives::checkbox::{Checkbox, CheckboxIndicator, CheckboxState};
use crate::API_CLIENT;
#[component]
pub fn CheckboxPropertyValue(
    space_id: String,
    object_id: String,
    prop: Signal<bool>,
) -> Element {
    rsx! {
        div { "class": "checkbox-holder",
            Checkbox {
                class: "checkbox",
                name: "tos-check",
                aria_label: "checkbox",
                on_checked_change: move |e| {
                    let sp = space_id.clone();
                    let ob = object_id.clone();
                    *prop.write() = if e == CheckboxState::Checked { true } else { false };
                    API_CLIENT.read().update_done_property(sp, ob, *prop.read());
                },
                default_checked: if prop() { CheckboxState::Checked } else { CheckboxState::Unchecked },
                CheckboxIndicator { class: "checkbox-indicator",
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
