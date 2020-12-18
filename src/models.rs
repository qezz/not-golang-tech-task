use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateBuff {
    pub question: String,
    pub answer: String,
}
