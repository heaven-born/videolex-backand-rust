use axum::Router;
use axum::routing::{get};
use lambda_http::{Error};
mod http_handler;

#[tokio::main]
async fn main() -> Result<(), Error> {


    // Extract some useful information from the request
    let app: Router<()> = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root));
    // `POST /users` goes to `create_user`
    //.route("/users", get(root));



    async fn root() -> &'static str {
        "Hello, World!"
    }

    Ok(())

}