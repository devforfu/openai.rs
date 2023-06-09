/*
 * OpenAI API
 *
 * APIs for sampling from and fine-tuning language models
 *
 * The version of the OpenAPI document: 1.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreateCompletionRequestPrompt : The prompt(s) to generate completions for, encoded as a string, array of strings, array of tokens, or array of token arrays.  Note that <|endoftext|> is the document separator that the model sees during training, so if a prompt is not specified the model will generate as if from the beginning of a new document. 



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateCompletionRequestPrompt {
}

impl CreateCompletionRequestPrompt {
    /// The prompt(s) to generate completions for, encoded as a string, array of strings, array of tokens, or array of token arrays.  Note that <|endoftext|> is the document separator that the model sees during training, so if a prompt is not specified the model will generate as if from the beginning of a new document. 
    pub fn new() -> CreateCompletionRequestPrompt {
        CreateCompletionRequestPrompt {
        }
    }
}


