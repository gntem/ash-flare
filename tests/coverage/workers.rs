use ash_flare::{Worker, WorkerContext};
use async_trait::async_trait;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[derive(Debug)]
pub struct QuickWorker;

#[async_trait]
impl Worker for QuickWorker {
    type Error = std::io::Error;

    async fn run(&mut self) -> Result<(), Self::Error> {
        sleep(Duration::from_millis(5)).await;
        Ok(())
    }
}

#[derive(Debug)]
pub struct FailingWorker {
    pub fail_count: u32,
}

#[async_trait]
impl Worker for FailingWorker {
    type Error = std::io::Error;

    async fn run(&mut self) -> Result<(), Self::Error> {
        self.fail_count += 1;
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "intentional failure",
        ))
    }
}

#[derive(Debug)]
pub struct StatefulQuickWorker {
    pub id: String,
    pub context: Arc<WorkerContext>,
}

#[async_trait]
impl Worker for StatefulQuickWorker {
    type Error = std::io::Error;

    async fn run(&mut self) -> Result<(), Self::Error> {
        self.context.set(&self.id, serde_json::json!("running"));
        sleep(Duration::from_millis(5)).await;
        Ok(())
    }
}

#[derive(Debug)]
pub struct StatefulFailingWorker {
    pub id: String,
    pub context: Arc<WorkerContext>,
}

#[async_trait]
impl Worker for StatefulFailingWorker {
    type Error = std::io::Error;

    async fn run(&mut self) -> Result<(), Self::Error> {
        self.context.set(&self.id, serde_json::json!("failed"));
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "intentional failure",
        ))
    }
}
