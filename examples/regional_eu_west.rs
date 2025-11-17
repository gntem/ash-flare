//! EU-West Regional IoT Device Supervisor
//! Manages IoT devices in the EU-West region with distributed coordination

mod iot_device;

use ash_flare::distributed::SupervisorServer;
use ash_flare::{RestartIntensity, RestartPolicy, RestartStrategy, SupervisorSpec};
use iot_device::{DeviceConfig, IoTDevice};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    println!("=== EU-West Regional Supervisor Starting ===\n");

    // Create supervisor for EU-West region devices
    let supervisor = SupervisorSpec::new("eu-west-supervisor")
        .with_restart_strategy(RestartStrategy::OneForOne)
        .with_restart_intensity(RestartIntensity {
            max_restarts: 5,
            within_seconds: 10,
        })
        // Smart meters (permanent restart)
        .with_worker(
            "meter-eu-001",
            || {
                IoTDevice::new(DeviceConfig {
                    device_id: "meter-eu-001".to_string(),
                    region: "eu-west".to_string(),
                    failure_rate: 0.04,
                    check_interval_ms: 2200,
                    max_uptime_secs: 22,
                })
            },
            RestartPolicy::Permanent,
        )
        .with_worker(
            "meter-eu-002",
            || {
                IoTDevice::new(DeviceConfig {
                    device_id: "meter-eu-002".to_string(),
                    region: "eu-west".to_string(),
                    failure_rate: 0.06,
                    check_interval_ms: 1800,
                    max_uptime_secs: 28,
                })
            },
            RestartPolicy::Permanent,
        )
        // Industrial sensor (permanent)
        .with_worker(
            "industrial-501",
            || {
                IoTDevice::new(DeviceConfig {
                    device_id: "industrial-501".to_string(),
                    region: "eu-west".to_string(),
                    failure_rate: 0.02, // Lower failure rate for industrial
                    check_interval_ms: 2500,
                    max_uptime_secs: 35,
                })
            },
            RestartPolicy::Permanent,
        )
        // Temporary field test device
        .with_worker(
            "field-test-alpha",
            || {
                IoTDevice::new(DeviceConfig {
                    device_id: "field-test-alpha".to_string(),
                    region: "eu-west".to_string(),
                    failure_rate: 0.20, // High failure rate for test device
                    check_interval_ms: 1200,
                    max_uptime_secs: 12,
                })
            },
            RestartPolicy::Temporary,
        );

    let handle = ash_flare::SupervisorHandle::start(supervisor);

    // Start distributed control server on Unix socket
    let socket_path = "/tmp/supervisor-eu-west.sock";
    let _ = std::fs::remove_file(socket_path);

    println!("Starting remote control server at: {}\n", socket_path);
    let server = SupervisorServer::new(handle);
    let server_handle = tokio::spawn(async move {
        if let Err(e) = server.listen_unix(socket_path).await {
            eprintln!("Server error: {}", e);
        }
    });

    // Wait for server to start
    sleep(Duration::from_secs(2)).await;

    // Create client to interact via distributed API
    let client = ash_flare::distributed::RemoteSupervisorHandle::connect_unix(socket_path)
        .await
        .expect("Failed to connect to local supervisor");

    // Monitor and query devices
    sleep(Duration::from_secs(30)).await;

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

    println!("\n\n=== Shutting down EU-West Regional Supervisor ===");
    let _ = client.shutdown().await;
    server_handle.abort();
    let _ = std::fs::remove_file(socket_path);
}
