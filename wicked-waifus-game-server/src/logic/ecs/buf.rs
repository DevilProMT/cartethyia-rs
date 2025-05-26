use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::atomic::{AtomicI32, Ordering};
use wicked_waifus_protocol::FightBuffInformation;

pub struct BufManager {
    active_buf_set: HashMap<i32, FightBuffInformation>,
    next_handle: AtomicI32,
    recycled_handles: HashMap<i32, VecDeque<i32>>,
}
const OVERRIDE_BUFFS: &[i64] = &[
    3003,       // Remove wall run prohibition
    3004,       // Remove gliding prohibition
    1213,       // Reduce stamina while flying
    1214,       // Reduce stamina while flying in sprint
    1215,       // Reduce stamina while flying up in sprint
    1216,       // Reduce stamina while flying down in sprint
    640012051,  // Allow flying -> tag: 1151923109
    //1101014001, // crown birthday
    //1101013015, // female
    //1101013013, // male
    1001000000, // Character initial buff mount
    70000077,   // Role-IOU deduction control
    291724064, // Male MC Tag
    291724065, // Female MC Tag
    1101008001, // Born with Fusion Tag
    1101008002, // Born with Electro Tag
    1101008003, // Born with Aero Tag
    1101008004, // Born with Spectro Tag
    1101008005, // Born with Havoc Tag
];

const ROLE_OVERRIDES: &[(i32, &[i64])] = &[
    (1407, &[
        // ciaconna's forte buffs are completely fucked to get from an algorithm and i hate kuro!
        1407000000,
        1407000001,
        1407900002
    ]),
    (1205, &[
        1206006000,
        1206006901
    ]),
    (1409, &[
        1409001010
    ]),
    // (1105, &[
    //     1105001015
    // ])
];

fn get_role_buff_overrides(role_id: i32) -> Option<&'static [i64]> {
    for &(role, buff) in ROLE_OVERRIDES {
        if role == role_id {
            return Some(buff);
        }
    }
    None
}

const BLACKLISTED_BUFFS: &[i64] = &[
    // use this to remove buffs from the final of buffs, consider this the final final pass.
];

impl BufManager {
    pub fn create(&mut self, buf: &mut FightBuffInformation) {
        let handle = self
            .recycled_handles
            .get_mut(&buf.handle_id)
            .and_then(|ids| ids.pop_front())
            .unwrap_or_else(|| self.next_handle.fetch_add(1, Ordering::Relaxed));

        buf.handle_id = handle;
        buf.server_id = handle;
        buf.message_id = handle as i64;

        self.active_buf_set.entry(handle).or_insert(buf.clone());
    }

    #[inline(always)]
    pub fn remove_entity_buffs(&mut self, entity_id: i64) {
        let handles = self.active_buf_set.iter()
            .filter(|(_, buff)| buff.entity_id == entity_id)
            .map(|(&handle, _)| handle)
            .collect::<Vec<_>>();
        for handle in handles {
            self.remove(handle);
        }
    }

    #[inline(always)]
    pub fn remove(&mut self, handle: i32) -> bool {
        if let Some(buf) = self.active_buf_set.remove(&handle) {
            self.recycled_handles
                .entry(handle)
                .or_default()
                .push_back(buf.handle_id);
            true
        } else {
            false
        }
    }

    pub fn create_permanent_buffs(&mut self, origin_id: i64, role_id: i32) -> Vec<FightBuffInformation> {
        // let mut buffs = vec![];
        let mut buffs = wicked_waifus_data::buff_data::iter().filter(|(id, buf)| {
            id.to_string().starts_with(&role_id.to_string()) // must be part of char kit :)
            && 
            !id.to_string().contains("666")// KURO IS EVIL
            && 
            buf.duration_policy == 1
            && 
            !buf.ge_desc.contains("【废弃】") // remove "deprecated" buffs
            // && 
            // buf.game_attribute_id == 0
            && 
            buf.ongoing_tag_requirements.len() < 2
            &&
            buf.ongoing_tag_ignores.is_empty()
            &&
            buf.removal_tag_ignores.is_empty()
            // &&
            // buf.extra_effect_id != 2
            // &&
            // buf.extra_effect_id != 6
            &&
            buf.extra_effect_id != 14
            &&
            buf.gameplay_cue_ids.is_empty()
        })
        .map(|x| *x.0)
        .collect::<Vec<i64>>();

        let chain_buff_data: HashSet<i64> = wicked_waifus_data::resonant_chain_data::iter().filter(|c| c.group_id == role_id).flat_map(|c| c.buff_ids.clone()).collect();
        buffs = buffs.iter().filter(|id| !chain_buff_data.contains(id)).copied().collect();
        buffs.extend(OVERRIDE_BUFFS.iter().copied());
        if let Some(role_buff_overrides) = get_role_buff_overrides(role_id) {
            buffs.extend(role_buff_overrides.iter().copied());
        }
        buffs = buffs.iter().filter(|id| !BLACKLISTED_BUFFS.contains(id)).copied().collect();
        buffs.sort();
        buffs.dedup();
        
        buffs
            .iter()
            .map(|id| {
                let mut buff = FightBuffInformation {
                    handle_id: 0,
                    buff_id: *id,
                    level: 1,
                    stack_count: 1,
                    instigator_id: origin_id,
                    entity_id: origin_id,
                    apply_type: 0,
                    duration: -1f32,
                    left_duration: -1f32,
                    context: vec![],
                    is_active: true,
                    server_id: 0,
                    message_id: 0,
                };
                self.create(&mut buff);
                buff
            })
            .collect::<Vec<_>>()
    }

    pub fn create_buff(&mut self, origin_id: i64, buff_id: i64) -> FightBuffInformation {
        let mut buff = FightBuffInformation {
            handle_id: 0,
            buff_id,
            level: 1,
            stack_count: 1,
            instigator_id: origin_id,
            entity_id: origin_id,
            apply_type: 0,
            duration: -1f32,
            left_duration: -1f32,
            context: vec![],
            is_active: true,
            server_id: 0,
            message_id: 0,
        };
        self.create(&mut buff);
        buff
    }
}

impl Default for BufManager {
    fn default() -> Self {
        Self {
            active_buf_set: Default::default(),
            next_handle: AtomicI32::new(1),
            recycled_handles: Default::default(),
        }
    }
}
