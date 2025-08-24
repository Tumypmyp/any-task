use dioxus::prelude::*;
use openapi::models::*;
use crate::property::*;

#[component]
pub fn PropertyValue(
    space_id: String,
    object_id: String,
    data: Option<ApimodelPeriodPropertyWithValue>,
    options: Option<Vec<ApimodelPeriodTag>>,
) -> Element {
    match data {
        Some(ApimodelPeriodPropertyWithValue::ApimodelPeriodTextPropertyValue(text)) => {
            rsx! {
                TextPropertyValue {
                    space_id: &space_id,
                    object_id: &object_id,
                    prop: use_signal(|| *text),
                }
            }
        }
        Some(
            ApimodelPeriodPropertyWithValue::ApimodelPeriodCheckboxPropertyValue(
                checkbox,
            ),
        ) => {
            rsx! {
                CheckboxPropertyValue {
                    space_id: &space_id,
                    object_id: &object_id,
                    prop: use_signal(|| *checkbox),
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
                    options: options.unwrap_or_default(),
                }
            }
        }
        Some(ApimodelPeriodPropertyWithValue::ApimodelPeriodDatePropertyValue(date)) => {
            rsx! {
                DatePropertyValue {
                    space_id: &space_id,
                    object_id: &object_id,
                    prop: use_signal(|| *date),
                }
            }
        }
        _ => rsx!(),
    }
}
