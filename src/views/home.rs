use crate::API_CLIENT;
use crate::Logout;
use crate::Route;
use crate::components::action::*;
use crate::components::base::message;
use crate::components::button::Button;
use crate::components::button::ButtonVariant;
use crate::components::column::Column;
use crate::components::header::{Header, Title};
use dioxus::prelude::*;
use openapi::models::Space;
#[component]
pub fn Home() -> Element {
    let resp = use_resource(move || {
        let client = API_CLIENT.read().clone();
        async move { client.list_spaces().await }
    });
    let resp_value = resp.read();
    let spaces = match resp_value.as_ref() {
        Some(Ok(objs)) => objs,
        Some(Err(err)) => {
            tracing::debug!("error: {:#?}", err);
            message::error("Failed to load spaces, retry later", err);
            return rsx! {
                ActionHolder { position: Position::Left, Logout {} }
            };
        }
        None => {
            return rsx! { "Loading..." };
        }
    };
    tracing::info!("Opened home");
    rsx! {
        Header {
            Title { title: "Spaces" }
        }
        Column {
            for space in spaces.data.clone().unwrap_or_default() {
                SpaceButton { space }
            }
        }
        ActionHolder { position: Position::Left, Logout {} }
    }
}
#[component]
fn SpaceButton(space: Space) -> Element {
    let nav = navigator();
    let space_id = space.id.unwrap_or_default();
    let space_name = space.name.unwrap_or_default();
    rsx! {
        Button {
            id: "{space_id}",
            width: "90vw",
            height: "8vh",
            variant: ButtonVariant::Primary,
            style: "font-size: 1.1rem;",
            onclick: move |_| {
                nav.push(Route::Space {
                    space_id: space_id.clone(),
                });
            },
            "{space_name}"
        }
    }
}
