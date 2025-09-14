use std::collections::HashSet;
use std::sync::Arc;
use axum::extract::State;
use axum::Json;
use http::StatusCode;
use crate::ai::Ai;
use crate::transport::transport::{ExplainWordRequest, ExplainWordResponse, TtsRequest, TtsResponse};
#[utoipa::path(
    post,
    path = "/explain_word",
    responses(
        (status = 200, description = "Explains word", body = ExplainWordResponse),
        (status = INTERNAL_SERVER_ERROR)
    ),
    request_body(
         content_type = "application/json",
         content = ExplainWordRequest,
    )
)]
pub async fn get_menu(State(open_ai): State<Arc<impl Ai>>, Json(payload): Json<ExplainWordRequest>) -> Result<Json<ExplainWordResponse>, (StatusCode, Json<String>)> {
    let parts_of_speeches = HashSet::from(["adjective","noun","verb","phrasal verb","adverb", "pronoun", "preposition", "conjunction", "interjection", "other"]);
    let explained_word = open_ai.explain_word(
        payload.word.as_str(),
        &*payload.context.as_str(),
        &*payload.native_language.as_str() ,
        parts_of_speeches,
    ).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())))?;

    Ok(Json(explained_word.into()))
}



#[utoipa::path(
    post,
    path = "/text-to-speech",
    responses(
        (status = 200, description = "TTS", body = TtsResponse),
        (status = INTERNAL_SERVER_ERROR)
    ),
    request_body(
         content_type = "application/json",
         content = TtsRequest,
    )
)]
pub async fn tts(State(open_ai): State<Arc<impl Ai>>, Json(payload): Json<TtsRequest>) -> Result<Json<TtsResponse>, (StatusCode, Json<String>)> {
    let resp = open_ai.tts(
        payload.text.as_str(),
        payload.instruction.as_str()
    ).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())))?;

    Ok(Json(TtsResponse{base64_data:resp}))
}



