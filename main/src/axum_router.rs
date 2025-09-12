use axum::Router;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use utoipa_swagger_ui::SwaggerUi;
use crate::endpoints;

pub fn axum_router_wrapper() -> Router {

    let (router,  api_doc) = OpenApiRouter::<()>::new()
        .routes(routes!(endpoints::get_menu))  // Or auto from module (see notes)
        .split_for_parts();

    Router::new()
        .merge(router)
        .merge(SwaggerUi::new("/swagger-ui").url("/openapi.json", api_doc))  // Optional Swagger UI

}
