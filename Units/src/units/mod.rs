// Player races
pub mod dragonborn;
pub mod dwarf;
pub mod elf;
pub mod gnome;
pub mod half_elf;
pub mod half_orc;
pub mod halfling;
pub mod human;
pub mod tiefling;

// Monster races
pub mod goblin;
pub mod hobgoblin;
pub mod kobold;
pub mod lizardfolk;
pub mod orc;
pub mod triton;

// Shapeshifters and special races
pub mod changeling;

// Undead races
pub mod skeleton;
pub mod undead;
pub mod zombie;

// Re-export all concrete unit types
pub use dwarf::{DwarfVeteranWarrior, DwarfWarrior, DwarfYoungWarrior};
pub use elf::{ElfArcher, ElfMage, ElfWarrior};
pub use goblin::{GoblinChief, GoblinGrunt};
// Human units temporarily disabled - being refactored
// pub use human::HumanNoble;
pub use orc::{OrcEliteSwordsman, OrcSwordsman, OrcYoungSwordsman};
// pub use orc::OrcGrunt; // TODO: Implement OrcGrunt
