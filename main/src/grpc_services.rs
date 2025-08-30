use tonic::{Request, Response, Status};
use tonic::service::Routes;
use crate::http_handler::transport::{MenuItem, MenuRequest, MenuResponse, OrderRequest, OrderResponse};
use crate::http_handler::transport::restaurant_service_server::{RestaurantService, RestaurantServiceServer};

#[derive(Debug, Default)]
pub struct RestaurantServiceImp {}

#[tonic::async_trait]
impl RestaurantService for RestaurantServiceImp {
    async fn get_menu(&self, request: Request<MenuRequest>) -> Result<Response<MenuResponse>, Status> {
        println!("!!!!!!!{:?}", request);
        Ok(Response::new(MenuResponse { items: vec![MenuItem{ name: Option::from("Pizza".to_string()), price: 10.0}] }))
    }

    async fn place_order(&self, request: Request<OrderRequest>) -> Result<Response<OrderResponse>, Status> {
        todo!()
    }
}


pub fn create_grpc_routes() -> Routes {
    let mut router = tonic::service::Routes::default();
    let imp = RestaurantServiceImp::default();
    router.add_service(RestaurantServiceServer::new(imp))
}

pub fn create_grpc_routes_axum() -> axum::routing::Router
{
    create_grpc_routes().into_axum_router()
}
