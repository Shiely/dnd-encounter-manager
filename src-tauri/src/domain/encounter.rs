// src-tauri/src/domain/encounter.rs
use serde::{Deserialize, Serialize};
use crate::domain::{Combatant, UndoStack, Command};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Encounter {
    pub id: String,
    pub name: String,
    pub combatants: Vec<Combatant>,
    pub undo_stack: UndoStack,
    pub round: u32,
    pub current_turn_index: usize,
    pub is_active: bool,
}

impl Encounter {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            combatants: Vec::new(),
            undo_stack: UndoStack::new(5),
            round: 1,
            current_turn_index: 0,
            is_active: false,
        }
    }

    pub fn add_combatant(&mut self, combatant: Combatant) {
        self.combatants.push(combatant);
    }

    pub fn remove_combatant(&mut self, id: &str) -> Option<Combatant> {
        if let Some(pos) = self.combatants.iter().position(|c| c.id == id) {
            Some(self.combatants.remove(pos))
        } else {
            None
        }
    }

    pub fn get_combatant_mut(&mut self, id: &str) -> Option<&mut Combatant> {
        self.combatants.iter_mut().find(|c| c.id == id)
    }

    pub fn sort_by_initiative(&mut self) {
        self.combatants.sort_by(|a, b| {
            b.initiative.value.cmp(&a.initiative.value)
                .then_with(|| b.initiative.tiebreaker.cmp(&a.initiative.tiebreaker))
        });
    }

    pub fn next_turn(&mut self) {
        if self.combatants.is_empty() {
            return;
        }
        self.current_turn_index = (self.current_turn_index + 1) % self.combatants.len();
        if self.current_turn_index == 0 {
            self.round += 1;
        }
    }

    pub fn previous_turn(&mut self) {
        if self.combatants.is_empty() {
            return;
        }
        if self.current_turn_index == 0 {
            self.round = self.round.saturating_sub(1);
            self.current_turn_index = self.combatants.len() - 1;
        } else {
            self.current_turn_index -= 1;
        }
    }

    pub fn execute_command(&mut self, command: Box<dyn Command>, target_id: &str) -> Option<String> {
        if let Some(combatant) = self.get_combatant_mut(target_id) {
            command.execute(combatant);
            let desc = command.description();
            self.undo_stack.push(command);
            Some(desc)
        } else {
            None
        }
    }

    pub fn undo_last(&mut self) -> Option<String> {
        if let Some(target_id) = self.get_last_command_target() {
            if let Some(combatant) = self.get_combatant_mut(&target_id) {
                return self.undo_stack.undo_last(combatant);
            }
        }
        None
    }

    // Helper to track which combatant the last command affected (simplified)
    fn get_last_command_target(&self) -> Option<String> {
        // In a real implementation, you'd store the target_id with each command
        // For now, we'll assume the last modified combatant
        self.combatants.last().map(|c| c.id.clone())
    }

    pub fn start(&mut self) {
        self.is_active = true;
        self.sort_by_initiative();
        self.current_turn_index = 0;
        self.round = 1;
    }

    pub fn end(&mut self) {
        self.is_active = false;
    }
}