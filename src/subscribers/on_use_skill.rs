use function_name::named;

use crate::{
    battle::BattleContext,
    kreide::{
        helpers::{
            get_avatar_from_entity, get_avatar_from_servant_entity, get_skill_from_skilldata,
        },
        types::{
            RPG_GameCore_AbilityStatic, RPG_GameCore_BattleEventDataComponent,
            RPG_GameCore_EntityType, RPG_GameCore_SkillCharacterComponent, RPG_GameCore_TeamType,
        },
    },
    models::{
        events::{Event, OnUseSkillEvent},
        misc::Avatar,
    },
    safe_call,
    subscribers::battle::{get_battle_instance, ON_USE_SKILL_Detour},
};
use anyhow::{anyhow, Result};

// Called when a manual skill is used. Does not account for insert skills (out of turn automatic skills)
#[named]
pub fn on_use_skill(
    instance_ptr: usize,
    skill_index: i32,
    a3: usize,
    a4: bool,
    skill_extra_use_param: i32,
) {
    log::debug!(function_name!());
    safe_call!(unsafe {
        let instance = RPG_GameCore_SkillCharacterComponent(instance_ptr);
        let entity = instance._OwnerRef().unwrap();
        let skill_owner = {
            let skill_owner = RPG_GameCore_AbilityStatic::get_actual_owner(entity);
            if let Ok(skill_owner) = skill_owner
                && !skill_owner.is_null()
            {
                skill_owner
            } else {
                entity
            }
        };

        let mut event: Option<Result<Event>> = None;
        if skill_owner._Team().unwrap() == RPG_GameCore_TeamType::TeamLight {
            let skill_data = instance.get_skill_data(skill_index, skill_extra_use_param);

            if let Ok(skill_data) = skill_data {
                match skill_owner._EntityType().unwrap() {
                    RPG_GameCore_EntityType::Avatar => {
                        let e = match get_skill_from_skilldata(skill_data) {
                            Ok(skill) => match get_avatar_from_entity(skill_owner) {
                                Ok(avatar) => Ok(Event::OnUseSkill(OnUseSkillEvent {
                                    avatar: Avatar {
                                        id: avatar.id,
                                        name: avatar.name,
                                    },
                                    skill,
                                })),
                                Err(e) => {
                                    Err(anyhow!("{} Avatar Event Error: {}", function_name!(), e))
                                }
                            },
                            Err(e) => Err(anyhow!(
                                "{} Avatar Skill Event Error: {}",
                                function_name!(),
                                e
                            )),
                        };
                        event = Some(e)
                    }
                    RPG_GameCore_EntityType::Servant => {
                        let e = match get_skill_from_skilldata(skill_data) {
                            Ok(skill) => match get_avatar_from_servant_entity(
                                get_battle_instance(),
                                skill_owner,
                            ) {
                                Ok(avatar) => Ok(Event::OnUseSkill(OnUseSkillEvent {
                                    avatar: Avatar {
                                        id: avatar.id,
                                        name: avatar.name,
                                    },
                                    skill,
                                })),
                                Err(e) => {
                                    Err(anyhow!("{} Servant Event Error: {}", function_name!(), e))
                                }
                            },
                            Err(e) => Err(anyhow!(
                                "{} Servant Skill Event Error: {}",
                                function_name!(),
                                e
                            )),
                        };
                        event = Some(e);
                    }
                    RPG_GameCore_EntityType::BattleEvent => {
                        let battle_event_data_comp = instance._CharacterDataRef().unwrap();
                        let avatar_entity =
                            RPG_GameCore_BattleEventDataComponent(battle_event_data_comp.0)
                                ._SourceCaster_k__BackingField()
                                .unwrap();

                        let e = match get_skill_from_skilldata(skill_data) {
                            Ok(skill) => match get_avatar_from_entity(avatar_entity) {
                                Ok(avatar) => Ok(Event::OnUseSkill(OnUseSkillEvent {
                                    avatar: Avatar {
                                        id: avatar.id,
                                        name: avatar.name,
                                    },
                                    skill,
                                })),
                                Err(e) => {
                                    Err(anyhow!("{} Summon Event Error: {}", function_name!(), e))
                                }
                            },
                            Err(e) => Err(anyhow!(
                                "{} Summon Skill Event Error: {}",
                                function_name!(),
                                e
                            )),
                        };
                        event = Some(e);
                    }
                    _ => log::warn!(
                        "Light entity type {:?} was not matched",
                        skill_owner._EntityType()
                    ),
                }
            }
        }
        if let Some(event) = event {
            BattleContext::handle_event(event);
        }
    });

    ON_USE_SKILL_Detour.call(instance_ptr, skill_index, a3, a4, skill_extra_use_param);
}
