/*
 * OpenAI API
 *
 * APIs for sampling from and fine-tuning language models
 *
 * The version of the OpenAPI document: 1.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateEditResponse {
    #[serde(rename = "object")]
    pub object: String,
    #[serde(rename = "created")]
    pub created: i32,
    #[serde(rename = "choices")]
    pub choices: Vec<crate::models::CreateCompletionResponseChoicesInner>,
    #[serde(rename = "usage")]
    pub usage: Box<crate::models::CreateCompletionResponseUsage>,
}

impl CreateEditResponse {
    pub fn new(object: String, created: i32, choices: Vec<crate::models::CreateCompletionResponseChoicesInner>, usage: crate::models::CreateCompletionResponseUsage) -> CreateEditResponse {
        CreateEditResponse {
            object,
            created,
            choices,
            usage: Box::new(usage),
        }
    }
}


