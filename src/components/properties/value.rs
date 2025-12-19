use crate::{components::button::*, helpers::*};
use dioxus::prelude::*;
use openapi::models::*;
#[component]
pub fn PropertyValue(
    space_id: String,
    object_id: String,
    data: ReadSignal<Option<ApimodelPropertyWithValue>>,
    info: ReadSignal<(PropertyInfo, PropertySettings)>,
) -> Element {
    let (p_info, settings) = info();
    rsx! {
        div {
            style: "display: flex; align-items: center; justify-content: center;",
            width: "{info().1.width}vw",
            height: "{info().1.height}vh",
            // background: "#444555",
            match data() {
                Some(ApimodelPropertyWithValue::Text(v)) => {
                    v.render(space_id, object_id, p_info, settings)
                }
                Some(ApimodelPropertyWithValue::Checkbox(v)) => {
                    v.render(space_id, object_id, p_info, settings)
                }
                Some(ApimodelPropertyWithValue::Select(v)) => {
                    v.render(space_id, object_id, p_info, settings)
                }
                Some(ApimodelPropertyWithValue::Date(v)) => {
                    v.render(space_id, object_id, p_info, settings)
                }
                _ => rsx! {
                    Button { variant: ButtonVariant::Ghost, " " }
                },
            }
        }
    }
}
