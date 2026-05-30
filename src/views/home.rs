use crate::API_CLIENT;
use crate::Logout;
use crate::components::action::*;
use crate::components::button::Button;
use crate::components::button::ButtonHolder;
use crate::components::button::ButtonVariant;
use crate::components::column::Column;
use crate::components::header::{Header, Title};
use crate::components::input::*;
use dioxus::prelude::*;
#[component]
pub fn Home() -> Element {
    rsx! {
        Header {
            Title { title: "Spaces" }
        }
        Spaces {}
        JoinSpace {}
        ActionHolder { position: Position::Left, Logout {} }
    }
}
#[component]
fn Spaces() -> Element {
    let resp = use_resource(move || async move {
        match API_CLIENT.read().as_ref() {
            Some(client) => client.fetch_spaces().await,
            None => Err(anyhow::anyhow!("No API client available")),
        }
    });
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
                SpaceButton { id: space.0, name: space.1 }
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
            onclick: move |_| {},
            "{name}"
        }
    }
}
#[component]
pub fn JoinSpace() -> Element {
    let mut invite_url = use_signal(String::new);
    let mut status_message = use_signal(String::new);
    let mut is_loading = use_signal(|| false);
    let on_join = move |_| {
        if invite_url.read().is_empty() {
            return;
        }
        spawn(async move {
            is_loading.set(true);
            status_message.set("Verifying invite...".to_string());
            let url = invite_url.read().clone();
            match API_CLIENT.read().as_ref().unwrap().join_space_from_link(&url).await {
                Ok(space_name) => {
                    status_message
                        .set(
                            format!(
                                "Success! Request sent to join '{}'. Waiting for owner approval.",
                                space_name,
                            ),
                        );
                    invite_url.set(String::new());
                }
                Err(e) => {
                    status_message.set(format!("Error: {}", e));
                }
            }
            is_loading.set(false);
        });
    };
    rsx! {
        Column { style: "padding: 20px; gap: 10px;",
            Button { "Join a Space" }
            ButtonHolder {
                Input {
                    r#type: "text",
                    value: "{invite_url}",
                    oninput: move |
                            e : FormEvent | invite_url.set(e.value()),
                    placeholder: "Paste anytype:// or invite.any.coop link",
                    disabled: *is_loading.read(),
                }
                Button {
                    onclick: on_join,
                    disabled: * is_loading.read() || invite_url.read()
                            .is_empty(),
                    if *is_loading.read() {
                        "Processing..."
                    } else {
                        "Join"
                    }
                }
            }
            if !status_message.read().is_empty() {
                Button { style: "font-size: 0.9em; margin-top: 10px;", "{status_message}" }
            }
        }
    }
}
