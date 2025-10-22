//! # Combat Result Module
//!
//! Defines the outcome data structure returned after combat resolution.

/// Result of a combat encounter between two units.
///
/// Contains comprehensive information about what happened during combat,
/// including damage dealt, hit success, and casualties. This structure
/// is returned by `resolve_combat()` and can be used for logging,
/// UI updates, and game state changes.
///
/// # Examples
///
/// ```ignore
/// use combat::{resolve_combat, CombatStats, DamageType, RangeCategory, Resistances};
///
/// let mut attacker = CombatStats::new(100, 20, 5, RangeCategory::Melee, Resistances::default());
/// let mut defender = CombatStats::new(80, 15, 4, RangeCategory::Melee, Resistances::default());
///
/// let result = resolve_combat(&mut attacker, &mut defender, DamageType::Slash);
///
/// println!("Attacker dealt {} damage", result.attacker_damage_dealt);
/// if result.defender_casualties > 0 {
///     println!("Defender was defeated!");
/// }
/// ```
#[derive(Debug, Clone)]
pub struct CombatResult {
    /// Total damage dealt by the attacker
    pub attacker_damage_dealt: u32,
    /// Total damage dealt by the defender
    pub defender_damage_dealt: u32,
    /// Whether the attacker successfully hit at least once
    pub attacker_hit: bool,
    /// Whether the defender successfully hit at least once
    pub defender_hit: bool,
    /// Number of attacker casualties (0 or 1)
    pub attacker_casualties: u32,
    /// Number of defender casualties (0 or 1)
    pub defender_casualties: u32,
}
