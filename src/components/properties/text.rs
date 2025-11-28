use crate::components::button::ButtonWithHolder;
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
        ButtonWithHolder { width: "{info().width}vw", "{value}" }
    }
}
