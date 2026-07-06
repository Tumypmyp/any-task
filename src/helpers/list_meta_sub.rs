use crate::protos::anytype_model::block::*;

use crate::protos::event::block::dataview::view_update::*;
use crate::protos::event::block::dataview::*;
use crate::protos::event::object::details::*;
use dioxus::prelude::*;
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SetMetaState {
    pub id: String,
    pub name: String,
    pub set_of: Vec<String>,
    pub views: Vec<content::dataview::View>,
    pub active_view_id: String,
}

pub static SET_META: GlobalSignal<SetMetaState> = Signal::global(SetMetaState::default);

impl SetMetaState {
    pub fn handle_set(&mut self, v: Set) {
        let Some(fields) = v.details.map(|d| d.fields) else {
            return;
        };
        self.name = extract_string(fields.get("name"));
        self.set_of = extract_list_strings(fields.get("setOf"));
    }
    pub fn handle_amend(&mut self, v: Amend) {
        for kv in v.details {
            match kv.key.as_str() {
                "name" => self.name = get_string(kv.value.unwrap_or_default()),
                "setOf" => self.set_of = extract_list_strings(kv.value.as_ref()),
                _ => {}
            }
        }
    }
    pub fn handle_view_set(&mut self, v: ViewSet) {
        let Some(new_view) = v.view else { return };

        if let Some(existing) = self.views.iter_mut().find(|view| view.id == v.view_id) {
            *existing = new_view;
        } else {
            self.views.push(new_view);
        }
    }
    pub fn handle_view_delete(&mut self, v: ViewDelete) {
        self.views.retain(|view| view.id != v.view_id);
        if self.active_view_id == v.view_id {
            self.active_view_id = self
                .views
                .first()
                .map(|view| view.id.clone())
                .unwrap_or_default();
        }
    }

    pub fn handle_view_update(&mut self, v: ViewUpdate) {
        let Some(view) = self.views.iter_mut().find(|view| view.id == v.view_id) else {
            tracing::error!("got update on nonexisting view: {}", v.view_id);
            return;
        };

        if let Some(f) = v.fields {
            view.name = f.name;
        }

        // --- Filters ---
        for change in v.filter {
            match change.operation {
                Some(filter::Operation::Add(add)) => {
                    let pos =
                        insert_pos_by_id(&add.after_id, view.filters.iter().map(|f| f.id.as_str()));
                    for (i, item) in add.items.into_iter().enumerate() {
                        view.filters.insert(pos + i, item);
                    }
                }
                Some(filter::Operation::Remove(rem)) => {
                    view.filters.retain(|f| !rem.ids.contains(&f.id));
                }
                Some(filter::Operation::Update(u)) => {
                    if let Some(f) = view.filters.iter_mut().find(|f| f.id == u.id) {
                        if let Some(item) = u.item {
                            *f = item;
                        }
                    }
                }
                Some(filter::Operation::Move(mv)) => {
                    let mut moved = Vec::with_capacity(mv.ids.len());

                    for target_id in &mv.ids {
                        if let Some(idx) = view.filters.iter().position(|s| s.id == *target_id) {
                            moved.push(view.filters.remove(idx));
                        }
                    }

                    let pos =
                        insert_pos_by_id(&mv.after_id, view.filters.iter().map(|s| s.id.as_str()));

                    for (i, item) in moved.into_iter().enumerate() {
                        view.filters.insert(pos + i, item);
                    }
                }
                None => {}
            }
        }

        // --- Sorts ---
        for change in v.sort {
            match change.operation {
                Some(sort::Operation::Add(add)) => {
                    let pos =
                        insert_pos_by_id(&add.after_id, view.sorts.iter().map(|s| s.id.as_str()));
                    for (i, item) in add.items.into_iter().enumerate() {
                        view.sorts.insert(pos + i, item);
                    }
                }
                Some(sort::Operation::Remove(rem)) => {
                    view.sorts.retain(|s| !rem.ids.contains(&s.id));
                }
                Some(sort::Operation::Update(u)) => {
                    if let Some(s) = view.sorts.iter_mut().find(|s| s.id == u.id) {
                        if let Some(item) = u.item {
                            *s = item;
                        }
                    }
                }
                Some(sort::Operation::Move(mv)) => {
                    let mut moved = Vec::with_capacity(mv.ids.len());

                    for target_id in &mv.ids {
                        if let Some(idx) = view.sorts.iter().position(|s| s.id == *target_id) {
                            moved.push(view.sorts.remove(idx));
                        }
                    }

                    let pos =
                        insert_pos_by_id(&mv.after_id, view.sorts.iter().map(|s| s.id.as_str()));

                    for (i, item) in moved.into_iter().enumerate() {
                        view.sorts.insert(pos + i, item);
                    }
                }
                None => {}
            }
        }
    }
    pub fn handle_view_order(&mut self, v: ViewOrder) {
        self.views.sort_by_cached_key(|view| {
            v.view_ids
                .iter()
                .position(|id| id == &view.id)
                .unwrap_or(usize::MAX)
        });
    }
}

fn get_string(v: prost_types::Value) -> String {
    match v.kind {
        Some(prost_types::value::Kind::StringValue(s)) => s,
        _ => String::new(),
    }
}

/// Finds the insertion index for a new item.
/// If `after_id` is empty, or if the ID is not found, it defaults to index 0.
fn insert_pos_by_id<'a>(after_id: &str, mut ids: impl Iterator<Item = &'a str>) -> usize {
    if after_id.is_empty() {
        return 0;
    }
    ids.position(|id| id == after_id)
        .map(|i| i + 1)
        .unwrap_or(0)
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
fn extract_list_strings(v: Option<&prost_types::Value>) -> Vec<String> {
    v.and_then(|v| {
        if let Some(prost_types::value::Kind::ListValue(lv)) = &v.kind {
            Some(
                lv.values
                    .iter()
                    .filter_map(|v| {
                        if let Some(prost_types::value::Kind::StringValue(s)) = &v.kind {
                            Some(s.clone())
                        } else {
                            None
                        }
                    })
                    .collect(),
            )
        } else {
            None
        }
    })
    .unwrap_or_default()
}
