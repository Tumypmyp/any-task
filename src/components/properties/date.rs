use std::str::FromStr;
use dioxus::prelude::*;
use openapi::models::ApimodelPeriodDatePropertyValue;
use chrono::{DateTime, FixedOffset, NaiveTime};
use dioxus_primitives::{ContentAlign, ContentSide};
use dioxus_primitives::popover::{PopoverContent, PopoverRoot, PopoverTrigger};
use crate::API_CLIENT;
#[component]
pub fn DateTimePropertyValue(
    space_id: String,
    object_id: String,
    prop: Signal<ApimodelPeriodDatePropertyValue>,
) -> Element {
    let property_key = prop().key.unwrap();
    let date = prop().date.unwrap_or_default();
    let dt = use_signal(|| DateTime::parse_from_rfc3339(&date).unwrap_or_default());
    rsx! {
        DatePropertyValue {
            space_id: space_id.clone(),
            object_id: object_id.clone(),
            dt,
        }
        TimePropertyValue {
            space_id,
            object_id,
            property_key,
            dt,
        }
    }
}
#[component]
pub fn DatePropertyValue(
    space_id: String,
    object_id: String,
    dt: Signal<DateTime<FixedOffset>>,
) -> Element {
    let date = dt().format("%d/%m/%y");
    rsx! {
        div { class: "button-holder", key: "{object_id}",
            button {
                class: "button",
                display: "flex",
                "flex-direction": "row",
                "data-style": "outline",
                "{date}"
            }
        }
    }
}
#[component]
pub fn TimePropertyValue(
    space_id: String,
    object_id: String,
    property_key: String,
    dt: Signal<DateTime<FixedOffset>>,
) -> Element {
    let mut time = use_signal(|| dt().format("%H:%M").to_string());
    let mut time_set = use_signal(|| time());
    let mut open = use_signal(|| false);
    let space_id_clone = use_signal(|| space_id.clone());
    let object_id_clone = use_signal(|| object_id.clone());
    let property_key_clone = use_signal(|| property_key.clone());
    rsx! {
        PopoverRoot {
            class: "button-holder",
            // key: "{object_id}",
            open: open(),
            on_open_change: move |v| {
                if v == true {
                    time_set.set(time());
                }
                open.set(v);
            },
            PopoverTrigger {
                div {
                    class: "button",
                    display: "flex",
                    "flex-direction": "row",
                    "data-style": "outline",
                    "{time}"
                }
            }
            PopoverContent { gap: "0.25rem", side: ContentSide::Bottom,
                h3 {
                    padding_top: "0.25rem",
                    padding_bottom: "0.25rem",
                    width: "100%",
                    text_align: "center",
                    margin: 0,
                    "Set Time"
                }
                input {
                    class: "input",
                    value: "{time_set.read()}",
                    oninput: move |event| {
                        *time_set.write() = event.value();
                    },
                }
                button {
                    class: "button",
                    "data-style": "outline",
                    onclick: move |_| {
                        if let Ok(t) = NaiveTime::from_str(&time_set.read()) {
                            let new_dt = dt()
                                .with_time(t)
                                .unwrap();
                            API_CLIENT
                                .read()
                                .update_date_property(
                                    space_id_clone(),
                                    object_id_clone(),
                                    property_key_clone(),
                                    new_dt,
                                );    
                            time.set(time_set());
                        }
                        open.set(false);
                    },
                    "Confirm"
                }
                button {
                    class: "button",
                    "data-style": "outline",
                    onclick: move |_| {
                        open.set(false);
                    },
                    "Cancel"
                }
            }
        }
    }
}
