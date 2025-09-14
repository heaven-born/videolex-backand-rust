use crate::transport::transport::ExplainWordResponse;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub(crate) struct ExplainWordOutput {
    word_or_phrase: String,
    part_of_speech: String,
    attached_word_meaning: String,
    attached_word_meaning_native: String
}

impl From<ExplainWordOutput> for ExplainWordResponse {
    fn from(src: ExplainWordOutput) -> Self {
        ExplainWordResponse {
            word_or_phrase: src.word_or_phrase,
            part_of_speech: src.part_of_speech,
            attached_word_meaning: src.attached_word_meaning,
            attached_word_meaning_native: src.attached_word_meaning_native,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum Error {
    AiError(String),
    GeneralError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AiError(msg) => write!(f, "AI error: {}", msg),
            Error::GeneralError(msg) => write!(f, "General error: {}", msg),
        }
    }
}
