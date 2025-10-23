use crate::action::ActionInstance;
use crate::world_state::WorldState;

/// Type alias for action lifecycle callbacks used by the executor.
pub type ActionCallback = Box<dyn Fn(&ActionInstance)>;

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
    /// Optional callback invoked when an action starts. Receives a reference to the ActionInstance.
    pub on_start: Option<ActionCallback>,
    /// Optional callback invoked when an action completes. Receives a reference to the ActionInstance.
    pub on_complete: Option<ActionCallback>,
}

impl Default for ActionExecutor {
    fn default() -> Self {
        Self::new()
    }
}

impl ActionExecutor {
    pub fn new() -> Self {
        Self {
            current: None,
            on_start: None,
            on_complete: None,
        }
    }

    /// Start executing the given action. If another action is running, it is aborted.
    pub fn start(&mut self, action: RuntimeAction) {
        // Abort existing and start new
        self.current = Some(action.clone());

        // If we can extract the ActionInstance, call the start callback
        match &self.current {
            Some(RuntimeAction::Instant(ai)) | Some(RuntimeAction::Timed { instance: ai, .. }) => {
                if let Some(cb) = &self.on_start {
                    cb(ai);
                }
            }
            None => {}
        }
    }

    /// Update executor by `dt` seconds; returns true if action completed this tick.
    pub fn update(&mut self, dt: f32, world: &mut WorldState) -> bool {
        if let Some(r) = &mut self.current {
            match r {
                RuntimeAction::Instant(ai) => {
                    // Apply effects and trigger completion callback
                    world.apply_effects(&ai.effects);
                    if let Some(cb) = &self.on_complete {
                        cb(ai);
                    }
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
                        if let Some(cb) = &self.on_complete {
                            cb(instance);
                        }
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
        // Do not call on_complete for abort; simply drop current
        self.current = None;
    }

    /// Set the on_start callback. The callback is invoked synchronously from `start`.
    pub fn set_on_start<F>(&mut self, f: F)
    where
        F: Fn(&ActionInstance) + 'static,
    {
        self.on_start = Some(Box::new(f));
    }

    /// Set the on_complete callback. The callback is invoked synchronously from `update` when an action finishes.
    pub fn set_on_complete<F>(&mut self, f: F)
    where
        F: Fn(&ActionInstance) + 'static,
    {
        self.on_complete = Some(Box::new(f));
    }
}
