/*
 * OpenAI API
 *
 * APIs for sampling from and fine-tuning language models
 *
 * The version of the OpenAPI document: 1.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreateChatCompletionRequestStop : Up to 4 sequences where the API will stop generating further tokens. 



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateChatCompletionRequestStop {
}

impl CreateChatCompletionRequestStop {
    /// Up to 4 sequences where the API will stop generating further tokens. 
    pub fn new() -> CreateChatCompletionRequestStop {
        CreateChatCompletionRequestStop {
        }
    }
}


