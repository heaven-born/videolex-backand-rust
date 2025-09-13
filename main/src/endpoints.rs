use std::collections::HashSet;
use async_openai::Client;
use axum::Json;
use http::StatusCode;
use crate::ai::OpenIA;
use crate::ai::Ai;
use crate::transport::transport::{ExplainWordRequest, ExplainWordResponse};
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
pub async fn get_menu(Json(payload): Json<ExplainWordRequest>) -> Result<Json<ExplainWordResponse>, (StatusCode, Json<String>)> {
    let open_ai  = OpenIA::new(Client::new());

    let parts_of_speeches = HashSet::from(["adjective","noun","verb","phrasal verb","adverb", "pronoun", "preposition", "conjunction", "interjection", "other"]);
    let explained_word = open_ai.explain_word(
        &*payload.word,
        &*payload.context ,
        &*payload.native_language ,
        parts_of_speeches ,
    ).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())))?;

    Ok(Json(explained_word.into()))
}



