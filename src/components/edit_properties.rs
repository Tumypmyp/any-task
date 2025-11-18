use crate::components::button::{ButtonHolder, ButtonWithHolder};
use crate::components::popover::*;
use crate::components::row::*;
use crate::components::slider::*;
use crate::helpers::*;
use dioxus::prelude::*;
use std::vec;
#[component]
pub fn PropertiesOrder(
    show_properties: Store<Vec<PropertyInfo>>,
    other_properties: Store<Vec<PropertyInfo>>,
) -> Element {
    rsx! {
        Row {
            for (index , property) in show_properties.read().clone().iter().enumerate() {
                PropertyHolder {
                    key: "{property.id.as_str()}",
                    index,
                    id: property.id.clone(),
                    show_properties,
                    other_properties,
                }
            }
        }
    }
}
#[component]
pub fn PropertyHolder(
    index: usize,
    id: PropertyID,
    show_properties: Store<Vec<PropertyInfo>>,
    // other_proerties might change if deselect the shown
    other_properties: Store<Vec<PropertyInfo>>,
) -> Element {
    let property = show_properties.get(index).unwrap();
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
                        show_properties.get_mut(index).unwrap().width = v;
                    },
                    SliderTrack {
                        SliderRange {}
                        SliderThumb {}
                    }
                }
                ButtonWithHolder {
                    onclick: move |_| {
                        other_properties
                            .with_mut(|v| {
                                v.push((show_properties.get(index).unwrap())());
                                let cmp = |p1: &PropertyInfo, p2: &PropertyInfo| {
                                    p1.name.cmp(&p2.name)
                                };
                                v.sort_by(cmp);
                            });

                        show_properties
                            .with_mut(|v| {
                                v.retain(|p| p.id != id);
                            });
                    },
                    "Hide property"
                }
            }
        }
    }
}
