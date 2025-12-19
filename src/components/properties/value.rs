use crate::{
    components::button::*,
    helpers::*,
    properties::*,
};
use dioxus::prelude::*;
use openapi::models::*;
#[component]
pub fn PropertyValue(
    space_id: String,
    object_id: String,
    data: ReadSignal<Option<ApimodelPropertyWithValue>>,
    // info: ReadSignal<PropertyInfo>,
    info: ReadSignal<(PropertyInfo, PropertySettings)>,
) -> Element {
    rsx!{
        div {
            style: "display: flex; align-items: center; justify-content: center;",
            width: "{info().1.width}vw",
            height: "{info().1.height}vh",
            // background: "#444555",
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
                        DateTimePropertyValues {
                            space_id: &space_id,
                            object_id: &object_id,
                            prop: use_signal(|| *date),
                            info,
                        }
                    }
                }
                _ => {
                    rsx! {
                        Button { variant: ButtonVariant::Ghost, width: "{info().1.width}vw", " " }
                    }
                }
            }
        }
    }
}
