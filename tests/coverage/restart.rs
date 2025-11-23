use ash_flare::RestartIntensity;

#[test]
fn test_restart_intensity_default() {
    let intensity = RestartIntensity::default();
    assert_eq!(intensity.max_restarts, 3);
    assert_eq!(intensity.within_seconds, 5);
}

#[test]
fn test_restart_intensity_const_new() {
    const INTENSITY: RestartIntensity = RestartIntensity::new(10, 20);
    assert_eq!(INTENSITY.max_restarts, 10);
    assert_eq!(INTENSITY.within_seconds, 20);
}

#[test]
fn test_restart_intensity_zero_restarts() {
    let intensity = RestartIntensity::new(0, 5);
    assert_eq!(intensity.max_restarts, 0);
}

#[test]
fn test_restart_intensity_zero_seconds() {
    let intensity = RestartIntensity::new(5, 0);
    assert_eq!(intensity.within_seconds, 0);
}

#[test]
fn test_restart_intensity_large_values() {
    let intensity = RestartIntensity::new(1000, 3600);
    assert_eq!(intensity.max_restarts, 1000);
    assert_eq!(intensity.within_seconds, 3600);
}
