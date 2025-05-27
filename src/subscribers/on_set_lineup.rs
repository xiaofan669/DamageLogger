use function_name::named;

use crate::{
    battle::BattleContext,
    kreide::{
        helpers::get_avatar_from_id,
        types::{RPG_Client_BattleAssetPreload, RPG_GameCore_LineUpCharacter},
    },
    models::{
        events::{Event, OnSetLineupEvent},
        misc::Avatar,
    },
    safe_call,
    subscribers::battle::ON_SET_LINEUP_Detour,
};
use anyhow::{anyhow, Error};

#[named]
pub fn on_set_lineup(instance: usize, is_async: bool, on_load_finish: usize) {
    log::debug!(function_name!());
    safe_call!(unsafe {
        let battle_asset_preload = RPG_Client_BattleAssetPreload(instance);
        let battle_lineup_data = battle_asset_preload._LineupData().unwrap();
        let light_team = battle_lineup_data.LightTeam().unwrap();
        let mut avatars = Vec::<Avatar>::new();
        let mut errors = Vec::<Error>::new();
        for character in light_team.to_vec::<RPG_GameCore_LineUpCharacter>() {
            let avatar_id = character.CharacterID().unwrap();
            log::debug!("AVATAR ID: {avatar_id}");
            match get_avatar_from_id(avatar_id) {
                Ok(avatar) => avatars.push(avatar),
                Err(e) => {
                    errors.push(e);
                }
            }
        }

        // Unsure if you can have more than one support char
        let extra_team = battle_lineup_data.ExtraTeam().unwrap();
        for character in extra_team.to_vec::<RPG_GameCore_LineUpCharacter>() {
            let avatar_id = character.CharacterID().unwrap();
            log::debug!("AVATAR ID: {avatar_id}");
            match get_avatar_from_id(avatar_id) {
                Ok(avatar) => avatars.push(avatar),
                Err(e) => {
                    errors.push(e);
                }
            }
        }

        let event = if !errors.is_empty() {
            let errors = errors.iter().map(|e| format!("{e}\n")).collect::<String>();
            Err(anyhow!(errors))
        } else {
            Ok(Event::OnSetBattleLineup(OnSetLineupEvent { avatars }))
        };
        BattleContext::handle_event(event);
    });

    ON_SET_LINEUP_Detour.call(instance, is_async, on_load_finish);
}
