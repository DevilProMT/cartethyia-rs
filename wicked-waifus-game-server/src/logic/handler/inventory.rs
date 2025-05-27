use crate::logic::{ecs::world::WorldEntity, player::Player};
use crate::logic::thread_mgr::NetContext;
use wicked_waifus_data::base_property_data;
use wicked_waifus_data::phantom_item_data::PhantomItemData;
use wicked_waifus_data::summon_cfg_data::{self, SummonCfgData};
use wicked_waifus_data::template_config_data::TemplateConfigData;
use wicked_waifus_protocol::summon::ESummonType;
use crate::logic::ecs::entity::{Entity};
use wicked_waifus_protocol::{
    AddVisionEquipGroupRequest, AddVisionEquipGroupResponse, ApplyVisionGroupRequest, ApplyVisionGroupResponse, ChangeVisionGroupNameRequest, ChangeVisionGroupNameResponse, DeleteVisionEquipGroupRequest, DeleteVisionEquipGroupResponse, EEntityType, EntityAddNotify, EntityConfigType, EntityPb, EntityState, ErrorCode, FightBuffInformation, ItemDeprecateRequest, ItemDeprecateResponse, ItemExchangeInfo, ItemExchangeInfoRequest, ItemExchangeInfoResponse, ItemLockRequest, ItemLockResponse, NormalItemRequest, NormalItemResponse, PhantomItem, PhantomItemRequest, PhantomItemResponse, PhantomPropInfo, PhantomPutOnRequest, PhantomPutOnResponse, PutVisionGroupToTopRequest, PutVisionGroupToTopResponse, RefreshVisionEquipGroupData as ProtoRefreshVisionEquipGroupData, RolePhantomEquipInfo, RolePhantomPropInfo, VisionEquipGroupInfoRequest, VisionEquipGroupInfoResponse, VisionSkillChangeNotify, VisionSkillInformation, WeaponItem, WeaponItemRequest, WeaponItemResponse
};
use wicked_waifus_protocol_internal::RefreshVisionEquipGroupData;
use crate::logic::{
    components::{
        Attribute, EntityConfig, Equip, FightBuff, Movement, OwnerPlayer, PlayerOwnedEntityMarker,
        Position, RoleSkin, Visibility, VisionSkill, Summoner
    },
    ecs::component::ComponentContainer,
};
use crate::modify_component;

const MAX_POSITIONS: usize = 5;

//for replace echo
fn update_incr_id_owners(player: &mut Player, position: i32, new_inc_id: i32, new_role_id: i32) {
    let role_ids: Vec<i32> = player.role_list.keys().cloned().collect();
    for role_id in role_ids {
        if role_id == new_role_id {
            continue;
        }
        if let Some(role) = player.role_list.get_mut(&role_id) {
            if let Some(owner_id) = role.phantom_map.get_mut(&position) {
                if *owner_id == new_inc_id {
                    *owner_id = 0;
                }
            }
        }
    }
}

pub fn summon_vision_phantom(
    player: &Player, 
    world: &mut WorldEntity, 
    template_cfg: &TemplateConfigData, 
    phantom_data: &PhantomItemData, 
    summon_cfg: &SummonCfgData,
    owner_entity: i64
) -> (Entity, Vec<i64>) {
    let mut vision_buffs: Vec<FightBuffInformation> = Vec::new();
    let vision_config_id = template_cfg.id;

    let vision_entity = world.create_entity(
        vision_config_id,
        EEntityType::Vision.into(),
        player.basic_info.cur_map_id,
    );

    for buff_id in &summon_cfg.born_buff_id {
        vision_buffs.push(world.create_buff(vision_entity.entity_id, *buff_id));
    }

    (world
        .create_builder(vision_entity)
        .with(ComponentContainer::PlayerOwnedEntityMarker(PlayerOwnedEntityMarker {
            entity_type: EEntityType::Vision,
        }))
        .with(ComponentContainer::EntityConfig(EntityConfig {
            camp: 0,
            config_id: vision_config_id,
            config_type: EntityConfigType::Template,
            entity_type: EEntityType::Vision,
            entity_state: EntityState::Born,
        }))
        .with(ComponentContainer::OwnerPlayer(OwnerPlayer(
            player.basic_info.id,
        )))
        .with(ComponentContainer::Position(Position(
            player.location.position.clone(),
        )))
        .with(ComponentContainer::Visibility(Visibility {
            is_visible: false,
            is_actor_visible: true,
        }))
        .with(ComponentContainer::Attribute(Attribute::from_data(
            base_property_data::iter()
                .find(|d| d.id == template_cfg.components_data.attribute_component.clone().unwrap().property_id.unwrap())
                .unwrap(),
            None,
            None,
        )))
        .with(ComponentContainer::FightBuff(FightBuff { fight_buff_infos: vision_buffs, ..Default::default() }))
        .with(ComponentContainer::Summoner(Summoner {
            summoner_id: owner_entity,
            summon_cfg_id: phantom_data.skill_id,
            summon_skill_id: phantom_data.skill_id,
            summon_type: ESummonType::ESummonTypeConcomitantVision.into()
        }))
        .build(), summon_cfg.born_buff_id.clone()
    )
}

