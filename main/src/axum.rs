mod http_handler;
mod grpc_services;

use http::{StatusCode, Uri};
use lambda_http::Error;
use crate::grpc_services::{create_grpc_routes_axum, RestaurantServiceImp};

#[tokio::main]
async fn main()  {
    use axum::{ response::IntoResponse, };
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    async fn fallback(uri: Uri) -> (StatusCode, String) {
        (StatusCode::NOT_FOUND, format!("No route for {uri}"))
    }
    axum::serve(listener, create_grpc_routes_axum().fallback(fallback)).await.unwrap()
}