use std::sync::Arc;
use async_openai::Client;
use axum::Router;
use http::{StatusCode, Uri};
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use utoipa_swagger_ui::SwaggerUi;
use crate::ai::OpenAI;
use crate::endpoints;

pub fn axum_router_wrapper() -> Router {

    let open_ai = Arc::new(OpenAI {client:Client::new()});

    let (router,  api_doc) = OpenApiRouter::<Arc<OpenAI>>::new()
        .routes(routes!(endpoints::explain_word))
        .routes(routes!(endpoints::tts))
        .routes(routes!(endpoints::word_card))
        .split_for_parts();

    async fn fallback(uri: Uri) -> (StatusCode, String) {
        (StatusCode::NOT_FOUND, format!("No route for {uri}"))
    }

    Router::new()
        .merge(router)
        .merge(SwaggerUi::new("/swagger-ui").url("/openapi.json", api_doc))
        .with_state(open_ai)
        .fallback(fallback)


}