fn add_phantom_skill_for_role(player: &mut Player, world: &mut WorldEntity, inc_id: i32, role_id: i32) {
    let Some(id) = player
        .inventory
        .get_phantom_id(inc_id) else {
        return;
    };

    let entity_id = world.get_entity_id(role_id);

    if let Some(phantom_data) = wicked_waifus_data::phantom_item_data::iter()
        .find(|data| data.item_id == id)
    {
       if let Some(summon_data) = summon_cfg_data::iter().find(|(_, r)| r.id == phantom_data.skill_id)
       {
            let template_cfg = wicked_waifus_data::template_config_data::iter()
                .find(|cfg| cfg.1.blueprint_type == summon_data.1.blueprint_type)
                .unwrap()
                .1;
            let (vision, _) = summon_vision_phantom(
                    player,
                    world,
                    template_cfg,
                    phantom_data,
                    summon_data.1,
                    entity_id as i64,
                );

            let mut pb = EntityPb {
                id: vision.entity_id as i64,
                ..Default::default()
            };

            world
                .get_entity_components(vision.entity_id)
                .into_iter()
                .for_each(|comp| comp.set_pb_data(&mut pb));
            player.notify(EntityAddNotify {
                entity_pbs: vec![pb],
                remove_tag_ids: true,
            });
            player.notify(VisionSkillChangeNotify {
                entity_id: entity_id,
                vision_entity_id: vision.entity_id as i64,
                vision_skill_infos: vec![
                    VisionSkillInformation {
                        skill_id: phantom_data.skill_id,
                        level: 25,
                        quality: 5,
                        ..Default::default()
                    },
                    VisionSkillInformation {
                        skill_id: player.explore_tools.active_explore_skill,
                        ..Default::default()
                    },
                ],
                ..Default::default()
            })
       };
       
    }
}

// Check duplicate echo :wheelchair:
fn update_equipment_for_role(
    player: &mut Player,
    world: &mut WorldEntity,
    role_id: i32,
    position: i32,
    new_inc_id: i32,
) -> Result<(), ErrorCode> {
    if new_inc_id != 0 {
        let duplicate = {
            let r = player.role_list.get(&role_id).unwrap();
            r.phantom_map.values().any(|&id| id == new_inc_id)
        };
        if duplicate {
            return Err(ErrorCode::ErrPhantomEquipDuplicate);
        }
        if let Some((_, prev)) = player
            .role_list
            .iter_mut()
            .find(|(&id, r)| id != role_id && r.phantom_map.get(&position) == Some(&new_inc_id))
        {
            let old_id = prev.phantom_map.insert(position, 0).unwrap_or(0);
            player.used_incr_ids.remove(&old_id);
        }
        if position == 0 {
            add_phantom_skill_for_role(player, world, new_inc_id, role_id);
        }
        
    }
    if let Some(rm) = player.role_list.get_mut(&role_id) {
        let old = rm.phantom_map.insert(position, 0).unwrap_or(0);
        player.used_incr_ids.remove(&old);
        if new_inc_id != 0 {
            if player.used_incr_ids.contains(&new_inc_id) {
                return Err(ErrorCode::ErrPhantomEquiped);
            }
            player.used_incr_ids.insert(new_inc_id);
            rm.phantom_map.insert(position, new_inc_id);
        }
    }
    Ok(())
}

pub fn on_normal_item_request(
    ctx: &NetContext,
    _: NormalItemRequest,
    response: &mut NormalItemResponse,
) {
    tracing::debug!("Received NormalItemRequest, returning player inventory");
    response.normal_item_list = ctx.player.inventory.to_normal_item_list();
}

pub fn on_item_lock_request(
    _ctx: &mut NetContext,
    _: ItemLockRequest,
    response: &mut ItemLockResponse,
) {
    response.error_code = ErrorCode::ErrInteracting as i32;
}

pub fn on_item_deprecate_request(
    _ctx: &mut NetContext,
    _: ItemDeprecateRequest,
    response: &mut ItemDeprecateResponse,
) {
    response.error_code = ErrorCode::ErrInteracting as i32;
}

pub fn on_weapon_item_request(
    ctx: &NetContext,
    _: WeaponItemRequest,
    response: &mut WeaponItemResponse,
) {
    response.weapon_item_list = ctx.player.inventory.to_weapon_item_list();
}

