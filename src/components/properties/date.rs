use crate::API_CLIENT;
use crate::components::button::*;
use crate::components::date_picker::*;
use crate::components::popover::*;
use crate::components::select::*;
use crate::helpers::models::DateTimeFormat;
use crate::helpers::*;
use dioxus::prelude::*;
use openapi::models::DatePropertyValue;
use time::format_description::well_known::Rfc3339;
use time::macros::{format_description, offset};
use time::{Date, OffsetDateTime, Time, UtcDateTime, UtcOffset};
#[component]
pub fn DateSettingsEdit(
    format: DateTimeFormat,
    on_change: EventHandler<DateTimeFormat>,
) -> Element {
    rsx! {}
}
impl PropertyRenderer for DatePropertyValue {
    fn render(
        &self,
        space_id: String,
        object_id: String,
        _info: PropertyInfo,
        settings: PropertySettings,
    ) -> Element {
        match settings {
            PropertySettings::Date(s) => {
                rsx! {
                    DateTimeValues {
                        space_id: &space_id,
                        object_id: &object_id,
                        prop: self.clone(),
                        settings: s.date_format,
                    }
                }
            }
            _ => rsx! {},
        }
    }
}
#[component]
pub fn DateTimeValues(
    space_id: String,
    object_id: String,
    prop: DatePropertyValue,
    settings: DateTimeFormat,
) -> Element {
    let property_name = use_signal(|| prop.name.unwrap());
    let property_key = use_signal(|| prop.key.unwrap());
    let date = prop.date.unwrap_or_default();
    let space_id = use_signal(|| space_id.clone());
    let object_id = use_signal(|| object_id.clone());
    let offset = UtcOffset::current_local_offset()
        .unwrap_or(
            offset! {
                + 0
            },
        );
    let dt = use_signal(|| {
        UtcDateTime::parse(&date, &Rfc3339).unwrap().to_offset(offset)
    });
    rsx! {
        if settings == DateTimeFormat::DateTime || settings == DateTimeFormat::Date {
            DateValue {
                space_id,
                object_id,
                property_key,
                property_name,
                dt,
            }
        }
        if settings == DateTimeFormat::DateTime || settings == DateTimeFormat::Time {
            TimeValue {
                space_id,
                object_id,
                property_key,
                property_name,
                dt,
            }
        }
    }
}
#[component]
pub fn DateValue(
    space_id: Signal<String>,
    object_id: Signal<String>,
    property_key: Signal<String>,
    property_name: Signal<String>,
    dt: Signal<OffsetDateTime>,
) -> Element {
    let mut selected_date = use_signal(|| dt().date());
    tracing::debug!("{:#?}", selected_date().clone());
    rsx! {
        DatePicker {
            selected_date: selected_date(),
            on_value_change: move |v| {
                if let Some(d) = v {
                    tracing::info!("Selected date changed: {:?}", v);
                    dt.set(dt().replace_date(d));
                    tracing::debug!("change date to: {:?}", dt);
                    API_CLIENT
                        .read()
                        .update_datetime_property(
                            space_id(),
                            object_id(),
                            property_key(),
                            dt().to_utc(),
                        );
                    selected_date.set(d);
                }
            },
        }
    }
}
#[component]
pub fn TimeValue(
    space_id: Signal<String>,
    object_id: Signal<String>,
    property_key: Signal<String>,
    property_name: Signal<String>,
    dt: Signal<OffsetDateTime>,
) -> Element {
    let format = format_description!("[hour]:[minute]");
    let mut time = use_signal(|| dt().format(format).unwrap());
    let mut time_set = use_signal(|| time());
    let mut open = use_signal(|| false);
    rsx! {
        PopoverRoot {
            open: open(),
            on_open_change: move |v| {
                if v == true {
                    time_set.set(time());
                }
                open.set(v);
            },
            PopoverTrigger { "{time}" }
            PopoverContent {
                PopoverHeader { text: "{property_name}" }
                Input { value: time_set }
                Button {
                    variant: ButtonVariant::Outline,
                    onclick: move |_| {
                        if let Ok(t) = Time::parse(&time_set.read(), format) {
                            dt.set(dt().replace_time(t));
                            API_CLIENT
                                .read()
                                .update_datetime_property(
                                    space_id(),
                                    object_id(),
                                    property_key(),
                                    dt().to_utc(),
                                );
                            time.set(time_set());
                        }
                        open.set(false);
                    },
                    "Confirm"
                }
                CancelPopoverButton { open }
            }
        }
    }
}
