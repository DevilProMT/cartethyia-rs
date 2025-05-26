use serde::Deserialize;

use crate::{EntityLogic, EntityType};

#[derive(Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct BlueprintConfigData {
    pub id: i32,
    pub blueprint_type: String,
    pub entity_type: EntityType,
    pub entity_logic: EntityLogic,
    pub model_id: i32,
    pub half_height: i32,
    pub track_height: i32,
}