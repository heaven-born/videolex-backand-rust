use axum::{Json, Router};
use axum::routing::post;
use http::StatusCode;
use tonic::{IntoRequest, Request, Response, Status};
use crate::grpc_services::service_api::{get_menu};
use crate::http_handler::transport::{InternalErrorResponse, MenuItem, MenuRequest, MenuResponse};
use crate::http_handler::transport::restaurant_service_server::RestaurantService;


#[derive(Debug, Default)]
pub struct RestaurantServiceImp {}

#[tonic::async_trait]
impl RestaurantService for RestaurantServiceImp {
    async fn get_menu(&self, request: Request<MenuRequest>) -> Result<Response<MenuResponse>, Status> {

        println!("!!!!!!!{:?}", request);
        Ok(Response::new(MenuResponse { items: vec![MenuItem{ name: Option::from("Pizza".to_string()), price: 10.0}] }))
    }
}


mod service_api {
    use axum::Json;
    use http::StatusCode;
    use tonic::{IntoRequest};
    use crate::grpc_services::RestaurantServiceImp;
    use crate::http_handler::transport::{InternalErrorResponse, MenuRequest, MenuResponse};
    use crate::http_handler::transport::restaurant_service_server::RestaurantService;


    #[utoipa::path(
        post,
        path = "/menu",
        responses(
            (status = 200, description = "Menu found successfully", body = MenuResponse),
            (status = NOT_FOUND, description = "Pet was not found")
        ),
        request_body(
             content_type = "application/json",
             content = MenuRequest,
             description = "A description "
        )
    )]
    pub async fn get_menu(Json(payload): Json<MenuRequest>) -> Result<Json<MenuResponse>, (StatusCode, Json<String>)> {
        let service = RestaurantServiceImp::default();

        let request: tonic::Request<MenuRequest>  = payload.into_request();
        let error = InternalErrorResponse{error_message: "oops".parse().unwrap() };
        service.get_menu(request)
            .await
            .map(|resp|Json(resp.into_inner()))
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(format!("Error: {}. Status code: {}",error.error_message,e.code()))))
    }

}


pub fn axum_router_wrapper() -> Router {



    Router::new().route("/menu", post(get_menu))
}
