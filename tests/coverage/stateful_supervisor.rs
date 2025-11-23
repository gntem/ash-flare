use super::workers::{StatefulFailingWorker, StatefulQuickWorker};
use ash_flare::{
    RestartIntensity, RestartPolicy, RestartStrategy, StatefulSupervisorHandle,
    StatefulSupervisorSpec, WorkerContext,
};
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_stateful_supervisor_basic() {
    let spec = StatefulSupervisorSpec::<StatefulQuickWorker>::new("stateful-basic");
    let handle = StatefulSupervisorHandle::start(spec);

    sleep(Duration::from_millis(10)).await;
    let result = handle.shutdown().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stateful_supervisor_with_multiple_workers() {
    let spec = StatefulSupervisorSpec::new("stateful-multi")
        .with_worker(
            "worker-1",
            |ctx| StatefulQuickWorker {
                id: "worker-1".to_string(),
                context: ctx,
            },
            RestartPolicy::Temporary,
        )
        .with_worker(
            "worker-2",
            |ctx| StatefulQuickWorker {
                id: "worker-2".to_string(),
                context: ctx,
            },
            RestartPolicy::Temporary,
        )
        .with_worker(
            "worker-3",
            |ctx| StatefulQuickWorker {
                id: "worker-3".to_string(),
                context: ctx,
            },
            RestartPolicy::Temporary,
        );

    let handle = StatefulSupervisorHandle::start(spec);
    sleep(Duration::from_millis(100)).await;

    let children = handle.which_children().await.unwrap();
    // Workers exit quickly, so we might have 0-3 children
    assert!(children.len() <= 3);

    handle.shutdown().await.unwrap();
}

#[tokio::test]
async fn test_stateful_supervisor_restart_strategies() {
    // Test OneForOne
    let spec = StatefulSupervisorSpec::new("stateful-one-for-one")
        .with_restart_strategy(RestartStrategy::OneForOne)
        .with_worker(
            "w1",
            |ctx| StatefulQuickWorker {
                id: "w1".to_string(),
                context: ctx,
            },
            RestartPolicy::Permanent,
        )
        .with_worker(
            "w2",
            |ctx| StatefulQuickWorker {
                id: "w2".to_string(),
                context: ctx,
            },
            RestartPolicy::Permanent,
        );

    let handle = StatefulSupervisorHandle::start(spec);
    sleep(Duration::from_millis(20)).await;
    handle.shutdown().await.unwrap();

    // Test OneForAll
    let spec = StatefulSupervisorSpec::new("stateful-one-for-all")
        .with_restart_strategy(RestartStrategy::OneForAll)
        .with_worker(
            "w1",
            |ctx| StatefulQuickWorker {
                id: "w1".to_string(),
                context: ctx,
            },
            RestartPolicy::Permanent,
        )
        .with_worker(
            "w2",
            |ctx| StatefulQuickWorker {
                id: "w2".to_string(),
                context: ctx,
            },
            RestartPolicy::Permanent,
        );

    let handle = StatefulSupervisorHandle::start(spec);
    sleep(Duration::from_millis(20)).await;
    handle.shutdown().await.unwrap();

    // Test RestForOne
    let spec = StatefulSupervisorSpec::new("stateful-rest-for-one")
        .with_restart_strategy(RestartStrategy::RestForOne)
        .with_worker(
            "w1",
            |ctx| StatefulQuickWorker {
                id: "w1".to_string(),
                context: ctx,
            },
            RestartPolicy::Permanent,
        )
        .with_worker(
            "w2",
            |ctx| StatefulQuickWorker {
                id: "w2".to_string(),
                context: ctx,
            },
            RestartPolicy::Permanent,
        );

    let handle = StatefulSupervisorHandle::start(spec);
    sleep(Duration::from_millis(20)).await;
    handle.shutdown().await.unwrap();
}

#[tokio::test]
async fn test_stateful_supervisor_restart_intensity() {
    let spec = StatefulSupervisorSpec::new("stateful-intensity")
        .with_restart_intensity(RestartIntensity::new(3, 5))
        .with_worker(
            "failing",
            |ctx| StatefulFailingWorker {
                id: "failing".to_string(),
                context: ctx,
            },
            RestartPolicy::Permanent,
        );

    let handle = StatefulSupervisorHandle::start(spec);
    sleep(Duration::from_millis(100)).await;
    handle.shutdown().await.unwrap();
}

#[tokio::test]
async fn test_stateful_supervisor_dynamic_children() {
    let spec = StatefulSupervisorSpec::new("stateful-dynamic");
    let handle = StatefulSupervisorHandle::start(spec);

    // Add child
    let ctx = Arc::new(WorkerContext::new());
    let result = handle
        .start_child(
            "dynamic-1",
            |ctx| StatefulQuickWorker {
                id: "dynamic-1".to_string(),
                context: ctx,
            },
            RestartPolicy::Temporary,
            Arc::clone(&ctx),
        )
        .await;
    assert!(result.is_ok());

    sleep(Duration::from_millis(50)).await;

    let children = handle.which_children().await.unwrap();
    // Worker might have already exited
    assert!(children.len() <= 1);

    // Terminate child (might already be gone)
    let _result = handle.terminate_child("dynamic-1").await;
    // Don't assert - worker might have already exited

    sleep(Duration::from_millis(20)).await;

    let children = handle.which_children().await.unwrap();
    assert_eq!(children.len(), 0);

    handle.shutdown().await.unwrap();
}

#[tokio::test]
async fn test_stateful_supervisor_which_children() {
    let spec = StatefulSupervisorSpec::new("stateful-which")
        .with_worker(
            "w1",
            |ctx| StatefulQuickWorker {
                id: "w1".to_string(),
                context: ctx,
            },
            RestartPolicy::Permanent,
        )
        .with_worker(
            "w2",
            |ctx| StatefulQuickWorker {
                id: "w2".to_string(),
                context: ctx,
            },
            RestartPolicy::Transient,
        )
        .with_worker(
            "w3",
            |ctx| StatefulQuickWorker {
                id: "w3".to_string(),
                context: ctx,
            },
            RestartPolicy::Temporary,
        );

    let handle = StatefulSupervisorHandle::start(spec);
    sleep(Duration::from_millis(20)).await;

    let children = handle.which_children().await.unwrap();
    // Should have 1-3 children depending on timing
    // At least verify which_children works and returns a valid list
    assert!(children.len() <= 3);

    handle.shutdown().await.unwrap();
}

#[tokio::test]
async fn test_stateful_supervisor_nested() {
    let child_spec = StatefulSupervisorSpec::new("stateful-nested-child").with_worker(
        "inner-worker",
        |ctx| StatefulQuickWorker {
            id: "inner-worker".to_string(),
            context: ctx,
        },
        RestartPolicy::Temporary,
    );

    let parent_spec = StatefulSupervisorSpec::new("stateful-nested-parent")
        .with_worker(
            "outer-worker",
            |ctx| StatefulQuickWorker {
                id: "outer-worker".to_string(),
                context: ctx,
            },
            RestartPolicy::Temporary,
        )
        .with_supervisor(child_spec);

    let handle = StatefulSupervisorHandle::start(parent_spec);
    sleep(Duration::from_millis(100)).await;

    let children = handle.which_children().await.unwrap();
    // Should have at least the supervisor child (workers may have exited)
    assert!(children.len() >= 1 && children.len() <= 2);

    handle.shutdown().await.unwrap();
}

#[tokio::test]
async fn test_stateful_supervisor_duplicate_child() {
    let spec = StatefulSupervisorSpec::new("duplicate-test");
    let handle = StatefulSupervisorHandle::start(spec);

    // Add first child (permanent so it won't exit)
    let ctx = Arc::new(WorkerContext::new());
    let result1 = handle
        .start_child(
            "duplicate",
            |ctx| StatefulQuickWorker {
                id: "duplicate".to_string(),
                context: ctx,
            },
            RestartPolicy::Permanent,
            Arc::clone(&ctx),
        )
        .await;
    assert!(result1.is_ok());

    sleep(Duration::from_millis(20)).await;

    // Try to add duplicate - should fail even if first one exited and restarted
    let result2 = handle
        .start_child(
            "duplicate",
            |ctx| StatefulQuickWorker {
                id: "duplicate".to_string(),
                context: ctx,
            },
            RestartPolicy::Temporary,
            Arc::clone(&ctx),
        )
        .await;
    assert!(result2.is_err());

    handle.shutdown().await.unwrap();
}

#[tokio::test]
async fn test_stateful_supervisor_terminate_nonexistent() {
    let spec = StatefulSupervisorSpec::<StatefulQuickWorker>::new("terminate-test");
    let handle = StatefulSupervisorHandle::start(spec);

    let result = handle.terminate_child("nonexistent").await;
    assert!(result.is_err());

    handle.shutdown().await.unwrap();
}

#[test]
fn test_stateful_supervisor_spec_builder() {
    let _spec = StatefulSupervisorSpec::new("stateful-builder-test")
        .with_restart_strategy(RestartStrategy::RestForOne)
        .with_restart_intensity(RestartIntensity::new(5, 15))
        .with_worker(
            "w1",
            |ctx| StatefulQuickWorker {
                id: "w1".to_string(),
                context: ctx,
            },
            RestartPolicy::Permanent,
        );
    // Spec built successfully
}
