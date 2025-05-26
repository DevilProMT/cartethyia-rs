use super::System;

use crate::{
    config,
    logic::{ecs::world::World, player::Player},
    query_with,
};
use crate::{logic::ecs::component::ComponentContainer, query_components};
use std::cell::RefMut;
use wicked_waifus_protocol::{AttributeChangedNotify, EAttributeType, GameplayAttributeData};

pub(super) struct EnergyRegenSystem;

impl System for EnergyRegenSystem {
    fn tick(&self, world: &mut World, players: &mut [RefMut<Player>]) {
        let world_entity = world.get_world_entity();
        let default_unlocks = &config::get_config().default_unlocks;

        if !default_unlocks.unlock_max_energy {
            return;
        }

        for (entity, _, mut attribute) in
            query_with!(world_entity, PlayerOwnedEntityMarker, Attribute)
        {
            let mut attributes_changed = Vec::new();

            let energy_types = vec![
                (EAttributeType::Energy, EAttributeType::EnergyMax),
                (
                    EAttributeType::SpecialEnergy1,
                    EAttributeType::SpecialEnergy1Max,
                ),
                (
                    EAttributeType::SpecialEnergy2,
                    EAttributeType::SpecialEnergy2Max,
                ),
                (
                    EAttributeType::SpecialEnergy3,
                    EAttributeType::SpecialEnergy3Max,
                ),
                (
                    EAttributeType::SpecialEnergy4,
                    EAttributeType::SpecialEnergy4Max,
                ),
                (
                    EAttributeType::ElementEnergy,
                    EAttributeType::ElementEnergyMax,
                ),
            ];

            for (energy_type, max_energy_type) in energy_types {
                let (current_energy, _) = attribute
                    .attr_map
                    .get(&energy_type)
                    .copied()
                    .unwrap_or((0, 0));

                let (max_energy, _) = attribute
                    .attr_map
                    .get(&max_energy_type)
                    .copied()
                    .unwrap_or((0, 0));

                if current_energy < max_energy {
                    attribute
                        .attr_map
                        .insert(energy_type, (max_energy, 0));

                    attributes_changed.push(GameplayAttributeData {
                        current_value: max_energy,
                        value_increment: max_energy - current_energy,
                        attribute_type: energy_type.into(),
                    });
                }
            }

            if !attributes_changed.is_empty() {
                let notify = AttributeChangedNotify {
                    id: entity.into(),
                    attributes: attributes_changed,
                };

                if let (Some(owner),) =
                    query_components!(world_entity, i64::from(entity), OwnerPlayer)
                {
                    if let Some(player) = players.iter_mut().find(|p| p.basic_info.id == owner.0) {
                        player.notify(notify);
                    }
                }
            }
        }
    }
}

impl EnergyRegenSystem {
    pub fn new() -> Self {
        Self
    }
}
