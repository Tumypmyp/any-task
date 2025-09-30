use dioxus::prelude::*;
use openapi::models::ApimodelPeriodTextPropertyValue;
use crate::components::base::ButtonWithHolder;
#[component]
pub fn TextPropertyValue(
    space_id: String,
    object_id: String,
    prop: Signal<ApimodelPeriodTextPropertyValue>,
) -> Element {
    let value = prop().text.unwrap();
    rsx! {
        ButtonWithHolder { width: "15vw", "{value}" }
    }
}
