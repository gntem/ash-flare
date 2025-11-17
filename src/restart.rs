//! Restart policies and strategies for supervision

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// Restart strategy for supervisor children
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RestartStrategy {
    /// Restart only the failed child (`:one_for_one`)
    OneForOne,
    /// Restart all children if any child fails (`:one_for_all`)
    OneForAll,
    /// Restart failed child and all children started after it (`:rest_for_one`)
    RestForOne,
}

impl Default for RestartStrategy {
    fn default() -> Self {
        Self::OneForOne
    }
}

/// When to restart a child
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RestartPolicy {
    /// Always restart when child terminates (`:permanent`)
    Permanent,
    /// Never restart (`:temporary`)
    Temporary,
    /// Restart only if abnormal termination (`:transient`)
    Transient,
}

impl Default for RestartPolicy {
    fn default() -> Self {
        Self::Permanent
    }
}

/// Restart intensity limits with max restarts within a time window
#[derive(Debug, Clone, Copy)]
pub struct RestartIntensity {
    /// Maximum number of restarts allowed
    pub max_restarts: usize,
    /// Within this time period (in seconds)
    pub within_seconds: u64,
}

impl Default for RestartIntensity {
    fn default() -> Self {
        Self {
            max_restarts: 3,
            within_seconds: 5,
        }
    }
}

/// Tracks restart history for intensity monitoring using a sliding time window
#[derive(Debug)]
pub(crate) struct RestartTracker {
    intensity: RestartIntensity,
    restart_times: VecDeque<Instant>,
}

impl RestartTracker {
    pub(crate) fn new(intensity: RestartIntensity) -> Self {
        Self {
            intensity,
            restart_times: VecDeque::new(),
        }
    }

    /// Records a restart and returns true if intensity limit exceeded
    pub(crate) fn record_restart(&mut self) -> bool {
        let now = Instant::now();
        let cutoff = now - Duration::from_secs(self.intensity.within_seconds);

        // Remove old restarts outside the time window
        while let Some(&time) = self.restart_times.front() {
            if time < cutoff {
                self.restart_times.pop_front();
            } else {
                break;
            }
        }

        self.restart_times.push_back(now);

        // Check if we've exceeded the limit
        self.restart_times.len() > self.intensity.max_restarts
    }

    #[allow(dead_code)]
    pub(crate) fn reset(&mut self) {
        self.restart_times.clear();
    }
}
