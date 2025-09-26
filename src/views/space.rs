use dioxus::prelude::*;
use crate::components::Title;
use crate::API_CLIENT;
use crate::Actions;
use crate::Search;
#[component]
pub fn Space(space_id: String) -> Element {
    tracing::info!("loading space {space_id}");
    let space_id = use_signal(|| space_id.clone());
    rsx! {
        SpaceTitle { space_id }
        Search { space_id, types: vec!["set".to_string()] }
        Search { space_id, types: vec!["collection".to_string()] }
        Actions {}
    }
}
#[component]
pub fn SpaceTitle(space_id: Signal<String>) -> Element {
    let mut name = use_signal(|| "".to_string());
    let resp = use_resource(move || async move {
        API_CLIENT.read().get_space(space_id).await
    });
    match &*resp.read() {
        Some(Ok(p)) => {
            name.set(p.clone().space.unwrap_or_default().name.unwrap_or_default());
        }
        _ => {}
    }
    rsx! {
        Title { title: "{name}" }
    }
}
