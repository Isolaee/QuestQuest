use crate::action::ActionInstance;
use crate::world_state::{FactValue, WorldState};

#[derive(Clone, Debug)]
pub struct AttackTemplate {
    pub name_base: String,
    pub damage: i32,
    pub cost: f32,
    pub range: i32,
}

impl AttackTemplate {
    pub fn ground_for_state(
        &self,
        state: &WorldState,
        agent: Option<String>,
    ) -> Vec<ActionInstance> {
        let mut out: Vec<ActionInstance> = Vec::new();

        for (k, v) in state.facts.iter() {
            if k == "EnemyAt" || k.starts_with("EnemyAt:") {
                if let FactValue::Str(loc) = v {
                    let enemy_id = if k == "EnemyAt" {
                        None
                    } else {
                        k.split_once(':').map(|(_, id)| id.to_string())
                    };

                    let mut preconds: Vec<(String, FactValue)> = Vec::new();
                    preconds.push(("At".to_string(), FactValue::Str(loc.clone())));
                    if let Some(eid) = &enemy_id {
                        let alive_key = format!("EnemyAlive:{}", eid);
                        if let Some(FactValue::Bool(true)) = state.get(&alive_key) {
                            preconds.push((alive_key.clone(), FactValue::Bool(true)));
                        }
                    } else if let Some(FactValue::Bool(true)) = state.get("EnemyAlive") {
                        preconds.push(("EnemyAlive".to_string(), FactValue::Bool(true)));
                    }

                    let mut effects: Vec<(String, FactValue)> = Vec::new();
                    if let Some(eid) = &enemy_id {
                        let health_key = format!("EnemyHealth:{}", eid);
                        if let Some(FactValue::Int(h)) = state.get(&health_key) {
                            let new_h = (*h - self.damage).max(0);
                            effects.push((health_key.clone(), FactValue::Int(new_h)));
                            if new_h <= 0 {
                                effects.push((
                                    format!("EnemyAlive:{}", eid.clone()),
                                    FactValue::Bool(false),
                                ));
                            }
                        } else if state.get("EnemyAlive").is_some() || enemy_id.is_some() {
                            let id_str = enemy_id.clone().unwrap_or_default();
                            effects
                                .push((format!("EnemyAlive:{}", id_str), FactValue::Bool(false)));
                        }
                    } else if let Some(FactValue::Int(h)) = state.get("EnemyHealth") {
                        let new_h = (*h - self.damage).max(0);
                        effects.push(("EnemyHealth".to_string(), FactValue::Int(new_h)));
                        if new_h <= 0 {
                            effects.push(("EnemyAlive".to_string(), FactValue::Bool(false)));
                        }
                    } else if state.get("EnemyAlive").is_some() {
                        effects.push(("EnemyAlive".to_string(), FactValue::Bool(false)));
                    }

                    let name = if let Some(eid) = &enemy_id {
                        format!("{}:{}@{}", self.name_base, eid, loc)
                    } else {
                        format!("{}@{}", self.name_base, loc)
                    };

                    let instance = ActionInstance {
                        name,
                        preconditions: preconds,
                        effects,
                        cost: self.cost,
                        agent: agent.clone(),
                    };
                    out.push(instance);
                }
            }
        }

        out
    }
}
