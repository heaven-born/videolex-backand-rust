mod grpc_services;

use std::convert::Infallible;
use std::pin::Pin;
use std::task::{Context, Poll};
use lambda_http::{Error};
use tower::{ServiceBuilder, ServiceExt};
use axum::{routing::get, Router};
use futures::Stream;
use http::Response;
use http_body_util::combinators::{BoxBody, Frame};
use hyper::Request;
use hyper::body::Incoming;
use tonic::body::Body;
use tonic::service::RoutesBuilder;


mod http_handler;
use crate::grpc_services::RestaurantServiceImp;
use crate::http_handler::transport::restaurant_service_server::RestaurantServiceServer;


pub mod proto {
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("service_descriptor");
}

#[tokio::main]
async fn main() -> Result<(), Error> {

    //let addr = "[::1]:50051".parse()?;
    let imp = RestaurantServiceImp::default();

    //println!("GreeterServer listening on {}", addr);

    //use crate::http_handler::transport::*;
    //use crate::http_handler::transport::restaurant_service_server::RestaurantService;

    //let router = map_to_tonic!(RestaurantServiceImp::default(), RestaurantService);


    let app: Router<()> = Router::new().route("/", get(root));
    async fn root() -> &'static str {
        "Hello, World!"
    }


    //let mut router = tonic::service::Routes::default();
    //router.add_service(app[])

    //let rest_router = Router::new().route("/hello", get(root));


    //let tonic_router = tonic::service::Routes::default();

    //let kk = tonic_router.clone().merge(app);

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build_v1alpha()
        .unwrap();
  /*
    Server::builder()
        .add_routes(router)
        .add_service(RestaurantServiceServer::new(imp))
        .add_service(reflection_service)
        .serve(addr)
        .await?;
   */
   // let http_router = Router::new()
     //   .route("/", get(|| async { "Hello from Axum!" }));

    //let axum_routerl = http_router.into_service::<_>();

    //let grpc_svc = ServiceBuilder::new().service(RestaurantServiceServer::new(RestaurantServiceImp::default()));
    //let http_svc = ServiceBuilder::new().service(http_router);


    let mut builder = RoutesBuilder::default();
    builder.add_service(RestaurantServiceServer::new(RestaurantServiceImp::default()));
    builder.routes();

    use axum::body::Body as AxumBody;
    use hyper::body::Body as HyperBody;
    //use hyper::body::Body;
    use hyper::Response as HyperResponse;
    use tonic::body::Body as TonicBody;
    use tonic::Response as TonicResponse;
    use http_body::Body as HttpBody;
    use bytes::Bytes;



    struct AxumBodyStream<B: HttpBody> {
        body: B,
    }

    impl<B: HttpBody> AxumBodyStream<B> {
        fn new(body: B) -> Self {
            AxumBodyStream { body }
        }
    }

    impl Stream for AxumBodyStream<AxumBody> {
        type Item = Result<Bytes, Box<dyn std::error::Error + Send + Sync>>;

        fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            match Pin::new(&mut self.body).poll_frame(cx) {
                Poll::Ready(Some(Ok(frame))) => {
                    // Extract data from the frame, if present
                    if let Some(data) = frame.into_data().ok() {
                        Poll::Ready(Some(Ok(data)))
                    } else {
                        // Skip non-data frames (e.g., trailers) or handle as needed
                        Poll::Ready(Some(Ok(Bytes::new())))
                    }
                }
                Poll::Ready(Some(Err(e))) => {
                    // Error is already Box<dyn std::error::Error + Send + Sync>
                    Poll::Ready(Some(Err(e.into())))
                }
                Poll::Ready(None) => Poll::Ready(None), // Stream is done
                Poll::Pending => Poll::Pending, // Not ready yet
            }
        }
    }
    //use hyper::server::Server;

    use futures_util::stream::{self};
    use http_body_util::BodyExt;
    use http_body_util::StreamBody;

    /*
    fn convert_to_box_body<B>(body: B) -> BoxBody<Bytes, Status>
    where
        B: http_body::Body<Data = Bytes, Error = Status> + Send + 'static,
    {
        let arc_body = Arc::new(body);
        BoxBody::new(arc_body)
    }*/

    async fn closure(req: Request<Incoming>) -> Result<Response<BoxBody<Bytes,Error>>, String> {
        let http_router = Router::new()
            .route("/", get(|| async { "Hello from Axum!" }));
        let grpc = ServiceBuilder::new().service(RestaurantServiceServer::new(RestaurantServiceImp::default()));
        let http = ServiceBuilder::new().service(http_router);
            if req.uri().path().starts_with("/proto/") {
                let kk: Result<Response<Body>, Infallible> = grpc.oneshot(req).await;
                let (parts, body) = kk.unwrap().into_parts();
                //let hyper_body = HyperBody::wrap_stream(TonicBodyStream::new(body));
                //let body: StreamBody<_> = StreamBody::new(TonicBodyStream::new(body));
                //let body: Box<dyn Stream<Item=_>> = Box::new(TonicBodyStream::new(body));
                //let httpBody = Box::new(StreamBody::new(TonicBodyStream::new(body)));
                //convert_to_box_body(body);
                //let tt = TonicBodyStream::new(body).boxed();



                //BoxBody::new(body);
                //let hyper_body:BoxBody<Bytes,Error>   = stream.boxed().into();
                //hyper::body::Body::wrap_stream(stream)

                //BoxBody::default();
                //let body = BoxBody::new(Body::default()); // This is Send + Sync
                Ok(Response::from_parts(parts, BoxBody::default()))
            } else {
                let axum_resp = http.oneshot(req).await;
                let (parts, body) = axum_resp.unwrap().into_parts();
                //let hyper_body = HyperBody::wrap_stream(AxumBodyStream::new(body));
                //let body: StreamBody<_> = StreamBody::new(AxumBodyStream::new(body));
                //let body: Box<dyn Stream<Item=_>> = Box::new(AxumBodyStream::new(body));
                //BoxBody::new(body);
                //let boxed_body = body.boxed();
                Ok(Response::from_parts(parts, BoxBody::default()))
            }
    }

    let merged_service = hyper::service::service_fn(closure);




    use hyper::service::service_fn;
    use hyper_util::rt::TokioIo;
    use hyper_util::server::conn::auto::Builder;


    /*
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    println!("Listening on http://127.0.0.1:3000");

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::spawn(async move {
            //let service = service_fn(merged_service);
            //let service = Shared::new(service_fn(merged_service));

            if let Err(err) = Builder::new(TokioExecutor::new()).serve_connection(io,merged_service).await {
                eprintln!("server connection error: {err}");
            }
        });
    }

     */

    //Ok(());
    //let mut router = tonic::service::Routes::default();
    //router.add_service(RestaurantServiceServer::new(imp))
        //.into_axum_router()
        //.route("/", get(|| async { "Hello from Axum!" }))
    //let rr = router;
    //let axum_router = std::mem::replace(router.axum_router_mut(), Router::new())
        //.route("/test", get(|| async { "Hello from Axum!" }));
    //*router.axum_router_mut() = axum_router;

    /*
    let addr = "[::1]:50051".parse()?;
    TonicServer::builder()
        ///.add_routes(create_grpc_routes())
        .add_service(RestaurantServiceServer::new(imp))
        .add_service(reflection_service)
        .serve(addr)
        .await?;
        
     */




    Ok(())

}