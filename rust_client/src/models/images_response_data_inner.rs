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
pub struct ImagesResponseDataInner {
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "b64_json", skip_serializing_if = "Option::is_none")]
    pub b64_json: Option<String>,
}

impl ImagesResponseDataInner {
    pub fn new() -> ImagesResponseDataInner {
        ImagesResponseDataInner {
            url: None,
            b64_json: None,
        }
    }
}


