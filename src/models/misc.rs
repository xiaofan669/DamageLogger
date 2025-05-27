use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Avatar {
    pub id: u32,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Enemy {
    pub id: u32,
    pub uid: u32,
    pub name: String,
    pub base_stats: Stats,
    pub game_entity_ptr: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BattleEntity {
    pub entity: Entity,
    pub battle_stats: BattleStats,
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct BattleStats {
    pub hp: f64,
    pub attack: f64,
    pub defense: f64,
    pub speed: f64,
    pub av: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Stats {
    pub level: u32,
    pub hp: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Entity {
    pub uid: u32,
    pub team: Team,
}

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize)]
pub enum Team {
    Player,
    Enemy,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Stat {
    HP(f64),
    Attack(f64),
    Defense(f64),
    Speed(f64),
    AV(f64),
}

impl fmt::Display for Avatar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Skill {
    pub name: String,
    #[serde(rename = "type")]
    pub skill_type: String,
}

impl fmt::Display for Skill {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.skill_type, self.name)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct TurnInfo {
    pub action_value: f64,
    pub cycle: u32,
    pub wave: u32,
    pub avatars_turn_damage: Vec<f64>,
    pub total_damage: f64,
    pub monster_hps: Vec<serde_json::Value>,
}
