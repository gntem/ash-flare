//! Common types used throughout the supervision tree

use crate::restart::RestartPolicy;
use serde::{Deserialize, Serialize};

use bincode::{Decode, Encode};
use dashmap::DashMap;
use std::sync::Arc;

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

/// Shared context for stateful workers with in-memory key-value store.
///
/// Provides a process-local, concurrency-safe storage for workers to share state.
/// The store is backed by `DashMap` for lock-free concurrent access.
#[derive(Clone)]
pub struct WorkerContext {
    store: Arc<DashMap<String, serde_json::Value>>,
}

impl WorkerContext {
    /// Creates a new empty WorkerContext.
    pub fn new() -> Self {
        Self {
            store: Arc::new(DashMap::new()),
        }
    }

    /// Gets a value from the store by key.
    ///
    /// Returns `None` if the key doesn't exist.
    pub fn get(&self, key: &str) -> Option<serde_json::Value> {
        self.store.get(key).map(|entry| entry.value().clone())
    }

    /// Sets a value in the store.
    pub fn set(&self, key: impl Into<String>, value: serde_json::Value) {
        self.store.insert(key.into(), value);
    }

    /// Deletes a key from the store.
    ///
    /// Returns the previous value if it existed.
    pub fn delete(&self, key: &str) -> Option<serde_json::Value> {
        self.store.remove(key).map(|(_, v)| v)
    }

    /// Updates a value in the store using a function.
    ///
    /// If the key doesn't exist, the function is called with `None`.
    /// If the function returns `Some(value)`, the value is inserted/updated.
    /// If the function returns `None`, the key is removed (if it existed).
    pub fn update<F>(&self, key: &str, f: F)
    where
        F: FnOnce(Option<serde_json::Value>) -> Option<serde_json::Value>,
    {
        match self.store.entry(key.to_string()) {
            dashmap::mapref::entry::Entry::Occupied(mut entry) => {
                let old_value = entry.get().clone();
                match f(Some(old_value)) {
                    Some(new_value) => {
                        entry.insert(new_value);
                    }
                    None => {
                        entry.remove();
                    }
                }
            }
            dashmap::mapref::entry::Entry::Vacant(entry) => {
                if let Some(new_value) = f(None) {
                    entry.insert(new_value);
                }
            }
        }
    }
}

impl Default for WorkerContext {
    fn default() -> Self {
        Self::new()
    }
}
