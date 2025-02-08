use wicked_waifus_protocol::{ErrorCode, TeleportDataRequest, TeleportDataResponse, TeleportNotify, TeleportReason, TeleportTransferRequest, TeleportTransferResponse, TeleportFinishRequest, TeleportFinishResponse, TransitionOptionPb, TransitionType, LeaveSceneNotify, JoinSceneNotify};
use wicked_waifus_data::{ComponentsData, level_entity_config_data, RawVectorData, TeleportComponent};

use crate::logic::player::Player;
use crate::logic::utils::world_util;

pub fn on_teleport_data_request(
    _player: &mut Player,
    _: TeleportDataRequest,
    response: &mut TeleportDataResponse,
) {
    // TODO: [WWPS-1] Real implementation should fetch completed / uncompleted from db, lets return completed
    response.error_code = ErrorCode::Success.into();
    response.ids = wicked_waifus_data::teleporter_data::iter()
        .map(|teleporter| teleporter.id)
        .collect::<Vec<_>>();
}

pub fn on_teleport_transfer_request(
    player: &mut Player,
    request: TeleportTransferRequest,
    response: &mut TeleportTransferResponse,
) {
    tracing::debug!("received transfer request for teleport id: {}", request.id);
    let Some(teleport) = wicked_waifus_data::teleporter_data::iter()
        .find(|teleporter| request.id == teleporter.id) else {
        response.error_code = ErrorCode::ErrTeleportIdNotExist.into();
        return;
    };

    println!("received transfer request for teleport entity id: {}", &teleport.teleport_entity_config_id);
    let Some(tp) = level_entity_config_data::get(&teleport.teleport_entity_config_id) else {
        response.error_code = ErrorCode::ErrTeleportEntityNotExist.into();
        return;
    };

    let Some(teleport_component) = &tp.components_data.teleport_component else {
        response.error_code = ErrorCode::ErrTeleportComponentNotExist.into();
        return;
    };

    if teleport_component.disabled.unwrap_or(false) ||
        teleport_component.teleporter_id.is_none() {
        response.error_code = ErrorCode::ErrTeleportGmGetCreatureGenCfgFailed.into();
    }
    if teleport_component.teleporter_id.unwrap() != request.id {
        response.error_code = ErrorCode::ErrTeleportComponentNotMatch.into();
    } else {
        response.error_code = ErrorCode::Success.into();
        response.map_id = teleport.map_id;
        let (x, y, z) = get_teleport_position(
            &tp.transform,
            teleport_component,
        );
        response.pos_x = x;
        response.pos_y = y;
        response.pos_z = z;
        response.pitch = 0f32;
        response.yaw = 0f32;
        response.roll = 0f32;

        // TODO: simplify (player.world.curr_map_id, palyer.basic_info.cur_map_id and player.location.instance_id)

        if player.location.instance_id == teleport.map_id {
            player.notify(TeleportNotify {
                map_id: teleport.map_id,
                pos_x: x,
                pos_y: y,
                pos_z: z,
                pos_a: 0.0,
                reason: TeleportReason::Gm.into(),
                game_ctx: None,
                transition_option: Some(TransitionOptionPb::default()),
                disable_auto_fade: false,
            });
        } else {
            // remove entity
            player.notify(LeaveSceneNotify {
                player_id: player.basic_info.id,
                scene_id: "".to_string(),
                transition_option: Some(TransitionOptionPb::default()),
            });
            let scene_info = world_util::build_scene_information(&player);
            player.notify(JoinSceneNotify {
                scene_info: Some(scene_info),
                max_entity_id: i64::MAX,
                transition_option: Some(TransitionOptionPb::default()),
            });
        }
    }
}

pub fn on_teleport_finish_request(
    _player: &mut Player,
    _: TeleportFinishRequest,
    response: &mut TeleportFinishResponse,
) {
    // Should we store here if the player is in the correct position?
    //  Use internal OnTeleportProcess??
    response.error_code = ErrorCode::Success.into();
}

fn get_teleport_position(transform: &[RawVectorData], component: &TeleportComponent) -> (f32, f32, f32) {
    // TODO: Review this formula, allegedly
    //      - transform[0] is position component
    //      - transform[2] is rotation component
    //      - transform[2] is scale component
    let (x, y, z) = (transform[0].x / 100.0, transform[0].y / 100.0, transform[0].z / 100.0);
    match &component.teleport_position {
        None => (x, y, z),
        Some(teleport_position) => (
            x + (teleport_position.x.unwrap_or_default()),
            y + (teleport_position.y.unwrap_or_default()),
            z + (teleport_position.z.unwrap_or_default()),
        )
    }
}