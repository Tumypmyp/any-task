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
    properties: Store<Vec<(PropertyInfo, PropertySettings)>>,
    all_properties: Store<Vec<PropertyInfo>>,
) -> Element {
    rsx! {
        Row { position: Position::Middle,
            Button { variant: ButtonVariant::Secondary, "Add Properties" }
        }
        for (i , property) in all_properties.read().clone().iter().enumerate() {
            // if property.show {
            ShowProperty {
                key: "{property.id.as_str()}",
                // index: i,
                // id: property.id.clone(),
                property: property.clone(),
                properties,
            }
        }
        // for (i , property) in properties.read().clone().iter().enumerate() {
        //     if !property.show {
        //         ShowProperty {
        //             key: "{property.id.as_str()}-chosen",
        //             index: i,
        //             id: property.id.clone(),
        //             properties,
        //         }
        //     }
        // }
    }
}
#[component]
pub fn ShowProperty( property: PropertyInfo, properties: Store<Vec<(PropertyInfo, PropertySettings)>>) -> Element {
    // let name = properties.get(index).unwrap()().0.name.clone();
    let name = property.clone().name;
    rsx! {
        Button {
            variant: ButtonVariant::Ghost,
            onclick: move |_| {
                properties
                    .with_mut(|v| { v.push((property.clone(), PropertySettings::default())) });
            },
            "{name}"
        }
    }
}
