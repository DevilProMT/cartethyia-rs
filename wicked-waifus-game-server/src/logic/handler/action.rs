use std::collections::HashMap;

use wicked_waifus_protocol::{
    CommonTagData, EntityCommonTagNotify, EntityStateReadyNotify, FightBuffInformation, ItemRewardNotify, NormalItemUpdateNotify, RewardItemInfo, WR
};

use wicked_waifus_data::pb_components::action::{
    AddBuffToEntity, AddBuffToPlayer, ChangeSelfEntityState, CollectParams, RemoveBuffFromEntity, RemoveBuffFromPlayer, UnlockTeleportTrigger
};
use wicked_waifus_data::pb_components::entity_state::EntityStateComponent;

use crate::logic::ecs::component::ComponentContainer;
use crate::logic::ecs::world::WorldEntity;
use crate::logic::handler::handle_action;
use crate::logic::player::{ItemUsage, Player};
use crate::logic::utils::tag_utils;
use crate::query_components;

pub fn collect_action(
    player: &mut Player,
    _entity_id: i64,
    level_entity_data: &wicked_waifus_data::LevelEntityConfigData,
    template_config: &wicked_waifus_data::TemplateConfigData,
	_: CollectParams
) {
    if let Some(reward_component) = level_entity_data
        .components_data
        .reward_component
        .as_ref()
        .or(template_config.components_data.reward_component.as_ref())
    {
        if reward_component.disabled.unwrap_or(false) {
            return;
        }
        // TODO: check the use of reward_type and drop_on_event
        //      Seems type 0 is reward from preview, while 1 and 2 is unknown
        if let Some(reward_id) = reward_component.reward_id {
            let drop = wicked_waifus_data::drop_package_data::get(&reward_id).unwrap();
            let usages = drop
                .drop_preview
                .iter()
                .map(|(&id, &quantity)| ItemUsage { id, quantity })
                .collect::<Vec<_>>();
            let updated_items = player.inventory.add_items(&usages);
            let normal_item_list = player
                .inventory
                .to_normal_item_list_filtered(updated_items.keys().cloned().collect::<Vec<i32>>());
            player.notify(NormalItemUpdateNotify {
                normal_item_list,
                no_tips: false,
            });
            // UpdateHandBookActiveStateMapNotify
            let mut rewards: HashMap<i32, WR> = HashMap::new();
            rewards.insert(
                0,
                WR {
                    item_list: drop
                        .drop_preview
                        .iter()
                        .map(|(&id, &quantity)| RewardItemInfo {
                            show_plan_id: 0, // TODO: Check how to get this
                            item_id: id,
                            count: quantity,
                            incr_id: 0,
                        })
                        .collect::<Vec<_>>(),
                },
            );
            player.notify(ItemRewardNotify {
                drop_id: reward_id,
                reason: 15000,
                magnification: 1,
                reward_items: rewards,
            });
        }
        // TODO: Should we remove entity?? get pcap
    }
}

#[inline(always)]
pub fn unlock_teleport_trigger_action(
	player: &mut Player, 
	_entity_id: i64,
    _level_entity_data: &wicked_waifus_data::LevelEntityConfigData,
    _template_config: &wicked_waifus_data::TemplateConfigData,
	action: UnlockTeleportTrigger
) {
    player.unlock_teleport(action.teleport_id)
}