pub fn on_vision_equip_group_info_request(
    ctx: &mut NetContext,
    _: VisionEquipGroupInfoRequest,
    response: &mut VisionEquipGroupInfoResponse,
) {
    response.vision_equip_list = ctx.player
        .vision_equip_groups
        .iter()
        .map(|g| ProtoRefreshVisionEquipGroupData {
            inc_id: g.inc_id.clone(),
            name: g.name.clone(),
        })
        .collect();
    response.error_code = ErrorCode::Success as i32;
}

pub fn on_add_vision_equip_group_request(
    ctx: &mut NetContext,
    request: AddVisionEquipGroupRequest,
    response: &mut AddVisionEquipGroupResponse,
) {
    let player = &mut ctx.player;
    
    if player.vision_equip_groups.len() >= 20 {
        response.error_code = ErrorCode::ErrVisionSkillSlotNotFound as i32;
        return;
    }

    let role = match player.role_list.get(&request.role_id) {
        Some(r) => r,
        None => {
            response.error_code = ErrorCode::NotValidRole as i32;
            return;
        }
    };

    let mut inc_ids = [0; MAX_POSITIONS];
    for (&pos, &inc_id) in &role.phantom_map {
        if (0..MAX_POSITIONS as i32).contains(&pos) {
            inc_ids[pos as usize] = inc_id;
        }
    }

    let new_group = RefreshVisionEquipGroupData {
        inc_id: inc_ids.to_vec(),
        name: request.name.clone(),
    };

    player.vision_equip_groups.push(new_group);

    response.vision_equip_list = player
        .vision_equip_groups
        .iter()
        .map(|g| ProtoRefreshVisionEquipGroupData {
            inc_id: g.inc_id.clone(),
            name: g.name.clone(),
        })
        .collect();

    response.error_code = ErrorCode::Success as i32;
}

pub fn on_delete_vision_equip_group_request(
    ctx: &mut NetContext,
    request: DeleteVisionEquipGroupRequest,
    response: &mut DeleteVisionEquipGroupResponse,
) {
    let player = &mut ctx.player;

    let idx = request.index as usize;
    if request.index < 0 || idx >= player.vision_equip_groups.len() {
        response.error_code = ErrorCode::ErrVisionSkillSlotNotFound as i32;
        return;
    }
    player.vision_equip_groups.remove(idx);
    response.vision_equip_list = player
        .vision_equip_groups
        .iter()
        .map(|g| ProtoRefreshVisionEquipGroupData {
            inc_id: g.inc_id.clone(),
            name: g.name.clone(),
        })
        .collect();
    response.error_code = ErrorCode::Success as i32;
}

pub fn on_change_vision_group_name_request(
    ctx: &mut NetContext,
    request: ChangeVisionGroupNameRequest,
    response: &mut ChangeVisionGroupNameResponse,
) {
    let player = &mut ctx.player;

    let idx = request.index as usize;
    if request.index < 0 || idx >= player.vision_equip_groups.len() {
        response.error_code = ErrorCode::ErrInteractOptionIndexInvalid as i32;
        return;
    }
    player.vision_equip_groups[idx].name = request.name.clone();
    response.vision_equip_list = player
        .vision_equip_groups
        .iter()
        .map(|g| ProtoRefreshVisionEquipGroupData {
            inc_id: g.inc_id.clone(),
            name: g.name.clone(),
        })
        .collect();
    response.error_code = ErrorCode::Success as i32;
}

pub fn on_put_vision_group_to_top_request(
    ctx: &mut NetContext,
    request: PutVisionGroupToTopRequest,
    response: &mut PutVisionGroupToTopResponse,
) {
    let player = &mut ctx.player;

    let idx = request.index as usize;
    if request.index < 0 || idx >= player.vision_equip_groups.len() {
        response.error_code = ErrorCode::ErrInteractOptionIndexInvalid as i32;
        return;
    }
    let group = player.vision_equip_groups.remove(idx);
    player.vision_equip_groups.insert(0, group);
    response.vision_equip_list = player
        .vision_equip_groups
        .iter()
        .map(|g| ProtoRefreshVisionEquipGroupData {
            inc_id: g.inc_id.clone(),
            name: g.name.clone(),
        })
        .collect();
    response.error_code = ErrorCode::Success as i32;
}

