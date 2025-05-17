pub use attribute::*;
pub use advice::*;
pub use animal::*;
pub use chat::*;
pub use combat::*;
pub use coop::*;
pub use dummy::*;
pub use entity::*;
pub use friend::*;
pub use gacha::*;
pub use inventory::*;
pub use guide::*;
pub use lord_gym::*;
pub use mail::*;
pub use map::*;
pub use misc::*;
pub use role::*;
pub use scene::*;
use wicked_waifus_protocol::message::Message;
pub use skill::*;
pub use teleport::*;
pub use tutorial::*;

mod attribute;
mod advice;
mod animal;
mod chat;
mod combat;
mod coop;
mod dummy;
mod entity;
mod friend;
mod gacha;
mod guide;
mod inventory;
mod lord_gym;
mod mail;
mod map;
mod misc;
mod role;
mod scene;
mod skill;
mod teleport;
mod tutorial;

macro_rules! handle_request {
    ($($name:ident $(, $inner_package:ident)?;)*) => {
        fn handle_request(player: &mut super::player::Player, mut msg: Message) {
            use ::wicked_waifus_protocol::{MessageID, Protobuf};

            ::paste::paste! {
                match msg.get_message_id() {
                    $(
                        ::wicked_waifus_protocol::$($inner_package::)?[<$name Request>]::MESSAGE_ID => {
                            let Ok(request) = ::wicked_waifus_protocol::$($inner_package::)?[<$name Request>]::decode(&*msg.remove_payload()) else {
                                tracing::debug!("failed to decode {}, player_id: {}", stringify!($($inner_package::)?[<$name Request>]), player.basic_info.id);
                                return;
                            };

                            tracing::debug!("logic: processing request {}", stringify!($($inner_package::)?[<$name Request>]));

                            let mut response = ::wicked_waifus_protocol::$($inner_package::)?[<$name Response>]::default();
                            [<on_ $($inner_package:snake _)? $name:snake _request>](player, request, &mut response);

                            player.respond(response, msg.get_rpc_id());
                        },
                    )*
                    unhandled => {
                         ::tracing::warn!("can't find handler for request with message_id={unhandled}");
                         let tmp = &*msg.remove_payload();
                         let (name, value) = wicked_waifus_protocol::proto_dumper::get_debug_info(
                             unhandled, tmp,
                         ).unwrap_or_else(|err| ("Error", err.to_string()));
                        tracing::debug!("trying to log unhandled data for message {name} with:\n{value}")
                    }
                }
            }
        }
    };
}

macro_rules! handle_push {
    ($($name:ident $(, $inner_package:ident)?;)*) => {
        fn handle_push(player: &mut super::player::Player, mut msg: Message) {
            use ::wicked_waifus_protocol::{MessageID, Protobuf};

            ::paste::paste! {
                match msg.get_message_id() {
                    $(
                        ::wicked_waifus_protocol::$($inner_package::)?[<$name Push>]::MESSAGE_ID => {
                            let Ok(push) = ::wicked_waifus_protocol::$($inner_package::)?[<$name Push>]::decode(&*msg.remove_payload()) else {
                                tracing::debug!("failed to decode {}, player_id: {}", stringify!($($inner_package::)?[<$name Push>]), player.basic_info.id);
                                return;
                            };

                            tracing::debug!("logic: processing push {}", stringify!($($inner_package::)?[<$name Push>]));

                            [<on_ $($inner_package:snake _)? $name:snake _push>](player, push);
                        },
                    )*
                    unhandled => {
                         ::tracing::warn!("can't find handler for push with message_id={unhandled}");
                         let tmp = &*msg.remove_payload();
                         let (name, value) = wicked_waifus_protocol::proto_dumper::get_debug_info(
                             unhandled, tmp,
                         ).unwrap_or_else(|err| ("Error", err.to_string()));
                        tracing::debug!("trying to log unhandled data for message {name} with:\n{value}")
                    }
                }
            }
        }
    };
}

macro_rules! handle_action {
    ($($variant:ident);* $(,)?) => {
        use wicked_waifus_data::pb_components::action::Action;
        use crate::logic::player::Player;

        fn perform_action(
            player: &mut Player,
            entity_id: i64,
            level_entity_data: &wicked_waifus_data::LevelEntityConfigData,
            template_config: &wicked_waifus_data::TemplateConfigData,
            element: Action
        ) {
            ::paste::paste! {
                match element {
                    $(
                        Action::$variant(inner) => {
                          paste::paste! {
                               [<$variant:snake _action>](player, entity_id, level_entity_data, template_config, inner)
                           }
                        },
                        _ => tracing::debug!("hi")
                    )*
                }
            }
        }
    };
}

