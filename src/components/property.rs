use dioxus::prelude::*;
use openapi::models::*;
use crate::components::select_property::SelectPropertyValue;
use crate::components::checkbox_property::CheckboxPropertyValue;
#[component]
pub fn PropertyValue(
    space_id: String,
    object_id: String,
    data: Option<ApimodelPeriodPropertyWithValue>,
) -> Element {
    match data {
        Some(
            ApimodelPeriodPropertyWithValue::ApimodelPeriodCheckboxPropertyValue(
                checkbox,
            ),
        ) => {
            rsx! {
                CheckboxPropertyValue {
                    space_id: &space_id,
                    object_id: &object_id,
                    prop: use_signal(|| checkbox.checkbox.unwrap_or_default()),
                }
            }
        }
        Some(
            ApimodelPeriodPropertyWithValue::ApimodelPeriodSelectPropertyValue(select),
        ) => {
            rsx! {
                SelectPropertyValue {
                    space_id: &space_id,
                    object_id: &object_id,
                    prop: use_signal(|| *select),
                }
            }
        }
        _ => rsx!(),
    }
}
