use crate::components::checkbox::*;
use crate::{
    components::button::*,
    helpers::*, //protos::anytype_model::block::content::text::Style::Checkbox,
};
use dioxus::prelude::*;
use dioxus_primitives::checkbox;
#[component]
pub fn PropertyValue(
    // space_id: String,
    // object_id: String,
    data: ReadSignal<prost_types::Value>,
    // info: ReadSignal<(RelationInfo, PropertySettings)>,
) -> Element {
    let s = match data().kind {
        Some(prost_types::value::Kind::StringValue(s)) => s,
        Some(prost_types::value::Kind::NumberValue(n)) => n.to_string(),
        Some(prost_types::value::Kind::BoolValue(v)) => {
            return rsx! {
                Checkbox { default_checked: if v { checkbox::CheckboxState::Checked } else { checkbox::CheckboxState::Unchecked } }
            };
        }
        Some(prost_types::value::Kind::StructValue(v)) => format!("{:#?}", v),
        Some(prost_types::value::Kind::ListValue(v)) => format!("{:#?}", v),
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
