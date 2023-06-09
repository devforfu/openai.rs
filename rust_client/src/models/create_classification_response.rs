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
pub struct CreateClassificationResponse {
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(rename = "search_model", skip_serializing_if = "Option::is_none")]
    pub search_model: Option<String>,
    #[serde(rename = "completion", skip_serializing_if = "Option::is_none")]
    pub completion: Option<String>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "selected_examples", skip_serializing_if = "Option::is_none")]
    pub selected_examples: Option<Vec<crate::models::CreateClassificationResponseSelectedExamplesInner>>,
}

impl CreateClassificationResponse {
    pub fn new() -> CreateClassificationResponse {
        CreateClassificationResponse {
            object: None,
            model: None,
            search_model: None,
            completion: None,
            label: None,
            selected_examples: None,
        }
    }
}


