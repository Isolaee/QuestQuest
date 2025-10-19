/// Result of a combat encounter
#[derive(Debug, Clone)]
pub struct CombatResult {
    pub attacker_damage_dealt: u32,
    pub defender_damage_dealt: u32,
    pub attacker_hit: bool,
    pub defender_hit: bool,
    pub attacker_casualties: u32,
    pub defender_casualties: u32,
}
