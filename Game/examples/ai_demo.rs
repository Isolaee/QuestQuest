use game::GameWorld;
use game::Team;
use graphics::HexCoord;
use units::unit_race::Terrain as UnitTerrain;
use units::units::human::warrior::HumanWarrior;

fn main() {
    // Create tiny world
    let mut world = GameWorld::new(3);
    world.turn_system.start_game();

    // Add a player unit at (0,0)
    let player = HumanWarrior::new(
        "PlayerHero".to_string(),
        HexCoord::new(0, 0),
        UnitTerrain::Grasslands,
    );
    let mut player_unit = game::GameUnit::new(Box::new(player));
    player_unit.set_team(Team::Player);
    let player_id = world.add_unit(player_unit);

    // Add an enemy unit at (1,0)
    let enemy = HumanWarrior::new(
        "EnemyGrunt".to_string(),
        HexCoord::new(1, 0),
        UnitTerrain::Grasslands,
    );
    let enemy_unit = game::GameUnit::new_with_team(Box::new(enemy), Team::Enemy);
    let enemy_id = world.add_unit(enemy_unit);

    println!(
        "World created. Player: {:?}, Enemy: {:?}",
        player_id, enemy_id
    );

    // Ensure it's Enemy turn so AI will run
    world.turn_system.set_team_control(Team::Enemy, false);
    // Force turn to Enemy
    world.turn_system.end_turn(); // advance from Player to Enemy

    // Run AI for current team (Enemy)
    println!(
        "Running AI for current team: {:?}",
        world.turn_system.current_team()
    );
    world.run_ai_for_current_team();

    println!(
        "AI run complete. Pending combat: {:?}",
        world.pending_combat.is_some()
    );
}
