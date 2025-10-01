use openapi::models::ApimodelPeriodTag;
#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct PropertyID(pub String);
impl PropertyID {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
pub const NAME_PROPERTY_ID_STR: &str = "name_property_id";
#[derive(Clone, Debug)]
pub struct PropertyViewInfo {
    pub id: PropertyID,
    pub name: String,
    pub show: bool,
    pub options: Vec<ApimodelPeriodTag>,
    pub width: f64,
}
