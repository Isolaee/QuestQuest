pub enum StructureType {
    // Fortifications
    StoneWall,      // High defense, blocks movement unless destroyed
    WoodenWall,     // Medium defense, flammable
    Watchtower,     // Grants vision & range bonus
    Gate,           // Allows friendly passage, blocks enemies
    
    // Buildings
    House,          // Healing/rest bonus
    Barracks,       // Recruitment point, defense bonus
    Arsenal,        // Attack bonus for occupying units
    
    // Defensive
    Barricade,      // Quick to build, medium defense
    Trench,         // Defense bonus, harder to attack into
    Spikes,         // Damages melee attackers
}