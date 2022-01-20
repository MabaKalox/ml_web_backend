use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TextManipulationMod {
    Summarization,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextManipulation {
    pub text: Vec<String>,
    pub mode: TextManipulationMod,
}
