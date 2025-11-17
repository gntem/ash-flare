use ash_flare::{RestartIntensity, RestartPolicy, RestartStrategy};

#[test]
fn test_restart_strategy_defaults() {
    assert_eq!(RestartStrategy::default(), RestartStrategy::OneForOne);
}

#[test]
fn test_restart_policy_defaults() {
    assert_eq!(RestartPolicy::default(), RestartPolicy::Permanent);
}

#[test]
fn test_restart_intensity_defaults() {
    let intensity = RestartIntensity::default();
    assert_eq!(intensity.max_restarts, 3);
    assert_eq!(intensity.within_seconds, 5);
}

#[test]
fn test_restart_intensity_custom() {
    let intensity = RestartIntensity {
        max_restarts: 10,
        within_seconds: 30,
    };
    assert_eq!(intensity.max_restarts, 10);
    assert_eq!(intensity.within_seconds, 30);
}

#[test]
fn test_restart_strategy_equality() {
    assert_eq!(RestartStrategy::OneForOne, RestartStrategy::OneForOne);
    assert_ne!(RestartStrategy::OneForOne, RestartStrategy::OneForAll);
    assert_ne!(RestartStrategy::OneForAll, RestartStrategy::RestForOne);
}

#[test]
fn test_restart_policy_equality() {
    assert_eq!(RestartPolicy::Permanent, RestartPolicy::Permanent);
    assert_ne!(RestartPolicy::Permanent, RestartPolicy::Temporary);
    assert_ne!(RestartPolicy::Temporary, RestartPolicy::Transient);
}