pub fn on_apply_vision_group_request(
    ctx: &mut NetContext,
    request: ApplyVisionGroupRequest,
    response: &mut ApplyVisionGroupResponse,
) {
    let player = &mut ctx.player;
    let world = &mut ctx.world;

    let idx = request.index as usize;
    if request.index < 0 || idx >= player.vision_equip_groups.len() {
        response.error_code = ErrorCode::ErrInteractOptionIndexInvalid as i32;
        return;
    }
    let role_id = request.role_id;
    let backup_used = player.used_incr_ids.clone();
    let backup_map = player
        .role_list
        .get(&role_id)
        .map(|r| r.phantom_map.clone());
    let inc_ids = player.vision_equip_groups[idx].inc_id.clone();

    for pos in 0..MAX_POSITIONS as i32 {
        let _ = update_equipment_for_role(player,world.get_mut_world_entity(), role_id, pos, 0);
    }
    let mut error = None;
    for (pos, &inc_id) in inc_ids.iter().enumerate() {
        let pos_i32 = pos as i32;
        if let Err(e) = update_equipment_for_role(player,world.get_mut_world_entity(), role_id, pos_i32, inc_id) {
            error = Some(e);
            break;
        }
    }
    if let Some(e) = error {
        player.used_incr_ids = backup_used;
        if let Some(map) = backup_map {
            if let Some(r) = player.role_list.get_mut(&role_id) {
                r.phantom_map = map;
            }
        }
        response.error_code = e as i32;
        return;
    }
    response.equip_info_list = player
        .role_list
        .values()
        .map(|r| {
            let mut ids = vec![0; MAX_POSITIONS];
            for (&pos, &inc_id) in &r.phantom_map {
                if (0..MAX_POSITIONS as i32).contains(&pos) {
                    ids[pos as usize] = inc_id;
                }
            }
            RolePhantomEquipInfo {
                role_id: r.role_id,
                phantom_item_incr_id: ids,
            }
        })
        .collect();
    response.error_code = ErrorCode::Success as i32;
}

pub fn on_phantom_item_request(
    ctx: &mut NetContext,
    _: PhantomItemRequest,
    response: &mut PhantomItemResponse,
) {
    let player = &mut ctx.player;

    response.phantom_item_list = player.inventory.to_phantom_item_list();
    response.equip_info_list = player
        .role_list
        .values()
        .filter(|r| !r.phantom_map.is_empty())
        .map(|r| {
            let mut incr_ids = vec![0; MAX_POSITIONS];
            for (pos, &inc_id) in &r.phantom_map {
                if *pos >= 0 && *pos < MAX_POSITIONS as i32 {
                    incr_ids[*pos as usize] = inc_id;
                }
            }
            RolePhantomEquipInfo {
                role_id: r.role_id,
                phantom_item_incr_id: incr_ids,
            }
        })
        .collect();
    response.ows = vec![];
    response.phantom_skin_list = vec![]; // PhantomCustomizeItem.json maybe this
    response.max_cost = 12; // maybe get properly???
}

pub fn on_phantom_put_on_request(
    ctx: &mut NetContext,
    request: PhantomPutOnRequest,
    response: &mut PhantomPutOnResponse,
) {
    let player = &mut ctx.player;
    let world = &mut ctx.world;

    let position = request.pos;
    if position < 0 || position >= MAX_POSITIONS as i32 {
        response.error_code = ErrorCode::ErrPhantomInvalidPos as i32;
        return;
    }

    let role_id = request.role_id;
    let backup_used = player.used_incr_ids.clone();
    let backup_map = player
        .role_list
        .get(&role_id)
        .map(|r| r.phantom_map.clone());

    let result = update_equipment_for_role(player, world.get_mut_world_entity(), role_id, position, request.inc_id);
    if let Err(err) = result {
        player.used_incr_ids = backup_used;
        if let Some(map) = backup_map {
            if let Some(r) = player.role_list.get_mut(&role_id) {
                r.phantom_map = map;
            }
        }
        response.error_code = err as i32;
    } else {
        response.error_code = ErrorCode::Success as i32;
    }

    response.equip_info_list = player
        .role_list
        .values()
        .map(|r| {
            let mut ids = vec![0; MAX_POSITIONS];
            for (&pos, &inc_id) in &r.phantom_map {
                if (0..MAX_POSITIONS as i32).contains(&pos) {
                    ids[pos as usize] = inc_id;
                }
            }
            RolePhantomEquipInfo {
                role_id: r.role_id,
                phantom_item_incr_id: ids,
            }
        })
        .collect();
}

pub fn on_item_exchange_info_request(
    _ctx: &mut NetContext,
    _: ItemExchangeInfoRequest,
    response: &mut ItemExchangeInfoResponse,
) {
    response.item_exchange_infos = wicked_waifus_data::item_exchange_content_data::iter()
        .map(|item_exchange_content_data| ItemExchangeInfo {
            item_id: item_exchange_content_data.item_id,
            today_times: 0, // TODO: For stats only, not used for PS so far
            total_times: 0, // TODO: For stats only, not used for PS so far
            daily_limit: 0, // At the time of writing there is no limits
            total_limit: 0, // At the time of writing there is no limits
        })
        .collect();
}
