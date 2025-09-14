use std::collections::HashSet;
use async_openai::Client;
use async_openai::config::OpenAIConfig;
use async_openai::{
    types::{ CreateChatCompletionRequestArgs},
};
use async_openai::types::{ChatCompletionRequestUserMessage, CreateSpeechRequestArgs, ResponseFormatJsonSchema, SpeechModel, Voice};
use async_openai::types::ResponseFormat;
use async_openai::types::ChatCompletionRequestMessage::User;
use async_openai::types::ChatCompletionRequestUserMessageContent::Text;
use utoipa::gen::serde_json::json;
use crate::domain::{Error, ExplainWordOutput};
use base64::{engine::general_purpose, Engine as _};



pub(crate) trait Ai {
    async fn explain_word(&self, word: &str, context: &str, native_language: &str, available_part_of_speeches: HashSet<&str> ) -> Result<ExplainWordOutput,Error>;
    async fn tts(&self, text: &str, instructions: &str ) -> Result<String,Error>;


}

pub struct OpenAI {
    pub(crate) client: Client<OpenAIConfig>}
impl Ai for OpenAI {
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

        let content =
            self.client.chat().create(request).await
            .map_err(|_e| Error::AiError(_e.to_string()))?
            .choices.first().map( |c| c.message.content.clone()).flatten()
            .ok_or(Error::AiError("No content in result".to_string()))?;

        serde_json::from_str(content.as_str()).map_err(|e| {
            eprintln!("Error deserializing response: {}", e);
            Error::GeneralError(e.to_string())
        })
    }

    async fn tts(&self, text: &str, instructions: &str) -> Result<String, Error> {

        let request = CreateSpeechRequestArgs::default()
            .model(SpeechModel::Tts1)
            .voice(Voice::Alloy)
            .input(text.to_string())
            .instructions(instructions.to_string()).build().unwrap();

        let content =
            self.client.audio().speech(request).await
            .map_err(|_e| Error::AiError(_e.to_string()))?
            .bytes;

         Ok(general_purpose::STANDARD.encode(content))

    }
}