pub fn change_self_entity_state_action(
    player: &mut Player,
    entity_id: i64,
    level_entity_data: &wicked_waifus_data::LevelEntityConfigData,
    template_config: &wicked_waifus_data::TemplateConfigData,
    action: ChangeSelfEntityState,
) {
    let state = tag_utils::get_tag_id_by_name(action.entity_state.as_str());

    // TODO: update Tag::CommonEntityTags too??
    let old_state = {
        let world_ref = player.world.borrow();
        let world = world_ref.get_world_entity();
        let mut state_tag = query_components!(world, entity_id, StateTag).0.unwrap();
        let old_state = state_tag.state_tag_id;
        tracing::debug!("ChangeSelfEntityState: old state {old_state} -> new state: {state}");
        state_tag.state_tag_id = state;
        old_state
    };

    if let Some(entity_state_component) = level_entity_data
        .components_data
        .entity_state_component
        .as_ref()
        .or(template_config
            .components_data
            .entity_state_component
            .as_ref())
        .cloned()
    {
        let entity_state_component: EntityStateComponent = entity_state_component; // TODO: Remove this line, used for casting only

        // TODO: implement rest of cases
        if let Some(state_change_behaviors) = entity_state_component.state_change_behaviors {
            for state_change_behavior in state_change_behaviors {
                // TODO: implement rest of cases
                let expected = tag_utils::get_tag_id_by_name(state_change_behavior.state.as_str());

                if expected == state {
                    if let Some(actions) = state_change_behavior.action {
                        for sub in actions {
                            handle_action(
                                player,
                                entity_id,
                                level_entity_data,
                                template_config,
                                sub,
                            );
                        }
                    }
                }
            }
        }
    }

    player.notify(EntityCommonTagNotify {
        id: entity_id,
        tags: vec![
            CommonTagData {
                tag_id: old_state,
                remove_tag_ids: false,
            }, // Remove
            CommonTagData {
                tag_id: state,
                remove_tag_ids: true,
            }, // Add
        ],
    });

    player.notify(EntityStateReadyNotify {
        entity_id,
        tag_id: state,
        ready: true, // TODO: Always true? or shall we compare it to something??
    });
}

fn add_buff_to_entity(
    world: &mut WorldEntity,
    entity_ids: Vec<i64>,
    buff_ids: Vec<i64>,
) {
    for entity_id in entity_ids {
        let (Some(mut buff_component),) = query_components!(world, entity_id, FightBuff) else {
            continue;
        };

        for buff_id in &buff_ids {
            buff_component.fight_buff_infos.push(FightBuffInformation {
                handle_id: 1,
                buff_id: *buff_id,
                level: 1,
                stack_count: 1,
                instigator_id: 0,
                entity_id: 0,
                apply_type: 0,
                duration: -1.0,
                left_duration: -1.0,
                context: vec![],
                is_active: true,
                server_id: 1,
                message_id: 1,
            });
        }
    }
}

pub fn add_buff_to_entity_action(
    player: &mut Player,
    entity_id: i64,
    level_entity_data: &wicked_waifus_data::LevelEntityConfigData,
    template_config: &wicked_waifus_data::TemplateConfigData,
	params: AddBuffToEntity
) {
    tracing::info!("entity buff request received with the following details: {:#?}.", params);
    let mut world_ref = player.world.borrow_mut();
    let world = world_ref.get_mut_world_entity();

    match params {
        AddBuffToEntity::SingleEntityBuffs(single_entity_buffs) => {
            add_buff_to_entity(world, vec![single_entity_buffs.entity_id], single_entity_buffs.buff_ids)
        },
        AddBuffToEntity::MultipleEntitiesBuff(multiple_entities_buff) => {
            add_buff_to_entity(world, multiple_entities_buff.entity_ids, multiple_entities_buff.buff_ids)
        },
        AddBuffToEntity::SelfEntityBuff(self_entity_buff) => {
            add_buff_to_entity(world, vec![entity_id], self_entity_buff.buff_ids)
        },
    }
}

pub fn remove_buff_from_entity_action(
    player: &mut Player,
    entity_id: i64,
    level_entity_data: &wicked_waifus_data::LevelEntityConfigData,
    template_config: &wicked_waifus_data::TemplateConfigData,
	params: RemoveBuffFromEntity
) {
    tracing::info!("entity buff request received with the following details: {:#?}.", params);
    let mut world_ref = player.world.borrow_mut();
    let world = world_ref.get_mut_world_entity();
}

pub fn add_buff_to_player_action(
    player: &mut Player,
    entity_id: i64,
    level_entity_data: &wicked_waifus_data::LevelEntityConfigData,
    template_config: &wicked_waifus_data::TemplateConfigData,
	params: AddBuffToPlayer
) {
    tracing::info!("entity buff request received with the following details: {:#?}.", params);
    let mut world_ref = player.world.borrow_mut();
    let world = world_ref.get_mut_world_entity();
}

pub fn remove_buff_from_player_action(
    player: &mut Player,
    entity_id: i64,
    level_entity_data: &wicked_waifus_data::LevelEntityConfigData,
    template_config: &wicked_waifus_data::TemplateConfigData,
	params: RemoveBuffFromPlayer
) {
    tracing::info!("entity buff request received with the following details: {:#?}.", params);
    let mut world_ref = player.world.borrow_mut();
    let world = world_ref.get_mut_world_entity();
}