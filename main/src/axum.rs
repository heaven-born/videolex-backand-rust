
mod grpc_services;
mod http_handler;

use http::{StatusCode, Uri};
use grpc_services::axum_router_wrapper;

#[tokio::main]
async fn main()  {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    async fn fallback(uri: Uri) -> (StatusCode, String) {
        (StatusCode::NOT_FOUND, format!("No route for {uri}"))
    }
    axum::serve(listener, axum_router_wrapper().fallback(fallback)).await.unwrap()
}

