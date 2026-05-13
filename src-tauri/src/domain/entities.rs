// src-tauri/src/domain/entities.rs
use serde::{Deserialize, Serialize};
use crate::domain::value_objects::{HitPoints, Initiative, Condition};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Combatant {
    pub id: String,
    pub name: String,
    pub hit_points: HitPoints,
    pub initiative: Initiative,
    pub conditions: Vec<Condition>,
    pub is_player: bool,
}

impl Combatant {
    pub fn new(id: String, name: String, max_hp: i32, is_player: bool) -> Self {
        Self {
            id,
            name,
            hit_points: HitPoints::new(max_hp),
            initiative: Initiative::new(0),
            conditions: Vec::new(),
            is_player,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Monster {
    pub id: String,
    pub name: String,
    pub hit_points: HitPoints,
    pub initiative: Initiative,
    pub conditions: Vec<Condition>,
}

impl Monster {
    pub fn new(id: String, name: String, max_hp: i32) -> Self {
        Self {
            id,
            name,
            hit_points: HitPoints::new(max_hp),
            initiative: Initiative::new(0),
            conditions: Vec::new(),
        }
    }
}