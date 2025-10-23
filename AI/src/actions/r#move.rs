use crate::world_state::{FactValue, WorldState};
use crate::action::{ActionInstance, ActionTemplate};

/// Very small helper to build a Move action template for a concrete from->to.
pub fn move_template(from: &str, to: &str, cost: f32) -> ActionTemplate {
    ActionTemplate {
        name: format!("Move:{}->{}", from, to),
        preconditions: vec![("At".to_string(), FactValue::Str(from.to_string()))],
        effects: vec![("At".to_string(), FactValue::Str(to.to_string()))],
        cost,
    }
}
