use crate::components::button::*;
use crate::components::column::*;
use crate::components::label::*;
use crate::components::properties::*;
use crate::components::row::*;
use crate::components::separator::*;
use crate::components::slider::*;
use crate::helpers::*;
use dioxus::prelude::*;
use std::vec;
#[component]
pub fn PropertiesRow(properties: Store<Vec<Vec<(PropertyInfo, PropertySettings)>>>) -> Element {
    rsx! {
        Column {
            for (i , vec_property) in properties.read().clone().iter().enumerate() {
                for (j , property) in vec_property.iter().enumerate() {
                    Property { i, j, properties }
                    Separator {}
                }
            }
        }
    }
}
#[component]
pub fn Property(
    i: usize,
    j: usize,
    properties: Store<Vec<Vec<(PropertyInfo, PropertySettings)>>>,
) -> Element {
    let property = properties.get(i).unwrap().get(j).unwrap();
    let name = property().0.name;
    let mut mutate_settings = move |callback: Box<dyn FnOnce(&mut PropertySettings)>| {
        properties.with_mut(|rows| {
            if let Some(row) = rows.get_mut(i) {
                if let Some((_, settings)) = row.get_mut(j) {
                    callback(settings);
                }
            }
        });
    };
    let edit = match property().1 {
        PropertySettings::Date(settings) => {
            rsx! {
                DateSettingsEdit {
                    format: settings.date_format,
                    on_change: move |new_format: DateTimeFormat| mutate_settings(
                        Box::new(move |s| {
                            if let PropertySettings::Date(d) = s {
                                d.date_format = new_format;
                            }
                        }),
                    ),
                }
                GeneralPropertyEdit {
                    settings: settings.general,
                    on_change: move |new_settings: GeneralPropertySettings| mutate_settings(
                        Box::new(move |s| {
                            if let PropertySettings::Date(d) = s {
                                d.general = new_settings;
                            }
                        }),
                    ),
                }
            }
        }
        PropertySettings::General(settings) => {
            rsx! {
                GeneralPropertyEdit {
                    settings,
                    on_change: move |new_settings: GeneralPropertySettings| mutate_settings(
                        Box::new(move |s| {
                            if let PropertySettings::General(g) = s {
                                *g = new_settings;
                            }
                        }),
                    ),
                }
            }
        }
        PropertySettings::Checkbox(settings) => {
            rsx! {
                SizeSlider {
                    size: settings.size,
                    on_change: move |new_size: f64| mutate_settings(
                        Box::new(move |s| {
                            if let PropertySettings::Checkbox(c) = s {
                                c.size = new_size;
                            }
                        }),
                    ),
                }
            }
        }
    };
    rsx! {
        Row { position: Position::Middle,
            Button { variant: ButtonVariant::Secondary, "{name}" }
            Button {
                variant: ButtonVariant::Destructive,
                onclick: move |_| {
                    properties
                        .with_mut(|v| {
                            if j < v.len() {
                                v.remove(j);
                            }
                        });
                },
                "X"
            }
        }
        {edit}
        Row { position: Position::Middle,
            Button {
                variant: if 0 < j { ButtonVariant::Primary } else { ButtonVariant::Ghost },
                onclick: move |_| {
                    properties
                        .with_mut(|v| {
                            if 0 < j {
                                v[i].swap(j - 1, j);
                            }
                        });
                },
                "<"
            }
            Column {
                Button {
                    onclick: move |_| {
                        properties
                            .with_mut(|rows| {
                                let item = rows
                                    .get_mut(i)
                                    .and_then(|row| {
                                        if j < row.len() { Some(row.remove(j)) } else { None }
                                    });
                                if let Some(extracted_item) = item {
                                    if 0 < i {
                                        let prev_row = &mut rows[i - 1];
                                        if j <= prev_row.len() {
                                            prev_row.insert(j, extracted_item);
                                        } else {
                                            prev_row.push(extracted_item);
                                        }
                                    } else {
                                        rows.push(vec![extracted_item]);
                                    }
                                }
                            });
                    },
                    "^"
                }
                Button {
                    onclick: move |_| {
                        properties

                            .with_mut(|rows| {
                                let item = rows
                                    .get_mut(i)
                                    .and_then(|row| {
                                        if j < row.len() { Some(row.remove(j)) } else { None }
                                    });
                                if let Some(extracted_item) = item {
                                    if i + 1 < rows.len() {
                                        let next_row = &mut rows[i + 1];
                                        if j <= next_row.len() {
                                            next_row.insert(j, extracted_item);
                                        } else {
                                            next_row.push(extracted_item);
                                        }
                                    } else {
                                        rows.push(vec![extracted_item]);
                                    }
                                }
                            });
                    },
                    "V"
                }
            }
            Button {
                variant: if j + 1 < properties.read()[i].len() { ButtonVariant::Primary } else { ButtonVariant::Ghost },
                onclick: move |_| {
                    properties
                        .with_mut(|v| {
                            if j + 1 < v[i].len() {
                                v[i].swap(j, j + 1);
                            }
                        });
                },
                ">"
            }
        }
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
