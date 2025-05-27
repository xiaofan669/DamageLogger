use function_name::named;

use crate::{
    battle::BattleContext,
    kreide::{
        helpers::{fixpoint_raw_to_raw, get_avatar_from_entity},
        types::{
            RPG_GameCore_AbilityProperty, RPG_GameCore_EntityType,
            RPG_GameCore_TurnBasedAbilityComponent,
        },
    },
    models::{
        events::{Event, OnStatChangeEvent},
        misc::{Entity, Stat, Team},
    },
    safe_call,
};

#[named]
pub fn handle_hp_change(turn_based_ability_component: usize) {
    log::debug!(function_name!());
    safe_call!(unsafe {
        let component = RPG_GameCore_TurnBasedAbilityComponent(turn_based_ability_component);
        let hp = fixpoint_raw_to_raw(
            &component
                .get_property(RPG_GameCore_AbilityProperty::CurrentHP)
                .unwrap(),
        );
        let entity = component._OwnerRef().unwrap();

        match entity._EntityType().unwrap() {
            RPG_GameCore_EntityType::Avatar => {
                let e = match get_avatar_from_entity(entity) {
                    Ok(avatar) => Ok(Event::OnStatChange(OnStatChangeEvent {
                        entity: Entity {
                            uid: avatar.id,
                            team: Team::Player,
                        },
                        stat: Stat::HP(hp),
                    })),
                    Err(e) => Err(anyhow::anyhow!(
                        "{} Avatar Event Error: {}",
                        function_name!(),
                        e
                    )),
                };
                BattleContext::handle_event(e);
            }
            RPG_GameCore_EntityType::Monster => {
                BattleContext::handle_event(Ok(Event::OnStatChange(OnStatChangeEvent {
                    entity: Entity {
                        uid: entity._RuntimeID_k__BackingField().unwrap(),
                        team: Team::Enemy,
                    },
                    stat: Stat::HP(hp),
                })));
            }
            _ => {}
        }
    });
}
