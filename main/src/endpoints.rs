use std::collections::HashSet;
use std::sync::Arc;
use axum::extract::State;
use axum::Json;
use http::StatusCode;
use crate::ai::Ai;
use crate::transport::transport::{ExplainWordRequest, ExplainWordResponse, GuessCefrlLevelRequest, GuessCefrlLevelResponse, TtsRequest, TtsResponse, WordCardRequest, WordCardResponse};
#[utoipa::path(
    post,
    path = "/explain-word",
    responses(
        (status = 200, description = "Explains word", body = ExplainWordResponse),
        (status = INTERNAL_SERVER_ERROR)
    ),
    request_body(
         content_type = "application/json",
         content = ExplainWordRequest,
    )
)]
pub async fn explain_word(State(open_ai): State<Arc<impl Ai>>, Json(payload): Json<ExplainWordRequest>) -> Result<Json<ExplainWordResponse>, (StatusCode, Json<String>)> {
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


#[utoipa::path(
    post,
    path = "/word-card",
    responses(
        (status = 200, description = "Explains word", body = WordCardResponse),
        (status = INTERNAL_SERVER_ERROR)
    ),
    request_body(
         content_type = "application/json",
         content = WordCardRequest,
    )
)]
pub async fn word_card(State(open_ai): State<Arc<impl Ai>>, Json(payload): Json<WordCardRequest>) -> Result<Json<WordCardResponse>, (StatusCode, Json<String>)> {
    let pos = payload.part_of_speech.as_str();
    let word = payload.word;

    let prompt = format!("Explain meaning of English word '{word}({pos})' in 60 words. Provide examples. Use simple English and very simple vocabulary. Use plain formatting (not markdown).");

    let meaning = open_ai.ask(
        prompt.as_str()
    ).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())))?;

    //let make_this_way= "imbued with emotions on people's faces if possible";
    let example_image_gen_prompt = format!("Extract one example from the given bellow definition that better fits for visualizing ${pos} English word '${word}'. Give me extended 100 words description of a picture or photo of the extracted example. Do not provide the extracted example. Just give me the picture description. Definition: {meaning}");

    let example_image_prompt = open_ai.ask(
        example_image_gen_prompt.as_str()
    ).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())))?;

    let image_base64 = open_ai.gen_image(
        example_image_prompt.as_str()
    ).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())))?;

    let voice = open_ai.tts(
        meaning.as_str(), ""
    ).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())))?;

    let resp = WordCardResponse{ img_base64: image_base64, voice_base64: voice, explanation: meaning };
    Ok(Json(resp.into()))
}


#[utoipa::path(
    post,
    path = "/guess-cefr-word-level",
    request_body(
         content_type = "application/json",
         content = GuessCefrlLevelRequest,
    ),
    responses(
        (status = 200, description = "Guess level", body = GuessCefrlLevelResponse),
        (status = INTERNAL_SERVER_ERROR)
    ),
)]
pub async fn guess_cefr_word_level(State(open_ai): State<Arc<impl Ai>>, Json(payload): Json<GuessCefrlLevelRequest>) -> Result<Json<GuessCefrlLevelResponse>, (StatusCode, Json<String>)> {
    let resp = open_ai.cefr_word_level(
        payload.word.as_str(), payload.part_of_speech.as_str()
    ).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())))?;
    let resp = GuessCefrlLevelResponse{ level: resp.level};
    Ok(Json(resp.into()))
}





