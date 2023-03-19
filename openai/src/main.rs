use openai_api::apis::{configuration::Configuration, open_ai_api::list_models};


#[tokio::main]
async fn main() {
    if let Some(config) = Configuration::from_env() {
        if let Ok(result) = list_models(&config).await {
            for item in result.data {
                println!("{item:?}");
            }
        }
    }
}
