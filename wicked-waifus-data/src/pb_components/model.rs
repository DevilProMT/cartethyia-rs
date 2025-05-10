use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ModelType {
	r#type: Option<String>,
	model_id: Option<i32>
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ModelComponent {
	pub half_height: Option<i32>,
    pub disabled: Option<bool>,
	pub model_type: Option<ModelType>
}