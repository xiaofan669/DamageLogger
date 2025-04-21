use std::fmt;

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Avatar {
    pub id: u32,
    pub name: String,
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
    pub avatars_turn_damage: Vec<f64>,
    pub total_damage: f64,
    // pub turn_owner: Avatar
}
