use tonic::{Request, Response, Status};
use crate::http_handler::transport::{MenuRequest, MenuResponse, OrderRequest, OrderResponse};
use crate::http_handler::transport::restaurant_service_server::{RestaurantService, RestaurantServiceServer};

#[derive(Debug, Default)]
pub struct RestaurantServiceImp {}

#[tonic::async_trait]
impl RestaurantService for RestaurantServiceImp {
    async fn get_menu(&self, request: Request<MenuRequest>) -> Result<Response<MenuResponse>, Status> {
        Ok(Response::new(MenuResponse { items: vec![] }))
    }

    async fn place_order(&self, request: Request<OrderRequest>) -> Result<Response<OrderResponse>, Status> {
        todo!()
    }
}
