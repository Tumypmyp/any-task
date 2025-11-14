use openapi::models::ApimodelPeriodTag;
#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct PropertyID(pub String);
impl PropertyID {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
#[derive(Clone, Debug, PartialEq)]
pub enum DateTimeFormat {
    DateTime,
    Date,
    Time,
}
pub const NAME_PROPERTY_ID_STR: &str = "name_property_id";
#[derive(Clone, Debug)]
pub struct PropertyInfo {
    pub id: PropertyID,
    pub name: String,
    pub options: Vec<ApimodelPeriodTag>,
    pub date_format: DateTimeFormat,
    pub width: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ViewInfo {
    pub id: String,
    pub name: String,
}