handle_request! {
    // Advice
    Advice;
    AdviceSet;

    // Animal
    AnimalDie;
    AnimalDrop;
    AnimalDestroy;

    // Attribute
    AttributeChanged;
    FormationAttr;

    // Chat  (TODO: Review TODOs)
    PrivateChat;
    PrivateChatData;
    PrivateChatHistory;
    PrivateChatOperate;

    // Combat (TODO: Review this on_..., port some from go)
    CombatSendPack, combat_message;
    // CombatMessagePostInfo, combat_message; // TODO: Review this niggerianism, Encrypted shadow data

    // Coop
    LobbyList;

    // Entity (TODO: Review this on_..., port some from go)
    EntityActive;
    EntityOnLanded;
    EntityPosition;
    EntityAccessRange;
    EntityInteract;
    EntityFollowTrack;
    GetRewardTreasureBox;

    // Friend (TODO: Implement them)
    FriendAll;
    // FriendApplySend;
    // FriendRecentlyTeam;
    PlayerBasicInfoGet;

    // Gacha
    Gacha;
    GachaInfo;
    GachaUsePool;

    // Guide
    GuideInfo;
    GuideTrigger;
    GuideFinish;

    // Inventory
    NormalItem;
    WeaponItem;
    PhantomItem;
    ValidTimeItem;
    ItemExchangeInfo;

    // Lord Gym (TODO: Review this on_..., port some from go)
    LordGymInfo;

    // Mail
    MailBindInfo;

    // Map
    DarkCoastDelivery;
    MapCancelTrace;
    MapTrace;
    MapTraceInfo;
    MapUnlockFieldInfo;
    PlayerAccessEffectArea;

    // LevelPlayStateListAsyncRequest // Example: "x9l": [{"inst_id": 902,"level_play_ids": [166700009,157700000]}]

    // Misc (TODO: Review this on_..., port some from go)
    InputSetting;
    InputSettingUpdate;
    LanguageSettingUpdate;
    ServerPlayStationPlayOnlyState;
    LoadingConfig;

    // Player (TODO: Review this on_..., port some from go)
    // ModifySignature;
    // ModifyName;
    // ChangeHeadPhoto;
    PlayerTitleData;

    // Role (TODO: Review this on_..., port some from go)
    RoleShowListUpdate;
    ClientCurrentRoleReport;
    RoleFavorList;
    UpdateFormation;

    // Scene (TODO: Review this on_..., port some from go)
    SceneTrace;
    SceneLoadingFinish;
    UpdateSceneDate;
    AccessPathTimeServerConfig;
    UnlockRoleSkinList;
    PlayerHeadData;

    // Shop (TODO: Review this on_..., port some from go)
    // PayInfo;
    // PayGiftInfo;
    // PayShopItemUpdate;
    PayShopInfo;
    // PayShopUpdate;
    // MonthCard;
    PayInfo;

    // Skill (TODO: Review this on_..., port some from go)
    VisionExploreSkillSet;
    ExploreSkillRouletteSet;
    // RoleActivateSkill;

    // Teleport
    TeleportData;
    TeleportTransfer;
    TeleportFinish;

    // Tower (TODO: Review this on_..., port some from go)
    // TowerSeasonUpdate;
    // Tower;

    // Tutorial
    TutorialInfo;
    TutorialReceive;
    TutorialUnlock;

    // TODO: Implement all this properly, workaround for game enter
    EntityPatrolStop;
    InitRange;
    Activity;
    BattlePass;
    SlashAndTowerInfo;

    // Role
    RoleVisionRecommendData;
    RoleVisionRecommendAttr;
    PlayerMotion;

    // Formation
    GetFormationData;

    // Misc
    FishingData;
    EnergySync;
    GetDetectionLabelInfo;
    MonthCard;
    InfluenceInfo;
    ForgeInfo;
    AchievementInfo;
    ExchangeReward;
    Liveness;
    WebSign;
    PhotoMemory;
    WeaponSkin;
    VisionEquipGroupInfo;
    UpdatePlayStationBlockAccount;
    AdventureManual;
    Tower;
    ExploreProgress;
    ReportData;
    UpdateVoxelEnv;
    SimpleTrackReportAsync;
    TowerSeasonUpdate;
}

handle_push! {
    // Entity
    MovePackage;

    // Misc
    VersionInfo;
}

