use dioxus::prelude::*;
use openapi::models::Tag;
use serde::{Deserialize, Serialize};
#[derive(Eq, Hash, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct PropertyID(pub String);
impl PropertyID {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub enum DateTimeFormat {
    #[default]
    DateTime,
    Date,
    Time,
}
pub const NAME_PROPERTY_ID_STR: &str = "name_property_id";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PropertyInfo {
    pub id: PropertyID,
    pub name: String,
    pub optional: OptionalInfo,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum OptionalInfo {
    Select(Vec<Tag>),
    Date,
    Checkbox,
    #[default]
    Other,
}
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeneralPropertySettings {
    pub width: f64,
    pub height: f64,
}
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct DateSettings {
    pub general: GeneralPropertySettings,
    pub date_format: DateTimeFormat,
}
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckboxSettings {
    pub size: f64,
}
impl Default for CheckboxSettings {
    fn default() -> Self {
        Self { size: 5.0 }
    }
}
impl Default for DateSettings {
    fn default() -> Self {
        Self {
            date_format: DateTimeFormat::Date,
            general: GeneralPropertySettings {
                width: 10.0,
                height: 10.0,
            },
        }
    }
}
impl Default for GeneralPropertySettings {
    fn default() -> Self {
        Self { width: 15.0, height: 10.0 }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PropertySettings {
    General(GeneralPropertySettings),
    Date(DateSettings),
    Checkbox(CheckboxSettings),
}
impl Default for PropertySettings {
    fn default() -> Self {
        Self::General(GeneralPropertySettings::default())
    }
}
impl PropertySettings {
    pub fn height(&self) -> f64 {
        match self {
            Self::General(s) => s.height,
            Self::Date(s) => s.general.height,
            Self::Checkbox(s) => s.size,
        }
    }
    pub fn width(&self) -> f64 {
        match self {
            Self::General(s) => s.width,
            Self::Date(s) => s.general.width,
            Self::Checkbox(s) => s.size,
        }
    }
    pub fn set_height(&mut self, val: f64) {
        match self {
            Self::General(s) => s.height = val,
            Self::Date(s) => s.general.height = val,
            _ => {}
        }
    }
    pub fn set_width(&mut self, val: f64) {
        match self {
            Self::General(s) => s.width = val,
            Self::Date(s) => s.general.width = val,
            _ => {}
        }
    }
}
pub trait PropertyRenderer {
    fn render(
        &self,
        space_id: String,
        object_id: String,
        info: PropertyInfo,
        settings: PropertySettings,
    ) -> Element;
}
#[derive(Clone, Debug, PartialEq)]
pub struct ViewInfo {
    pub id: String,
    pub name: String,
}
