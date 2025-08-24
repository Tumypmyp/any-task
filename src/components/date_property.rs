use dioxus::prelude::*;
use openapi::models::ApimodelPeriodDatePropertyValue;
use crate::{components::info};
use chrono::{DateTime};

#[component]
pub fn DatePropertyValue(
    space_id: String,
    object_id: String,
    prop: Signal<ApimodelPeriodDatePropertyValue>,
) -> Element {
    println!("date is {prop:#?}");
    let date = prop().date.unwrap_or_default();
    let d = DateTime::parse_from_rfc3339(&date).unwrap_or_default();
    let date = d.format("%d/%m/%y");
    
    rsx! {
        div { "class": "button-holder", key: "{object_id}",
            button {
                class: "button",
                width: "20vw",
                display: "flex",
                "z-index": "1000",
                "flex-direction": "row",
                "data-style": "outline",
                onclick: move |_| {
                    info("hey".to_string());
                },
                "{date}"
            }
        }
    }
}
