use crate::{
    components::button::{ButtonHolder, ButtonWithHolder},
    helpers::PropertyInfo,
    properties::*,
};
use dioxus::prelude::*;
use openapi::models::*;
#[component]
pub fn PropertyValue(
    space_id: String,
    object_id: String,
    data: ReadSignal<Option<ApimodelPeriodPropertyWithValue>>,
    info: ReadSignal<PropertyInfo>,
) -> Element {
    match data() {
        Some(ApimodelPeriodPropertyWithValue::ApimodelPeriodTextPropertyValue(text)) => {
            rsx! {
                TextPropertyValue {
                    space_id: &space_id,
                    object_id: &object_id,
                    prop: use_signal(|| *text),
                    info,
                }
            }
        }
        Some(ApimodelPeriodPropertyWithValue::ApimodelPeriodCheckboxPropertyValue(checkbox)) => {
            rsx! {
                CheckboxPropertyValue {
                    space_id: &space_id,
                    object_id: &object_id,
                    prop: use_signal(|| *checkbox),
                    info,
                }
            }
        }
        Some(ApimodelPeriodPropertyWithValue::ApimodelPeriodSelectPropertyValue(select)) => {
            rsx! {
                SelectPropertyValue {
                    space_id: &space_id,
                    object_id: &object_id,
                    prop: use_signal(|| *select),
                    info,
                }
            }
        }
        Some(ApimodelPeriodPropertyWithValue::ApimodelPeriodDatePropertyValue(date)) => {
            rsx! {
                ButtonHolder {
                    DateTimePropertyValues {
                        space_id: &space_id,
                        object_id: &object_id,
                        prop: use_signal(|| *date),
                        info,
                    }
                }
            }
        }
        _ => {
            rsx! {
                ButtonWithHolder { width: "{info().width}vw", " " }
            }
        }
    }
}
