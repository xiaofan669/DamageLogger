use function_name::named;

use crate::{
    battle::BattleContext,
    models::events::{Event, OnUpdateCycleEvent},
    subscribers::battle::ON_UPDATE_CYCLE_Detour,
};

#[named]
pub fn on_update_cycle(instance: usize) -> u32 {
    log::debug!(function_name!());
    let cycle = ON_UPDATE_CYCLE_Detour.call(instance);
    BattleContext::handle_event(Ok(Event::OnUpdateCycle(OnUpdateCycleEvent { cycle })));
    cycle
}
