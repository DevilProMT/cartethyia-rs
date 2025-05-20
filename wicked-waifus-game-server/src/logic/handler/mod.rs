pub use action::*;
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

mod action;
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

                            // tracing::debug!("logic: processing request {}", stringify!($($inner_package::)?[<$name Request>]));

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
    ($($variant:ident),* $(,)?) => {
        use wicked_waifus_data::pb_components::action::Action;
        use crate::logic::player::Player;

        fn perform_action(
            player: &mut Player,
            entity_id: i64,
            level_entity_data: &wicked_waifus_data::LevelEntityConfigData,
            template_config: &wicked_waifus_data::TemplateConfigData,
            action: Action
        ) {
            ::paste::paste! {
                match action {
                    $(
                        Action::$variant(inner) => {
                            [<$variant:snake _action>](
                                player,
                                entity_id,
                                level_entity_data,
                                template_config,
                                inner.params
                            )
                        },
                    )*
                    _ => {
                        ::tracing::warn!("Action not implemented for: {:?}", action);
                    }
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
    OrderApplyBuff;
    OrderRemoveBuff;
    ApplyGameplayEffect;
    RemoveGameplayEffect;

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

    ApplyGameplayEffect;
    RemoveGameplayEffect;

    // Misc
    VersionInfo;
}

handle_action! {
    // ExecBattleAction,
    // WaitBattleCondition,
    // SetBattleState,
    // PlayFlow,
    Collect,
    // LeisureInteract,
    UnlockTeleportTrigger,
    // EnableTemporaryTeleport,
    // OpenSystemBoard,
    // OpenSystemFunction,
    ChangeSelfEntityState,
    // SetPlayerOperationRestriction,
    // Wait,
    // ChangeEntityState,
    // Log,
    // EnableNearbyTracking,
    // TeleportDungeon,
    // DestroySelf,
    // CameraLookAt,
    // StopCameraLookAt,
    // EnterOrbitalCamera,
    // ExitOrbitalCamera,
    // SendAiEvent,
    // SetInteractionLockState,
    // AwakeEntity,
    // ChangeLiftTarget,
    // CalculateVar,
    AddBuffToPlayer,
    RemoveBuffFromPlayer,
    AddBuffToEntity,
    RemoveBuffFromEntity,
    // Prompt,
    // SetEntityVisible,
    // DestroyEntity,
    // GuideTrigger,
    // TriggerCameraShake,
    // SetVar,
    // VehicleEnter,
    // VehicleExitPlayer,
    // LockEntity,
    // UnlockEntity,
    // CommonTip,
    // CommonTip2,
    // PostAkEvent,
    // VehicleEnterNpc,
    // VehicleExitNpc,
    // PlayerLookAt,
    // PlayBubble,
    // AddPlayBubble,
    // ClearPlayBubble,
    // ExecRiskHarvestEffect,
    // EnableLevelPlay,
    // ClaimLevelPlayReward,
    // SettlementDungeon,
    // ExitDungeon,
    // FinishDungeon,
    // RecordDungeonEvent,
    // RecoverDurability,
    // FadeInScreen,
    // FadeOutScreen,
    // ChangeNpcPerformState,
    // EntityTurnTo,
    // EntityLookAt,
    // ToggleMapMarkState,
    // RandomVar,
    // ModifySceneItemAttributeTag,
    // VehicleWaterfallClimbing,
    // VehicleTeleport,
    // RogueGotoNextFloor,
    // RogueReceiveReward,
    // RogueSelectRoom,
    // RogueActivatePortal,
    // MowingTowerGotoNextFloor,
    // SlashAndTowerGotoNextFloor,
    // PlayMontage,
    // OpenSystemBoardWithReturn,
    // UnlockSystemItem,
    // SetSportsState,
    // OpenSimpleGameplay,
    // PlayEffect,
    // PlayEffect2,
    // RestorePlayerCameraAdjustment,
    // AdjustPlayerCamera,
    // SetPlayerPos,
    // MoveWithSpline,
    // EnableSplineMoveModel,
    // ToggleScanSplineEffect,
    // MoveSceneItem,
    // StopSceneItemMove,
    // FireBullet,
    // ClearFishingCabinInSaleItems,
    // AcceptFishingEntrust,
    // DestroyFishingBoat,
    // SetJigsawItem,
    // SetJigsawFoundation,
    // SetTeleControl,
    // SetEntityClientVisible,
    // ToggleHighlightExploreUi,
    // ExecAlertSystemAction,
    // AddFlowInteractOption,
    // RemoveFlowInteractOption,
    // EnableHostility,
    // ChangePhantomFormation,
    // RestorePhantomFormation,
    // ChangeTimer,
    // ToggleTimerPauseState,
    // ChangeFightTeam,
    // AddTrialFollowShooter,
    // RemoveTrialFollowShooter,
    // AddTrialCharacter,
    // RemoveTrialCharacter,
    // SetAreaState,
    // SwitchSubLevels,
    // ChangeTeamPosition,
    // GetItem,
    // CreatePrefab,
    // DestroyPrefab,
    // CompleteGuide,
    // PlayDynamicSettlement,
    // UsePhantomSkill,
    // HideTargetRange,
    // ChangeOtherState,
    // SetRegionConfig,
    // SetReviveRegion,
    // ExecResurrection,
    // ShowTargetRange,
    // SetTime,
    // SetTimeLockState,
    // EnableSystem,
    // EnableAoiNotify,
    // SetForceLock,
    // PlayRegisteredMontage,
    // SetAudioState,
    // HideGroup,
    // ShowHidedGroup,
    // HideSpecificEntities,
    // ShowSpecificEntities,
    // RemovePreloadResource,
    // Preload,
    // EnableAI,
    // SwitchDataLayers,
    // DestroyQuest,
    // DestroyQuestItem,
    // PromptQuestChapterUI,
    // TakePlotPhoto,
    // SetWuYinQuState,
    // RunActions,
    // ManualOccupations,
    // SetWeather,
    // SendNpcMail,
    // EnableFunction,
    // FocusOnMapMark,
    // CharacterLookAt,
    // AddGuestCharacter,
    // RemoveGuestCharacter,
    // TeleportToAndEnterVehicle,
    // SetAreaTimeState,
    // ResetPlayerCameraFocus,
    // ResetLevelPlay,
    // VehicleSprint,
    // VehicleMoveWithPathLine,
    // ClientPreEnableSubLevels,
    // GuestOperateUiAnimation,
    // ChangeEntityCamp,
    // NewMoveWithSpline,
    // DangoAbyssActivatePortal,
    // DangoAbyssCreateRewardTreasureBox,
    // DangoAbyssGotoNextFloor,
    // DangoAbyssReceiveReward,
    // SummonEntity,
    // GetRewardByInteract,
    // OpenQte,
    // ActiveAntiGravitySafePoint,
    // BvbPlayDialog,
    // BvbSendSystemEvent,
    // BvbSendAiEvent,
    // BvbPlayerOperationConstraint,
    // ExecClientBattleAction,
    // TriggerSpecificScanEffect,
    // SetActorVar,
    // RunActorCustomEvent,
    // StopUiScreenEffect,
    // StopNewMoveWithSpline,
    // RequestSystemFunction
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