use std::collections::HashMap;

/// Small axial hex coordinate used for tile locations in the prototype.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct HexCoord {
    pub q: i32,
    pub r: i32,
}

/// Simple fact value enum for small prototype.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FactValue {
    Bool(bool),
    Int(i32),
    Str(String),
    Hex(HexCoord),
}

/// WorldState: small hashmap from string keys to FactValue.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorldState {
    pub facts: HashMap<String, FactValue>,
}

impl Default for WorldState {
    fn default() -> Self {
        Self::new()
    }
}

impl WorldState {
    pub fn new() -> Self {
        Self {
            facts: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: impl Into<String>, value: FactValue) {
        self.facts.insert(key.into(), value);
    }

    pub fn get(&self, key: &str) -> Option<&FactValue> {
        self.facts.get(key)
    }

    pub fn satisfies(&self, key: &str, value: &FactValue) -> bool {
        self.get(key) == Some(value)
    }

    pub fn apply_effects(&mut self, effects: &[(String, FactValue)]) {
        for (k, v) in effects {
            self.facts.insert(k.clone(), v.clone());
        }
    }
}
