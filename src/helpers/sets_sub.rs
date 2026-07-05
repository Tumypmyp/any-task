use crate::helpers::models::*;
use crate::protos::event::object::details::*;
use crate::protos::event::object::subscription::*;
use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Default)]
pub struct SetsState {
    pub order: Vec<String>,
    pub details: HashMap<String, SetDetails>,
}

pub static SETS: GlobalSignal<SetsState> = Signal::global(SetsState::default);

impl SetsState {
    pub fn matches_sub_id(sub_id: &str) -> bool {
        sub_id.starts_with("sets-sub-")
    }
    pub fn sub_id(space_id: &str) -> String {
        format!("sets-sub-{}", space_id)
    }
    pub fn handle_add(&mut self, v: Add) {
        insert_ordered(&mut self.order, v.id, &v.after_id);
    }
    pub fn handle_remove(&mut self, v: Remove) {
        self.order.retain(|id| id != &v.id);
        self.details.remove(&v.id);
    }
    pub fn handle_set(&mut self, v: Set) {
        let det = SetDetails {
            object_id: v.id.clone(),
            name: extract_string(v.details.as_ref().and_then(|d| d.fields.get("name"))),
            layout: extract_number(
                v.details
                    .as_ref()
                    .and_then(|d| d.fields.get("resolvedLayout")),
            ),
        };
        self.details.insert(v.id, det);
    }
    pub fn handle_amend(&mut self, v: Amend) {
        let Some(details) = self.details.get_mut(&v.id) else {
            tracing::warn!("got amend, but set was not loaded");
            return;
        };
        for kv in v.details {
            match kv.key.as_str() {
                "name" => details.name = get_string(kv.value.unwrap()),
                "resolvedLayout" => details.layout = extract_number((&kv.value).into()),
                _ => {}
            }
        }
    }
}

fn insert_ordered(order: &mut Vec<String>, id: String, after_id: &str) {
    order.retain(|existing| existing != &id);
    if after_id.is_empty() {
        order.insert(0, id);
    } else {
        let pos = order
            .iter()
            .position(|existing| existing == after_id)
            .map(|i| i + 1)
            .unwrap_or(order.len());
        order.insert(pos, id);
    }
}

pub fn extract_string(val: Option<&prost_types::Value>) -> String {
    if let Some(prost_types::Value {
        kind: Some(prost_types::value::Kind::StringValue(s)),
    }) = val
    {
        s.clone()
    } else {
        String::new()
    }
}

pub fn extract_number(val: Option<&prost_types::Value>) -> i32 {
    if let Some(prost_types::Value {
        kind: Some(prost_types::value::Kind::NumberValue(n)),
    }) = val
    {
        *n as i32
    } else {
        0
    }
}

fn get_string(v: prost_types::Value) -> String {
    match v.kind {
        Some(prost_types::value::Kind::StringValue(s)) => s,
        _ => String::new(),
    }
}
