use function_name::named;

use crate::{
    kreide::types::FixPoint,
    subscribers::{battle::ON_DIRECT_CHANGE_HP_Detour, handle_hp_change::handle_hp_change},
};

#[named]
pub fn on_direct_change_hp(instance: usize, a1: i32, a2: FixPoint, a3: usize) {
    log::debug!(function_name!());
    ON_DIRECT_CHANGE_HP_Detour.call(instance, a1, a2, a3);
    handle_hp_change(instance);
}
