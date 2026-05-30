use crate::components::button::Button;
use crate::helpers::*;
use dioxus::prelude::*;
use openapi::models::NumberPropertyValue;
impl PropertyRenderer for NumberPropertyValue {
    fn render(
        &self,
        space_id: String,
        object_id: String,
        _info: PropertyInfo,
        _settings: PropertySettings,
    ) -> Element {
        rsx! {
            NumberValue {
                space_id: &space_id,
                object_id: &object_id,
                prop: self
                                            .clone(),
            }
        }
    }
}
#[component]
pub fn NumberValue(
    space_id: String,
    object_id: String,
    prop: NumberPropertyValue,
) -> Element {
    let value = prop.number.unwrap_or_default();
    rsx! {
        Button { width: "100%", height: "100%", "{value}" }
    }
}
