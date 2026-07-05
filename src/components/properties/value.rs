use crate::components::button::*;
use crate::components::date_picker::*;
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
    let selected_date = use_memo(move || match data().kind {
        Some(prost_types::value::Kind::NumberValue(v)) => {
            time::OffsetDateTime::from_unix_timestamp(v as i64)
                .ok()
                .map(|dt| dt.date())
        }
        _ => None,
    });
    match all_properties().get(&relation_key) {
        Some(info) if info.format == RelationFormat::Date => {
            return rsx! {
                DatePicker { selected_date, disabled: true }
            };
        }
        Some(info) if info.format == RelationFormat::Checkbox => {
            return rsx! {
                CheckboxValue { data }
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
