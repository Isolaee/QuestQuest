use crate::action::ActionInstance;
use crate::world_state::WorldState;

/// Runtime representation of an action that can be executed by an agent.
#[derive(Clone, Debug)]
pub enum RuntimeAction {
    Instant(ActionInstance),
    Timed {
        instance: ActionInstance,
        duration: f32,
        elapsed: f32,
    },
}

/// Executor that runs a sequence of grounded action instances against a mutable WorldState.
/// It supports start/update/abort lifecycle semantics. The executor applies effects when
/// an action completes (or immediately for Instant).
pub struct ActionExecutor {
    pub current: Option<RuntimeAction>,
}

impl Default for ActionExecutor {
    fn default() -> Self {
        Self::new()
    }
}

impl ActionExecutor {
    pub fn new() -> Self {
        Self { current: None }
    }

    /// Start executing the given action. If another action is running, it is aborted.
    pub fn start(&mut self, action: RuntimeAction) {
        self.current = Some(action);
    }

    /// Update executor by `dt` seconds; returns true if action completed this tick.
    pub fn update(&mut self, dt: f32, world: &mut WorldState) -> bool {
        if let Some(r) = &mut self.current {
            match r {
                RuntimeAction::Instant(ai) => {
                    world.apply_effects(&ai.effects);
                    self.current = None;
                    return true;
                }
                RuntimeAction::Timed {
                    instance,
                    duration,
                    elapsed,
                } => {
                    *elapsed += dt;
                    if *elapsed >= *duration {
                        world.apply_effects(&instance.effects);
                        self.current = None;
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Abort current action without applying its effects.
    pub fn abort(&mut self) {
        self.current = None;
    }
}
