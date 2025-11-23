// Minimal example to verify stateful supervisor compiles and works
use ash_flare::{
    RestartPolicy, StatefulSupervisorHandle, StatefulSupervisorSpec, Worker, WorkerContext,
};
use async_trait::async_trait;
use std::sync::Arc;

struct SimpleWorker {
    id: u32,
    ctx: Arc<WorkerContext>,
}

#[async_trait]
impl Worker for SimpleWorker {
    type Error = std::io::Error;

    async fn run(&mut self) -> Result<(), Self::Error> {
        let key = format!("worker-{}", self.id);
        self.ctx.set(key, serde_json::json!(self.id));
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create stateful supervisor with shared context
    let spec = StatefulSupervisorSpec::new("auction-supervisor")
        .with_worker(
            "auction-worker-1",
            move |ctx: Arc<WorkerContext>| SimpleWorker { id: 1, ctx },
            RestartPolicy::Temporary,
        )
        .with_worker(
            "auction-worker-2",
            move |ctx: Arc<WorkerContext>| SimpleWorker { id: 2, ctx },
            RestartPolicy::Temporary,
        );

    // Get reference to shared store
    let store = Arc::clone(spec.context());

    // Start the supervisor
    let handle = StatefulSupervisorHandle::start(spec);

    // Give workers time to run
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Access shared store
    println!("Worker 1 value: {:?}", store.get("worker-1"));
    println!("Worker 2 value: {:?}", store.get("worker-2"));

    // Demonstrate KV operations
    store.set("bid-count", serde_json::json!(0));
    store.update("bid-count", |v| {
        let count = v.and_then(|v| v.as_u64()).unwrap_or(0);
        Some(serde_json::json!(count + 1))
    });
    println!("Bid count: {:?}", store.get("bid-count"));

    handle.shutdown().await?;
    Ok(())
}
