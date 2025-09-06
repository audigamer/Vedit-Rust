use std::collections::{HashMap, HashSet};

use macroquad::input::{get_keys_down, KeyCode};

pub struct ActionMap {
    pub actions: HashMap<String, Action>
}

impl ActionMap {
    pub fn default() -> Self {
        Self { actions: HashMap::from([
            (String::from("left"), Action { key_list: HashSet::from([KeyCode::A, KeyCode::Left]) }),
            (String::from("right"), Action { key_list: HashSet::from([KeyCode::D, KeyCode::Right]) }),
            (String::from("up"), Action { key_list: HashSet::from([KeyCode::W, KeyCode::Up]) }),
            (String::from("down"), Action { key_list: HashSet::from([KeyCode::S, KeyCode::Down]) }),
        ]) }
    }

    pub fn get_action_by_name(&self, name: &str) -> Option<&Action> {
        self.actions.get(name)
    }

    pub fn is_action_active(&self, action_name: &str) -> bool {
        if let Some(action) = self.get_action_by_name(action_name) {
            action.is_active()
        } else {
            false
        }
    }
}

pub struct Action {
    pub key_list: HashSet<KeyCode>
}

impl Action {
    pub fn is_active(&self) -> bool {
        get_keys_down()
        .intersection(&self.key_list)
        .collect::<HashSet<_>>()
        .len() > 0
    }
}