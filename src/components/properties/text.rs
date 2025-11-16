use crate::components::button::ButtonWithHolder;
use crate::helpers::*;
use dioxus::prelude::*;
use openapi::models::ApimodelPeriodTextPropertyValue;
#[component]
pub fn TextPropertyValue(
    space_id: String,
    object_id: String,
    prop: Signal<ApimodelPeriodTextPropertyValue>,
    info: ReadSignal<PropertyInfo>,
) -> Element {
    let value = prop().text.unwrap();
    rsx! {
        ButtonWithHolder { width: "{info().width}vw", "{value}" }
    }
}
