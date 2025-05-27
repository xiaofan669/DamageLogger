use function_name::named;

use crate::{
    battle::BattleContext,
    kreide::types::RPG_GameCore_TurnBasedGameMode,
    models::events::{Event, OnUpdateWaveEvent},
    safe_call,
    subscribers::battle::ON_UPDATE_WAVE_Detour,
};

#[named]
pub fn on_update_wave(instance: usize) {
    log::debug!(function_name!());
    ON_UPDATE_WAVE_Detour.call(instance);
    safe_call!({
        BattleContext::handle_event(Ok(Event::OnUpdateWave(OnUpdateWaveEvent {
            wave: RPG_GameCore_TurnBasedGameMode(instance)
                ._WaveMonsterCurrentCount()
                .unwrap() as _,
        })));
    });
}
