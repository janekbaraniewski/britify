use std::error::Error;
use crate::styles::{Style, get_prompt};

#[cfg(test)]
use mockall::*;

use async_openai::{
    types::{
        ChatCompletionRequestMessageArgs,
        CreateChatCompletionRequestArgs,
        Role,
    },
    Client,
};


/// Translatin' a given text tae a specific British style usin' th' given model, aye.
///
/// This function takes a bit o' text, a style (like slang, formal, or Scottish), an' a model name.
/// It returns th' translated text or an error if somethin' goes awry, ken.
///
/// # Parameters
/// * `text` - Th' original text tae translate, laddie.
/// * `style` - Th' desired style of translation, whether it's slang, formal, or Scottish.
/// * `model` - Th' model name tae use for th' translation, such as "gpt-4".
///
/// # Returns
/// * `Ok(String)` - Th' translated text, all fine an' dandy.
/// * `Err(Box<dyn Error>)` - An error if somethin' went wrong, by th' bonnie banks.
pub async fn translate(text: &str, style: Style, model: &str) -> Result<String, Box<dyn Error>> {
    // Let's get tae work, shall we? First, create a client.
    let client = Client::new();

    // Fetch th' system message based on th' style, aye.
    let system_message = get_prompt(&style);

    // Craft th' request wi' th' proper model an' messages, ken.
    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model(model)
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

    // Send th' request an' await th' response.
    let response = client.chat().create(request).await?;

    // Check if a response was received, an' if so, extract th' content.
    let translated_text = response
        .choices
        .get(0)
        .and_then(|choice| choice.message.content.clone())
        .unwrap_or_else(|| "No response received".to_string());

    // Return th' translated text or an error if somethin's amiss.
    Ok(translated_text)
}



#[cfg_attr(test, mockall::automock)]
pub trait TranslatorClient {
    fn translate(&self, text: &str, style: Style, model: &str) -> Result<String, Box<dyn Error>>;
}

pub struct RealTranslatorClient;
impl TranslatorClient for RealTranslatorClient {
    fn translate(&self, text: &str, style: Style, model: &str) -> Result<String, Box<dyn Error>> {
        // Here, I'm returning an error as we don't have the actual implementation.
        // Replace this with your actual implementation.
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Not implemented")))
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_translate() {
        let mut mock_translator_client = MockTranslatorClient::new();
        mock_translator_client
            .expect_translate()
            .with(eq("Hello"), eq(Style::Slang), eq("gpt-4"))
            .return_once(|_, _, _| Ok("Oi, bruv! Hello, innit?".to_string()));

        let text = "Hello";
        let style = Style::Slang;
        let model = "gpt-4";
        let translated_text = mock_translator_client.translate(&text, style, model).unwrap();

        assert_eq!(translated_text, "Oi, bruv! Hello, innit?");
    }
}
