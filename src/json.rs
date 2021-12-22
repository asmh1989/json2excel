use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct D {
    #[serde(rename = "SMILES")]
    pub smiles: String,
    #[serde(rename = "ID")]
    pub id: String,
}
