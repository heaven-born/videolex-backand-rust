use std::sync::Arc;
use async_openai::Client;
use axum::Router;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use utoipa_swagger_ui::SwaggerUi;
use crate::ai::OpenAI;
use crate::endpoints;

pub fn axum_router_wrapper() -> Router {

    let open_ai = Arc::new(OpenAI {client:Client::new()});

    let (router,  api_doc) = OpenApiRouter::<Arc<OpenAI>>::new()
        .routes(routes!(endpoints::get_menu))
        .routes(routes!(endpoints::tts))
        .split_for_parts();

    Router::new()
        .merge(router)
        .merge(SwaggerUi::new("/swagger-ui").url("/openapi.json", api_doc))
        .with_state(open_ai)

}
