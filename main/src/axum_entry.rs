
#[path = "endpoints.rs"]
mod endpoints;
#[path = "transport.rs"]
mod transport;

use axum::{Json, Router};
use axum::routing::{get, post};
use http::{StatusCode, Uri};
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main()  {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    async fn fallback(uri: Uri) -> (StatusCode, String) {
        (StatusCode::NOT_FOUND, format!("No route for {uri}"))
    }
    axum::serve(listener, axum_router_wrapper().fallback(fallback)).await.unwrap()
}


pub fn axum_router_wrapper() -> Router {

    let (router,  api_doc) = OpenApiRouter::<()>::new()
        .routes(routes!(endpoints::get_menu))  // Or auto from module (see notes)
        .split_for_parts();

    Router::new()
        .merge(router)
        .merge(SwaggerUi::new("/swagger-ui").url("/openapi.json", api_doc))  // Optional Swagger UI

}
