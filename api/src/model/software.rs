use serde::Deserialize;

// ---------------------------------------------------------------------------------------------------------------
#[derive(Debug, Deserialize)]
pub struct Software {
    pub name: String,
    pub software_version: f32,
    pub description: String,
    pub developer: String,
    pub version: Option<i32>,
}
