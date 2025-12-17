use crate::components::button::Button;
use crate::helpers::*;
use dioxus::prelude::*;
use openapi::models::ApimodelTextPropertyValue;
#[component]
pub fn TextPropertyValue(
    space_id: String,
    object_id: String,
    prop: Signal<ApimodelTextPropertyValue>,
    info: ReadSignal<PropertyInfo>,
) -> Element {
    let value = prop().text.unwrap_or_default();
    rsx! {
        Button { width: "100%", height: "100%", "{value}" }
    }
}
