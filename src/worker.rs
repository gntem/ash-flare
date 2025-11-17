//! Worker trait and related types

use crate::restart::RestartPolicy;
use crate::types::{ChildExitReason, ChildId};
use async_trait::async_trait;
use std::fmt;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

/// A trait that all workers must implement to work with the supervisor tree.
/// This allows for generic workers that can handle any type of work.
#[async_trait]
pub trait Worker: Send + Sync + 'static {
    /// The type of error this worker can return
    type Error: std::error::Error + Send + Sync + 'static;

    /// Run the worker's main loop - this should run until completion or error
    async fn run(&mut self) -> Result<(), Self::Error>;

    /// Called when the worker is initialized
    async fn initialize(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Called when the worker is being shut down
    async fn shutdown(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

/// Specification for creating and restarting a worker
pub(crate) struct WorkerSpec<W: Worker> {
    pub id: ChildId,
    pub worker_factory: Arc<dyn Fn() -> W + Send + Sync>,
    pub restart_policy: RestartPolicy,
}

impl<W: Worker> Clone for WorkerSpec<W> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            worker_factory: Arc::clone(&self.worker_factory),
            restart_policy: self.restart_policy,
        }
    }
}

impl<W: Worker> WorkerSpec<W> {
    pub(crate) fn new(
        id: impl Into<String>,
        factory: impl Fn() -> W + Send + Sync + 'static,
        restart_policy: RestartPolicy,
    ) -> Self {
        Self {
            id: id.into(),
            worker_factory: Arc::new(factory),
            restart_policy,
        }
    }

    pub(crate) fn create_worker(&self) -> W {
        (self.worker_factory)()
    }
}

impl<W: Worker> fmt::Debug for WorkerSpec<W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WorkerSpec")
            .field("id", &self.id)
            .field("restart_policy", &self.restart_policy)
            .finish()
    }
}

/// Running worker process with its specification and task handle
pub(crate) struct WorkerProcess<W: Worker> {
    pub spec: WorkerSpec<W>,
    pub handle: Option<JoinHandle<()>>,
}

impl<W: Worker> WorkerProcess<W> {
    pub(crate) fn spawn<Cmd>(
        spec: WorkerSpec<W>,
        supervisor_name: String,
        control_tx: mpsc::UnboundedSender<Cmd>,
    ) -> Self
    where
        Cmd: From<WorkerTermination> + Send + 'static,
    {
        let worker = spec.create_worker();
        let worker_id = spec.id.clone();
        let handle = tokio::spawn(async move {
            run_worker(supervisor_name, worker_id, worker, control_tx).await;
        });

        Self {
            spec,
            handle: Some(handle),
        }
    }

    pub(crate) async fn stop(&mut self) {
        if let Some(handle) = self.handle.take() {
            handle.abort();
            let _ = handle.await;
        }
    }
}

impl<W: Worker> Drop for WorkerProcess<W> {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.take() {
            handle.abort();
        }
    }
}

/// Message sent when a worker terminates
pub(crate) struct WorkerTermination {
    pub id: ChildId,
    pub reason: ChildExitReason,
}

async fn run_worker<W: Worker, Cmd>(
    supervisor_name: String,
    worker_id: ChildId,
    mut worker: W,
    control_tx: mpsc::UnboundedSender<Cmd>,
) where
    Cmd: From<WorkerTermination>,
{
    let qualified_name = format!("{}/{}", supervisor_name, worker_id);

    // Initialize the worker
    if let Err(err) = worker.initialize().await {
        slog::error!(slog_scope::logger(), "worker initialization failed";
            "worker" => &qualified_name,
            "error" => %err
        );
        let _ = control_tx.send(
            WorkerTermination {
                id: worker_id,
                reason: ChildExitReason::Abnormal,
            }
            .into(),
        );
        return;
    }

    slog::debug!(slog_scope::logger(), "worker started"; "worker" => &qualified_name);

    // Run the worker's main loop
    let exit_reason = match worker.run().await {
        Ok(()) => {
            slog::debug!(slog_scope::logger(), "worker completed normally"; "worker" => &qualified_name);
            ChildExitReason::Normal
        }
        Err(err) => {
            slog::warn!(slog_scope::logger(), "worker failed";
                "worker" => &qualified_name,
                "error" => %err
            );
            ChildExitReason::Abnormal
        }
    };

    // Shutdown the worker
    if let Err(err) = worker.shutdown().await {
        slog::error!(slog_scope::logger(), "worker shutdown failed";
            "worker" => &qualified_name,
            "error" => %err
        );
    }

    slog::debug!(slog_scope::logger(), "worker stopped"; "worker" => &qualified_name);

    // Notify supervisor of termination
    let _ = control_tx.send(
        WorkerTermination {
            id: worker_id,
            reason: exit_reason,
        }
        .into(),
    );
}

/// Errors returned by worker operations.
#[derive(Debug)]
pub enum WorkerError {
    /// Command channel was closed unexpectedly
    CommandChannelClosed(String),
    /// Worker panicked during execution
    WorkerPanicked(String),
    /// Worker failed with an error
    WorkerFailed(String),
}

impl fmt::Display for WorkerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WorkerError::CommandChannelClosed(name) => {
                write!(f, "command channel to {} is closed", name)
            }
            WorkerError::WorkerPanicked(name) => {
                write!(f, "worker {} panicked", name)
            }
            WorkerError::WorkerFailed(msg) => {
                write!(f, "worker failed: {}", msg)
            }
        }
    }
}

impl std::error::Error for WorkerError {}
