use dioxus::prelude::*;
use openapi::models::ApimodelTag;
#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct PropertyID(pub String);
impl PropertyID {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub enum DateTimeFormat {
    #[default]
    DateTime,
    Date,
    Time,
}
pub const NAME_PROPERTY_ID_STR: &str = "name_property_id";
#[derive(Clone, Debug, PartialEq)]
pub struct PropertyInfo {
    pub id: PropertyID,
    pub name: String,
    pub optional: OptionalInfo,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum OptionalInfo {
    Select(Vec<ApimodelTag>),
    Date,
    #[default]
    Other,
}

#[derive(Clone, Debug)]
pub struct GeneralPropertySettings {
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DateSettings {
    pub date_format: DateTimeFormat,
    pub width: f64,
    pub height: f64,
}
impl Default for DateSettings {
    fn default() -> Self {
        Self {
            date_format: DateTimeFormat::Date,
            width: 10.0,
            height: 10.0,
        }
    }
}
#[derive(Clone, Debug)]
pub enum PropertySettings {
    General(GeneralPropertySettings),
    Date(DateSettings),
}
impl Default for PropertySettings {
    fn default() -> Self {
        Self::General(GeneralPropertySettings {
            width: 10.0,
            height: 10.0,
        })
    }
}
impl PropertySettings {
    pub fn height(&self) -> f64 {
        match self {
            Self::General(s) => s.height,
            Self::Date(s) => s.height,
        }
    }
    pub fn width(&self) -> f64 {
        match self {
            Self::General(s) => s.width,
            Self::Date(s) => s.width,
        }
    }
    pub fn set_height(&mut self, val: f64) {
        match self {
            Self::General(s) => s.height = val,
            Self::Date(s) => s.height = val,
        }
    }

    pub fn set_width(&mut self, val: f64) {
        match self {
            Self::General(s) => s.width = val,
            Self::Date(s) => s.width = val,
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
