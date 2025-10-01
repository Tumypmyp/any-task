use dioxus::prelude::*;
use openapi::models::ApimodelPeriodTextPropertyValue;
use crate::components::base::ButtonWithHolder;
use crate::helpers::*;
#[component]
pub fn TextPropertyValue(
    space_id: String,
    object_id: String,
    prop: Signal<ApimodelPeriodTextPropertyValue>,
    info: ReadSignal<PropertyViewInfo>,
) -> Element {
    let value = prop().text.unwrap();
    rsx! {
        ButtonWithHolder { width: "{info().width}vw", "{value}" }
    }
}
