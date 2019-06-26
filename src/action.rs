/// Represents variant types of content action
#[derive(Serialize, Deserialize, Debug)]
pub enum Action {
    Clear,
    Get,
    Set(String),
}
