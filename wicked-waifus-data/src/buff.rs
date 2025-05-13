use serde::Deserialize;

#[derive(Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct BuffData {
    pub id: i64,
	pub ge_desc: String,
	pub duration_policy: i32,
	pub extra_effect_parameters: Option<Vec<String>>,
	pub game_attribute_i_d: i32
}