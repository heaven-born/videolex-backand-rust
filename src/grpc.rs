use lambda_http::{Error};
use tonic::{Request, Response, Status};
use tonic::transport::Server;

mod http_handler;
use http_handler::transport::restaurant_service_server::RestaurantService;
use crate::http_handler::transport::{MenuRequest, MenuResponse, OrderRequest, OrderResponse};
use crate::http_handler::transport::restaurant_service_server::RestaurantServiceServer;

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

pub mod proto {
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("service_descriptor");
}

#[tokio::main]
async fn main() -> Result<(), Error> {

    let addr = "[::1]:50051".parse()?;
    let imp = RestaurantServiceImp::default();

    println!("GreeterServer listening on {}", addr);

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build_v1alpha()
        .unwrap();

    Server::builder()
        .add_service(RestaurantServiceServer::new(imp))
        .add_service(reflection_service)
        .serve(addr)
        .await?;

    Ok(())

}