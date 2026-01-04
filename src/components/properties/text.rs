use crate::components::button::Button;
use crate::helpers::*;
use dioxus::prelude::*;
use openapi::models::TextPropertyValue;
impl PropertyRenderer for TextPropertyValue {
    fn render(
        &self,
        space_id: String,
        object_id: String,
        _info: PropertyInfo,
        _settings: PropertySettings,
    ) -> Element {
        rsx! {
            TextValue {
                space_id: &space_id,
                object_id: &object_id,
                prop: self.clone(),
            }
        }
    }
}
#[component]
pub fn TextValue(space_id: String, object_id: String, prop: TextPropertyValue) -> Element {
    let value = prop.text.unwrap_or_default();
    rsx! {
        Button { width: "100%", height: "100%", "{value}" }
    }
}
