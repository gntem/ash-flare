use super::workers::QuickWorker;
use ash_flare::{RestartPolicy, RestartStrategy, SupervisorHandle, SupervisorSpec};
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_supervisor_name() {
    let spec = SupervisorSpec::<QuickWorker>::new("test-name");
    let handle = SupervisorHandle::start(spec);

    assert_eq!(handle.name(), "test-name");

    handle.shutdown().await.unwrap();
}

#[tokio::test]
async fn test_supervisor_restart_strategy() {
    let spec = SupervisorSpec::<QuickWorker>::new("strategy-test")
        .with_restart_strategy(RestartStrategy::OneForAll);
    let handle = SupervisorHandle::start(spec);

    let strategy = handle.restart_strategy().await.unwrap();
    assert_eq!(strategy, RestartStrategy::OneForAll);

    handle.shutdown().await.unwrap();
}

#[tokio::test]
async fn test_supervisor_uptime() {
    let spec = SupervisorSpec::<QuickWorker>::new("uptime-test");
    let handle = SupervisorHandle::start(spec);

    sleep(Duration::from_millis(100)).await;

    let uptime = handle.uptime().await.unwrap();
    // Uptime should be a valid u64
    let _ = uptime; // Just verify we can get it

    handle.shutdown().await.unwrap();
}

#[tokio::test]
async fn test_supervisor_operations_after_shutdown() {
    let spec = SupervisorSpec::<QuickWorker>::new("shutdown-test");
    let handle = SupervisorHandle::start(spec);

    handle.shutdown().await.unwrap();
    
    // Give it time to shut down
    sleep(Duration::from_millis(50)).await;

    // Operations after shutdown should fail
    let result = handle.which_children().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_stateful_supervisor_name() {
    use super::workers::StatefulQuickWorker;
    use ash_flare::{StatefulSupervisorHandle, StatefulSupervisorSpec};

    let spec = StatefulSupervisorSpec::<StatefulQuickWorker>::new("stateful-name");
    let handle = StatefulSupervisorHandle::start(spec);

    assert_eq!(handle.name(), "stateful-name");

    handle.shutdown().await.unwrap();
}

#[tokio::test]
async fn test_stateful_supervisor_restart_strategy() {
    use super::workers::StatefulQuickWorker;
    use ash_flare::{StatefulSupervisorHandle, StatefulSupervisorSpec};

    let spec = StatefulSupervisorSpec::<StatefulQuickWorker>::new("stateful-strategy")
        .with_restart_strategy(RestartStrategy::RestForOne);
    let handle = StatefulSupervisorHandle::start(spec);

    let strategy = handle.restart_strategy().await.unwrap();
    assert_eq!(strategy, RestartStrategy::RestForOne);

    handle.shutdown().await.unwrap();
}

#[tokio::test]
async fn test_stateful_supervisor_uptime() {
    use super::workers::StatefulQuickWorker;
    use ash_flare::{StatefulSupervisorHandle, StatefulSupervisorSpec};

    let spec = StatefulSupervisorSpec::<StatefulQuickWorker>::new("stateful-uptime");
    let handle = StatefulSupervisorHandle::start(spec);

    sleep(Duration::from_millis(100)).await;

    let uptime = handle.uptime().await.unwrap();
    // Uptime should be a valid u64
    let _ = uptime; // Just verify we can get it

    handle.shutdown().await.unwrap();
}

#[tokio::test]
async fn test_stateful_supervisor_operations_after_shutdown() {
    use super::workers::StatefulQuickWorker;
    use ash_flare::{StatefulSupervisorHandle, StatefulSupervisorSpec};

    let spec = StatefulSupervisorSpec::<StatefulQuickWorker>::new("stateful-shutdown");
    let handle = StatefulSupervisorHandle::start(spec);

    handle.shutdown().await.unwrap();
    
    sleep(Duration::from_millis(50)).await;

    let result = handle.which_children().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_start_child_after_shutdown() {
    let spec = SupervisorSpec::<QuickWorker>::new("start-after-shutdown");
    let handle = SupervisorHandle::start(spec);

    handle.shutdown().await.unwrap();
    sleep(Duration::from_millis(50)).await;

    let result = handle
        .start_child("late-child", || QuickWorker, RestartPolicy::Temporary)
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_terminate_child_after_shutdown() {
    let spec = SupervisorSpec::<QuickWorker>::new("terminate-after-shutdown");
    let handle = SupervisorHandle::start(spec);

    handle.shutdown().await.unwrap();
    sleep(Duration::from_millis(50)).await;

    let result = handle.terminate_child("child").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_restart_strategy_after_shutdown() {
    let spec = SupervisorSpec::<QuickWorker>::new("strategy-after-shutdown");
    let handle = SupervisorHandle::start(spec);

    handle.shutdown().await.unwrap();
    sleep(Duration::from_millis(50)).await;

    let result = handle.restart_strategy().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_uptime_after_shutdown() {
    let spec = SupervisorSpec::<QuickWorker>::new("uptime-after-shutdown");
    let handle = SupervisorHandle::start(spec);

    handle.shutdown().await.unwrap();
    sleep(Duration::from_millis(50)).await;

    let result = handle.uptime().await;
    assert!(result.is_err());
}
