#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct PropertyID(pub String);
impl PropertyID {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
