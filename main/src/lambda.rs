use bytes::Bytes;
use futures_util::StreamExt;
use http_body_util::BodyExt;
use lambda_http::{run, service_fn, Body, Error};
use lambda_http::Body::Binary;
use tonic_web::GrpcWebCall;
use tower::{Layer, Service, ServiceExt};

mod http_handler;
mod grpc_services;
use crate::grpc_services::{create_grpc_routes, create_grpc_routes_axum};

#[tokio::main]
async fn main() -> Result<(), Error> {

    let service = tower::ServiceBuilder::new().service(create_grpc_routes_axum());

    //let grpc_service = create_grpc_routes();
    let service = tonic_web::GrpcWebLayer::new().layer(service);

    let func = service_fn(move |event: http::Request<lambda_http::Body> | {
        let mut service = service.clone();
        println!("{:?}", event);

        async move {
            let resp = service.oneshot(event).await;

            let (status,resp) = match resp {
                Ok(resp) => {
                    println!("0-0{:?}", resp);
                    (resp.status(),resp)
                },
                Err(e) => {
                   panic!("Error was: {:?}", e);
                }
            };
            //let status = resp.unwrap().status();
            //println!("Error: {:?}", resp.err());

            //let mut body_bytes = Vec::new();
            //GrpcWebCall::
            let  body =resp.into_body();
            println!("status : {:?}", status);
            //let hyper_body: UnsyncBoxBody<Binary, String> = body.into();
            /*let mut stream = body.into_data_stream();

            while let Some(chunk) = stream.next().await {
                let chunk: Result<Bytes, _> = chunk;
                body_bytes.extend_from_slice(&chunk.unwrap());
            }*/


            // Build Lambda-compatible response
            let lambda_resp = lambda_http::Response::builder()
                .status(status)
                .header("content-type", "application/grpc") // preserve gRPC content-type
                .body(body).unwrap();
                //.unwrap();

            println!("{:?}", lambda_resp);
            //lambda_resp
            Ok::<lambda_http::Response<tonic::body::Body>, String>(lambda_resp)
        }
    });

    run(func).await
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