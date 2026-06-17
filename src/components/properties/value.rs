use crate::components::checkbox::*;
use crate::protos::anytype_model::RelationFormat;
use crate::{
    components::button::*,
    helpers::*, //protos::anytype_model::block::content::text::Style::Checkbox,
};
use dioxus::prelude::*;
#[component]
pub fn PropertyValue(
    relation_key: RelationKey,
    data: ReadSignal<prost_types::Value>,
    all_properties: ReadSignal<std::collections::HashMap<RelationKey, RelationInfo>>,
) -> Element {
    let checked = use_memo(move || match data().kind {
        Some(prost_types::value::Kind::BoolValue(true)) => "checked".to_string(),
        _ => "unchecked".to_string(),
    });
    match all_properties.get(&relation_key) {
        Some(info) if info.format == RelationFormat::Date => return rsx! { "date" },
        _ => {}
    }
    let s = match data().kind {
        Some(prost_types::value::Kind::StringValue(s)) => s,
        Some(prost_types::value::Kind::NumberValue(n)) => n.to_string(),
        Some(prost_types::value::Kind::BoolValue(v)) => {
            return rsx! {
                Checkbox { disabled: true, value: checked }
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
