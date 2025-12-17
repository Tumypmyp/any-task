use crate::API_CLIENT;
use crate::components::button::*;
use crate::components::popover::*;
use crate::components::row::*;
use crate::components::scroll_area::*;
use crate::helpers::models::DateTimeFormat;
use crate::helpers::*;
use dioxus::prelude::*;
use std::vec;

#[component]
pub fn AddProperties(
    properties: Store<Vec<PropertyInfo>>,
) -> Element {
   rsx! {
    Row { position: Position::Middle,
        Button { "Add Properties" }
    }
    for (i , property) in properties.read().clone().iter().enumerate() {
        if property.show {
            ShowProperty {
                key: "{property.id.as_str()}-chosen",
                index: i,
                id: property.id.clone(),
                properties,
            }
        }
    }
    for (i , property) in properties.read().clone().iter().enumerate() {
        if !property.show {
            ShowProperty {
                key: "{property.id.as_str()}-chosen",
                index: i,
                id: property.id.clone(),
                properties,
            }
        }
    }
}
}
#[component]
pub fn ShowProperty(index: usize, id: PropertyID, properties: Store<Vec<PropertyInfo>>) -> Element {
    let name = (properties.get(index).unwrap())().name.clone();
    rsx! {
        Button {
            variant: if (properties.get(index).unwrap())().show { ButtonVariant::Primary } else { ButtonVariant::Ghost },
            onclick: move |_| {
                properties
                    .with_mut(|v| {
                        if let Some(property) = v.iter_mut().find(|p| p.id == id) {
                            property.show = !property.show;
                        }
                    });
            },
            "{name}"
        }
    }
}
