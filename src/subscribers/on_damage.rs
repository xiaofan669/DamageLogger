use crate::{
    battle::BattleContext,
    kreide::{
        helpers::{
            fixpoint_raw_to_raw, fixpoint_to_raw, get_avatar_from_entity,
            get_avatar_from_servant_entity, get_skill_type_str,
        },
        types::{
            RPG_GameCore_AbilityProperty, RPG_GameCore_AbilityStatic, RPG_GameCore_EntityType,
            RPG_GameCore_GameEntity, RPG_GameCore_TeamType, RPG_GameCore_TurnBasedAbilityComponent,
            NOPBAAAGGLA,
        },
    },
    models::{
        events::{Event, OnDamageEvent},
        misc::Avatar,
    },
    safe_call,
    subscribers::battle::{get_battle_instance, ON_DAMAGE_Detour},
};
use anyhow::{anyhow, Result};

// Called on any instance of damage
#[function_name::named]
#[allow(clippy::too_many_arguments)]
pub fn on_damage(
    task_context: usize,
    damage_by_attack_property: usize,
    nopbaaaggla: usize,
    attacker_ability: usize,
    defender_ability: usize,
    attacker: usize,
    defender: usize,
    attacker_task_single_target: usize,
    flag: bool,
    obkbghmgbne: usize,
) -> bool {
    log::debug!(function_name!());
    let res = ON_DAMAGE_Detour.call(
        task_context,
        damage_by_attack_property,
        nopbaaaggla,
        attacker_ability,
        defender_ability,
        attacker,
        defender,
        attacker_task_single_target,
        flag,
        obkbghmgbne,
    );
    safe_call!(unsafe {
        let nopbaaaggla = NOPBAAAGGLA(nopbaaaggla);
        let attacker = RPG_GameCore_GameEntity(attacker);

        let mut event: Option<Result<Event>> = None;
        if attacker._Team().unwrap() == RPG_GameCore_TeamType::TeamLight {
            let damage = fixpoint_to_raw(&nopbaaaggla.JFKEEOMKMLI().unwrap());
            let damage_type = get_skill_type_str(nopbaaaggla.APDDLHNGGIM().unwrap());
            let attack_owner = {
                let attack_owner = RPG_GameCore_AbilityStatic::get_actual_owner(attacker);
                if let Ok(attack_owner) = attack_owner
                    && !attack_owner.is_null()
                {
                    attack_owner
                } else {
                    attacker
                }
            };

            match attack_owner._EntityType().unwrap() {
                RPG_GameCore_EntityType::Avatar => {
                    let e = match get_avatar_from_entity(attack_owner) {
                        Ok(avatar) => Ok(Event::OnDamage(OnDamageEvent {
                            attacker: Avatar {
                                id: avatar.id,
                                name: avatar.name,
                            },
                            damage,
                            damage_type,
                        })),
                        Err(e) => Err(anyhow!("{} Avatar Event Error: {}", function_name!(), e)),
                    };
                    event = Some(e);
                }
                RPG_GameCore_EntityType::Servant => {
                    let e =
                        match get_avatar_from_servant_entity(get_battle_instance(), attack_owner) {
                            Ok(avatar) => Ok(Event::OnDamage(OnDamageEvent {
                                attacker: Avatar {
                                    id: avatar.id,
                                    name: avatar.name,
                                },
                                damage,
                                damage_type,
                            })),
                            Err(e) => {
                                Err(anyhow!("{} Servant Event Error: {}", function_name!(), e))
                            }
                        };
                    event = Some(e);
                }
                RPG_GameCore_EntityType::Snapshot => {
                    // Unsure if this is if only a servant died and inflicted a DOT
                    let character_data_comp =
                        RPG_GameCore_TurnBasedAbilityComponent(attacker_ability)
                            ._CharacterDataRef()
                            .unwrap();
                    let e = match character_data_comp
                        .Summoner()
                        .and_then(|summoner_entity| get_avatar_from_entity(summoner_entity))
                    {
                        Ok(avatar) => Ok(Event::OnDamage(OnDamageEvent {
                            attacker: Avatar {
                                id: avatar.id,
                                name: avatar.name,
                            },
                            damage,
                            damage_type,
                        })),
                        Err(e) => Err(anyhow!("{} Snapshot Event Error: {}", function_name!(), e)),
                    };
                    event = Some(e);
                }
                _ => log::warn!(
                    "Light entity type {:?} was not matched",
                    attacker._EntityType()
                ),
            }
        }
        if let Some(event) = event {
            if defender_ability != 0 {
                let hp = RPG_GameCore_TurnBasedAbilityComponent(defender_ability)
                    .get_property(RPG_GameCore_AbilityProperty::CurrentHP)
                    .unwrap();
                log::info!("Monster HP: {}", fixpoint_raw_to_raw(&hp));
            }

            BattleContext::handle_event(event);
        }
    });
    res
}
