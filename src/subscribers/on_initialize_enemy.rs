use function_name::named;

use crate::{
    battle::BattleContext,
    kreide::{
        helpers::{fixpoint_to_raw, get_textmap_content},
        types::RPG_GameCore_MonsterDataComponent,
    },
    models::{
        events::{Event, OnInitializeEnemyEvent},
        misc::{Enemy, Stats},
    },
    safe_call,
    subscribers::battle::ON_INITIALIZE_ENEMY_Detour,
};

#[named]
pub fn on_initialize_enemy(instance: usize, turn_based_ability_component: usize) {
    log::debug!(function_name!());
    ON_INITIALIZE_ENEMY_Detour.call(instance, turn_based_ability_component);

    safe_call!({
        let monster_data_component = RPG_GameCore_MonsterDataComponent(instance);
        let row_data = monster_data_component._MonsterRowData().unwrap();
        let row = row_data._Row().unwrap();
        let monster_id = monster_data_component.get_monster_id().unwrap();
        let base_stats = Stats {
            level: row_data.get_level().unwrap(),
            hp: fixpoint_to_raw(&monster_data_component._DefaultMaxHP().unwrap()),
        };

        let name_id = row.MonsterName().unwrap();
        let monster_name = get_textmap_content(&name_id);
        let entity = monster_data_component._OwnerRef().unwrap();

        BattleContext::handle_event(Ok(Event::OnInitializeEnemy(OnInitializeEnemyEvent {
            enemy: Enemy {
                id: monster_id,
                uid: entity._RuntimeID_k__BackingField().unwrap(),
                name: monster_name.to_string(),
                base_stats,
                game_entity_ptr: entity.0,
            },
        })));
    });
}
