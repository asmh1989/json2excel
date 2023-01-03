use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct D {
    #[serde(rename = "SMILES")]
    pub smiles: String,
    #[serde(rename = "ID")]
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActureHerg {
    #[serde(rename = "smiles", alias = "SMILES")]
    pub smiles: String,
    #[serde(rename = "Acute_Toxicity")]
    pub acute_toxicity: bool,
    #[serde(rename = "hERG")]
    pub herg: bool,
    #[serde(rename = "fu")]
    pub fu: u32,
    #[serde(rename = "cl")]
    pub cl: f64,
    #[serde(rename = "vdss")]
    pub vdss: f64,
}
