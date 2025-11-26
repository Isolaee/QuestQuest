//! # Unit Macros Module
//!
//! This module previously contained macro implementations for the Unit trait.
//!
//! ## Architectural Change
//!
//! The Unit trait has been refactored to use default implementations for all shared functionality.
//! Units now only need to implement three core methods:
//!
//! - ase() - Returns a reference to the BaseUnit
//! - ase_mut() - Returns a mutable reference to the BaseUnit  
//! - ttacks() - Returns the unit's attack list
//!
//! All other trait methods have default implementations that delegate to BaseUnit.
//!
//! ## Unit Registration
//!
//! For unit registration with the factory, see the submit_unit! macro in
//! [unit_registry](crate::unit_registry) module.
//!
//! ## Migration Guide
//!
//! Old pattern (with macros):
//! `ignore
//! impl MyUnit { /* ... */ }
//! crate::impl_unit_delegate!(MyUnit);
//! `
//!
//! New pattern (trait implementation):
//! `ignore
//! impl crate::unit_trait::Unit for MyUnit {
//!     fn base(&self) -> &BaseUnit { &self.base }
//!     fn base_mut(&mut self) -> &mut BaseUnit { &mut self.base }
//!     fn attacks(&self) -> &[Attack] { &self.attacks }
//!     
//!     // Override only if needed:
//!     // fn evolution_previous(&self) -> Option<String> { ... }
//!     // fn evolution_next(&self) -> Option<String> { ... }
//! }
//! `

// This file intentionally left minimal - macros have been replaced with trait default implementations
