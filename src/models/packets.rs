use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::misc::{Avatar, Skill, TurnInfo};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Packet {
    size: u32,
    body: Vec<u8>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
struct Payload<'a, T: Serialize> {
    #[serde(rename = "type")]
    payload_type: &'a str,
    data: T
}

impl Packet {
    // pub fn new<T: Serialize>(body: T) -> Result<Self> {
    //     let body = serde_json::to_vec(&body)?;
    //     Ok(Packet {
    //         size: body.len() as u32,
    //         body,
    //     })
    // }

    pub fn from_event_packet(event_packet: EventPacket) -> Result<Self> {
        let payload = Payload {
            payload_type: event_packet.name(),
            data: event_packet,
        };
        let body = serde_json::to_vec(&payload)?;
        Ok(Packet {
            size: body.len() as u32,
            body,
        })

    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut stream = Vec::new();
        // stream.extend_from_slice(&self.size.to_ne_bytes());
        stream.extend_from_slice(&self.body);

        stream
    }
}

macro_rules! event_packet {
    ($(
        $variant_name:ident { $ ($arg_name:ident : $arg_type:ty),* }
    )*) => {
        #[derive(Serialize, Clone)]
        #[serde(untagged)]
        pub enum EventPacket {
            $(
                $variant_name { $($arg_name : $arg_type),* },
            )*
        }

        impl EventPacket {
            pub fn name(&self) -> &'static str {
                match self {
                    $(
                        Self::$variant_name { .. } => stringify!($variant_name),
                    )*
                }
            }
        }
    };
}

event_packet!(
    // Heartbeat {}
    Error { msg: String }
    // Game
    BattleBegin {}
    SetBattleLineup { avatars: Vec<Avatar> }
    OnDamage { attacker: Avatar, damage: f64 }
    TurnBegin { action_value: f64 }
    TurnEnd { avatars: Vec<Avatar>, avatars_damage: Vec<f64>, total_damage: f64, action_value: f64 }
    OnKill { attacker: Avatar }
    OnUseSkill { avatar: Avatar, skill: Skill }

    BattleEnd { avatars: Vec<Avatar>, turn_history: Vec<TurnInfo>, turn_count: usize, total_damage: f64, action_value: f64 }
);