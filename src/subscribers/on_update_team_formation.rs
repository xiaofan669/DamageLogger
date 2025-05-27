use function_name::named;

use crate::{
    battle::BattleContext,
    kreide::types::{RPG_GameCore_TeamFormationComponent, RPG_GameCore_TeamType, EDJEDBLFIKE},
    models::{
        events::{Event, OnUpdateTeamFormationEvent},
        misc::{Entity, Team},
    },
    safe_call,
    subscribers::battle::ON_UPDATE_TEAM_FORMATION_Detour,
};

#[named]
pub fn on_update_team_formation(instance: usize) {
    log::debug!(function_name!());
    ON_UPDATE_TEAM_FORMATION_Detour.call(instance);
    safe_call!({
        let instance = RPG_GameCore_TeamFormationComponent(instance);
        if instance._Team().unwrap() == RPG_GameCore_TeamType::TeamDark {
            let team = instance._TeamFormationDatas();
            // Is this necessary?
            if let Ok(team) = team {
                let entities = team
                    .items()
                    .to_vec_sized::<EDJEDBLFIKE>(team.size())
                    .iter()
                    .map(|entity_formation| Entity {
                        uid: entity_formation
                            ._OwnerRef()
                            .unwrap()
                            ._RuntimeID_k__BackingField()
                            .unwrap(),
                        team: Team::Enemy,
                    })
                    .collect::<Vec<Entity>>();

                BattleContext::handle_event(Ok(Event::OnUpdateTeamFormation(
                    OnUpdateTeamFormationEvent {
                        entities,
                        team: Team::Enemy,
                    },
                )));
            }
        }
    });
}
