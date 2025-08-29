use tonic::transport::Server;
mod grpc_services;
use lambda_http::{Error};

mod http_handler;
use crate::grpc_services::RestaurantServiceImp;
use crate::http_handler::transport::restaurant_service_server::RestaurantServiceServer;


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