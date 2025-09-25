use std::collections::HashSet;
use async_openai::Client;
use async_openai::config::OpenAIConfig;
use async_openai::{
    types::{ CreateChatCompletionRequestArgs},
};
use async_openai::types::{ChatCompletionRequestUserMessage, CreateImageRequestArgs, CreateSpeechRequestArgs, Image, ImageModel, ImageQuality, ImageSize, ResponseFormatJsonSchema, SpeechModel, Voice};
use async_openai::types::ResponseFormat;
use async_openai::types::ChatCompletionRequestMessage::User;
use async_openai::types::ChatCompletionRequestUserMessageContent::Text;
use utoipa::gen::serde_json::json;
use crate::domain::{Error, ExplainWordOutput, GuessCefrWordLevelOutput};
use base64::{engine::general_purpose, Engine as _};



pub(crate) trait Ai {
    async fn explain_word(&self, word: &str, context: &str, native_language: &str, available_part_of_speeches: HashSet<&str> ) -> Result<ExplainWordOutput,Error>;
    async fn tts(&self, text: &str, instructions: &str ) -> Result<String,Error>;
    async fn gen_image(&self, prompt: &str) -> Result<String, Error>;
    async fn ask( &self, prompt: &str) -> Result<String,Error>;

    async fn cefr_word_level( &self, word: &str, pos: &str, ) -> Result<GuessCefrWordLevelOutput,Error>; 
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

    async fn gen_image(&self, prompt: &str) -> Result<String, Error> {
        let request = CreateImageRequestArgs::default()
            .prompt(prompt)
            .n(1)
            .model(ImageModel::Other("gpt-image-1".to_string()))
            //.response_format(ImageResponseFormat::Url)
            .quality(ImageQuality::Medium)
            .size(ImageSize::S1024x1024)
            .user("async-openai")
            .build().unwrap();


        let image = self
            .client
            .images()
            .create(request)
            .await
            .map_err(|_e| Error::AiError(_e.to_string()))?
            .data;

        let image2 = image
            .first()
            .ok_or_else(|| Error::AiError("No images returned".to_string()))?;

        let base_64_data = match image2.as_ref() {
            Image::B64Json { b64_json, .. } => b64_json.as_ref(),
            _ => panic!("Unexpected image type."),
        };

        Ok(base_64_data.clone())

    }


    async fn ask(
        &self,
        prompt: &str,
    ) -> Result<String,Error> {

        let request = CreateChatCompletionRequestArgs::default()
            .response_format(ResponseFormat::Text)
            .model("gpt-4.1")
            .messages(vec![
                User(
                    ChatCompletionRequestUserMessage{
                        content: Text(prompt.to_string()),
                        name: None,
                    }
                )

            ])
            .build().unwrap();

            self.client.chat().create(request).await
                .map_err(|_e| Error::AiError(_e.to_string()))?
                .choices.first().map( |c| c.message.content.clone()).flatten()
                .ok_or(Error::AiError("No content in result".to_string()))

    }

    async fn cefr_word_level(&self, word: &str, pos: &str) -> Result<GuessCefrWordLevelOutput, Error> {
        let json_schema = ResponseFormatJsonSchema{
            description: None,
            name: "explain_word".to_string(),
            strict: Some(true),
            schema: Some(json!(
                {
                    "type": "object",
                    "properties": {
                        "level": {
                            "enum": ["A1", "A2", "B1", "B2", "C1", "C2"],
                            "description": "CEFR level"
                        },
                    },
                    "additionalProperties": false,
                    "required": ["level"],

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
                            format!("Guess CERF english level for word '{word}'(part of speech: {pos})")
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
}

