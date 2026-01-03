use crate::components::add_properties::*;
use crate::components::button::*;
use crate::components::properties_row::PropertiesRow;
use crate::components::scroll_area::*;
use crate::components::separator::*;
use crate::components::sheet::*;
use crate::helpers::*;
use dioxus::prelude::*;
use dioxus_primitives::scroll_area::ScrollDirection;
use std::vec;
#[component]
pub fn EditView(
    properties: Store<Vec<(PropertyInfo, PropertySettings)>>,
    all_properties: Store<Vec<PropertyInfo>>,
    space_id: Signal<String>,
) -> Element {
    let mut open = use_signal(|| false);
    rsx! {
        ButtonHolder {
            Button {
                variant: ButtonVariant::Secondary,
                onclick: move |_| { open.set(true) },
                "Edit view"
            }
        }
        Sheet { open: open(), on_open_change: move |v| open.set(v),
            SheetContent { side: SheetSide::Bottom, style: "max-height: 70vh;",
                ScrollArea { direction: ScrollDirection::Vertical,
                    PropertiesRow { properties }
                    AddProperties { properties, all_properties }
                }
            }
        }
    }
}
