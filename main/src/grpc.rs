mod grpc_services;

use lambda_http::Error;


mod http_handler;


pub mod proto {
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("service_descriptor");
}

#[tokio::main]
async fn main() -> Result<(), Error> {

    //let addr = "[::1]:50051".parse()?;
    //let imp = RestaurantServiceImp::default();

    //let mut builder = RoutesBuilder::default();
    //builder.add_service(RestaurantServiceServer::new(RestaurantServiceImp::default()));
    //builder.routes();

    Ok(())

}