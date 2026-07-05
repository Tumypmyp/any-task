use crate::helpers::models::*;
use crate::protos::event::object::details::*;
use crate::protos::event::object::subscription::*;
use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Default)]
pub struct SpacesState {
    pub order: Vec<String>,
    pub details: HashMap<String, SpaceDetails>,
}

pub static SPACES: GlobalSignal<SpacesState> = Signal::global(SpacesState::default);
impl SpacesState {
    pub fn matches_sub_id(sub_id: &str) -> bool {
        sub_id == "spaces"
    }
    pub fn sub_id() -> String {
        "spaces".to_string()
    }
    pub fn handle_add(&mut self, v: Add) {
        insert_ordered(&mut self.order, v.id, &v.after_id);
    }
    pub fn handle_remove(&mut self, v: Remove) {
        self.order.retain(|id| id != &v.id);
        self.details.remove(&v.id);
    }
    pub fn handle_set(&mut self, v: Set) {
        let det = parse_space_details(&v.id, &v.details.unwrap_or_default().fields);
        self.details.insert(v.id, det);
    }
    pub fn handle_amend(&mut self, v: Amend) {
        let Some(det) = self.details.get_mut(&v.id) else {
            tracing::warn!("got amend, but space was not loaded");
            return;
        };
        for kv in v.details {
            match kv.key.as_str() {
                "name" => det.name = get_string(kv.value.unwrap()),
                "iconImage" => det.icon_image = get_string(kv.value.unwrap()),
                "description" => det.description = get_string(kv.value.unwrap()),
                "targetSpaceId" => det.target_space_id = get_string(kv.value.unwrap()),
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

pub fn parse_space_details(
    object_id: &str,
    fields: &std::collections::BTreeMap<String, prost_types::Value>,
) -> SpaceDetails {
    SpaceDetails {
        object_id: object_id.to_string(),
        target_space_id: extract_string(fields.get("targetSpaceId")),
        name: extract_string(fields.get("name")),
        icon_image: extract_string(fields.get("iconImage")),
        description: extract_string(fields.get("description")),
    }
}

fn get_string(v: prost_types::Value) -> String {
    match v.kind {
        Some(prost_types::value::Kind::StringValue(s)) => s,
        _ => String::new(),
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
