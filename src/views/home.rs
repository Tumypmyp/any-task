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
#[component]
pub fn Home() -> Element {
    rsx! {
        Header {
            Title { title: "Spaces" }
        }
        Spaces {}
        ActionHolder { position: Position::Left, Logout {} }
    }
}
#[component]
fn Spaces() -> Element {
    let resp = use_resource(move || {
        let client = API_CLIENT.read().clone();
        async move { client.list_spaces().await }
    });
    let Some(result) = &*resp.read() else {
        return rsx! { "Loading..." };
    };
    let spaces = match result {
        Ok(objs) => objs.data.clone().unwrap_or_default(),
        Err(err) => {
            tracing::debug!("Got error loading spaces: {:#?}", err);
            return rsx! { "Error: {err}" };
        }
    };
    rsx! {
        Column {
            for space in spaces {
                SpaceButton {
                    id: space.id.unwrap_or_default(),
                    name: space.name.unwrap_or_default(),
                }
            }
        }

    }
}
#[component]
fn SpaceButton(id: String, name: String) -> Element {
    let nav = navigator();
    rsx! {
        Button {
            id: "{id}",
            width: "90vw",
            height: "8vh",
            variant: ButtonVariant::Primary,
            style: "font-size: 1.1rem;",
            onclick: move |_| {
                nav.push(Route::Space {
                    space_id: id.clone(),
                });
            },
            "{name}"
        }
    }
}
