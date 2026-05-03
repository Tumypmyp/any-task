use crate::API_CLIENT;
use crate::Search;
use crate::components::action::*;
use crate::components::header::{Header, Title};
use dioxus::prelude::*;
#[component]
pub fn Space(space_id: String) -> Element {
    tracing::info!("loading space {space_id}");
    rsx! {
        SpaceTitle { space_id: space_id.clone() }
        Collections { space_id: space_id.clone() }
        ActionHolder { BaseActions {} }
    }
}

#[component]
pub fn Collections(space_id: String) -> Element {
    rsx! {
        Search {
            space_id: space_id.clone(),
            types: vec!["set".to_string(), "collection".to_string()],
        }
    }
}
#[component]
pub fn SpaceTitle(space_id: String) -> Element {
    let resp = use_resource(move || {
        let client = API_CLIENT.read().clone();
        let space_id = space_id.clone();
        async move { client.get_space(space_id).await }
    });
    let Some(result) = &*resp.read() else {
        return rsx! { "Loading..." };
    };
    let name = match result {
        Ok(obj) => obj
            .space
            .clone()
            .unwrap_or_default()
            .name
            .unwrap_or_default(),
        Err(err) => {
            tracing::debug!("Got error loading the space: {:#?}", err);
            return rsx! { "Error: {err}" };
        }
    };
    rsx! {
        Header {
            Title { title: "{name}" }
        }
    }
}
