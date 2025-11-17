//! US-West Regional IoT Device Supervisor
//! Manages IoT devices in the US-West region with distributed coordination

mod iot_device;

use ash_flare::distributed::SupervisorServer;
use ash_flare::{RestartIntensity, RestartPolicy, RestartStrategy, SupervisorSpec};
use iot_device::{DeviceConfig, IoTDevice};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    println!("=== US-West Regional Supervisor Starting ===\n");

    // Create supervisor for US-West region devices
    let supervisor = SupervisorSpec::new("us-west-supervisor")
        .with_restart_strategy(RestartStrategy::OneForOne)
        .with_restart_intensity(RestartIntensity {
            max_restarts: 5,
            within_seconds: 10,
        })
        // High-reliability devices (permanent restart)
        .with_worker(
            "sensor-001",
            || {
                IoTDevice::new(DeviceConfig {
                    device_id: "sensor-001".to_string(),
                    region: "us-west".to_string(),
                    failure_rate: 0.05, // 5% network failure rate
                    check_interval_ms: 2000,
                    max_uptime_secs: 20, // Fail after 20s (simulates hardware fault)
                })
            },
            RestartPolicy::Permanent,
        )
        .with_worker(
            "sensor-002",
            || {
                IoTDevice::new(DeviceConfig {
                    device_id: "sensor-002".to_string(),
                    region: "us-west".to_string(),
                    failure_rate: 0.03,
                    check_interval_ms: 1500,
                    max_uptime_secs: 25,
                })
            },
            RestartPolicy::Permanent,
        )
        // Temporary test device (don't restart on failure)
        .with_worker(
            "test-device-beta",
            || {
                IoTDevice::new(DeviceConfig {
                    device_id: "test-device-beta".to_string(),
                    region: "us-west".to_string(),
                    failure_rate: 0.15, // Higher failure rate for testing
                    check_interval_ms: 1000,
                    max_uptime_secs: 15,
                })
            },
            RestartPolicy::Temporary,
        )
        // Camera device (transient restart - only on errors, not normal shutdown)
        .with_worker(
            "camera-103",
            || {
                IoTDevice::new(DeviceConfig {
                    device_id: "camera-103".to_string(),
                    region: "us-west".to_string(),
                    failure_rate: 0.08,
                    check_interval_ms: 3000,
                    max_uptime_secs: 30,
                })
            },
            RestartPolicy::Transient,
        );

    let handle = ash_flare::SupervisorHandle::start(supervisor);

    // Start distributed control server on Unix socket
    let socket_path = "/tmp/supervisor-us-west.sock";
    let _ = std::fs::remove_file(socket_path); // Clean up old socket

    println!("Starting remote control server at: {}\n", socket_path);
    let server = SupervisorServer::new(handle);
    let server_handle = tokio::spawn(async move {
        if let Err(e) = server.listen_unix(socket_path).await {
            eprintln!("Server error: {}", e);
        }
    });

    // Wait a bit before trying to add children (server needs to start)
    sleep(Duration::from_secs(2)).await;

    // Create a new handle to interact with the supervisor via distributed API
    let client = ash_flare::distributed::RemoteSupervisorHandle::connect_unix(socket_path)
        .await
        .expect("Failed to connect to local supervisor");

    // Dynamically add a device after startup
    sleep(Duration::from_secs(3)).await;
    
    // Query children instead
    sleep(Duration::from_secs(25)).await;

    println!("\n>>> Querying active devices (via distributed API):\n");
    if let Ok(children) = client.which_children().await {
        for child in children {
            println!(
                "  - {} ({:?}, restart: {:?})",
                child.id, child.child_type, child.restart_policy
            );
        }
    }

    // Run until interrupted
    println!("\n>>> Regional supervisor running (Ctrl+C to stop)...\n");
    tokio::signal::ctrl_c().await.unwrap();

    println!("\n\n=== Shutting down US-West Regional Supervisor ===");
    let _ = client.shutdown().await;
    server_handle.abort();
    let _ = std::fs::remove_file(socket_path);
}
