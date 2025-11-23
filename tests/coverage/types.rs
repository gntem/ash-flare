use ash_flare::{ChildType, RestartPolicy, RestartStrategy};

#[test]
fn test_child_type_debug() {
    let worker = ChildType::Worker;
    let debug_str = format!("{:?}", worker);
    assert!(debug_str.contains("Worker"));

    let supervisor = ChildType::Supervisor;
    let debug_str = format!("{:?}", supervisor);
    assert!(debug_str.contains("Supervisor"));
}

#[test]
fn test_child_type_clone() {
    let worker = ChildType::Worker;
    let cloned = worker.clone();
    assert_eq!(cloned, ChildType::Worker);
}

#[test]
fn test_child_type_equality() {
    assert_eq!(ChildType::Worker, ChildType::Worker);
    assert_eq!(ChildType::Supervisor, ChildType::Supervisor);
    assert_ne!(ChildType::Worker, ChildType::Supervisor);
}

#[test]
fn test_restart_policy_debug() {
    let permanent = RestartPolicy::Permanent;
    assert!(format!("{:?}", permanent).contains("Permanent"));

    let temporary = RestartPolicy::Temporary;
    assert!(format!("{:?}", temporary).contains("Temporary"));

    let transient = RestartPolicy::Transient;
    assert!(format!("{:?}", transient).contains("Transient"));
}

#[test]
fn test_restart_policy_clone() {
    let policy = RestartPolicy::Permanent;
    let cloned = policy.clone();
    assert_eq!(cloned, RestartPolicy::Permanent);
}

#[test]
fn test_restart_policy_copy() {
    let policy = RestartPolicy::Transient;
    let copied = policy; // Copy trait
    assert_eq!(copied, RestartPolicy::Transient);
    assert_eq!(policy, RestartPolicy::Transient); // Original still valid
}

#[test]
fn test_restart_strategy_debug() {
    let one_for_one = RestartStrategy::OneForOne;
    assert!(format!("{:?}", one_for_one).contains("OneForOne"));

    let one_for_all = RestartStrategy::OneForAll;
    assert!(format!("{:?}", one_for_all).contains("OneForAll"));

    let rest_for_one = RestartStrategy::RestForOne;
    assert!(format!("{:?}", rest_for_one).contains("RestForOne"));
}

#[test]
fn test_restart_strategy_clone() {
    let strategy = RestartStrategy::OneForAll;
    let cloned = strategy.clone();
    assert_eq!(cloned, RestartStrategy::OneForAll);
}

#[test]
fn test_restart_strategy_copy() {
    let strategy = RestartStrategy::RestForOne;
    let copied = strategy; // Copy trait
    assert_eq!(copied, RestartStrategy::RestForOne);
    assert_eq!(strategy, RestartStrategy::RestForOne); // Original still valid
}

#[test]
fn test_restart_policy_default() {
    let default = RestartPolicy::default();
    assert_eq!(default, RestartPolicy::Permanent);
}

#[test]
fn test_restart_strategy_default() {
    let default = RestartStrategy::default();
    assert_eq!(default, RestartStrategy::OneForOne);
}
