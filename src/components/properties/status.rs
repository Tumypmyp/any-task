use crate::components::button::*;
use dioxus::prelude::*;

use crate::helpers::RELATION_OPTIONS;

#[component]
pub fn StatusValue(data: ReadSignal<prost_types::Value>) -> Element {
    let name = use_memo(move || {
        let id = match data().kind {
            Some(prost_types::value::Kind::StringValue(v)) => v,
            Some(prost_types::value::Kind::ListValue(list)) => {
                match list.values.into_iter().next().and_then(|v| v.kind) {
                    Some(prost_types::value::Kind::StringValue(v)) => v,
                    _ => return String::new(),
                }
            }
            _ => return String::new(),
        };
        RELATION_OPTIONS
            .read()
            .details
            .get(&id)
            .map(|opt| opt.name.clone())
            .unwrap_or_default()
    });
    tracing::debug!("relations: {:#?}", RELATION_OPTIONS.read());
    tracing::debug!("data: {:#?}, name: {:#?}", data, name);
    rsx! {
        Button { "{name}" }
    }
}
