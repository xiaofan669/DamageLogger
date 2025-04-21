
use super::misc::{Avatar, Skill};

pub enum Event {
    OnBattleBegin,
    OnSetLineup(OnSetLineupEvent),
    OnDamage(OnDamageEvent),
    OnTurnBegin(OnTurnBeginEvent),
    OnTurnEnd,
    OnKill(OnKillEvent),
    OnUseSkill(OnUseSkillEvent),
    OnBattleEnd(OnBattleEndEvent),
}

pub struct OnTurnBeginEvent {
    pub action_value: f64
}

pub struct OnBattleEndEvent {
    pub action_value: f64
}

pub struct OnUseSkillEvent {
    pub avatar: Avatar,
    pub skill: Skill
}

pub struct OnSetLineupEvent {
    pub avatars: Vec<Avatar>,
}

pub struct OnDamageEvent {
    pub attacker: Avatar,
    pub damage: f64,
}

pub struct OnKillEvent {
    pub attacker: Avatar,
}