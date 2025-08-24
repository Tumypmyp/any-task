use dioxus::prelude::*;
use openapi::models::ApimodelPeriodTextPropertyValue;

#[component]
pub fn TextPropertyValue(
    space_id: String,
    object_id: String,
    prop: Signal<ApimodelPeriodTextPropertyValue>,
) -> Element {
    let value = prop().text.unwrap();
    rsx! {
        div { class: "text-holder",
            button {
                class: "button",
                "{value}"
            }
        }
    }
}
