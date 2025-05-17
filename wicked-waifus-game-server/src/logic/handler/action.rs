use std::collections::HashMap;

use wicked_waifus_data::pb_components::action::{CollectParams, UnlockTeleportTrigger};
use wicked_waifus_protocol::{ItemRewardNotify, NormalItemUpdateNotify, RewardItemInfo, WR};

use crate::logic::{
    player::{ItemUsage, Player},
};

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
fn unlock_teleport_trigger_action(
	player: &mut Player, 
	_entity_id: i64,
    _level_entity_data: &wicked_waifus_data::LevelEntityConfigData,
    _template_config: &wicked_waifus_data::TemplateConfigData,
	action: UnlockTeleportTrigger
) {
    player.unlock_teleport(action.teleport_id)
}
