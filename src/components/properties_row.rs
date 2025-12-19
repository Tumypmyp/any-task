use crate::components::button::*;
use crate::components::separator::*;
use crate::components::label::*;
use crate::components::list::*;
use crate::components::row::*;
use crate::components::slider::*;
use crate::helpers::*;
use dioxus::prelude::*;
use std::vec;
#[component]
pub fn PropertiesRow(properties: Store<Vec<PropertyInfo>>) -> Element {
    rsx! {
        List {
            for (index , property) in properties.read().clone().iter().enumerate() {
                if property.show {
                    Property {
                        key: "{property.id.as_str()}",
                        index,
                        id: property.id.clone(),
                        properties,
                    }
                    Separator {}
                }
            }
        }
    }
}
#[component]
pub fn Property(index: usize, id: PropertyID, properties: Store<Vec<PropertyInfo>>) -> Element {
    let property = properties.get(index).unwrap();
    let name = property().name;
    // let mut width = use_signal(|| property().width);
    // let mut height = use_signal(|| property().height);
    rsx! {
        Row {
            Button { variant: ButtonVariant::Secondary,
                // width: "{current_value}vw",
                "{name}"
            }
            Button {
                variant: ButtonVariant::Destructive,
                onclick: move |_| {
                    properties
                        .with_mut(|v| {
                            if let Some(property) = v.iter_mut().find(|p| p.id == id) {
                                property.show = !property.show;
                            }
                        });
                },
                "X"
            }
        }
        Label { html_for: "width_slider", "Width" }
        Slider {
            id: "width_slider",
            label: "Property width",
            width: "50vw",
            horizontal: true,
            min: 5.0,
            max: 100.0,
            step: 1.0,
            // default_value: SliderValue::Single(width()),
            default_value: SliderValue::Single(properties.get(index).unwrap()().width),
            on_value_change: move |value: SliderValue| {
                let SliderValue::Single(v) = value;
                // width.set(v);
                properties.get_mut(index).unwrap().width = v;
            },
            SliderTrack {
                SliderRange {}
                SliderThumb {}
            }
        }
        Label { html_for: "height_slider", "Height" }
        Slider {
            id: "height_slider",
            label: "Property width",
            width: "50vw",
            horizontal: true,
            min: 5.0,
            max: 100.0,
            step: 1.0,
            // default_value: SliderValue::Single(height()),
            default_value: SliderValue::Single(properties.get(index).unwrap()().height),
            on_value_change: move |value: SliderValue| {
                let SliderValue::Single(v) = value;
                // height.set(v);
                properties.get_mut(index).unwrap().height = v;
            },
            SliderTrack {
                SliderRange {}
                SliderThumb {}
            }
        }
    }
}
