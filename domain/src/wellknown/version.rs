use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct Version {
    pub version: String,
}

impl Version {
    pub fn new() -> Self {
        Version {
            version: "v1.0.0".to_string(),
        }
    }
}