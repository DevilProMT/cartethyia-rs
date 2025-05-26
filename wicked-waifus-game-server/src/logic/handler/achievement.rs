use wicked_waifus_protocol::{
    AchievementInfoRequest, AchievementInfoResponse,
    AchievementGroupInfo, AchievementGroupEntry,
    AchievementEntry, AchievementProgress,
    UpdateAchievementInfoRequest, UpdateAchievementInfoResponse,
    ErrorCode,
};

use std::collections::HashMap;

use wicked_waifus_data::{achievement_data};

use crate::logic::thread_mgr::NetContext;

pub fn on_achievement_info_request(
    _ctx: &NetContext,
    _request: AchievementInfoRequest,
    response: &mut AchievementInfoResponse,
) {
    let mut data: HashMap<i32, Vec<i32>> = HashMap::new();

    for achievement in achievement_data::iter() {
        data.entry(achievement.group_id)
            .or_default()
            .push(achievement.id);
    }

    response.achievement_group_info_list = data
        .iter()
        .map(|(&group_id, achievements)| AchievementGroupInfo {
            achievement_group_entry: Some(AchievementGroupEntry {
                id: group_id,
                finish_time: 1747510785,
                is_receive: true,
            }),
            achievement_entry_list: achievements
                .iter()
                .map(|&achievement_id| AchievementEntry {
                    id: achievement_id,
                    finish_time: 1747510785,
                    is_receive: true,
                    progress: Some(AchievementProgress {
                        cur_progress: 1337,
                        total_progress: 1337,
                    }),
                })
                .collect(),
        })
        .collect();
}

pub fn on_update_achievement_info_request(
    _ctx: &NetContext,
    _request: UpdateAchievementInfoRequest,
    response: &mut UpdateAchievementInfoResponse,
) {
    response.error_code = ErrorCode::Success.into();
    response.achievement_entry_list= achievement_data::iter()
        .map(|achievement| AchievementEntry {
            id: achievement.id,
            finish_time: 1747510785,
            is_receive: true,
            progress: Some(AchievementProgress {
                cur_progress: 1337,
                total_progress: 1337,
            }),
        })
        .collect()
}
