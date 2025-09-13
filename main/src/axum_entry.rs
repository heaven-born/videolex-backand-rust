
mod endpoints;
mod transport;
mod axum_router;
mod ai;
mod domain;

use http::{StatusCode, Uri};
use crate::axum_router::axum_router_wrapper;

#[tokio::main]
async fn main()  {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    async fn fallback(uri: Uri) -> (StatusCode, String) {
        (StatusCode::NOT_FOUND, format!("No route for {uri}"))
    }
    axum::serve(listener, axum_router_wrapper().fallback(fallback)).await.unwrap()
}

