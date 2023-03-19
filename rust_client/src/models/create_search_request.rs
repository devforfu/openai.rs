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
pub struct CreateSearchRequest {
    /// Query to search against the documents.
    #[serde(rename = "query")]
    pub query: String,
    /// Up to 200 documents to search over, provided as a list of strings.  The maximum document length (in tokens) is 2034 minus the number of tokens in the query.  You should specify either `documents` or a `file`, but not both. 
    #[serde(rename = "documents", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub documents: Option<Option<Vec<String>>>,
    /// The ID of an uploaded file that contains documents to search over.  You should specify either `documents` or a `file`, but not both. 
    #[serde(rename = "file", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub file: Option<Option<String>>,
    /// The maximum number of documents to be re-ranked and returned by search.  This flag only takes effect when `file` is set. 
    #[serde(rename = "max_rerank", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub max_rerank: Option<Option<i32>>,
    /// A special boolean flag for showing metadata. If set to `true`, each document entry in the returned JSON will contain a \"metadata\" field.  This flag only takes effect when `file` is set. 
    #[serde(rename = "return_metadata", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub return_metadata: Option<Option<bool>>,
    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](/docs/guides/safety-best-practices/end-user-ids). 
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl CreateSearchRequest {
    pub fn new(query: String) -> CreateSearchRequest {
        CreateSearchRequest {
            query,
            documents: None,
            file: None,
            max_rerank: None,
            return_metadata: None,
            user: None,
        }
    }
}

