use function_name::named;

use crate::{
    battle::BattleContext,
    kreide::{
        helpers::{fixpoint_raw_to_raw, get_avatar_from_entity},
        types::{
            FixPoint, RPG_GameCore_AbilityProperty, RPG_GameCore_EntityType,
            RPG_GameCore_TurnBasedAbilityComponent,
        },
    },
    models::{
        events::{Event, OnStatChangeEvent},
        misc::{Entity, Stat, Team},
    },
    safe_call,
    subscribers::battle::ON_STAT_CHANGE_Detour,
};

#[named]
pub fn on_stat_change(
    instance: usize,
    property: RPG_GameCore_AbilityProperty,
    a2: i32,
    new_stat: FixPoint,
    a4: usize,
) {
    log::debug!(function_name!());
    ON_STAT_CHANGE_Detour.call(instance, property, a2, new_stat, a4);
    safe_call!(unsafe {
        let instance = RPG_GameCore_TurnBasedAbilityComponent(instance);
        let entity = instance._OwnerRef().unwrap();

        let stat = match property {
            RPG_GameCore_AbilityProperty::CurrentHP => {
                Some(Stat::HP(fixpoint_raw_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::Attack => {
                Some(Stat::Attack(fixpoint_raw_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::Defence => {
                Some(Stat::Defense(fixpoint_raw_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::Speed => {
                Some(Stat::Speed(fixpoint_raw_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ActionDelay => {
                Some(Stat::AV(fixpoint_raw_to_raw(&new_stat) * 10.0))
            }
            _ => None,
        };

        if let Some(stat) = stat {
            match entity._EntityType().unwrap() {
                RPG_GameCore_EntityType::Avatar => {
                    let e = match get_avatar_from_entity(entity) {
                        Ok(avatar) => Ok(Event::OnStatChange(OnStatChangeEvent {
                            entity: Entity {
                                uid: avatar.id,
                                team: Team::Player,
                            },
                            stat,
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
                        stat,
                    })));
                }
                _ => {}
            }
        }
    });
}
