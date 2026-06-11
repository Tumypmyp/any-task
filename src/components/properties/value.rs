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
        Button {
            style: "height: 100%; min-height: 0; \
                    width: 100%; min-width: 0; \
                    white-space: normal;
                    overflow: visible;
                    text-overflow: ellipsis;  \
                    box-sizing: border-box;\
                    ",
            variant: ButtonVariant::Outline,
            "{s}"
        }
    }
}
