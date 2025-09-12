use axum::{Json, Router};
use axum::routing::post;
use http::StatusCode;
use tonic::{IntoRequest, Request, Response, Status};

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
    use http::StatusCode;
    use tonic::{IntoRequest, Status};
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
    pub(crate) async fn get_menu(payload: MenuRequest) -> Result<MenuResponse, (StatusCode, String)> {

        let service = RestaurantServiceImp::default();

        let tonic_request: tonic::Request<MenuRequest> = payload.into_request();
        let error = InternalErrorResponse{error_message: "oops".parse().unwrap() };
        service.get_menu(tonic_request)
            .await
            .map(|resp|resp.into_inner())
            .map_err(|e: Status| (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}. Status code: {}",error.error_message,e.code())))
    }

}


pub fn axum_router_wrapper() -> Router {

    async fn get_menu_wrapper(Json(payload): Json<MenuRequest>) -> Result<Json<MenuResponse>, (StatusCode, Json<String>)> {
        let request: MenuRequest = payload.into();
        let res = service_api::get_menu(request).await;
        res.map(|resp|Json(resp)).map_err(|e| (e.0, Json(e.1)))
    }

    //async fn json_wrapper<T,K,L>(f: impl Fn(T) -> Result<K, (StatusCode, L)>) -> Fn(Json<T>) -> Result<Json<K>, (StatusCode, Json<L>)> {

    //}


    Router::new().route("/menu", post(get_menu_wrapper))
}
