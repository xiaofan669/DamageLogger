use function_name::named;

use crate::{
    battle::BattleContext,
    kreide::types::RPG_GameCore_TurnBasedGameMode,
    models::events::{Event, OnBattleBeginEvent},
    safe_call,
    subscribers::battle::{ON_BATTLE_BEGIN_Detour, TURN_BASED_GAME_MODE_REF},
};

#[named]
pub fn on_battle_begin(instance: usize) {
    log::debug!(function_name!());
    ON_BATTLE_BEGIN_Detour.call(instance);
    safe_call!(unsafe {
        TURN_BASED_GAME_MODE_REF = Some(RPG_GameCore_TurnBasedGameMode(instance));
        BattleContext::handle_event(Ok(Event::OnBattleBegin(OnBattleBeginEvent {
            max_waves: RPG_GameCore_TurnBasedGameMode(instance)
                ._WaveMonsterMaxCount_k__BackingField()
                .unwrap() as usize as _,
            max_cycles: RPG_GameCore_TurnBasedGameMode(instance)
                ._ChallengeTurnLimit_k__BackingField()
                .unwrap(),
            stage_id: RPG_GameCore_TurnBasedGameMode(instance)
                ._CurrentWaveStageID_k__BackingField()
                .unwrap(),
        })));
    });
}
