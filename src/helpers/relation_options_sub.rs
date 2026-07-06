use crate::protos::anytype_model::block::*;
use crate::protos::event::block::dataview::view_update::*;
use crate::protos::event::block::dataview::*;
use crate::protos::event::object::details::*;
use dioxus::prelude::*;
use std::collections::HashMap;

pub static RELATION_OPTIONS: GlobalSignal<RelationOptionsState> =
    Signal::global(RelationOptionsState::default);

#[derive(Default, Clone, Debug)]
pub struct RelationOptionsState {
    /// relation_key -> list of option ids (ordered)
    pub by_relation: HashMap<String, Vec<String>>,
    /// option_id -> option details
    pub details: HashMap<String, RelationOptionDetails>,
}

#[derive(Clone, Debug)]
pub struct RelationOptionDetails {
    pub id: String,
    pub name: String,
    pub relation_key: String,
    pub color: String,
}

impl RelationOptionsState {
    pub fn handle_set(&mut self, v: Set) {
        let fields = v.details.as_ref().map(|d| &d.fields);
        let relation_key = extract_string(fields.and_then(|f| f.get("relationKey")));
        let opt = RelationOptionDetails {
            id: v.id.clone(),
            name: extract_string(fields.and_then(|f| f.get("name"))),
            color: extract_string(fields.and_then(|f| f.get("relationOptionColor"))),
            relation_key: relation_key.clone(),
        };
        self.by_relation
            .entry(relation_key)
            .or_default()
            .push(v.id.clone());
        self.details.insert(v.id, opt);
    }
    pub fn handle_amend(&mut self, v: Amend) {
        if let Some(opt) = self.details.get_mut(&v.id) {
            for kv in v.details {
                match kv.key.as_str() {
                    "name" => opt.name = get_string(kv.value.unwrap_or_default()),
                    "relationOptionColor" => opt.color = get_string(kv.value.unwrap_or_default()),
                    _ => {}
                }
            }
        }
    }
}

fn get_string(v: prost_types::Value) -> String {
    match v.kind {
        Some(prost_types::value::Kind::StringValue(s)) => s,
        _ => String::new(),
    }
}
fn extract_string(val: Option<&prost_types::Value>) -> String {
    if let Some(prost_types::Value {
        kind: Some(prost_types::value::Kind::StringValue(s)),
    }) = val
    {
        s.clone()
    } else {
        String::new()
    }
}
