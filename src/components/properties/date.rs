use dioxus::prelude::*;
use openapi::models::ApimodelPeriodDatePropertyValue;
use dioxus_primitives::{ContentAlign, ContentSide};
use dioxus_primitives::popover::{PopoverContent, PopoverRoot, PopoverTrigger};
use dioxus_primitives::calendar::*;
use time::{Date, UtcDateTime, Time, UtcOffset, OffsetDateTime};
use time::format_description::well_known::Rfc3339;
use time::macros::{format_description, offset};
use crate::API_CLIENT;
#[component]
pub fn DateTimePropertyValue(
    space_id: String,
    object_id: String,
    prop: Signal<ApimodelPeriodDatePropertyValue>,
) -> Element {
    let property_key = prop().key.unwrap();
    let date = prop().date.unwrap_or_default();
    let space_id = use_signal(|| space_id.clone());
    let object_id = use_signal(|| object_id.clone());
    let property_key = use_signal(|| property_key.clone());
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
        div { class: "button-holder",
            DatePropertyValue {
                space_id,
                object_id,
                property_key,
                dt,
            }
        }
        div { class: "button-holder",
            TimePropertyValue {
                space_id,
                object_id,
                property_key,
                dt,
            }
        }
    }
}
#[component]
pub fn DatePropertyValue(
    space_id: Signal<String>,
    object_id: Signal<String>,
    property_key: Signal<String>,
    dt: Signal<OffsetDateTime>,
) -> Element {
    let format = format_description!("[year]-[month]-[day]");
    let mut date = use_signal(|| dt().format(format).unwrap());
    let mut date_set = use_signal(|| date());
    let mut open = use_signal(|| false);
    let mut selected_date = use_signal(|| None::<Date>);
    let mut view_date = use_signal(|| UtcDateTime::now().date());
    rsx! {
        PopoverRoot {
            key: "{object_id}",
            open: open(),
            on_open_change: move |v| {
                if v == true {
                    date_set.set(date());
                }
                open.set(v);
            },
            PopoverTrigger {
                key: "{object_id}",
                class: "button",
                display: "flex",
                "flex-direction": "row",
                "data-style": "outline",
                "{date}"
            }
            PopoverContent { gap: "0.25rem", side: ContentSide::Bottom,
                PopoverHeader { text: "Set Date" }
                div { class: "calendar-example", style: "padding: 20px;",
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
                }
                button {
                    class: "button",
                    "data-style": "outline",
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
#[component]
pub fn TimePropertyValue(
    space_id: Signal<String>,
    object_id: Signal<String>,
    property_key: Signal<String>,
    dt: Signal<OffsetDateTime>,
) -> Element {
    let format = format_description!("[hour]:[minute]");
    let mut time = use_signal(|| dt().format(format).unwrap());
    let mut time_set = use_signal(|| time());
    let mut open = use_signal(|| false);
    rsx! {
        PopoverRoot {
            key: "{object_id}",
            open: open(),
            on_open_change: move |v| {
                if v == true {
                    time_set.set(time());
                }
                open.set(v);
            },
            PopoverTrigger {
                class: "button",
                key: "{object_id}",
                display: "flex",
                "flex-direction": "row",
                "data-style": "outline",
                "{time}"
            }
            PopoverContent { gap: "0.25rem", side: ContentSide::Bottom,
                PopoverHeader { text: "Set Time" }
                Input { value: time_set }
                button {
                    class: "button",
                    "data-style": "outline",
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
#[component]
pub fn PopoverHeader(text: String) -> Element {
    rsx! {
        h3 {
            padding_top: "0.25rem",
            padding_bottom: "0.25rem",
            width: "100%",
            text_align: "center",
            margin: 0,
            "{text}"
        }
    }
}
#[component]
pub fn Input(value: Signal<String>) -> Element {
    rsx! {
        input {
            class: "input",
            value: "{value.read()}",
            oninput: move |event| {
                *value.write() = event.value();
            },
        }
    }
}
#[component]
pub fn CancelPopoverButton(open: Signal<bool>) -> Element {
    rsx! {
        button {
            class: "button",
            "data-style": "outline",
            onclick: move |_| {
                open.set(false);
            },
            "Cancel"
        }
    }
}
