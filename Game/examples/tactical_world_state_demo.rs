/// Demo showcasing the enhanced tactical world state extraction.
///
/// This example demonstrates how extract_detailed_world_state() provides
/// comprehensive tactical information including health states, movement points,
/// attack ranges, terrain data, clustering metrics, and threat levels.
use game::{GameObject, GameUnit, GameWorld, Team};
use graphics::HexCoord;
use units::{Terrain, UnitFactory};

fn main() {
    println!("=== Tactical World State Demo ===\n");

    // Create a small world
    let mut world = GameWorld::new(10);
    world.generate_terrain();
    world.turn_system.start_game();

    // Create player units at different positions
    let player1 = UnitFactory::create_human_warrior(
        "PlayerWarrior".to_string(),
        HexCoord::new(0, 0),
        Terrain::Grasslands,
    );
    let mut gp1 = GameUnit::new(player1);
    gp1.set_team(Team::Player);
    let p1_id = world.add_unit(gp1);

    let player2 = UnitFactory::create_elf_archer(
        "PlayerArcher".to_string(),
        HexCoord::new(1, 0),
        Terrain::Grasslands,
    );
    let mut gp2 = GameUnit::new(player2);
    gp2.set_team(Team::Player);
    let _p2_id = world.add_unit(gp2);

    // Create enemy units
    let enemy1 = UnitFactory::create_goblin_grunt(
        "GoblinGrunt".to_string(),
        HexCoord::new(3, 0),
        Terrain::Grasslands,
    );
    let mut ge1 = GameUnit::new_with_team(enemy1, Team::Enemy);
    // Damage the goblin to demonstrate wounded state
    ge1.unit_mut().take_damage(20);
    let e1_id = world.add_unit(ge1);

    let enemy2 = UnitFactory::create_goblin_grunt(
        "GoblinArcher".to_string(),
        HexCoord::new(4, 1),
        Terrain::Grasslands,
    );
    let ge2 = GameUnit::new_with_team(enemy2, Team::Enemy);
    let _e2_id = world.add_unit(ge2);

    // Extract basic world state
    println!("--- Basic World State ---");
    let basic_ws = world.extract_world_state_for_team(Team::Player);
    println!("Basic state contains {} facts", basic_ws.facts.len());
    for (key, value) in basic_ws.facts.iter().take(5) {
        println!("  {}: {:?}", key, value);
    }
    println!("  ... (truncated)\n");

    // Extract detailed world state
    println!("--- Enhanced Tactical World State ---");
    let detailed_ws = world.extract_detailed_world_state(Team::Player);
    println!(
        "Detailed state contains {} facts ({}x more information!)",
        detailed_ws.facts.len(),
        detailed_ws.facts.len() / basic_ws.facts.len().max(1)
    );

    // Display player unit information
    println!("\n=== Player Team Analysis ===");
    if let Some(stats) = detailed_ws.get(&format!("Team:AllyCount")) {
        println!("Allied Units: {:?}", stats);
    }
    if let Some(stats) = detailed_ws.get(&format!("Team:AverageHealth")) {
        println!("Average Health: {:?}", stats);
    }

    // Display warrior details
    println!("\n--- PlayerWarrior Details ---");
    let p1_str = p1_id.to_string();
    if let Some(health) = detailed_ws.get(&format!("Unit:{}:Health", p1_str)) {
        println!("  Health: {:?}", health);
    }
    if let Some(max_health) = detailed_ws.get(&format!("Unit:{}:MaxHealth", p1_str)) {
        println!("  Max Health: {:?}", max_health);
    }
    if let Some(health_pct) = detailed_ws.get(&format!("Unit:{}:HealthPercent", p1_str)) {
        println!("  Health %: {:?}", health_pct);
    }
    if let Some(moves) = detailed_ws.get(&format!("Unit:{}:MovesLeft", p1_str)) {
        println!("  Moves Left: {:?}", moves);
    }
    if let Some(attack_range) = detailed_ws.get(&format!("Unit:{}:AttackRange", p1_str)) {
        println!("  Attack Range: {:?}", attack_range);
    }
    if let Some(defense) = detailed_ws.get(&format!("Unit:{}:Defense", p1_str)) {
        println!("  Defense: {:?}", defense);
    }
    if let Some(allies) = detailed_ws.get(&format!("Unit:{}:NearbyAllies", p1_str)) {
        println!("  Nearby Allies: {:?}", allies);
    }
    if let Some(isolated) = detailed_ws.get(&format!("Unit:{}:IsIsolated", p1_str)) {
        println!("  Is Isolated: {:?}", isolated);
    }
    if let Some(nearest) = detailed_ws.get(&format!("Unit:{}:NearestEnemyDist", p1_str)) {
        println!("  Nearest Enemy Distance: {:?}", nearest);
    }
    if let Some(in_range) = detailed_ws.get(&format!("Unit:{}:EnemiesInRange", p1_str)) {
        println!("  Enemies in Attack Range: {:?}", in_range);
    }

    // Display enemy team information
    println!("\n=== Enemy Team Analysis ===");
    if let Some(stats) = detailed_ws.get(&format!("Team:EnemyCount")) {
        println!("Enemy Units: {:?}", stats);
    }

    // Display wounded goblin details
    println!("\n--- GoblinGrunt Details (Wounded) ---");
    let e1_str = e1_id.to_string();
    if let Some(health) = detailed_ws.get(&format!("Unit:{}:Health", e1_str)) {
        println!("  Health: {:?}", health);
    }
    if let Some(max_health) = detailed_ws.get(&format!("Unit:{}:MaxHealth", e1_str)) {
        println!("  Max Health: {:?}", max_health);
    }
    if let Some(health_pct) = detailed_ws.get(&format!("Unit:{}:HealthPercent", e1_str)) {
        println!("  Health %: {:?}", health_pct);
    }
    if let Some(wounded) = detailed_ws.get(&format!("Unit:{}:IsWounded", e1_str)) {
        println!("  Is Wounded: {:?}", wounded);
    }
    if let Some(threat) = detailed_ws.get(&format!("Unit:{}:ThreatLevel", e1_str)) {
        println!("  Threat Level: {:?}", threat);
    }
    if let Some(is_friendly) = detailed_ws.get(&format!("Unit:{}:IsFriendly", e1_str)) {
        println!("  Is Friendly (to Player): {:?}", is_friendly);
    }

    // Show terrain information
    println!("\n=== Terrain Information ===");
    let pos_key = "0,0".to_string();
    if let Some(terrain_type) = detailed_ws.get(&format!("Terrain:{}:Type", pos_key)) {
        println!("Terrain at (0,0):");
        println!("  Type: {:?}", terrain_type);
    }
    if let Some(move_cost) = detailed_ws.get(&format!("Terrain:{}:MoveCost", pos_key)) {
        println!("  Movement Cost: {:?}", move_cost);
    }

    // Strategic insights
    println!("\n=== Strategic Insights ===");

    // Find wounded enemies
    let wounded_enemies: Vec<String> = world
        .units
        .iter()
        .filter(|(_, u)| u.team() == Team::Enemy)
        .filter(|(id, _)| {
            if let Some(ai::FactValue::Bool(true)) =
                detailed_ws.get(&format!("Unit:{}:IsWounded", id))
            {
                true
            } else {
                false
            }
        })
        .map(|(id, u)| format!("{} ({})", u.name(), id))
        .collect();

    if !wounded_enemies.is_empty() {
        println!("Wounded Enemies (priority targets):");
        for enemy in wounded_enemies {
            println!("  - {}", enemy);
        }
    }

    // Find isolated units
    let isolated_units: Vec<String> = world
        .units
        .iter()
        .filter(|(_, u)| u.team() == Team::Player)
        .filter(|(id, _)| {
            if let Some(ai::FactValue::Bool(true)) =
                detailed_ws.get(&format!("Unit:{}:IsIsolated", id))
            {
                true
            } else {
                false
            }
        })
        .map(|(id, u)| format!("{} ({})", u.name(), id))
        .collect();

    if !isolated_units.is_empty() {
        println!("\nIsolated Allies (need support):");
        for unit in isolated_units {
            println!("  - {}", unit);
        }
    } else {
        println!("\n✓ All allied units have nearby support");
    }

    println!("\n=== Demo Complete ===");
    println!("The enhanced world state provides:");
    println!("  ✓ Health and combat status");
    println!("  ✓ Movement capabilities");
    println!("  ✓ Attack ranges and threat levels");
    println!("  ✓ Terrain information");
    println!("  ✓ Team clustering metrics");
    println!("  ✓ Distance calculations");
    println!("\nThis enables sophisticated AI behaviors like:");
    println!("  • Targeting wounded enemies");
    println!("  • Retreating when low on health");
    println!("  • Maintaining formation");
    println!("  • Using terrain advantages");
    println!("  • Coordinating attacks");
}
