use dioxus::prelude::*;
use openapi::models::ApimodelPeriodTextPropertyValue;
use crate::components::buttons::Button;
#[component]
pub fn TextPropertyValue(
    space_id: String,
    object_id: String,
    prop: Signal<ApimodelPeriodTextPropertyValue>,
) -> Element {
    let value = prop().text.unwrap();
    rsx! {
        Button { "{value}" }
    }
}
