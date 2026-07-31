use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

#[css_module("/src/components/drop_zone/style.css")]
struct Styles;

#[component]
pub fn Dropzone(
    #[props(default)] active: bool,
    #[props(default)] dragging: bool,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    onpointerenter: Option<EventHandler<PointerEvent>>,
    onpointerleave: Option<EventHandler<PointerEvent>>,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_drop_zone,
        "data-active": active.to_string(),
        "data-dragging": dragging.to_string(),
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div {
            onpointerenter: move |e| {
                if let Some(f) = &onpointerenter {
                    f.call(e);
                }
            },
            onpointerleave: move |e| {
                if let Some(f) = &onpointerleave {
                    f.call(e);
                }
            },
            ..merged,
        }
    }
}
