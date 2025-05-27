use crate::{battle::BattleContext, models::events::*, subscribers::battle::*};

#[function_name::named]
pub fn on_battle_end(instance: usize) {
    log::debug!(function_name!());
    ON_BATTLE_END_Detour.call(instance);
    BattleContext::handle_event(Ok(Event::OnBattleEnd));
    unsafe {
        TURN_BASED_GAME_MODE_REF = None;
    }
}
