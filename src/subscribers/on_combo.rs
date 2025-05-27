use function_name::named;

use crate::{
    battle::BattleContext,
    kreide::{
        helpers::{
            get_avatar_from_entity, get_avatar_from_servant_entity, get_skill_from_skilldata,
        },
        types::{
            RPG_GameCore_AbilityStatic, RPG_GameCore_BattleEventDataComponent,
            RPG_GameCore_EntityType, RPG_GameCore_TeamType, MMNDIEBMDNL,
        },
    },
    models::{
        events::{Event, OnUseSkillEvent},
        misc::Avatar,
    },
    safe_call,
    subscribers::battle::{get_battle_instance, ON_COMBO_Detour},
};
use anyhow::{anyhow, Result};

// Insert skills are out of turn automatic skills
#[named]
pub fn on_combo(instance: usize) {
    log::debug!(function_name!());

    ON_COMBO_Detour.call(instance);
    safe_call!(unsafe {
        let instance = MMNDIEBMDNL(instance);
        let turn_based_ability_component = instance.FIMNOPAAFEP().unwrap();
        let skill_character_component = instance.HECCDOHIAFD().unwrap();
        let entity = skill_character_component._OwnerRef().unwrap();
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
            let ability_name = instance.HMCDHMFHABF().unwrap().FKHHOBBFMEH().unwrap();

            let skill_name = turn_based_ability_component
                .get_ability_mapped_skill(ability_name)
                .unwrap();
            let character_data_ref = turn_based_ability_component._CharacterDataRef().unwrap();
            let character_config = character_data_ref._JsonConfig_k__BackingField().unwrap();
            let skill_index = character_config
                .get_skill_index_by_trigger_key(skill_name)
                .unwrap();
            let skill_data = skill_character_component.get_skill_data(skill_index, -1);

            if let Ok(skill_data) = skill_data {
                match skill_owner._EntityType().unwrap() {
                    RPG_GameCore_EntityType::Avatar => {
                        let e: std::result::Result<Event, anyhow::Error> =
                            match get_skill_from_skilldata(skill_data) {
                                Ok(skill) => match get_avatar_from_entity(skill_owner) {
                                    Ok(avatar) => {
                                        if skill.name.is_empty() {
                                            return;
                                        }
                                        Ok(Event::OnUseSkill(OnUseSkillEvent {
                                            avatar: Avatar {
                                                id: avatar.id,
                                                name: avatar.name,
                                            },
                                            skill,
                                        }))
                                    }
                                    Err(e) => Err(anyhow!(
                                        "{} Avatar Event Error: {}",
                                        function_name!(),
                                        e
                                    )),
                                },
                                Err(e) => Err(anyhow!(
                                    "{} Avatar Combo Skill Event Error: {}",
                                    function_name!(),
                                    e
                                )),
                            };
                        event = Some(e);
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
                        let battle_event_data_comp = RPG_GameCore_BattleEventDataComponent(
                            skill_character_component._CharacterDataRef().unwrap().0,
                        );
                        let avatar_entity = battle_event_data_comp
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
}
