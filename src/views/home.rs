use crate::API_CLIENT;
// use crate::Logout;
use crate::Route;
// use crate::components::action::*;
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
        // ActionHolder { position: Position::Left, Logout {} }
    }
}
#[component]
fn Spaces() -> Element {
    let resp = use_resource(move || async move { API_CLIENT().fetch_spaces().await });
    let Some(result) = &*resp.read() else {
        return rsx! { "Loading..." };
    };
    let spaces = match result {
        Ok(objs) => objs.clone(),
        Err(err) => {
            tracing::debug!("Got error loading spaces: {:#?}", err);
            return rsx! { "Error: {err}" };
        }
    };
    rsx! {
        Column {
            for space in spaces {
                SpaceButton { id: space }
            }
        }
    }
}

#[component]
fn SpaceButton(id: String) -> Element {
    // let nav = navigator();
    rsx! {
        Button {
            id: "{id}",
            width: "90vw",
            height: "8vh",
            variant: ButtonVariant::Primary,
            style: "font-size: 1.1rem;",
            onclick: move |_| {
            //     nav.push(Route::Space {
            //         space_id: id.clone(),
            //     });
            },
            "{id}"
        }
    }
}
