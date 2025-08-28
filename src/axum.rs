use axum::Router;
use axum::routing::{get};
use lambda_http::{Error};
mod http_handler;

use serde_json;
use tonic::{Request, Status};
use crate::http_handler::transport::MenuRequest;

// Macro to generate json_to_menu_request function
macro_rules! generate_json_to_request {
    // Match the trait and method signature, extracting the request type
    ($trait_name:ident, $method_name:ident, $request_type:path) => {
        // Generate the json_to_menu_request function
        fn json_to_menu_request(json_str: &str) -> Result<String,$request_type> {
            // Deserialize JSON string into the request type
            // Wrap in tonic::Request
            Ok(stringify!($method_name).to_string()) // Replace with actual method call logic
        }
    };
}

// Example usage with RestaurantService trait
generate_json_to_request!(RestaurantService, get_menu, MenuRequest);

#[tokio::main]
async fn main() -> Result<(), Error> {

    let r = json_to_menu_request("test");
    println!("{:?}", r);
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