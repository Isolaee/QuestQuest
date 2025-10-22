//! Minimal GOAP skeleton for QuestQuest
//!
//! This crate provides a tiny, dependency-free GOAP planner prototype intended
//! for integration into the `Game` crate later. It focuses on a simple WorldState
//! model, Action templates/instances, and a forward A* planner with bounded search.

use std::collections::{BinaryHeap, HashMap, HashSet};

/// Simple fact value enum for small prototype.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FactValue {
    Bool(bool),
    Int(i32),
    Str(String),
}

/// WorldState: small hashmap from string keys to FactValue.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorldState {
    facts: HashMap<String, FactValue>,
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

/// Helper: ground a template into an instance by copying preconds/effects and setting agent
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

/// Simple goal representation: a single fact must equal a value.
#[derive(Clone, Debug)]
pub struct Goal {
    pub key: String,
    pub value: FactValue,
}

/// Planner result: sequence of template indices (actions)
pub type Plan = Vec<usize>;

// Node for A* search in state-space
#[derive(Clone)]
struct SearchNode {
    state: WorldState,
    g: f32,
    f: f32,
    actions: Vec<usize>,
}

// For BinaryHeap ordering (min-heap by f)
impl PartialEq for SearchNode {
    fn eq(&self, other: &Self) -> bool {
        self.f == other.f
    }
}
impl Eq for SearchNode {}
impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

/// Very small forward A* planner with node limit.
/// Plan using grounded ActionInstance list (supports agent-tagged actions).
pub fn plan_instances(
    start: &WorldState,
    actions: &[ActionInstance],
    goal: &Goal,
    max_nodes: usize,
) -> Option<Plan> {
    let mut open = BinaryHeap::new();
    let mut seen: HashSet<Vec<(String, FactValue)>> = HashSet::new();

    let h0 = heuristic(start, goal);
    open.push(SearchNode {
        state: start.clone(),
        g: 0.0,
        f: h0,
        actions: Vec::new(),
    });

    let mut nodes_expanded = 0usize;

    while let Some(node) = open.pop() {
        nodes_expanded += 1;
        if nodes_expanded > max_nodes {
            return None; // give up
        }

        // Goal test
        if node.state.satisfies(&goal.key, &goal.value) {
            return Some(node.actions);
        }

        // Deduplicate by checking state's facts map converted into a sorted Vec
        let mut key_vec: Vec<(String, FactValue)> = node.state.facts.clone().into_iter().collect();
        key_vec.sort_by(|a, b| a.0.cmp(&b.0));
        if seen.contains(&key_vec) {
            continue;
        }
        seen.insert(key_vec);

        // Expand applicable actions
        for (i, a) in actions.iter().enumerate() {
            if a.is_applicable(&node.state) {
                let mut new_state = node.state.clone();
                new_state.apply_effects(&a.effects);
                let g2 = node.g + a.cost;
                let h2 = heuristic(&new_state, goal);
                let mut actions2 = node.actions.clone();
                actions2.push(i);

                open.push(SearchNode {
                    state: new_state,
                    g: g2,
                    f: g2 + h2,
                    actions: actions2,
                });
            }
        }
    }

    None
}

/// Backwards-compatible wrapper: accept ActionTemplate list and call planner on grounded instances.
pub fn plan(
    start: &WorldState,
    actions: &[ActionTemplate],
    goal: &Goal,
    max_nodes: usize,
) -> Option<Plan> {
    let instances: Vec<ActionInstance> = actions
        .iter()
        .map(|t| ground_action_from_template(t, None))
        .collect();
    plan_instances(start, &instances, goal, max_nodes)
}

/// Sequential team-level planner: plans for agents in order, applying each agent's plan to the world state
/// so subsequent agents plan with updated state. This is simple but effective for cooperative planning.
pub fn plan_for_team(
    start: &WorldState,
    actions: &[ActionInstance],
    goals_per_agent: &HashMap<String, Vec<Goal>>,
    agent_order: &[String],
    max_nodes_per_agent: usize,
) -> HashMap<String, Plan> {
    let mut result: HashMap<String, Plan> = HashMap::new();
    let mut current_state = start.clone();

    for agent in agent_order {
        // Gather actions applicable to this agent (agent==None actions considered global)
        let agent_actions: Vec<ActionInstance> = actions
            .iter()
            .filter(|a| match &a.agent {
                Some(id) => id == agent,
                None => true,
            })
            .cloned()
            .collect();

        // Try goals for this agent in priority order
        if let Some(goals) = goals_per_agent.get(agent) {
            let mut chosen_plan: Option<Plan> = None;
            for goal in goals {
                if let Some(plan) =
                    plan_instances(&current_state, &agent_actions, goal, max_nodes_per_agent)
                {
                    chosen_plan = Some(plan);
                    break;
                }
            }

            if let Some(plan) = chosen_plan {
                // Store plan and apply it to current_state
                // Apply actions in plan order
                for &action_index in &plan {
                    let action = &agent_actions[action_index];
                    current_state.apply_effects(&action.effects);
                }
                result.insert(agent.clone(), plan);
            } else {
                // No plan found: empty plan
                result.insert(agent.clone(), Vec::new());
            }
        } else {
            // No goals for this agent
            result.insert(agent.clone(), Vec::new());
        }
    }

    result
}

fn heuristic(state: &WorldState, goal: &Goal) -> f32 {
    if state.satisfies(&goal.key, &goal.value) {
        0.0
    } else {
        // Very simple: 1.0 if not satisfied
        1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn poof_of_concept_simple_plan() {
        // Start: at A
        let mut start = WorldState::new();
        start.insert("At".to_string(), FactValue::Str("A".to_string()));

        // Actions: MoveAtoB, MoveBtoC
        let a1 = ActionTemplate {
            name: "MoveAtoB".to_string(),
            preconditions: vec![("At".to_string(), FactValue::Str("A".to_string()))],
            effects: vec![("At".to_string(), FactValue::Str("B".to_string()))],
            cost: 1.0,
        };
        let a2 = ActionTemplate {
            name: "MoveBtoC".to_string(),
            preconditions: vec![("At".to_string(), FactValue::Str("B".to_string()))],
            effects: vec![("At".to_string(), FactValue::Str("C".to_string()))],
            cost: 1.0,
        };

        let actions = vec![a1, a2];
        let goal = Goal {
            key: "At".to_string(),
            value: FactValue::Str("C".to_string()),
        };

        let plan_res = plan(&start, &actions, &goal, 1000);
        assert!(plan_res.is_some());
        let plan = plan_res.unwrap();
        // Expect two actions: 0 then 1
        assert_eq!(plan, vec![0usize, 1usize]);
    }
}
