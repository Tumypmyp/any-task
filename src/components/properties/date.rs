use dioxus::prelude::*;
use openapi::models::ApimodelPeriodDatePropertyValue;
use crate::components::base::*;
use dioxus_primitives::calendar::*;
use time::{Date, UtcDateTime, Time, UtcOffset, OffsetDateTime};
use time::format_description::well_known::Rfc3339;
use time::macros::{format_description, offset};
use crate::API_CLIENT;
#[component]
pub fn DateTimePropertyValues(
    space_id: String,
    object_id: String,
    prop: Signal<ApimodelPeriodDatePropertyValue>,
) -> Element {
    let property_name = use_signal(|| prop().name.unwrap());
    let property_key = use_signal(|| prop().key.unwrap());
    let date = prop().date.unwrap_or_default();
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
        DatePropertyValue {
            space_id,
            object_id,
            property_key,
            property_name,
            dt,
        }
        TimePropertyValue {
            space_id,
            object_id,
            property_key,
            property_name,
            dt,
        }
    }
}
#[component]
pub fn DatePropertyValue(
    space_id: Signal<String>,
    object_id: Signal<String>,
    property_key: Signal<String>,
    property_name: Signal<String>,
    dt: Signal<OffsetDateTime>,
) -> Element {
    let format = format_description!("[year]-[month]-[day]");
    let mut date = use_signal(|| dt().format(format).unwrap());
    let mut date_set = use_signal(|| date());
    let mut open = use_signal(|| false);
    let mut selected_date = use_signal(|| None::<Date>);
    let mut view_date = use_signal(|| UtcDateTime::now().date());
    rsx! {
        ButtonHolder { width: "15vw",
            PopoverRoot {
                open: open(),
                on_open_change: move |v| {
                    if v == true {
                        date_set.set(date());
                    }
                    open.set(v);
                },
                PopoverTrigger { "{date}" }
                PopoverContent {
                    PopoverHeader { text: "{property_name}" }
                    Calendar {
                        selected_date: selected_date(),
                        on_date_change: move |date: Option<Date>| {
                            selected_date.set(date);
                        },
                        view_date: view_date(),
                        on_view_change: move |new_view: Date| {
                            view_date.set(new_view);
                        },
                        CalendarHeader {
                            CalendarNavigation {
                                CalendarPreviousMonthButton {}
                                CalendarMonthTitle {}
                                CalendarNextMonthButton {}
                            }
                        }
                        CalendarGrid {}
                    }
                    ButtonWithHolder {
                        variant: ButtonVariant::Outline,
                        onclick: move |_| {
                            if let Some(d) = selected_date() {
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
                                date_set.set(d.format(format).unwrap());
                                date.set(date_set());
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
}
#[component]
pub fn TimePropertyValue(
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
        ButtonHolder { width: "15vw",
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
                    ButtonWithHolder {
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
}
