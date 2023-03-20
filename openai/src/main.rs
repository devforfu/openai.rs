use anyhow::format_err;
use openai_api::apis::{configuration::Configuration, open_ai_api::{list_models, create_chat_completion}};
use openai_api::models::{ChatCompletionRequestMessage, CreateChatCompletionRequest};
use openai_api::models::chat_completion_request_message::Role;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Configuration::from_env().ok_or(format_err!("API key is not found"))?;

    let completion = ChatCompletion::new(&config);

    completion.create("gpt-4", &vec![
        (Role::System, "You're a helpful assistant"),
        (Role::User, "Who won the world series in 2020?"),
        (Role::Assistant, "The Los Angeles Dodgers won the World Series in 2020."),
        (Role::User, "Where was it played?"),
    ]).await;

    Ok(())
}

pub struct ChatCompletion<'a> {
    config: &'a Configuration,
}

impl<'a> ChatCompletion<'a> {
    pub fn new(config: &'a Configuration) -> Self {
        Self { config }
    }

    pub async fn create(&self, model: &str, messages: &Vec<(Role, &str)>) {
        let params = CreateChatCompletionRequest::new(
            model.to_string(),
            messages.iter().map(|(role, message)| ChatCompletionRequestMessage::new(*role, message.to_string())).collect(),
        );
        if let Ok(result) = create_chat_completion(&self.config, params).await {
            for choice in result.choices {
                println!("{}", choice.message.unwrap().content);
            }
        }
    }
}
