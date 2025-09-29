use dioxus::prelude::*;
use openapi::models::*;
use crate::properties::*;
#[component]
pub fn PropertyValue(
    space_id: String,
    object_id: String,
    data: ReadSignal<Option<ApimodelPeriodPropertyWithValue>>,
    options: ReadSignal<Vec<ApimodelPeriodTag>>,
) -> Element {
    match data() {
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
                    options: options(),
                }
            }
        }
        Some(ApimodelPeriodPropertyWithValue::ApimodelPeriodDatePropertyValue(date)) => {
            rsx! {
                DateTimePropertyValues {
                    space_id: &space_id,
                    object_id: &object_id,
                    prop: use_signal(|| *date),
                }
            }
        }
        _ => rsx!(),
    }
}
