//! A generic supervisor tree that coordinates workers of any type.
//! This library demonstrates how a supervisor can restart failed workers
//! while handling arbitrary workloads through a trait-based system.
//!
//! Inspired by Erlang/OTP supervision principles.

mod restart;
mod supervisor;
mod types;
mod worker;

pub mod distributed;

// Re-export public API
pub use restart::{RestartIntensity, RestartPolicy, RestartStrategy};
pub use supervisor::{SupervisorError, SupervisorHandle, SupervisorSpec};
pub use types::{ChildExitReason, ChildId, ChildInfo, ChildType};
pub use worker::{Worker, WorkerError};



