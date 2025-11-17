//! Supervisor errors

use std::fmt;

/// Errors returned by supervisor operations.
#[derive(Debug)]
pub enum SupervisorError {
    /// Supervisor has no children
    NoChildren(String),
    /// All children have failed
    AllChildrenFailed(String),
    /// Supervisor is shutting down
    ShuttingDown(String),
    /// Child with this ID already exists
    ChildAlreadyExists(String),
    /// Child with this ID not found
    ChildNotFound(String),
}

impl fmt::Display for SupervisorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SupervisorError::NoChildren(name) => {
                write!(f, "supervisor {} has no children", name)
            }
            SupervisorError::AllChildrenFailed(name) => {
                write!(f, "all children failed for supervisor {}", name)
            }
            SupervisorError::ShuttingDown(name) => {
                write!(f, "supervisor {} is shutting down", name)
            }
            SupervisorError::ChildAlreadyExists(id) => {
                write!(f, "child {} already exists", id)
            }
            SupervisorError::ChildNotFound(id) => {
                write!(f, "child {} not found", id)
            }
        }
    }
}

impl std::error::Error for SupervisorError {}
