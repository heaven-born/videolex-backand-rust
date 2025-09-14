
mod transport;
mod endpoints;
mod axum_router;
mod ai;
mod domain;

use ::axum::body::to_bytes;
use ::axum::Router;
use http::{Request, Response};
use lambda_http::{run, service_fn, Body, Error, RequestExt};
use tower::ServiceExt;
use lambda_http::Body as LambdaBody;
use axum_core::response::Response as AxumResponse;
use lambda_http::request::RequestContext::{ApiGatewayV1, ApiGatewayV2};
use crate::axum_router::axum_router_wrapper;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler = |event| handle_request(event, axum_router_wrapper().clone());
    println!("Starting lambda");
    run(service_fn(handler)).await
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
    let (_parts, body) = event.into_parts();


    let mut builder = Request::builder();
    builder.headers_mut().unwrap().extend(_parts.headers.clone());

    let stage = match _parts.request_context() {
        ApiGatewayV2(ref ctx) => ctx.stage.clone().unwrap_or_default(),
        ApiGatewayV1(ref ctx) => ctx.stage.clone().unwrap_or_default(),
        _ => "".to_string()
    };

    let stage_prefix = if stage.is_empty() { "" } else { "/" };

    let path_no_stage = _parts.uri.path()
        .strip_prefix(&format!("{stage_prefix}{stage}"))
        .unwrap_or(_parts.uri.path());


    let http_request: http::Request<LambdaBody> = builder
        .method(_parts.method.as_str())
        .uri(path_no_stage)
        .body(body.into())?;

    let response = router
        .oneshot(http_request)
        .await
        .map_err(|e| anyhow::anyhow!("Router error: {}", e))?;


    Ok(convert_axum_to_lambda(response).await)
}