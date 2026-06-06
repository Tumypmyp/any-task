use crate::{components::button::*, helpers::*};
use dioxus::prelude::*;
#[component]
pub fn PropertyValue(
    // space_id: String,
    // object_id: String,
    data: ReadSignal<prost_types::Value>,
    // info: ReadSignal<(RelationInfo, PropertySettings)>,
) -> Element {
    // let (p_info, settings) = info();
    rsx! {
        div { style: "display: flex; align-items: center; justify-content: center;",
            // width: "{info().1.width()}px",
            // height: "{info().1.height()}px",
            {
                match data().kind {
                    Some(prost_types::value::Kind::StringValue(s)) => rsx! {
                        Button { "{s}" }
                    },
                    _ => rsx! {
                        Button { variant: ButtonVariant::Ghost, " " }
                    },
                }
            }
        }
    }
}
