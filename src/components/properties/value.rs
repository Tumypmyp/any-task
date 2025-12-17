use crate::{
    components::button::{Button, ButtonHolder},
    helpers::PropertyInfo,
    properties::*,
};
use dioxus::prelude::*;
use openapi::models::*;
#[component]
pub fn PropertyValue(
    space_id: String,
    object_id: String,
    data: ReadSignal<Option<ApimodelPropertyWithValue>>,
    info: ReadSignal<PropertyInfo>,
) -> Element {
    rsx!{
        div {
            style: "align-items: center; justify-content: center;",
            width: "{info().width}vw", height: "{info().height}vh",
            // background: "#555555",
            match data() {
                Some(ApimodelPropertyWithValue::Text(text)) => {
                    rsx! {
                        TextPropertyValue {
                            space_id: &space_id,
                            object_id: &object_id,
                            prop: use_signal(|| *text),
                            info,
                        }
                    }
                }
                Some(ApimodelPropertyWithValue::Checkbox(checkbox)) => {
                    rsx! {
                        CheckboxPropertyValue {
                            space_id: &space_id,
                            object_id: &object_id,
                            prop: use_signal(|| *checkbox),
                            info,
                        }
                    }
                }
                Some(ApimodelPropertyWithValue::Select(select)) => {
                    rsx! {
                        SelectPropertyValue {
                            space_id: &space_id,
                            object_id: &object_id,
                            prop: use_signal(|| *select),
                            info,
                        }
                    }
                }
                Some(ApimodelPropertyWithValue::Date(date)) => {
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
                        Button { width: "{info().width}vw", " " }
                    }
                }
            }
        }
    }
}