handle_action! {
    ExecBattleAction(_);
    WaitBattleCondition(_);
    SetBattleState(_);
    Action::PlayFlow(action) => unimplemented_action! { action },
    Action::Collect(_) => collect_action(player, level_entity_data, template_config),
    Action::LeisureInteract(action) => unimplemented_action! { action },
    Action::UnlockTeleportTrigger(action) => unlock_teleport_trigger(player, action.params),
    Action::EnableTemporaryTeleport(action) => unimplemented_action! { action },
    Action::OpenSystemBoard(action) => unimplemented_action! { action },
    Action::OpenSystemFunction(action) => unimplemented_action! { action },
    Action::ChangeSelfEntityState(action) => change_self_entity_state(player, entity_id, level_entity_data, template_config, action.params),
    Action::SetPlayerOperationRestriction(action) => unimplemented_action! { action },
    Action::Wait(action) => unimplemented_action! { action },
    Action::ChangeEntityState(action) => unimplemented_action! { action },
    Action::Log(action) => unimplemented_action! { action },
    Action::EnableNearbyTracking(action) => unimplemented_action! { action },
    Action::TeleportDungeon(action) => unimplemented_action! { action },
    Action::DestroySelf(action) => unimplemented_action! { action },
    Action::CameraLookAt(action) => unimplemented_action! { action },
    Action::StopCameraLookAt(action) => unimplemented_action! { action },
    Action::EnterOrbitalCamera(action) => unimplemented_action! { action },
    Action::ExitOrbitalCamera(action) => unimplemented_action! { action },
    Action::SendAiEvent(action) => unimplemented_action! { action },
    Action::SetInteractionLockState(action) => unimplemented_action! { action },
    Action::AwakeEntity(action) => unimplemented_action! { action },
    Action::ChangeLiftTarget(action) => unimplemented_action! { action },
    Action::CalculateVar(action) => unimplemented_action! { action },
    Action::AddBuffToPlayer(action) => unimplemented_action! { action },
    Action::RemoveBuffFromPlayer(action) => unimplemented_action! { action },
    Action::AddBuffToEntity(action) => unimplemented_action! { action },
    Action::RemoveBuffFromEntity(action) => unimplemented_action! { action },
    Action::Prompt(action) => unimplemented_action! { action },
    Action::SetEntityVisible(action) => unimplemented_action! { action },
    Action::DestroyEntity(action) => unimplemented_action! { action },
    Action::GuideTrigger(action) => unimplemented_action! { action },
    Action::TriggerCameraShake(action) => unimplemented_action! { action },
    Action::SetVar(action) => unimplemented_action! { action },
    Action::VehicleEnter(action) => unimplemented_action! { action },
    Action::VehicleExitPlayer(action) => unimplemented_action! { action },
    Action::LockEntity(action) => unimplemented_action! { action },
    Action::UnlockEntity(action) => unimplemented_action! { action },
    Action::CommonTip(action) => unimplemented_action! { action },
    Action::CommonTip2(action) => unimplemented_action! { action },
    Action::PostAkEvent(action) => unimplemented_action! { action },
    Action::VehicleEnterNpc(action) => unimplemented_action! { action },
    Action::VehicleExitNpc(action) => unimplemented_action! { action },
    Action::PlayerLookAt(action) => unimplemented_action! { action },
    Action::PlayBubble(action) => unimplemented_action! { action },
    Action::AddPlayBubble(action) => unimplemented_action! { action },
    Action::ClearPlayBubble(action) => unimplemented_action! { action },
    Action::ExecRiskHarvestEffect(action) => unimplemented_action! { action },
    Action::EnableLevelPlay(action) => unimplemented_action! { action },
    Action::ClaimLevelPlayReward(action) => unimplemented_action! { action },
    Action::SettlementDungeon(action) => unimplemented_action! { action },
    Action::ExitDungeon(action) => unimplemented_action! { action },
    Action::FinishDungeon(action) => unimplemented_action! { action },
    Action::RecordDungeonEvent(action) => unimplemented_action! { action },
    Action::RecoverDurability(action) => unimplemented_action! { action },
    Action::FadeInScreen(action) => unimplemented_action! { action },
    Action::FadeOutScreen(action) => unimplemented_action! { action },
    Action::ChangeNpcPerformState(action) => unimplemented_action! { action },
    Action::EntityTurnTo(action) => unimplemented_action! { action },
    Action::EntityLookAt(action) => unimplemented_action! { action },
    Action::ToggleMapMarkState(action) => unimplemented_action! { action },
    Action::RandomVar(action) => unimplemented_action! { action },
    Action::ModifySceneItemAttributeTag(action) => unimplemented_action! { action },
    Action::VehicleWaterfallClimbing(action) => unimplemented_action! { action },
    Action::VehicleTeleport(action) => unimplemented_action! { action },
    Action::RogueGotoNextFloor(action) => unimplemented_action! { action },
    Action::RogueReceiveReward(action) => unimplemented_action! { action },
    Action::RogueSelectRoom(action) => unimplemented_action! { action },
    Action::RogueActivatePortal(action) => unimplemented_action! { action },
    Action::MowingTowerGotoNextFloor(action) => unimplemented_action! { action },
    Action::SlashAndTowerGotoNextFloor(action) => unimplemented_action! { action },
    Action::PlayMontage(action) => unimplemented_action! { action },
    Action::OpenSystemBoardWithReturn(action) => unimplemented_action! { action },
    Action::UnlockSystemItem(action) => unimplemented_action! { action },
    Action::SetSportsState(action) => unimplemented_action! { action },
    Action::OpenSimpleGameplay(action) => unimplemented_action! { action },
    Action::PlayEffect(action) => unimplemented_action! { action },
    Action::PlayEffect2(action) => unimplemented_action! { action },
    Action::RestorePlayerCameraAdjustment(action) => unimplemented_action! { action },
    Action::AdjustPlayerCamera(action) => unimplemented_action! { action },
    Action::SetPlayerPos(action) => unimplemented_action! { action },
    Action::MoveWithSpline(action) => unimplemented_action! { action },
    Action::EnableSplineMoveModel(action) => unimplemented_action! { action },
    Action::ToggleScanSplineEffect(action) => unimplemented_action! { action },
    Action::MoveSceneItem(action) => unimplemented_action! { action },
    Action::StopSceneItemMove(action) => unimplemented_action! { action },
    Action::FireBullet(action) => unimplemented_action! { action },
    Action::ClearFishingCabinInSaleItems(action) => unimplemented_action! { action },
    Action::AcceptFishingEntrust(action) => unimplemented_action! { action },
    Action::DestroyFishingBoat(action) => unimplemented_action! { action },
    Action::SetJigsawItem(action) => unimplemented_action! { action },
    Action::SetJigsawFoundation(action) => unimplemented_action! { action },
    Action::SetTeleControl(action) => unimplemented_action! { action },
    Action::SetEntityClientVisible(action) => unimplemented_action! { action },
    Action::ToggleHighlightExploreUi(action) => unimplemented_action! { action },
    Action::ExecAlertSystemAction(action) => unimplemented_action! { action },
    Action::AddFlowInteractOption(action) => unimplemented_action! { action },
    Action::RemoveFlowInteractOption(action) => unimplemented_action! { action },
    Action::EnableHostility(action) => unimplemented_action! { action },
    Action::ChangePhantomFormation(action) => unimplemented_action! { action },
    Action::RestorePhantomFormation(action) => unimplemented_action! { action },
    Action::ChangeTimer(action) => unimplemented_action! { action },
    Action::ToggleTimerPauseState(action) => unimplemented_action! { action },
    Action::ChangeFightTeam(action) => unimplemented_action! { action },
    Action::AddTrialFollowShooter(action) => unimplemented_action! { action },
    Action::RemoveTrialFollowShooter(action) => unimplemented_action! { action },
    Action::AddTrialCharacter(action) => unimplemented_action! { action },
    Action::RemoveTrialCharacter(action) => unimplemented_action! { action },
    Action::SetAreaState(action) => unimplemented_action! { action },
    Action::SwitchSubLevels(action) => unimplemented_action! { action },
    Action::ChangeTeamPosition(action) => unimplemented_action! { action },
    Action::GetItem(action) => unimplemented_action! { action },
    Action::CreatePrefab(action) => unimplemented_action! { action },
    Action::DestroyPrefab(action) => unimplemented_action! { action },
    Action::CompleteGuide(action) => unimplemented_action! { action },
    Action::PlayDynamicSettlement(action) => unimplemented_action! { action },
    Action::UsePhantomSkill(action) => unimplemented_action! { action },
    Action::HideTargetRange(action) => unimplemented_action! { action },
    Action::ChangeOtherState(action) => unimplemented_action! { action },
    Action::SetRegionConfig(action) => unimplemented_action! { action },
    Action::SetReviveRegion(action) => unimplemented_action! { action },
    Action::ExecResurrection(action) => unimplemented_action! { action },
    Action::ShowTargetRange(action) => unimplemented_action! { action },
    Action::SetTime(action) => unimplemented_action! { action },
    Action::SetTimeLockState(action) => unimplemented_action! { action },
    Action::EnableSystem(action) => unimplemented_action! { action },
    Action::EnableAoiNotify(action) => unimplemented_action! { action },
    Action::SetForceLock(action) => unimplemented_action! { action },
    Action::PlayRegisteredMontage(action) => unimplemented_action! { action },
    Action::SetAudioState(action) => unimplemented_action! { action },
    Action::HideGroup(action) => unimplemented_action! { action },
    Action::ShowHidedGroup(action) => unimplemented_action! { action },
    Action::HideSpecificEntities(action) => unimplemented_action! { action },
    Action::ShowSpecificEntities(action) => unimplemented_action! { action },
    Action::RemovePreloadResource(action) => unimplemented_action! { action },
    Action::Preload(action) => unimplemented_action! { action },
    Action::EnableAI(action) => unimplemented_action! { action },
    Action::SwitchDataLayers(action) => unimplemented_action! { action },
    Action::DestroyQuest(action) => unimplemented_action! { action },
    Action::DestroyQuestItem(action) => unimplemented_action! { action },
    Action::PromptQuestChapterUI(action) => unimplemented_action! { action },
    Action::TakePlotPhoto(action) => unimplemented_action! { action },
    Action::SetWuYinQuState(action) => unimplemented_action! { action },
    Action::RunActions(action) => unimplemented_action! { action },
    Action::ManualOccupations(action) => unimplemented_action! { action },
    Action::SetWeather(action) => unimplemented_action! { action },
    Action::SendNpcMail(action) => unimplemented_action! { action },
    Action::EnableFunction(action) => unimplemented_action! { action },
    Action::FocusOnMapMark(action) => unimplemented_action! { action },
    Action::CharacterLookAt(action) => unimplemented_action! { action },
    Action::AddGuestCharacter(action) => unimplemented_action! { action },
    Action::RemoveGuestCharacter(action) => unimplemented_action! { action },
    Action::TeleportToAndEnterVehicle(action) => unimplemented_action! { action },
    Action::SetAreaTimeState(action) => unimplemented_action! { action },
    Action::ResetPlayerCameraFocus(action) => unimplemented_action! { action },
    Action::ResetLevelPlay(action) => unimplemented_action! { action },
    Action::VehicleSprint(action) => unimplemented_action! { action },
    Action::VehicleMoveWithPathLine(action) => unimplemented_action! { action },
    Action::ClientPreEnableSubLevels(action) => unimplemented_action! { action },
    Action::GuestOperateUiAnimation(action) => unimplemented_action! { action },
    Action::ChangeEntityCamp(action) => unimplemented_action! { action },
    Action::NewMoveWithSpline(action) => unimplemented_action! { action },
    Action::DangoAbyssActivatePortal(action) => unimplemented_action! { action },
    Action::DangoAbyssCreateRewardTreasureBox(action) => unimplemented_action! { action },
    Action::DangoAbyssGotoNextFloor(action) => unimplemented_action! { action },
    Action::DangoAbyssReceiveReward(action) => unimplemented_action! { action },
    Action::SummonEntity(action) => unimplemented_action! { action },
    Action::GetRewardByInteract(action) => unimplemented_action! { action },
    Action::OpenQte(action) => unimplemented_action! { action },
    Action::ActiveAntiGravitySafePoint(action) => unimplemented_action! { action },
    Action::BvbPlayDialog(action) => unimplemented_action! { action },
    Action::BvbSendSystemEvent(action) => unimplemented_action! { action },
    Action::BvbSendAiEvent(action) => unimplemented_action! { action },
    Action::BvbPlayerOperationConstraint(action) => unimplemented_action! { action },
    Action::ExecClientBattleAction(action) => unimplemented_action! { action },
    Action::TriggerSpecificScanEffect(action) => unimplemented_action! { action },
    Action::SetActorVar(action) => unimplemented_action! { action },
    Action::RunActorCustomEvent(action) => unimplemented_action! { action },
    Action::StopUiScreenEffect(action) => unimplemented_action! { action },
    Action::StopNewMoveWithSpline(action) => unimplemented_action! { action },
    Action::RequestSystemFunctiony
}

pub fn handle_logic_message(player: &mut super::player::Player, msg: Message) {
    match msg {
        Message::Request { .. } => handle_request(player, msg),
        Message::Push { .. } => handle_push(player, msg),
        _ => tracing::warn!(
            "handle_logic_message: wrong message type: {}, message_id: {}, player_id: {}",
            msg.get_message_type(),
            msg.get_message_id(),
            player.basic_info.id,
        ),
    }
}

pub fn handle_action(
    player: &mut Player,
    entity_id: i64,
    level_entity_data: &wicked_waifus_data::LevelEntityConfigData,
    template_config: &wicked_waifus_data::TemplateConfigData,
    element: Action
) {
    perform_action(player, entity_id, level_entity_data, template_config, element)
}