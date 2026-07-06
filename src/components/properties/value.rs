use crate::components::button::*;
use crate::components::properties::*;
use crate::helpers::*;
use crate::protos::anytype_model::RelationFormat;
use dioxus::prelude::*;

#[component]
pub fn PropertyValue(
    relation_key: RelationKey,
    data: ReadSignal<prost_types::Value>,
    all_properties: ReadSignal<std::collections::HashMap<RelationKey, RelationInfo>>,
) -> Element {
    match all_properties().get(&relation_key) {
        Some(info) if info.format == RelationFormat::Date => {
            return rsx! {
                DateValue { data }
            };
        }
        Some(info) if info.format == RelationFormat::Checkbox => {
            return rsx! {
                CheckboxValue { data }
            };
        }
        Some(info) if info.format == RelationFormat::Status => {
            return rsx! {
                StatusValue { data }
            };
        }
        _ => {}
    }

    let s = match data().kind {
        Some(prost_types::value::Kind::StringValue(s)) => s,
        Some(prost_types::value::Kind::NumberValue(n)) => n.to_string(),
        Some(prost_types::value::Kind::StructValue(v)) => format!("{:#?}", v),
        Some(prost_types::value::Kind::ListValue(v)) => format!("{:#?}", v),
        _ => "".to_string(),
    };
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
