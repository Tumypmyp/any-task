use dioxus::prelude::*;
#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum ButtonVariant {
    #[default]
    Outline,
    Primary,
    Secondary,
    Destructive,
    Ghost,
}
impl ButtonVariant {
    pub fn class(&self) -> &'static str {
        match self {
            ButtonVariant::Primary => "primary",
            ButtonVariant::Secondary => "secondary",
            ButtonVariant::Destructive => "destructive",
            ButtonVariant::Outline => "outline",
            ButtonVariant::Ghost => "ghost",
        }
    }
}
#[component]
pub fn Button(
    #[props(default)]
    variant: ButtonVariant,
    #[props(extends = GlobalAttributes)]
    #[props(extends = button)]
    attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    onmousedown: Option<EventHandler<MouseEvent>>,
    onmouseup: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    rsx! {
        button {
            class: "button",
            style: "
                text-align: center;
                white-space: nowrap; 
                overflow: hidden; 
                text-overflow: ellipsis; 
            ",
            "data-style": variant.class(),
            onclick: move |event| {
                if let Some(f) = &onclick {
                    f.call(event);
                }
            },
            onmousedown: move |event| {
                if let Some(f) = &onmousedown {
                    f.call(event);
                }
            },
            onmouseup: move |event| {
                if let Some(f) = &onmouseup {
                    f.call(event);
                }
            },
            ..attributes,
            {children}
        }
    }
}
#[component]
pub fn ButtonHolder(
    children: Element,
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        div {
            style: "
                display: flex; 
                max-width: 95%; 
                padding: 3px; 
                flex-direction: column; 
                align-items: center; 
            ",
            ..attributes,
            {children}
        }
    }
}
#[component]
pub fn ButtonWithHolder(
    #[props(default)]
    variant: ButtonVariant,
    #[props(extends = GlobalAttributes)]
    #[props(extends = button)]
    attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    onmousedown: Option<EventHandler<MouseEvent>>,
    onmouseup: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    rsx! {
        ButtonHolder {
            Button {
                variant,
                attributes,
                onclick,
                onmousedown,
                onmouseup,
                children,
            }
        }
    }
}
