#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(static_mut_refs)]

macro_rules! subscribe_function {
    (
        $detour:ident,
        $target:expr,
        $reroute:path
    ) => {
        #[allow(clippy::missing_transmute_annotations)]
        $detour.initialize(std::mem::transmute($target), $reroute)?;
        $detour.enable()?;
    };
}

pub mod battle;
mod handle_hp_change;
mod on_battle_begin;
mod on_battle_end;
mod on_combo;
mod on_damage;
mod on_direct_change_hp;
mod on_direct_damage_hp;
mod on_entity_defeated;
mod on_initialize_enemy;
mod on_set_lineup;
mod on_stat_change;
mod on_turn_begin;
mod on_turn_end;
mod on_update_cycle;
mod on_update_team_formation;
mod on_update_wave;
mod on_use_skill;
