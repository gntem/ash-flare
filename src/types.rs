//! Common types used throughout the supervision tree

use crate::restart::RestartPolicy;
use serde::{Deserialize, Serialize};

/// Result of a worker's execution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChildExitReason {
    /// Normal termination
    Normal,
    /// Abnormal termination (error or panic)
    Abnormal,
    /// Shutdown requested
    Shutdown,
}

/// Child identifier type
pub type ChildId = String;

/// Information about a child process
#[derive(Debug, Clone)]
pub struct ChildInfo {
    pub id: ChildId,
    pub child_type: ChildType,
    pub restart_policy: Option<RestartPolicy>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChildType {
    Worker,
    Supervisor,
}
