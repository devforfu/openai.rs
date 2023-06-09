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
pub struct ListFineTuneEventsResponse {
    #[serde(rename = "object")]
    pub object: String,
    #[serde(rename = "data")]
    pub data: Vec<crate::models::FineTuneEvent>,
}

impl ListFineTuneEventsResponse {
    pub fn new(object: String, data: Vec<crate::models::FineTuneEvent>) -> ListFineTuneEventsResponse {
        ListFineTuneEventsResponse {
            object,
            data,
        }
    }
}


