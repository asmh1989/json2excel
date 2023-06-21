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
    pub acute_toxicity: Option<bool>,
    #[serde(rename = "hERG")]
    pub herg: Option<bool>,
    #[serde(rename = "hERG_Finetune")]
    pub herg_finetune: Option<f64>,
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

    #[serde(rename = "pred_class_permeab")]
    pub pred_class_permeab: Option<usize>,

    #[serde(rename = "pred_class_permeability")]
    pub pred_class_permeability: Option<usize>,

    #[serde(rename = "pred_class_solub")]
    pub pred_class_solub: Option<usize>,

    #[serde(rename = "pred_Peff")]
    pub pred_peff: Option<usize>,

    #[serde(rename = "logS")]
    pub log_s: Option<f64>,

    #[serde(rename = "LogP")]
    pub log_p: Option<f64>,
    #[serde(rename = "ChemAxon_logD")]
    pub chemaxon_logd: Option<f64>,
}
