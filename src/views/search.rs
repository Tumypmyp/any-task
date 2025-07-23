use dioxus::prelude::*;
use crate::Route;
use crate::views::*;

use dioxus_primitives::checkbox::{Checkbox, CheckboxIndicator, CheckboxState};

const STYLE_CSS: Asset = asset!("/assets/styling/style.css");
#[component]
pub fn Search(space_id: String) -> Element {
    let space_id = use_signal(|| space_id.clone());
    let objects = use_resource(move || async move {
        let mut config = openapi::apis::configuration::Configuration::new();
        config.bearer_access_token = Some(home::TOKEN.read().clone());
        let mut req = openapi::models::ApimodelPeriodSearchRequest::new();
        req.types = vec!["task".to_string()].into();

        openapi::apis::search_api::search_space(&config, "2025-05-20", &space_id.read(), req, None, None)
            .await
            .unwrap()
            .data
            .clone()
            .unwrap()
    });
    
    match &*objects.read_unchecked() {
        Some(s) => rsx! {
            document::Link { rel: "stylesheet", href: STYLE_CSS }
            div { id: "object-list",
                for object in s.clone() {
                    for prop in object.clone().properties.unwrap() {
                        match prop {
                            openapi::models::ApimodelPeriodCheckboxPropertyValue { checkbox, name, .. } => {
                                match name {
                                    Some(n) if n.as_str() == "Done" => {
                                        rsx! {
                                            Task { done: checkbox.unwrap_or_default(), name: object.clone().name.unwrap() }
                                        }
                                    }
                                    _ => rsx! {},
                                }
                            }
                            _ => rsx! {},
                        }
                    }
                }
            }
        },
        _ => rsx! {
            div { "Loading dogs..." }
        },
    }
}

#[component]
pub fn Task(done: bool, name: String) -> Element {
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
                        disabled: true,
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
