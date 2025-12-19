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
    pub options: Vec<ApimodelTag>,
}

#[derive(Clone, Debug)]
pub struct PropertySettings {
    pub date_format: DateTimeFormat,
    pub width: f64,
    pub height: f64,
    pub show: bool,
}
impl Default for PropertySettings {
    fn default() -> Self {
        Self {
            date_format: DateTimeFormat::default(),
            width: 20.0,
            height: 5.0,
            show: true,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ViewInfo {
    pub id: String,
    pub name: String,
}
