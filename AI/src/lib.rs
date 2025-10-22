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

// Helper alias to reduce type complexity warnings in clippy. Represents a
// deduplication key composed of the sorted state's fact vector and the
// sorted list of satisfied agent ids.
type StateSatisfiedKey = (Vec<(String, FactValue)>, Vec<String>);

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
    // Joint team planner: search for a global interleaved sequence of actions (indices into `actions`)
    // that results in at least one goal satisfied for each agent (one of the goals in their priority list).
    // If joint planning fails (node limit), fall back to the previous sequential per-agent planner.

    // Precompute per-agent action lists and mapping from global action index -> local index in that agent's list
    let mut per_agent_actions: HashMap<String, Vec<ActionInstance>> = HashMap::new();
    let mut global_to_local: HashMap<String, HashMap<usize, usize>> = HashMap::new();
    for agent in agent_order {
        let mut vec: Vec<ActionInstance> = Vec::new();
        let mut map: HashMap<usize, usize> = HashMap::new();
        for (i, a) in actions.iter().enumerate() {
            let include = match &a.agent {
                Some(id) => id == agent,
                None => true,
            };
            if include {
                map.insert(i, vec.len());
                vec.push(a.clone());
            }
        }
        per_agent_actions.insert(agent.clone(), vec);
        global_to_local.insert(agent.clone(), map);
    }

    // Helper: determine which agent goals are already satisfied in a state
    let mut initially_satisfied: HashSet<String> = HashSet::new();
    for agent in agent_order {
        if let Some(goals) = goals_per_agent.get(agent) {
            for g in goals {
                if start.satisfies(&g.key, &g.value) {
                    initially_satisfied.insert(agent.clone());
                    break;
                }
            }
        }
    }

    // Search node for joint planner
    #[derive(Clone)]
    struct TeamNode {
        state: WorldState,
        g: f32,
        f: f32,
        actions: Vec<usize>, // global action indices
        satisfied: HashSet<String>,
    }

    impl PartialEq for TeamNode {
        fn eq(&self, other: &Self) -> bool {
            self.f == other.f
        }
    }
    impl Eq for TeamNode {}
    impl PartialOrd for TeamNode {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Ord for TeamNode {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            // reverse because BinaryHeap is max-heap by default
            other
                .f
                .partial_cmp(&self.f)
                .unwrap_or(std::cmp::Ordering::Equal)
        }
    }

    // Heuristic: estimate remaining cost as the sum, for each unsatisfied agent,
    // of the minimal action.cost among actions that would satisfy one of their goals.
    // This is admissible (lower bound) because it ignores preconditions and assumes
    // a single action can be executed for each remaining agent.
    fn estimate_remaining_cost(
        satisfied: &HashSet<String>,
        actions: &[ActionInstance],
        goals_per_agent: &HashMap<String, Vec<Goal>>,
        agent_order: &[String],
    ) -> f32 {
        let mut total = 0.0f32;
        for agent in agent_order {
            if satisfied.contains(agent) {
                continue;
            }
            // For this agent, find the minimal cost among actions that have an effect
            // matching any of the agent's goals and are either owned by the agent or global.
            let mut best = None::<f32>;
            if let Some(goals) = goals_per_agent.get(agent) {
                'goal_loop: for g in goals {
                    for a in actions.iter() {
                        // Check agent tag: action can be global (None) or belong to this agent
                        let include = match &a.agent {
                            Some(id) => id == agent,
                            None => true,
                        };
                        if !include {
                            continue;
                        }
                        for (ek, ev) in a.effects.iter() {
                            if ek == &g.key && ev == &g.value {
                                let c = a.cost;
                                match best {
                                    None => best = Some(c),
                                    Some(b) if c < b => best = Some(c),
                                    _ => {}
                                }
                                // found an action satisfying this goal; no need to check more effects
                                continue 'goal_loop;
                            }
                        }
                    }
                }
            }
            total += best.unwrap_or(1.0); // fallback lower bound
        }
        total
    }

    let mut open = BinaryHeap::new();
    let mut seen: HashSet<StateSatisfiedKey> = HashSet::new();

    let total_agents = agent_order.len();

    open.push(TeamNode {
        state: start.clone(),
        g: 0.0,
        f: estimate_remaining_cost(&initially_satisfied, actions, goals_per_agent, agent_order),
        actions: Vec::new(),
        satisfied: initially_satisfied.clone(),
    });

    let mut nodes_expanded = 0usize;
    let mut found_joint_plan: Option<Vec<usize>> = None;

    while let Some(node) = open.pop() {
        nodes_expanded += 1;
        if nodes_expanded > max_nodes_per_agent {
            break; // give up on joint planning
        }

        // Goal test: all agents satisfied
        if node.satisfied.len() == total_agents {
            found_joint_plan = Some(node.actions.clone());
            break;
        }

        // Deduplicate by state+satisfied set
        let mut key_vec: Vec<(String, FactValue)> = node.state.facts.clone().into_iter().collect();
        key_vec.sort_by(|a, b| a.0.cmp(&b.0));
        let mut sat_vec: Vec<String> = node.satisfied.iter().cloned().collect();
        sat_vec.sort();
        if seen.contains(&(key_vec.clone(), sat_vec.clone())) {
            continue;
        }
        seen.insert((key_vec, sat_vec));

        // Expand applicable GLOBAL actions (all actions in `actions` may be used in the team plan)
        for (i, a) in actions.iter().enumerate() {
            if a.is_applicable(&node.state) {
                let mut new_state = node.state.clone();
                new_state.apply_effects(&a.effects);
                let g2 = node.g + a.cost;

                // Update satisfied set by checking for newly satisfied goals for any agent
                let mut new_satisfied = node.satisfied.clone();
                for agent in agent_order {
                    if new_satisfied.contains(agent) {
                        continue;
                    }
                    if let Some(goals) = goals_per_agent.get(agent) {
                        for g in goals {
                            if new_state.satisfies(&g.key, &g.value) {
                                new_satisfied.insert(agent.clone());
                                break;
                            }
                        }
                    }
                }

                let h2 =
                    estimate_remaining_cost(&new_satisfied, actions, goals_per_agent, agent_order);

                let mut actions2 = node.actions.clone();
                actions2.push(i);

                open.push(TeamNode {
                    state: new_state,
                    g: g2,
                    f: g2 + h2,
                    actions: actions2,
                    satisfied: new_satisfied,
                });
            }
        }
    }

    // Prepare result map
    let mut result: HashMap<String, Plan> = HashMap::new();

    if let Some(joint) = found_joint_plan {
        // Split joint plan into per-agent plans. We'll assign global (agent==None) actions to the
        // first agent (in agent_order) that still lacks a satisfied goal at that point; if all are
        // satisfied, assign to the first agent as fallback.
        // Initialize per-agent plan vectors
        for agent in agent_order {
            result.insert(agent.clone(), Vec::new());
        }

        let mut sim_state = start.clone();
        let mut satisfied = HashSet::new();
        // Track per-agent assigned (simulated) cost to enable cost-balancing when assigning global actions
        let mut assigned_costs: HashMap<String, f32> = HashMap::new();
        for agent in agent_order {
            assigned_costs.insert(agent.clone(), 0.0);
        }
        for agent in agent_order {
            if let Some(goals) = goals_per_agent.get(agent) {
                for g in goals {
                    if sim_state.satisfies(&g.key, &g.value) {
                        satisfied.insert(agent.clone());
                        break;
                    }
                }
            }
        }

        for &global_idx in &joint {
            if let Some(action) = actions.get(global_idx) {
                // Decide which agent this action is assigned to
                let assigned_agent = if let Some(aid) = &action.agent {
                    aid.clone()
                } else {
                    // Candidate agents are those for whom this global action exists in their per-agent action list
                    let mut candidates: Vec<&String> = Vec::new();
                    for agent in agent_order {
                        if let Some(map) = global_to_local.get(agent) {
                            if map.contains_key(&global_idx) {
                                candidates.push(agent);
                            }
                        }
                    }

                    // Fallback: if no candidate agent has this exact grounded action, allow any agent
                    if candidates.is_empty() {
                        candidates = agent_order.iter().collect();
                    }

                    // Score candidates: prefer agents for whom this action immediately satisfies an unsatisfied goal (big bonus),
                    // and prefer agents with lower current assigned cost (to balance load).
                    let mut best_agent: Option<String> = None;
                    let mut best_score: f32 = f32::NEG_INFINITY;
                    for &cand in &candidates {
                        let mut score: f32 = 0.0;
                        // bonus if this action's effects satisfy any unsatisfied goal for candidate
                        if let Some(goals) = goals_per_agent.get(cand) {
                            for g in goals {
                                if !satisfied.contains(cand)
                                    && action
                                        .effects
                                        .iter()
                                        .any(|(k, v)| k == &g.key && v == &g.value)
                                {
                                    score += 1000.0;
                                    break;
                                }
                            }
                        }
                        // penalize by currently assigned cost (prefer lower cost)
                        if let Some(c) = assigned_costs.get(cand) {
                            score -= *c;
                        }

                        if score > best_score {
                            best_score = score;
                            best_agent = Some(cand.clone());
                        }
                    }

                    best_agent.unwrap_or_else(|| agent_order[0].clone())
                };

                // Map global index -> local index for that agent
                let local_map = global_to_local.get(&assigned_agent).unwrap();
                if let Some(local_idx) = local_map.get(&global_idx) {
                    if let Some(plan_vec) = result.get_mut(&assigned_agent) {
                        plan_vec.push(*local_idx);
                    }
                    // update assigned cost for balancing
                    if let Some(cost) = assigned_costs.get_mut(&assigned_agent) {
                        *cost += action.cost;
                    }
                } else {
                    // As a fallback, try to find a matching action by equality (rare)
                    if let Some(agent_actions) = per_agent_actions.get(&assigned_agent) {
                        if let Some(pos) = agent_actions.iter().position(|aa| {
                            aa.name == action.name
                                && aa.effects == action.effects
                                && aa.preconditions == action.preconditions
                        }) {
                            if let Some(plan_vec) = result.get_mut(&assigned_agent) {
                                plan_vec.push(pos);
                                if let Some(cost) = assigned_costs.get_mut(&assigned_agent) {
                                    *cost += action.cost;
                                }
                            }
                        }
                    }
                }

                // Apply effects to sim_state and update satisfied set
                sim_state.apply_effects(&action.effects);
                // If this action satisfied a goal for the assigned agent, preference already applied via scoring,
                // but we still update the satisfied set here.
                for agent in agent_order {
                    if satisfied.contains(agent) {
                        continue;
                    }
                    if let Some(goals) = goals_per_agent.get(agent) {
                        for g in goals {
                            if sim_state.satisfies(&g.key, &g.value) {
                                satisfied.insert(agent.clone());
                                break;
                            }
                        }
                    }
                }
            }
        }

        return result;
    }

    // Fallback: original sequential per-agent planner
    // Log a debug message so callers can observe that joint planning failed and fallback was used.
    eprintln!("ai::plan_for_team: joint team planning failed after {} node expansions, falling back to sequential per-agent planner", nodes_expanded);
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
                for &action_index in &plan {
                    let action = &agent_actions[action_index];
                    current_state.apply_effects(&action.effects);
                }
                result.insert(agent.clone(), plan);
            } else {
                result.insert(agent.clone(), Vec::new());
            }
        } else {
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
