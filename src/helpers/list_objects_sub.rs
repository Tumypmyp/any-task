use crate::helpers::models::*;
use crate::protos::event::object::details::*;
use crate::protos::event::object::subscription::*;
use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Default)]
pub struct ListObjectsState {
    pub order: Vec<String>,
    pub details: HashMap<String, ObjectDetails>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ObjectDetails {
    pub id: String,
    pub name: String,
    pub fields: std::collections::BTreeMap<String, prost_types::Value>,
}

const LIST_SUB_PREFIX: &str = "list";
pub static LIST_OBJECTS: GlobalSignal<ListObjectsState> = Signal::global(ListObjectsState::default);

impl ListObjectsState {
    pub fn matches_sub_id(sub_id: &str) -> bool {
        sub_id.starts_with(LIST_SUB_PREFIX)
    }
    pub fn sub_id(list_id: &str) -> String {
        format!("{LIST_SUB_PREFIX}-{}", list_id)
    }
    pub fn handle_add(&mut self, v: Add) {
        insert_ordered(&mut self.order, v.id, &v.after_id);
    }
    pub fn handle_remove(&mut self, v: Remove) {
        self.order.retain(|id| id != &v.id);
        self.details.remove(&v.id);
    }
    pub fn handle_set(&mut self, v: Set) {
        let fields: std::collections::BTreeMap<String, prost_types::Value> = v
            .details
            .as_ref()
            .map(|d| d.fields.clone().into_iter().collect())
            .unwrap_or_default();
        let det = ObjectDetails {
            id: v.id.clone(),
            name: extract_string(fields.get("name")),
            fields,
        };
        self.details.insert(v.id, det);
    }
    pub fn handle_amend(&mut self, v: Amend) {
        if let Some(det) = self.details.get_mut(&v.id) {
            for kv in v.details {
                let val = kv.value.unwrap_or_default();
                match kv.key.as_str() {
                    "name" => det.name = get_string(val.clone()),
                    _ => {}
                }
                det.fields.insert(kv.key, val);
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

fn extract_number(val: Option<&prost_types::Value>) -> i32 {
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
