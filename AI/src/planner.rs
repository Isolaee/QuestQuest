use crate::action::{ActionInstance, ActionTemplate, Goal};
use crate::world_state::{FactValue, WorldState};
use std::collections::{BinaryHeap, HashMap};

pub type Plan = Vec<usize>;

#[derive(Clone)]
struct SearchNode {
    state: WorldState,
    g: f32,
    f: f32,
    actions: Vec<usize>,
}

impl PartialEq for SearchNode {
    fn eq(&self, other: &Self) -> bool {
        self.f == other.f && self.g == other.g
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
        match other
            .f
            .partial_cmp(&self.f)
            .unwrap_or(std::cmp::Ordering::Equal)
        {
            std::cmp::Ordering::Equal => other
                .g
                .partial_cmp(&self.g)
                .unwrap_or(std::cmp::Ordering::Equal),
            ord => ord,
        }
    }
}

/// Very small admissible heuristic: 0 if goal satisfied, otherwise 0.0 which reduces to Dijkstra.
fn heuristic(_state: &WorldState, _goal: &Goal, _actions: &[ActionInstance]) -> f32 {
    // Currently no heuristic implemented: return 0.0 for admissible (Dijkstra/A* fallback)
    0.0
}

fn state_key(state: &WorldState) -> Vec<(String, FactValue)> {
    let mut v: Vec<(String, FactValue)> = state.facts.clone().into_iter().collect();
    v.sort_by(|a, b| a.0.cmp(&b.0));
    v
}

/// Plan over grounded ActionInstance list using A*/Dijkstra. Returns indices into `actions`.
pub fn plan_instances(
    start: &WorldState,
    actions: &[ActionInstance],
    goal: &Goal,
    max_nodes: usize,
) -> Option<Plan> {
    let mut open = BinaryHeap::new();
    let start_h = heuristic(start, goal, actions);
    open.push(SearchNode {
        state: start.clone(),
        g: 0.0,
        f: start_h,
        actions: Vec::new(),
    });

    let mut best_g: HashMap<Vec<(String, FactValue)>, f32> = HashMap::new();
    let start_key = state_key(start);
    best_g.insert(start_key, 0.0);

    let mut nodes = 0usize;
    while let Some(node) = open.pop() {
        nodes += 1;
        if nodes > max_nodes {
            break;
        }

        if node.state.satisfies(&goal.key, &goal.value) {
            return Some(node.actions);
        }

        // For each applicable action, expand
        for (i, a) in actions.iter().enumerate() {
            if a.is_applicable(&node.state) {
                let mut new_state = node.state.clone();
                new_state.apply_effects(&a.effects);
                let g2 = node.g + a.cost;
                let key = state_key(&new_state);
                let recorded = best_g.get(&key).cloned();
                if recorded.is_none() || g2 < recorded.unwrap() {
                    best_g.insert(key, g2);
                    let h = heuristic(&new_state, goal, actions);
                    let mut actions2 = node.actions.clone();
                    actions2.push(i);
                    open.push(SearchNode {
                        state: new_state,
                        g: g2,
                        f: g2 + h,
                        actions: actions2,
                    });
                }
            }
        }
    }

    None
}

pub fn plan(
    start: &WorldState,
    actions: &[ActionTemplate],
    goal: &Goal,
    max_nodes: usize,
) -> Option<Plan> {
    let instances: Vec<ActionInstance> = actions
        .iter()
        .map(|t| crate::action::ground_action_from_template(t, None))
        .collect();
    plan_instances(start, &instances, goal, max_nodes)
}

// HashMap already imported above with other collections

/// Simple sequential per-agent planner: for each agent in `agent_order` try goals in order
/// using `plan_instances` on the agent-visible actions. Returns map agent_id -> plan
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
        // Filter actions visible to this agent (agent==None or matching agent id)
        let agent_actions: Vec<ActionInstance> = actions
            .iter()
            .filter(|a| match &a.agent {
                Some(id) => id == agent,
                None => true,
            })
            .cloned()
            .collect();

        if let Some(goals) = goals_per_agent.get(agent) {
            let mut chosen: Option<Plan> = None;
            for g in goals {
                if let Some(p) =
                    plan_instances(&current_state, &agent_actions, g, max_nodes_per_agent)
                {
                    chosen = Some(p);
                    break;
                }
            }

            if let Some(plan) = chosen {
                // apply plan effects to current_state and record
                for &idx in &plan {
                    let a = &agent_actions[idx];
                    current_state.apply_effects(&a.effects);
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
