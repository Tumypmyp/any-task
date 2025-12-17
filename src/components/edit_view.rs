use crate::PropertyInfo;
use crate::components::add_properties::ShowPropertiesSetting;
use crate::components::button::*;
use crate::components::properties_row::PropertiesRow;
use crate::components::sheet::*;
use dioxus::prelude::*;
use std::vec;
#[component]
pub fn EditView(properties: Store<Vec<PropertyInfo>>, space_id: Signal<String>) -> Element {
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
            SheetContent { side: SheetSide::Top,
                PropertiesRow { properties }
                ShowPropertiesSetting { space_id, properties }
            }
        }
    }
}
