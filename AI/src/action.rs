use crate::world_state::{FactValue, WorldState};

/// Action template describes preconditions, effects and cost.
#[derive(Clone, Debug)]
pub struct ActionTemplate {
    pub name: String,
    pub preconditions: Vec<(String, FactValue)>,
    pub effects: Vec<(String, FactValue)>,
    pub cost: f32,
}

impl ActionTemplate {
    pub fn is_applicable(&self, state: &WorldState) -> bool {
        self.preconditions
            .iter()
            .all(|(k, v)| state.satisfies(k, v))
    }
}

/// Simple planning goal: a single fact key/value pair to be achieved.
#[derive(Clone, Debug)]
pub struct Goal {
    pub key: String,
    pub value: FactValue,
}

/// Grounded (parameterized) action instance with concrete preconditions/effects.
#[derive(Clone, Debug)]
pub struct ActionInstance {
    pub name: String,
    pub preconditions: Vec<(String, FactValue)>,
    pub effects: Vec<(String, FactValue)>,
    pub cost: f32,
    /// Optional agent id that this action belongs to (for team-level planning)
    pub agent: Option<String>,
}

impl ActionInstance {
    pub fn is_applicable(&self, state: &WorldState) -> bool {
        self.preconditions
            .iter()
            .all(|(k, v)| state.satisfies(k, v))
    }
}

pub fn ground_action_from_template(
    template: &ActionTemplate,
    agent: Option<String>,
) -> ActionInstance {
    ActionInstance {
        name: template.name.clone(),
        preconditions: template.preconditions.clone(),
        effects: template.effects.clone(),
        cost: template.cost,
        agent,
    }
}
