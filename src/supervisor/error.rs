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
                write!(f, "supervisor '{}' has no children", name)
            }
            SupervisorError::AllChildrenFailed(name) => {
                write!(
                    f,
                    "all children failed for supervisor '{}' - restart intensity limit exceeded",
                    name
                )
            }
            SupervisorError::ShuttingDown(name) => {
                write!(
                    f,
                    "supervisor '{}' is shutting down - operation not permitted",
                    name
                )
            }
            SupervisorError::ChildAlreadyExists(id) => {
                write!(
                    f,
                    "child with id '{}' already exists - use a unique identifier",
                    id
                )
            }
            SupervisorError::ChildNotFound(id) => {
                write!(
                    f,
                    "child with id '{}' not found - it may have already terminated",
                    id
                )
            }
        }
    }
}

impl std::error::Error for SupervisorError {}
