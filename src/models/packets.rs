use serde::Serialize;

use super::misc::{Avatar, Enemy, Entity, Skill, Stat, Team, TurnInfo};

macro_rules! packet {
    ($(
        $variant_name:ident { $ ($arg_name:ident : $arg_type:ty),* }
    )*) => {
        #[derive(Serialize, Clone)]
        #[serde(untagged)]
        pub enum Packet {
            $(
                $variant_name { $($arg_name : $arg_type),* },
            )*
        }

        impl Packet {
            pub fn name(&self) -> &'static str {
                match self {
                    $(
                        Self::$variant_name { .. } => stringify!($variant_name),
                    )*
                }
            }

            pub fn payload(&self) -> serde_json::Value {
                match self {
                    $(
                        Self::$variant_name { .. } => serde_json::to_value(&self).unwrap(),
                    )*
                }
            }
        }
    };
}

packet!(
    Connected {
        version: String
    }
    Error {
        msg: String
    }

    // Game
    OnBattleBegin {
        max_waves: u32,
        max_cycles: u32,
        stage_id: u32
    }

    OnSetBattleLineup {
        avatars: Vec<Avatar>
    }

    OnDamage {
        attacker: Avatar,
        damage: f64,
        damage_type: &'static str
    }

    OnTurnBegin {
        action_value: f64,
        turn_owner: Option<Avatar>,
        monster_hps: Vec<serde_json::Value>,
        extra_data: serde_json::Value
    }

    OnTurnEnd {
        turn_info: TurnInfo
    }

    OnEntityDefeated {
        killer: Avatar,
        entity_defeated: Avatar
    }

    OnUseSkill {
        avatar: Avatar,
        skill: Skill
    }

    OnUpdateWave {
        wave: u32
    }

    OnUpdateCycle {
        cycle: u32
    }

    OnStatChange {
        entity: Entity,
        stat: Stat
    }

    OnUpdateTeamFormation {
        entities: Vec<Entity>,
        team: Team
    }

    OnInitializeEnemy {
        enemy: Enemy
    }

    OnBattleEnd {
        avatars: Vec<Avatar>,
        turn_history: Vec<TurnInfo>,
        av_history: Vec<TurnInfo>,
        turn_count: usize,
        total_damage: f64,
        action_value: f64,
        cycle: u32,
        wave: u32,
        stage_id: u32
    }
);
