use serde::Deserialize;

use wicked_waifus_commons::config_util::TomlConfig;
use wicked_waifus_database::DatabaseSettings;
use wicked_waifus_network::config::ServiceEndPoint;

#[derive(Deserialize)]
pub struct ServiceConfig {
    pub service_id: u32,
    pub database: DatabaseSettings,
    pub service_end_point: ServiceEndPoint,
    pub gateway_end_point: ServiceEndPoint,
    pub game_server_config: GameServerConfig,
}

#[derive(Deserialize)]
pub struct GameServerConfig {
    pub load_textmaps: bool,
    pub quadrant_size: f32,
}

impl TomlConfig for ServiceConfig {
    const DEFAULT_TOML: &str = include_str!("../gameserver.default.toml");
}
