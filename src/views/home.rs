use crate::API_CLIENT;
use crate::Logout;
use crate::Route;
use crate::components::action::*;
use crate::components::button::Button;
use crate::components::button::ButtonVariant;
use crate::components::column::Column;
use crate::components::header::{Header, Title};
use crate::components::input::*;
use crate::components::row::RowPosition;
use crate::components::row::*;
use crate::helpers::*;
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
    use_resource(move || {
        let _reconnect = RECONNECT_COUNT.read();
        let client = API_CLIENT.read().as_ref().cloned();
        async move {
            let Some(client) = client else {
                tracing::warn!("subscribe_spaces: no client yet");
                return;
            };
            if let Err(e) = client.subscribe_spaces().await {
                tracing::error!("subscribe_spaces failed: {e:#}");
            }
        }
    });
    use_drop(move || {
        spawn(async move {
            if let Some(client) = API_CLIENT.read().as_ref().cloned() {
                client.unsubscribe_spaces().await.ok();
            }
        });
    });
    let spaces = SPACES.read();
    rsx! {
        Column { style: "align-items: center; gap: 6px;",
            for obj_id in &spaces.order {
                if let Some(det) = spaces.details.get(obj_id) {
                    SpaceButton {
                        id: det.target_space_id.clone(),
                        name: det.name.clone(),
                    }
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
            width: "87vw",
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
#[derive(Clone, PartialEq)]
enum JoinStatus {
    Idle,
    Loading,
    Success(String),
    Error(String),
}
#[component]
pub fn JoinSpace() -> Element {
    let mut invite_url = use_signal(String::new);
    let mut status = use_signal(|| JoinStatus::Idle);
    let on_join = move |_| {
        let url = invite_url.read().clone();
        if url.is_empty() {
            return;
        }
        spawn(async move {
            status.set(JoinStatus::Loading);
            let client_guard = API_CLIENT.read();
            let Some(client) = client_guard.as_ref() else {
                status.set(JoinStatus::Error("API client not available".to_string()));
                return;
            };
            match client.join_space_from_link(&url).await {
                Ok(space_name) => {
                    invite_url.set(String::new());
                    status.set(JoinStatus::Success(format!(
                        "Request sent to join '{space_name}'. Waiting for owner approval.",
                    )));
                }
                Err(e) => {
                    status.set(JoinStatus::Error(format!("{e}")));
                }
            }
        });
    };
    let is_loading = matches!(*status.read(), JoinStatus::Loading);
    rsx! {
        Column { style: "padding: 20px; gap: 10px;",
            Header {
                Title { title: "Join a Space" }
            }
            Input {
                r#type: "text",
                value: "{invite_url}",
                oninput: move |e: FormEvent| invite_url.set(e.value()),
                placeholder: "Paste anytype:// or invite.any.coop link",
                disabled: is_loading,
            }
            Button {
                onclick: on_join,
                disabled: is_loading || invite_url.read().is_empty(),
                if is_loading {
                    "Processing..."
                } else {
                    "Join"
                }
            }
            match &*status.read() {
                JoinStatus::Success(msg) => rsx! {
                    p { style: "color: green; font-size: 0.9em;", "{msg}" }
                },
                JoinStatus::Error(msg) => rsx! {
                    p { style: "color: red; font-size: 0.9em;", "{msg}" }
                },
                _ => rsx! {},
            }
        }
    }
}
