use std::collections::HashSet;
use async_openai::Client;
use async_openai::config::OpenAIConfig;
use async_openai::{
    types::{ CreateChatCompletionRequestArgs},
};
use async_openai::types::{ChatCompletionRequestUserMessage,ResponseFormatJsonSchema};
use async_openai::types::ResponseFormat;
use async_openai::types::ChatCompletionRequestMessage::User;
use async_openai::types::ChatCompletionRequestUserMessageContent::Text;
use utoipa::gen::serde_json::json;
use crate::domain::{Error, ExplainWordOutput};


pub(crate) trait Ai {
    fn new(client: Client<OpenAIConfig>) -> Self;
    async fn explain_word(&self, word: &str, context: &str, native_language: &str, available_part_of_speeches: HashSet<&str> ) -> Result<ExplainWordOutput,Error>;


}

pub struct OpenIA {
    pub(crate) client: Client<OpenAIConfig>}
impl Ai for OpenIA{
    fn new(client: Client<OpenAIConfig>) -> OpenIA {
        OpenIA { client }
    }
    async fn explain_word(
        &self,
        word: &str,
        context: &str,
        native_language: &str,
        available_part_of_speeches: HashSet<&str>,
    ) -> Result<ExplainWordOutput,Error> {

        let json_schema = ResponseFormatJsonSchema{
            description: None,
            name: "explain_word".to_string(),
            strict: Some(true),
            schema: Some(json!(
                {
                    "type": "object",
                    "properties": {
                        "word_or_phrase": {
                            "type": "string",
                            "description": format!("Bare Infinitive form for word or phrasal verb '{word}' ")
                        },
                        "part_of_speech": {
                            "type": "string",
                            "enum": available_part_of_speeches,
                            "description": "Identified part of speech of the word"
                        },
                        "attached_word_meaning": {
                            "type": "string",
                            "description": "Brief meaning of the word."
                        },
                        "attached_word_meaning_native": {
                            "type": "string",
                            "description": format!("Translation to {native_language} or brief meaning in {native_language}.")
                        }
                    },
                    "additionalProperties": false,
                    "required": ["word_or_phrase", "part_of_speech", "attached_word_meaning", "attached_word_meaning_native"],

                }
            )),
        };
        let response_format = ResponseFormat::JsonSchema {json_schema};

        let request = CreateChatCompletionRequestArgs::default()
            .response_format(response_format)
            .model("gpt-4.1")
            .messages(vec![
                User(
                    ChatCompletionRequestUserMessage{
                        content: Text(
                            format!("Give me a bare infinitive form for '{word}' along with the identified part of speech. If this is phrasal verb then prioritize it over verb part of speech. The context where this text was taken from: \n+{context}")
                        ),
                        name: None,
                    }
                )

            ])
            .build().unwrap();

        let resp = self.client.chat().create(request).await
            .map_err(|_e| Error::AiError(_e.to_string()))?;
        let choose = resp.choices.first().unwrap().clone();
        let content = choose.message.content.unwrap();
        serde_json::from_str(content.as_str()).map_err(|e| {
            eprintln!("Error deserializing response: {}", e);
            Error::GeneralError(e.to_string())
        })
    }
}

