use dioxus_history::History;
use dioxus_sdk_storage::{LocalStorage, StorageBacking};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
const HISTORY_KEY: &str = "dioxus_history_state";
#[derive(Serialize, Deserialize, Clone, Debug)]
struct PersistentState {
    current: String,
    history: Vec<String>,
    future: Vec<String>,
}
pub struct PersistentHistory {
    state: RefCell<PersistentState>,
    base_path: Option<String>,
}
impl Default for PersistentHistory {
    fn default() -> Self {
        Self::with_initial_path("/")
    }
}
impl PersistentHistory {
    pub fn with_initial_path(path: impl ToString) -> Self {
        let state =
            <LocalStorage as StorageBacking>::get::<PersistentState>(&HISTORY_KEY.to_string())
                .unwrap_or_else(|| PersistentState {
                    current: path.to_string(),
                    history: Vec::new(),
                    future: Vec::new(),
                });
        Self {
            state: RefCell::new(state),
            base_path: None,
        }
    }
    fn sync_to_disk(&self) {
        let state = self.state.borrow();
        let _ = LocalStorage::set(HISTORY_KEY.to_string(), &*state);
    }
    pub fn with_prefix(mut self, prefix: impl ToString) -> Self {
        self.base_path = Some(prefix.to_string());
        self
    }
}
impl History for PersistentHistory {
    fn current_prefix(&self) -> Option<String> {
        self.base_path.clone()
    }
    fn current_route(&self) -> String {
        self.state.borrow().current.clone()
    }
    fn can_go_back(&self) -> bool {
        !self.state.borrow().history.is_empty()
    }
    fn go_back(&self) {
        let mut write = self.state.borrow_mut();
        if let Some(last) = write.history.pop() {
            let old = std::mem::replace(&mut write.current, last);
            write.future.push(old);
        }
        drop(write);
        self.sync_to_disk();
    }
    fn can_go_forward(&self) -> bool {
        !self.state.borrow().future.is_empty()
    }
    fn go_forward(&self) {
        let mut write = self.state.borrow_mut();
        if let Some(next) = write.future.pop() {
            let old = std::mem::replace(&mut write.current, next);
            write.history.push(old);
        }
        drop(write);
        self.sync_to_disk();
    }
    fn push(&self, new: String) {
        let mut write = self.state.borrow_mut();
        if write.current == new {
            return;
        }
        let old = std::mem::replace(&mut write.current, new);
        write.history.push(old);
        write.future.clear();
        drop(write);
        self.sync_to_disk();
    }
    fn replace(&self, path: String) {
        let mut write = self.state.borrow_mut();
        write.current = path;
        drop(write);
        self.sync_to_disk();
    }
    fn external(&self, url: String) -> bool {
        url.starts_with("http://") || url.starts_with("https://")
    }
    fn updater(&self, _callback: std::sync::Arc<dyn Fn() + Send + Sync>) {}
    fn include_prevent_default(&self) -> bool {
        false
    }
}
