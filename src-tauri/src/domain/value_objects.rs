// src-tauri/src/domain/value_objects.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct HitPoints {
    pub current: i32,
    pub max: i32,
    pub temporary: i32,
}

impl HitPoints {
    pub fn new(max: i32) -> Self {
        Self {
            current: max,
            max,
            temporary: 0,
        }
    }

    pub fn is_bloodied(&self) -> bool {
        self.current <= self.max / 2
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0
    }

    pub fn apply_damage(&mut self, amount: i32) {
        let remaining_temp = self.temporary - amount;
        if remaining_temp >= 0 {
            self.temporary = remaining_temp;
        } else {
            self.temporary = 0;
            self.current += remaining_temp; // remaining_temp is negative
        }
    }

    pub fn heal(&mut self, amount: i32) {
        self.current = (self.current + amount).min(self.max);
    }

    pub fn add_temporary(&mut self, amount: i32) {
        self.temporary = self.temporary.max(amount);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Initiative {
    pub value: i32,
    pub tiebreaker: u64, // for deterministic ordering
}

impl Initiative {
    pub fn new(value: i32) -> Self {
        Self {
            value,
            tiebreaker: 0,
        }
    }

    pub fn with_tiebreaker(mut self, tiebreaker: u64) -> Self {
        self.tiebreaker = tiebreaker;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Condition {
    Blinded,
    Charmed,
    Deafened,
    Frightened,
    Grappled,
    Incapacitated,
    Invisible,
    Paralyzed,
    Petrified,
    Poisoned,
    Prone,
    Restrained,
    Stunned,
    Unconscious,
    Exhaustion(u8),
}

impl Condition {
    pub fn name(&self) -> String {
        match self {
            Condition::Blinded => "Blinded".to_string(),
            Condition::Charmed => "Charmed".to_string(),
            Condition::Deafened => "Deafened".to_string(),
            Condition::Frightened => "Frightened".to_string(),
            Condition::Grappled => "Grappled".to_string(),
            Condition::Incapacitated => "Incapacitated".to_string(),
            Condition::Invisible => "Invisible".to_string(),
            Condition::Paralyzed => "Paralyzed".to_string(),
            Condition::Petrified => "Petrified".to_string(),
            Condition::Poisoned => "Poisoned".to_string(),
            Condition::Prone => "Prone".to_string(),
            Condition::Restrained => "Restrained".to_string(),
            Condition::Stunned => "Stunned".to_string(),
            Condition::Unconscious => "Unconscious".to_string(),
            Condition::Exhaustion(level) => format!("Exhaustion {}", level),
        }
    }
}