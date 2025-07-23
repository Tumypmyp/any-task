use dioxus::prelude::*;

use dioxus_primitives::checkbox::{Checkbox, CheckboxIndicator, CheckboxState};

#[component]
pub fn Task(done: bool, name: String, id: String) -> Element {
    rsx!{
        div { "class": "button-holder",
            button {
                class: "button",
                width: "90vw",
                display: "flex",
                "flex-direction": "row",
                "data-style": if done { "outline" } else { "secondary" },
                "{name}"
                div { "class": "checkbox-holder",
                    Checkbox {
                        class: "checkbox",
                        name: "tos-check",
                        aria_label: "Done",
                        on_checked_change: move |e| {
                            tracing::debug!("on checked{:#?}", e);
                        },
                        // disabled: true,
                        default_checked: if done { CheckboxState::Checked } else { CheckboxState::Unchecked },
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
    }
}
