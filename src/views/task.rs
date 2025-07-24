use dioxus::prelude::*;

use dioxus_primitives::checkbox::{Checkbox, CheckboxIndicator, CheckboxState};

use crate::views::API_CLIENT;

#[component]
pub fn Task(space_id: String, object_id: String, done: bool, name: String) -> Element {
    let mut done = use_signal(|| done);
    rsx!{
        div { "class": "button-holder",
            button {
                class: "button",
                width: "90vw",
                display: "flex",
                "flex-direction": "row",
                "data-style": if done() { "secondary" } else { "outline" },
                "{name}"
                div { "class": "checkbox-holder",
                    Checkbox {
                        class: "checkbox",
                        name: "tos-check",
                        aria_label: "Done",
                        on_checked_change: move |e| {
                            let sp = space_id.clone();
                            let ob = object_id.clone();
                            *done.write() = if e == CheckboxState::Checked { true } else { false };

                            API_CLIENT
                                .read()
                                .update_done_property(
                                    sp,
                                    ob,
                                    *done.read(),
                                );
                        },
                        default_checked: if done() { CheckboxState::Checked } else { CheckboxState::Unchecked },
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
