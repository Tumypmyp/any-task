use crate::components::checkbox::Checkbox;
use dioxus::prelude::*;
use dioxus_primitives::checkbox::CheckboxState;

#[component]
pub fn CheckboxValue(data: ReadSignal<prost_types::Value>) -> Element {
    let checked = use_memo(move || match data().kind {
        Some(prost_types::value::Kind::BoolValue(v)) => Some(if v {
            CheckboxState::Checked
        } else {
            CheckboxState::Unchecked
        }),
        _ => None,
    });
    rsx! {
        Checkbox { disabled: true, checked }
    }
}
