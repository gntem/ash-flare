//! Supervisor handle - public API for interacting with supervisors

use super::error::SupervisorError;
use super::runtime::{SupervisorCommand, SupervisorRuntime};
use super::spec::SupervisorSpec;
use crate::restart::RestartPolicy;
use crate::types::{ChildId, ChildInfo};
use crate::worker::{Worker, WorkerSpec};
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot};

/// Handle used to interact with a running supervisor tree.
#[derive(Clone)]
pub struct SupervisorHandle<W: Worker> {
    pub(crate) name: Arc<String>,
    pub(crate) control_tx: mpsc::UnboundedSender<SupervisorCommand<W>>,
}

impl<W: Worker> SupervisorHandle<W> {
    /// Spawns a supervisor tree based on the provided specification.
    pub fn start(spec: SupervisorSpec<W>) -> Self {
        let (control_tx, control_rx) = mpsc::unbounded_channel();
        let name_arc = Arc::new(spec.name.clone());
        let runtime = SupervisorRuntime::new(spec, control_rx, control_tx.clone());

        let runtime_name = Arc::clone(&name_arc);
        tokio::spawn(async move {
            runtime.run().await;
            println!("[{}] supervisor stopped", runtime_name);
        });

        Self {
            name: name_arc,
            control_tx,
        }
    }

    /// Dynamically starts a new child worker
    pub async fn start_child(
        &self,
        id: impl Into<String>,
        factory: impl Fn() -> W + Send + Sync + 'static,
        restart_policy: RestartPolicy,
    ) -> Result<ChildId, SupervisorError> {
        let (result_tx, result_rx) = oneshot::channel();
        let spec = WorkerSpec::new(id, factory, restart_policy);

        self.control_tx
            .send(SupervisorCommand::StartChild {
                spec,
                respond_to: result_tx,
            })
            .map_err(|_| SupervisorError::ShuttingDown(self.name().to_string()))?;

        result_rx
            .await
            .map_err(|_| SupervisorError::ShuttingDown(self.name().to_string()))?
    }

    /// Dynamically terminates a child
    pub async fn terminate_child(&self, id: &str) -> Result<(), SupervisorError> {
        let (result_tx, result_rx) = oneshot::channel();

        self.control_tx
            .send(SupervisorCommand::TerminateChild {
                id: id.to_string(),
                respond_to: result_tx,
            })
            .map_err(|_| SupervisorError::ShuttingDown(self.name().to_string()))?;

        result_rx
            .await
            .map_err(|_| SupervisorError::ShuttingDown(self.name().to_string()))?
    }

    /// Returns information about all children
    pub async fn which_children(&self) -> Result<Vec<ChildInfo>, SupervisorError> {
        let (result_tx, result_rx) = oneshot::channel();

        self.control_tx
            .send(SupervisorCommand::WhichChildren {
                respond_to: result_tx,
            })
            .map_err(|_| SupervisorError::ShuttingDown(self.name().to_string()))?;

        result_rx
            .await
            .map_err(|_| SupervisorError::ShuttingDown(self.name().to_string()))?
    }

    /// Requests a graceful shutdown of the supervisor tree.
    pub async fn shutdown(&self) -> Result<(), SupervisorError> {
        self.control_tx
            .send(SupervisorCommand::Shutdown)
            .map_err(|_| SupervisorError::ShuttingDown(self.name().to_string()))?;
        Ok(())
    }

    /// Returns the supervisor's name.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}
