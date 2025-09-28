use dioxus::prelude::*;
use dioxus_primitives::{ContentAlign, ContentSide};
use dioxus_primitives::popover::{
    self, PopoverContentProps, PopoverRootProps, PopoverTriggerProps,
};
use crate::components::buttons::*;
#[component]
pub fn PopoverHeader(text: String) -> Element {
    rsx! {
        h3 {
            padding_top: "0.25rem",
            padding_bottom: "0.25rem",
            width: "100%",
            text_align: "center",
            margin: 0,
            "{text}"
        }
    }
}
#[component]
pub fn Input(value: Signal<String>) -> Element {
    rsx! {
        input {
            class: "input",
            value: "{value.read()}",
            oninput: move |event| {
                *value.write() = event.value();
            },
        }
    }
}
#[component]
pub fn CancelPopoverButton(open: Signal<bool>) -> Element {
    rsx! {
        Button {
            variant: ButtonVariant::Outline,
            onclick: move |_| {
                open.set(false);
            },
            "Cancel"
        }
    }
}
#[component]
pub fn PopoverRoot(props: PopoverRootProps) -> Element {
    rsx! {
        popover::PopoverRoot {
            class: "popover",
            is_modal: props.is_modal,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            attributes: props.attributes,
            {props.children}
        }
    }
}
#[component]
pub fn PopoverTrigger(props: PopoverTriggerProps) -> Element {
    rsx! {
        popover::PopoverTrigger {
            class: "popover-trigger",
            display: "flex",
            "flex-direction": "row",
            "data-style": "outline",
            attributes: props.attributes,
            {props.children}
        }
    }
}
#[component]
pub fn PopoverContent(props: PopoverContentProps) -> Element {
    rsx! {
        popover::PopoverContent {
            class: "popover-content",
            gap: "0.25rem",
            side: ContentSide::Bottom,
            align: ContentAlign::Center,
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}
