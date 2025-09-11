
mod http_handler;
mod grpc_services;

use axum::body::to_bytes;
use axum::Router;
use http::{Request, Response};
use lambda_http::{run, service_fn, Body, Error, RequestExt};
use tower::ServiceExt;
use lambda_http::Body as LambdaBody;
use axum_core::response::Response as AxumResponse;
use grpc_services::axum_router_wrapper;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(|event:  Request<Body>| async {
        handle_request(event, axum_router_wrapper().clone()).await
    })).await
}

async fn convert_axum_to_lambda(axum_resp: AxumResponse) -> Response<LambdaBody> {
    let (parts, body) = axum_resp.into_parts();
    let bytes = to_bytes(body,usize::MAX).await.unwrap();

    let lambda_body = if bytes.is_empty() {
        LambdaBody::Empty
    } else if let Ok(text) = String::from_utf8(bytes.to_vec()) {
        LambdaBody::Text(text)
    } else {
        LambdaBody::Binary(bytes.to_vec())
    };
    Response::from_parts(parts, lambda_body)
}

async fn handle_request(event: Request<Body>, router: Router) -> Result<Response<Body>, Error> {
    let (ref _parts, body) = event.into_parts();

    let mut builder = Request::builder();
    builder.headers_mut().unwrap().extend(_parts.headers.clone());
    let http_request: http::Request<LambdaBody> = builder
        .method(_parts.method.as_str())
        .uri(format!("{}?{:?}", _parts.uri, _parts.query_string_parameters()))
        .body(body.into())?;

    let response = router
        .oneshot(http_request)
        .await
        .map_err(|e| anyhow::anyhow!("Router error: {}", e))?;


    Ok(convert_axum_to_lambda(response).await)
}