//! IoT Device Worker - simulates real-world device with telemetry

use ash_flare::Worker;
use async_trait::async_trait;
use rand::Rng;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;
use tokio::time::{Instant, sleep};

#[derive(Debug)]
pub struct DeviceError(pub String);

impl std::fmt::Display for DeviceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Device Error: {}", self.0)
    }
}

impl std::error::Error for DeviceError {}

#[derive(Clone)]
pub struct DeviceConfig {
    pub device_id: String,
    pub region: String,
    pub failure_rate: f32,      // 0.0 to 1.0 probability of failure
    pub check_interval_ms: u64, // How often device checks in
    pub max_uptime_secs: u64,   // Max time before simulated failure
}

pub struct IoTDevice {
    config: DeviceConfig,
    uptime: Arc<AtomicU64>,
    start_time: Instant,
    telemetry_count: u64,
}

impl IoTDevice {
    pub fn new(config: DeviceConfig) -> Self {
        Self {
            config,
            uptime: Arc::new(AtomicU64::new(0)),
            start_time: Instant::now(),
            telemetry_count: 0,
        }
    }

    async fn send_telemetry(&mut self) -> Result<(), DeviceError> {
        self.telemetry_count += 1;
        let uptime = self.start_time.elapsed().as_secs();
        self.uptime.store(uptime, Ordering::Relaxed);

        // Simulate random network issues
        let mut rng = rand::thread_rng();
        let random_value: f32 = rng.r#gen();
        if random_value < self.config.failure_rate {
            return Err(DeviceError(format!(
                "[{}] network timeout after {} messages",
                self.config.device_id, self.telemetry_count
            )));
        }

        // Simulate device crash after max uptime
        if uptime >= self.config.max_uptime_secs {
            return Err(DeviceError(format!(
                "[{}] hardware fault after {}s uptime",
                self.config.device_id, uptime
            )));
        }

        println!(
            "[{}/{}] telemetry #{}: temp={}°C, battery={}%, uptime={}s",
            self.config.region,
            self.config.device_id,
            self.telemetry_count,
            rng.r#gen_range(20..30),
            rng.r#gen_range(40..100),
            uptime
        );

        Ok(())
    }
}

#[async_trait]
impl Worker for IoTDevice {
    type Error = DeviceError;

    async fn initialize(&mut self) -> Result<(), Self::Error> {
        println!(
            "[{}/{}] device connected to regional supervisor",
            self.config.region, self.config.device_id
        );
        self.start_time = Instant::now();
        Ok(())
    }

    async fn run(&mut self) -> Result<(), Self::Error> {
        loop {
            self.send_telemetry().await?;
            sleep(Duration::from_millis(self.config.check_interval_ms)).await;
        }
    }

    async fn shutdown(&mut self) -> Result<(), Self::Error> {
        let uptime = self.start_time.elapsed().as_secs();
        println!(
            "[{}/{}] device disconnected (ran for {}s, sent {} messages)",
            self.config.region, self.config.device_id, uptime, self.telemetry_count
        );
        Ok(())
    }
}
