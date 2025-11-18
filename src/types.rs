//! Common types used throughout the supervision tree

use crate::restart::RestartPolicy;
use serde::{Deserialize, Serialize};

use bincode::{Decode, Encode};

/// Result of a worker's execution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
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
    /// Unique identifier for the child
    pub id: ChildId,
    /// Type of child (Worker or Supervisor)
    pub child_type: ChildType,
    /// Restart policy for the child (None for supervisors)
    pub restart_policy: Option<RestartPolicy>,
}

/// Type of child in supervision tree
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
pub enum ChildType {
    /// A worker process
    Worker,
    /// A nested supervisor
    Supervisor,
}
