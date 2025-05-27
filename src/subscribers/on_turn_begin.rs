use function_name::named;

use crate::{
    battle::BattleContext,
    kreide::{
        helpers::{
            fixpoint_raw_to_raw, get_avatar_from_entity, get_entity_ability_properties,
            get_entity_modifiers, get_monster_from_entity, get_servant_from_entity,
        },
        il2cpp::native::RuntimeType,
        types::{
            RPG_GameCore_AbilityProperty, RPG_GameCore_EntityType,
            RPG_GameCore_TurnBasedAbilityComponent, RPG_GameCore_TurnBasedGameMode,
        },
    },
    models::{
        events::{Event, OnTurnBeginEvent},
        misc::Avatar,
    },
    safe_call,
    subscribers::battle::{get_battle_instance, get_elapsed_av, ON_TURN_BEGIN_Detour},
};
use anyhow::anyhow;

#[named]
pub fn on_turn_begin(instance: usize) {
    log::debug!(function_name!());
    // Update AV first
    ON_TURN_BEGIN_Detour.call(instance);

    safe_call!(unsafe {
        let turn_owner_entity = RPG_GameCore_TurnBasedGameMode(instance)
            ._CurrentTurnActionEntity()
            .unwrap();

        let turn_owner = match turn_owner_entity._EntityType().unwrap() {
            RPG_GameCore_EntityType::Avatar => match get_avatar_from_entity(turn_owner_entity) {
                Ok(avatar) => Some(Avatar {
                    id: avatar.id,
                    name: avatar.name,
                }),
                Err(e) => {
                    return BattleContext::handle_event(Err(anyhow!(
                        "{} Avatar Event Error: {}",
                        function_name!(),
                        e
                    )))
                }
            },
            RPG_GameCore_EntityType::Monster => Some(
                get_monster_from_entity(turn_owner_entity).unwrap_or(Avatar {
                    id: 0,
                    name: String::from("Monster"),
                }),
            ),
            RPG_GameCore_EntityType::Servant => Some(
                get_servant_from_entity(turn_owner_entity).unwrap_or(Avatar {
                    id: 0,
                    name: String::from("Monster"),
                }),
            ),
            _ => None,
        };

        let modifiers = get_entity_modifiers(turn_owner_entity).unwrap_or_default();

        let ability_properties =
            get_entity_ability_properties(turn_owner_entity).unwrap_or_default();

        let entity_mgr = get_battle_instance()
            .unwrap()
            ._GameWorld()
            .unwrap()
            ._EntityManager()
            .unwrap();

        let monster_hps = BattleContext::get_instance()
            .battle_enemies
            .iter()
            .filter_map(|entity| {
                let Ok(entity) = entity_mgr.get_entity_by_runtime_id(entity.entity.uid) else {
                    return None;
                };

                if !entity.is_null() {
                    let turn_based_comp = entity
                        .get_component(RuntimeType::from_name(
                            "RPG.GameCore.TurnBasedAbilityComponent",
                        ))
                        .unwrap();

                    let monster_info = get_monster_from_entity(entity);

                    if !turn_based_comp.is_null()
                        && let Ok(monster_info) = monster_info
                    {
                        let turn_based_comp =
                            RPG_GameCore_TurnBasedAbilityComponent(turn_based_comp.0);

                        return Some(serde_json::json!({
                            "name": monster_info.name,
                            "hp": fixpoint_raw_to_raw(
                                &turn_based_comp
                                    .get_property(RPG_GameCore_AbilityProperty::CurrentHP)
                                    .unwrap(),
                            ),
                            "monster_id": monster_info.id
                        }));
                    }
                }
                None
            })
            .collect::<Vec<_>>();

        BattleContext::handle_event(Ok(Event::OnTurnBegin(OnTurnBeginEvent {
            action_value: get_elapsed_av(instance),
            turn_owner,
            monster_hps,
            extra_data: serde_json::json!({
                "modifiers": modifiers,
                "ability_roperties": ability_properties
            }),
        })));
    });
}
