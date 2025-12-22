use crate::components::button::*;
use crate::components::label::*;
use crate::components::list::*;
use crate::components::properties::*;
use crate::components::row::*;
use crate::components::separator::*;
use crate::components::slider::*;
use crate::helpers::*;
use dioxus::prelude::*;
use std::vec;

#[component]
pub fn PropertiesRow(properties: Store<Vec<(PropertyInfo, PropertySettings)>>) -> Element {
    rsx! {
        List {
            for (index , property) in properties.read().clone().iter().enumerate() {
                Property {
                    // should be different (use property view hash)
                    // key: "{property.0.id.as_str()}",
                    index,
                    properties,
                }
                Separator {}
            }
        }
    }
}

#[component]
pub fn Property(index: usize, properties: Store<Vec<(PropertyInfo, PropertySettings)>>) -> Element {
    let property = properties.get(index).unwrap();
    let name = property().0.name;
    let edit = match property().1 {
        PropertySettings::Date(settings) => rsx! {
            DateSettingsEdit {
                format: settings.date_format,
                on_change: move |new_format: DateTimeFormat| {
                    properties
                        .write()
                        .get_mut(index)
                        .map(|(_, s)| {
                            if let PropertySettings::Date(d) = s {
                                d.date_format = new_format;
                            }
                        });
                },
            }
            GeneralPropertyEdit {
                settings: settings.general,
                on_change: move |new_settings: GeneralPropertySettings| {
                    properties
                        .write()
                        .get_mut(index)
                        .map(|(_, s)| {
                            if let PropertySettings::Date(d) = s {
                                d.general = new_settings;
                            }
                        });
                },
            }
        },
        PropertySettings::General(settings) => rsx! {
            GeneralPropertyEdit {
                settings,
                on_change: move |new_settings: GeneralPropertySettings| {
                    properties
                        .write()
                        .get_mut(index)
                        .map(|(_, s)| {
                            if let PropertySettings::General(g) = s {
                                *g = new_settings;
                            }

                        });
                },
            }
        },
        PropertySettings::Checkbox(settings) => rsx! {
            SizeSlider {
                size: settings.size,
                on_change: move |new_size: f64| {
                    properties
                        .write()
                        .get_mut(index)
                        .map(|(_, s)| {
                            if let PropertySettings::Checkbox(c) = s {
                                c.size = new_size;
                            }
                        });
                },
            }
        },
    };
    rsx! {
        Row {
            Button { variant: ButtonVariant::Secondary, "{name}" }
            Button {
                variant: ButtonVariant::Destructive,
                onclick: move |_| {
                    properties
                        .with_mut(|v| {
                            if index < v.len() {
                                v.remove(index);
                            }
                        });
                },
                "X"
            }
        }
        {edit}
    }
}

#[component]
pub fn GeneralPropertyEdit(
    settings: GeneralPropertySettings,
    on_change: EventHandler<GeneralPropertySettings>,
) -> Element {
    rsx! {
        PropertySlider {
            id: "width_slider",
            label: "Width",
            value: settings.width,
            min: 5.0,
            max: 100.0,
            on_change: move |v| {
                let mut new_settings = settings.clone();
                new_settings.width = v;
                on_change.call(new_settings);
            },
        }
        PropertySlider {
            id: "height_slider",
            label: "Height",
            value: settings.height,
            min: 5.0,
            max: 100.0,
            on_change: move |v| {
                let mut new_settings = settings.clone();
                new_settings.height = v;
                on_change.call(new_settings);
            },
        }
    }
}

#[component]
fn SizeSlider(size: f64, on_change: EventHandler<f64>) -> Element {
    rsx! {
        PropertySlider {
            id: "size_slider",
            label: "Size",
            value: size,
            min: 5.0,
            max: 30.0,
            on_change: move |v| on_change.call(v),
        }
    }
}

#[component]
fn PropertySlider(
    id: String,
    label: String,
    value: f64,
    min: f64,
    max: f64,
    on_change: EventHandler<f64>,
) -> Element {
    rsx! {
        Label { html_for: "{id}", "{label}" }
        Slider {
            id: "{id}",
            label: "{label}",
            width: "50vw",
            horizontal: true,
            min,
            max,
            step: 1.0,
            default_value: SliderValue::Single(value),
            on_value_change: move |val: SliderValue| {
                let SliderValue::Single(v) = val;
                on_change.call(v);
            },
            SliderTrack {
                SliderRange {}
                SliderThumb {}
            }
        }
    }
}
