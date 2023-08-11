use std::error::Error;
use crate::styles::{Style, get_prompt};

use async_openai::{
    types::{
        ChatCompletionRequestMessageArgs,
        CreateChatCompletionRequestArgs,
        Role,
    },
    Client,
};

pub async fn translate(text: &str, style: Style, model: &str) -> Result<String, Box<dyn Error>> {
    // Your translation logic goes here
    // Depending on the style, apply the transformation
    // Return the translated text
    let client = Client::new();

    let system_message = get_prompt(&style);

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model(model) // Use the model parameter here
        .messages([
            ChatCompletionRequestMessageArgs::default()
                .role(Role::System)
                .content(system_message)
                .build()?,
            ChatCompletionRequestMessageArgs::default()
                .role(Role::User)
                .content("User Message: ".to_owned() + text)
                .build()?,
        ])
        .build()?;

    let response = client.chat().create(request).await?;

    // Check if a response was received and extract the content
    let translated_text = response
        .choices
        .get(0)
        .and_then(|choice| choice.message.content.clone())
        .unwrap_or_else(|| "No response received".to_string());

    Ok(translated_text) // Return the translated text as a String
}
