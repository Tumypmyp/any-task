use crate::{components::button::*, helpers::*};
use dioxus::prelude::*;
#[component]
pub fn PropertyValue(
    // space_id: String,
    // object_id: String,
    data: ReadSignal<prost_types::Value>,
    // info: ReadSignal<(RelationInfo, PropertySettings)>,
) -> Element {
    let s = match data().kind {
        Some(prost_types::value::Kind::StringValue(s)) => s,
        _ => "".to_string(),
    };
    // let (p_info, settings) = info();
    rsx! {
        button { style: "width: 100%; min-width: 0; overflow: hidden; \
                                        text-overflow: ellipsis; white-space: nowrap; \
                                        box-sizing: border-box;",
            "{s}"
        }
    }
}
