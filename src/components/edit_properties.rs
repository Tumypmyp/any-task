use crate::components::button::{Button, ButtonHolder};
use crate::components::popover::*;
use crate::components::row::*;
use crate::components::slider::*;
use crate::helpers::*;
use dioxus::prelude::*;
use std::vec;
#[component]
pub fn PropertiesOrder(properties: Store<Vec<PropertyInfo>>) -> Element {
    rsx! {
        Row {
            for (index , property) in properties.read().clone().iter().enumerate() {
                if property.show {
                    PropertyHolder {
                        key: "{property.id.as_str()}",
                        index,
                        id: property.id.clone(),
                        properties,
                    }
                }
            }
        }
    }
}
#[component]
pub fn PropertyHolder(
    index: usize,
    id: PropertyID,
    properties: Store<Vec<PropertyInfo>>,
) -> Element {
    let property = properties.get(index).unwrap();
    let mut open = use_signal(|| false);
    let mut current_value = use_signal(|| property().width);
    rsx! {
        PopoverRoot {
            open: open(),
            on_open_change: move |v| {
                open.set(v);
            },
            ButtonHolder {
                PopoverTrigger { width: "{current_value}vw", "{property().name}" }
            }
            PopoverContent {
                div { style: "margin-bottom: 15px; font-size: 16px; font-weight: bold;",
                    "{current_value:.0}"
                }
                Slider {
                    label: "Property width",
                    horizontal: true,
                    min: 10.0,
                    max: 50.0,
                    step: 1.0,
                    default_value: SliderValue::Single(current_value()),
                    on_value_change: move |value: SliderValue| {
                        let SliderValue::Single(v) = value;
                        current_value.set(v);
                        properties.get_mut(index).unwrap().width = v;
                    },
                    SliderTrack {
                        SliderRange {}
                        SliderThumb {}
                    }
                }
                Button {
                    onclick: move |_| {
                        properties
                            .with_mut(|v| {
                                if let Some(property) = v.iter_mut().find(|p| p.id == id) {
                                    property.show = !property.show;
                                }
                            });
                    },
                    "Hide property"
                }
            }
        }
    }
}
