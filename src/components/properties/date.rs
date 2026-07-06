use crate::components::date_picker::*;
use dioxus::prelude::*;

#[component]
pub fn DateValue(data: ReadSignal<prost_types::Value>) -> Element {
    let selected_date = use_memo(move || match data().kind {
        Some(prost_types::value::Kind::NumberValue(v)) => {
            time::OffsetDateTime::from_unix_timestamp(v as i64)
                .ok()
                .map(|dt| dt.date())
        }
        _ => None,
    });
    rsx! {
        DatePicker { selected_date, disabled: true }
    }
}
