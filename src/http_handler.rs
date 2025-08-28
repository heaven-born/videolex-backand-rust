use async_openai::Client;
use async_openai::types::{ChatCompletionRequestMessage, ChatCompletionRequestUserMessage, CreateChatCompletionRequestArgs};
use lambda_http::{Body, Error, Request, Response};
use async_openai::types::ChatCompletionRequestUserMessageContent::Text;

pub mod transport {
    include!(concat!(env!("OUT_DIR"), "/transport.rs"));
}

use transport::restaurant_service_server::RestaurantServiceServer;

//pub struct MyGreeter {}

//impl RestaurantServiceServer<T> for MyGreeter { }

pub async fn function_handler(event: Request) -> Result<Response<Body>, Error> {

    match event.uri().path() {
        "/path1" => println!("First of x is 1, b =  "),
        "/path2" => println!("First of x is 1, b =  "),
        other_path => panic!("Path  {} not found.", other_path),
    }



    let client = Client::new();

    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-4")
        .messages([ChatCompletionRequestMessage::User(
            ChatCompletionRequestUserMessage {
                content: Text("Give me a recipe of shrimps".into()),
                name: None,
            }
        )])
        .max_tokens(300_u16)
        .build()?;

    let response = client
        .chat()
        .create(request)
        .await?;



    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(response.choices.first().unwrap().message.content.clone().unwrap().into())
        .map_err(Box::new)?;
    Ok(resp)
}



#[cfg(test)]
mod tests {
}
