//! `RD Engine` is a tiny reducer-governed deterministic state core.
//!
//! The goal is legibility:
//! copy the pieces you want, rename the nouns, and adapt the reducer to your
//! own project without inheriting a large framework.

pub mod engine;
pub mod save;

pub use engine::{
    BucketMap, EngineAction, EngineEvent, EngineOutcome, EngineState, Reducer, ReducerExt,
    TemplateMap, apply_action,
};
pub use save::{EngineSaveEnvelope, load_state_from_path, save_state_to_path};
