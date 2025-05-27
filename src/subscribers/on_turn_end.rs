use function_name::named;

use crate::{
    battle::BattleContext,
    kreide::{
        helpers::{fixpoint_raw_to_raw, get_monster_from_entity},
        il2cpp::native::RuntimeType,
        types::{RPG_GameCore_AbilityProperty, RPG_GameCore_TurnBasedAbilityComponent},
    },
    models::events::{Event, OnTurnEndEvent},
    safe_call,
    subscribers::battle::{get_battle_instance, ON_TURN_END_Detour},
};

#[named]
pub fn on_turn_end(instance: usize, a1: i32) -> usize {
    log::debug!(function_name!());
    // Can match player v enemy turn w/
    // RPG.GameCore.TurnBasedGameMode.GetCurrentTurnTeam
    let monster_hps = safe_call!({
        let entity_mgr = get_battle_instance()
            .unwrap()
            ._GameWorld()
            .unwrap()
            ._EntityManager()
            .unwrap();

        BattleContext::get_instance()
            .battle_enemies
            .iter()
            .filter_map(|entity| unsafe {
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
            .collect::<Vec<_>>()
    })
    .unwrap_or_default();
    BattleContext::handle_event(Ok(Event::OnTurnEnd(OnTurnEndEvent { monster_hps })));
    ON_TURN_END_Detour.call(instance, a1)
}
