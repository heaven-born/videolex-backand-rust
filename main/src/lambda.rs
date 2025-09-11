use crate::http_handler::transport::restaurant_service_server::RestaurantService;
use http::{Request, Response};
use lambda_http::{run, service_fn, Body, Error};
use tonic::{IntoRequest, Status};
use tower::{Layer, Service, ServiceExt};

mod http_handler;
mod grpc_services;
use crate::grpc_services::{create_grpc_routes_axum, RestaurantServiceImp};
use crate::http_handler::transport::MenuRequest;
use crate::http_handler::transport::restaurant_service_server::RestaurantServiceServer;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let service = service_fn(handler);
    run(service).await
}

async fn handler(event: Request<Body>) -> Result<Response<Body>, Error> {
    let service = RestaurantServiceImp::default();

    match (event.method().as_str(), event.uri().path()) {
        // Handle GET_MENU via JSON
        ("POST", "/menu") => {
            let body = event.body();
            let menu_req: MenuRequest = serde_json::from_slice(body.as_ref())?;

            let tonic_request: tonic::Request<MenuRequest> = menu_req.into_request();
            let resp = service.get_menu(tonic_request)
                .await
                .map_err(|e: Status| Error::from(e.to_string()))?;

            let menu = resp.into_inner();
            let json_body = serde_json::to_string(&menu)?;

            Ok(Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .body(Body::Text(json_body))
                .unwrap())
        }

        _ => Ok(Response::builder()
            .status(404)
            .body(Body::Text("Not Found".to_string()))
            .unwrap()),
    }
}


//use bytes::Bytes;
//use http_body_util::Full;
//use crate::grpc_services::create_grpc_routes_axum;
/*
fn api_gateway_to_hyper(req: ApiGatewayV2httpRequest) -> Request < Full<Bytes> >
{
    // 1. Build the URI
    let path = req.raw_path.unwrap_or_else(|| "/".to_string());
    let query = req.raw_query_string.unwrap_or_default();
    let uri = if query.is_empty() {
        path.parse().unwrap()
    } else {
        format!("{}?{}", path, query).parse().unwrap()
    };

    // 2. Convert method
    let method = req.http_method;

    // 3. Convert body
    let body_bytes = match req.body {
        Some(s) => Bytes::from(s),
        None => Bytes::new(),
    };
  //  let body = Incoming::from(Full::new(body_bytes)); // wrap bytes as a Hyper Incoming body
    let body = Full::new(body_bytes); // Full<Bytes> implements http_body::Body


    // 4. Build request
    let mut builder = Request::builder()
        .uri(uri)
        .method(method);

    // 5. Add headers
    if let Some(headers) = req.headers {
        for (k, v) in headers.iter() {
            if let (Ok(name), Ok(value)) = (k.parse(), v.parse()) {
                builder = builder.header(name, value);
            }
        }
    }

    builder.body(body).unwrap()
}

 */