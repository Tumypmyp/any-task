use crate::AppSettings;
use crate::Logout;
use crate::components::action::*;
use crate::components::base::message;
use crate::components::button::{Button, ButtonVariant};
use crate::components::column::*;
use crate::components::header::{Header, Title};
use crate::components::input::Input;
use crate::components::row::*;
use crate::components::show_hide_button::ShowHideButton;
use crate::helpers::*;
use crate::mnemonic_store::*;
use dioxus::prelude::*;

#[component]
pub fn Settings() -> Element {
    let show: Signal<bool> = use_signal(|| false);
    let mnemonic = use_signal(|| load_mnemonic().unwrap_or_default());

    let copy_to_clipboard = use_callback(move |_| {
        let text = mnemonic.read().clone();
        spawn(async move {
            #[cfg(not(target_os = "android"))]
            let copied = tokio::task::spawn_blocking(move || {
                arboard::Clipboard::new().and_then(|mut cb| cb.set_text(text))
            })
            .await
            .map(|r| r.is_ok())
            .unwrap_or(false);

            #[cfg(target_os = "android")]
            let copied = document::eval(&format!(
                r#"await navigator.clipboard.writeText({text:?});"#
            ))
            .await
            .is_ok();

            if copied {
                message::info("Key was copied to clipboard", "");
            } else {
                message::error_with_description("Failed to copy to clipboard", "");
            }
        });
    });

    rsx! {
        Column { position: ColumnPosition::Middle, style: "gap: 6px;",
            Button { variant: ButtonVariant::Ghost, "Your login key" }
            Row { style: "position: relative; width: 100%;",
                Button {
                    variant: ButtonVariant::Ghost,
                    onclick: copy_to_clipboard,
                    style: format!(
                        "width: 100%; padding-right: 2.5rem; box-sizing: border-box; word-break: break-all; -webkit-text-security: {};",
                        if show() { "none" } else { "disc" },
                    ),
                    "{mnemonic}"
                }
                ShowHideButton { show }
            }
            Logout {}
        }
        Actions { GoBack {} }
    }
}
