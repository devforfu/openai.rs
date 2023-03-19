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
pub struct CreateFineTuneRequest {
    /// The ID of an uploaded file that contains training data.  See [upload file](/docs/api-reference/files/upload) for how to upload a file.  Your dataset must be formatted as a JSONL file, where each training example is a JSON object with the keys \"prompt\" and \"completion\". Additionally, you must upload your file with the purpose `fine-tune`.  See the [fine-tuning guide](/docs/guides/fine-tuning/creating-training-data) for more details. 
    #[serde(rename = "training_file")]
    pub training_file: String,
    /// The ID of an uploaded file that contains validation data.  If you provide this file, the data is used to generate validation metrics periodically during fine-tuning. These metrics can be viewed in the [fine-tuning results file](/docs/guides/fine-tuning/analyzing-your-fine-tuned-model). Your train and validation data should be mutually exclusive.  Your dataset must be formatted as a JSONL file, where each validation example is a JSON object with the keys \"prompt\" and \"completion\". Additionally, you must upload your file with the purpose `fine-tune`.  See the [fine-tuning guide](/docs/guides/fine-tuning/creating-training-data) for more details. 
    #[serde(rename = "validation_file", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub validation_file: Option<Option<String>>,
    /// The name of the base model to fine-tune. You can select one of \"ada\", \"babbage\", \"curie\", \"davinci\", or a fine-tuned model created after 2022-04-21. To learn more about these models, see the [Models](https://platform.openai.com/docs/models) documentation. 
    #[serde(rename = "model", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub model: Option<Option<String>>,
    /// The number of epochs to train the model for. An epoch refers to one full cycle through the training dataset. 
    #[serde(rename = "n_epochs", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub n_epochs: Option<Option<i32>>,
    /// The batch size to use for training. The batch size is the number of training examples used to train a single forward and backward pass.  By default, the batch size will be dynamically configured to be ~0.2% of the number of examples in the training set, capped at 256 - in general, we've found that larger batch sizes tend to work better for larger datasets. 
    #[serde(rename = "batch_size", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<Option<i32>>,
    /// The learning rate multiplier to use for training. The fine-tuning learning rate is the original learning rate used for pretraining multiplied by this value.  By default, the learning rate multiplier is the 0.05, 0.1, or 0.2 depending on final `batch_size` (larger learning rates tend to perform better with larger batch sizes). We recommend experimenting with values in the range 0.02 to 0.2 to see what produces the best results. 
    #[serde(rename = "learning_rate_multiplier", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub learning_rate_multiplier: Option<Option<f32>>,
    /// The weight to use for loss on the prompt tokens. This controls how much the model tries to learn to generate the prompt (as compared to the completion which always has a weight of 1.0), and can add a stabilizing effect to training when completions are short.  If prompts are extremely long (relative to completions), it may make sense to reduce this weight so as to avoid over-prioritizing learning the prompt. 
    #[serde(rename = "prompt_loss_weight", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub prompt_loss_weight: Option<Option<f32>>,
    /// If set, we calculate classification-specific metrics such as accuracy and F-1 score using the validation set at the end of every epoch. These metrics can be viewed in the [results file](/docs/guides/fine-tuning/analyzing-your-fine-tuned-model).  In order to compute classification metrics, you must provide a `validation_file`. Additionally, you must specify `classification_n_classes` for multiclass classification or `classification_positive_class` for binary classification. 
    #[serde(rename = "compute_classification_metrics", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub compute_classification_metrics: Option<Option<bool>>,
    /// The number of classes in a classification task.  This parameter is required for multiclass classification. 
    #[serde(rename = "classification_n_classes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub classification_n_classes: Option<Option<i32>>,
    /// The positive class in binary classification.  This parameter is needed to generate precision, recall, and F1 metrics when doing binary classification. 
    #[serde(rename = "classification_positive_class", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub classification_positive_class: Option<Option<String>>,
    /// If this is provided, we calculate F-beta scores at the specified beta values. The F-beta score is a generalization of F-1 score. This is only used for binary classification.  With a beta of 1 (i.e. the F-1 score), precision and recall are given the same weight. A larger beta score puts more weight on recall and less on precision. A smaller beta score puts more weight on precision and less on recall. 
    #[serde(rename = "classification_betas", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub classification_betas: Option<Option<Vec<f32>>>,
    /// A string of up to 40 characters that will be added to your fine-tuned model name.  For example, a `suffix` of \"custom-model-name\" would produce a model name like `ada:ft-your-org:custom-model-name-2022-02-15-04-21-04`. 
    #[serde(rename = "suffix", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub suffix: Option<Option<String>>,
}

impl CreateFineTuneRequest {
    pub fn new(training_file: String) -> CreateFineTuneRequest {
        CreateFineTuneRequest {
            training_file,
            validation_file: None,
            model: None,
            n_epochs: None,
            batch_size: None,
            learning_rate_multiplier: None,
            prompt_loss_weight: None,
            compute_classification_metrics: None,
            classification_n_classes: None,
            classification_positive_class: None,
            classification_betas: None,
            suffix: None,
        }
    }
}


