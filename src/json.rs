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
    #[serde(rename = "Skin_Sens")]
    pub skin_sens: u32,

    #[serde(rename = "Skin_Sens_matched")]
    pub skin_sens_matched: String,

    #[serde(rename = "LD50_Oral")]
    pub ld50_oral: u32,

    #[serde(rename = "LD50_Oral_matched")]
    pub ld50_oral_matched: String,

    #[serde(rename = "Genotoxic")]
    pub genotoxic: u32,

    #[serde(rename = "Genotoxic_matched")]
    pub genotoxic_matched: String,

    #[serde(rename = "NonGenotoxic")]
    pub non_genotoxic: u32,

    #[serde(rename = "NonGenotoxic_matched")]
    pub non_genotoxic_matched: String,

    #[serde(rename = "clint_prediction_regression")]
    pub clint_prediction_regression: Option<f64>,

    #[serde(rename = "clint_prediction_classification")]
    pub clint_prediction_classification: Option<usize>,
}
