use crate::action::ActionTemplate;
use crate::world_state::{FactValue, HexCoord};

/// Very small helper to build a Move action template for a concrete from->to using axial hex coords.
pub fn move_template(from: HexCoord, to: HexCoord, cost: f32) -> ActionTemplate {
    let name = format!("Move:({},{})->({},{})", from.q, from.r, to.q, to.r);
    ActionTemplate {
        name,
        preconditions: vec![("At".to_string(), FactValue::Hex(from))],
        effects: vec![("At".to_string(), FactValue::Hex(to))],
        cost,
    }
}
