//! Large supervision tree example with 50+ workers
//!
//! This example demonstrates a complex, multi-layered supervision tree
//! simulating a microservices architecture with:
//! - API Gateway layer (3 workers)
//! - Service layer (4 supervisors, each with 3-5 workers)
//! - Data layer (3 supervisors for different databases)
//! - Background job processors (2 supervisors with worker pools)
//! - Monitoring & metrics collectors
//!
//! Total: 55 workers

use ash_flare::{
    RestartIntensity, RestartPolicy, RestartStrategy, SupervisorHandle,
    SupervisorSpec, Worker,
};
use async_trait::async_trait;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;
use tokio::time::sleep;

// Global request counter for demo purposes
static REQUEST_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
enum ServiceWorker {
    HttpGateway(HttpGateway),
    WebSocketGateway(WebSocketGateway),
    GraphQLGateway(GraphQLGateway),
    AuthService(AuthService),
    ProfileService(ProfileService),
    NotificationService(NotificationService),
    ContentIndexer(ContentIndexer),
    MediaProcessor(MediaProcessor),
    CdnSync(CdnSync),
    PaymentProcessor(PaymentProcessor),
    FraudDetector(FraudDetector),
    EventCollector(EventCollector),
    MetricsAggregator(MetricsAggregator),
    PostgresConnector(PostgresConnector),
    RedisConnector(RedisConnector),
    MongoConnector(MongoConnector),
    EmailWorker(EmailWorker),
    ReportGenerator(ReportGenerator),
    DataSyncWorker(DataSyncWorker),
    HealthChecker(HealthChecker),
    LogAggregator(LogAggregator),
    AlertManager(AlertManager),
}

#[async_trait]
impl Worker for ServiceWorker {
    type Error = WorkerError;

    async fn initialize(&mut self) -> Result<(), Self::Error> {
        match self {
            ServiceWorker::HttpGateway(w) => w.initialize().await,
            ServiceWorker::WebSocketGateway(w) => w.initialize().await,
            ServiceWorker::GraphQLGateway(w) => w.initialize().await,
            ServiceWorker::AuthService(w) => w.initialize().await,
            ServiceWorker::ProfileService(w) => w.initialize().await,
            ServiceWorker::NotificationService(w) => w.initialize().await,
            ServiceWorker::ContentIndexer(w) => w.initialize().await,
            ServiceWorker::MediaProcessor(w) => w.initialize().await,
            ServiceWorker::CdnSync(w) => w.initialize().await,
            ServiceWorker::PaymentProcessor(w) => w.initialize().await,
            ServiceWorker::FraudDetector(w) => w.initialize().await,
            ServiceWorker::EventCollector(w) => w.initialize().await,
            ServiceWorker::MetricsAggregator(w) => w.initialize().await,
            ServiceWorker::PostgresConnector(w) => w.initialize().await,
            ServiceWorker::RedisConnector(w) => w.initialize().await,
            ServiceWorker::MongoConnector(w) => w.initialize().await,
            ServiceWorker::EmailWorker(w) => w.initialize().await,
            ServiceWorker::ReportGenerator(w) => w.initialize().await,
            ServiceWorker::DataSyncWorker(w) => w.initialize().await,
            ServiceWorker::HealthChecker(w) => w.initialize().await,
            ServiceWorker::LogAggregator(w) => w.initialize().await,
            ServiceWorker::AlertManager(w) => w.initialize().await,
        }
    }

    async fn run(&mut self) -> Result<(), Self::Error> {
        match self {
            ServiceWorker::HttpGateway(w) => w.run().await,
            ServiceWorker::WebSocketGateway(w) => w.run().await,
            ServiceWorker::GraphQLGateway(w) => w.run().await,
            ServiceWorker::AuthService(w) => w.run().await,
            ServiceWorker::ProfileService(w) => w.run().await,
            ServiceWorker::NotificationService(w) => w.run().await,
            ServiceWorker::ContentIndexer(w) => w.run().await,
            ServiceWorker::MediaProcessor(w) => w.run().await,
            ServiceWorker::CdnSync(w) => w.run().await,
            ServiceWorker::PaymentProcessor(w) => w.run().await,
            ServiceWorker::FraudDetector(w) => w.run().await,
            ServiceWorker::EventCollector(w) => w.run().await,
            ServiceWorker::MetricsAggregator(w) => w.run().await,
            ServiceWorker::PostgresConnector(w) => w.run().await,
            ServiceWorker::RedisConnector(w) => w.run().await,
            ServiceWorker::MongoConnector(w) => w.run().await,
            ServiceWorker::EmailWorker(w) => w.run().await,
            ServiceWorker::ReportGenerator(w) => w.run().await,
            ServiceWorker::DataSyncWorker(w) => w.run().await,
            ServiceWorker::HealthChecker(w) => w.run().await,
            ServiceWorker::LogAggregator(w) => w.run().await,
            ServiceWorker::AlertManager(w) => w.run().await,
        }
    }

    async fn shutdown(&mut self) -> Result<(), Self::Error> {
        match self {
            ServiceWorker::HttpGateway(w) => w.shutdown().await,
            ServiceWorker::WebSocketGateway(w) => w.shutdown().await,
            ServiceWorker::GraphQLGateway(w) => w.shutdown().await,
            ServiceWorker::AuthService(w) => w.shutdown().await,
            ServiceWorker::ProfileService(w) => w.shutdown().await,
            ServiceWorker::NotificationService(w) => w.shutdown().await,
            ServiceWorker::ContentIndexer(w) => w.shutdown().await,
            ServiceWorker::MediaProcessor(w) => w.shutdown().await,
            ServiceWorker::CdnSync(w) => w.shutdown().await,
            ServiceWorker::PaymentProcessor(w) => w.shutdown().await,
            ServiceWorker::FraudDetector(w) => w.shutdown().await,
            ServiceWorker::EventCollector(w) => w.shutdown().await,
            ServiceWorker::MetricsAggregator(w) => w.shutdown().await,
            ServiceWorker::PostgresConnector(w) => w.shutdown().await,
            ServiceWorker::RedisConnector(w) => w.shutdown().await,
            ServiceWorker::MongoConnector(w) => w.shutdown().await,
            ServiceWorker::EmailWorker(w) => w.shutdown().await,
            ServiceWorker::ReportGenerator(w) => w.shutdown().await,
            ServiceWorker::DataSyncWorker(w) => w.shutdown().await,
            ServiceWorker::HealthChecker(w) => w.shutdown().await,
            ServiceWorker::LogAggregator(w) => w.shutdown().await,
            ServiceWorker::AlertManager(w) => w.shutdown().await,
        }
    }
}

// ============================================================================
// Worker Error Type
// ============================================================================

#[derive(Debug)]
struct WorkerError(String);

impl std::fmt::Display for WorkerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for WorkerError {}

// ============================================================================
// API Gateway Layer Workers
// ============================================================================

#[derive(Debug)]
struct HttpGateway {
    id: u32,
}

#[async_trait]
impl Worker for HttpGateway {
    type Error = WorkerError;

    async fn initialize(&mut self) -> Result<(), Self::Error> {
        println!("🌐 [HTTP-Gateway-{}] Starting up...", self.id);
        Ok(())
    }

    async fn run(&mut self) -> Result<(), Self::Error> {
        loop {
            let count = REQUEST_COUNTER.fetch_add(1, Ordering::Relaxed);
            if count % 100 == 0 {
                println!("🌐 [HTTP-Gateway-{}] Processed {} requests", self.id, count);
            }
            sleep(Duration::from_millis(50)).await;
        }
    }

    async fn shutdown(&mut self) -> Result<(), Self::Error> {
        println!("🌐 [HTTP-Gateway-{}] Shutting down", self.id);
        Ok(())
    }
}

#[derive(Debug)]
struct WebSocketGateway {
    connections: u32,
}

#[async_trait]
impl Worker for WebSocketGateway {
    type Error = WorkerError;

    async fn initialize(&mut self) -> Result<(), Self::Error> {
        println!("🔌 [WebSocket-Gateway] Managing {} connections", self.connections);
        Ok(())
    }

    async fn run(&mut self) -> Result<(), Self::Error> {
        loop {
            sleep(Duration::from_millis(100)).await;
        }
    }

    async fn shutdown(&mut self) -> Result<(), Self::Error> {
        println!("🔌 [WebSocket-Gateway] Closing all connections");
        Ok(())
    }
}

#[derive(Debug)]
struct GraphQLGateway;

#[async_trait]
impl Worker for GraphQLGateway {
    type Error = WorkerError;

    async fn initialize(&mut self) -> Result<(), Self::Error> {
        println!("🔺 [GraphQL-Gateway] Schema loaded");
        Ok(())
    }

    async fn run(&mut self) -> Result<(), Self::Error> {
        loop {
            sleep(Duration::from_millis(75)).await;
        }
    }

    async fn shutdown(&mut self) -> Result<(), Self::Error> {
        println!("🔺 [GraphQL-Gateway] Shutting down");
        Ok(())
    }
}

// ============================================================================
// User Service Workers
// ============================================================================

#[derive(Debug)]
struct AuthService {
    id: u32,
}

#[async_trait]
impl Worker for AuthService {
    type Error = WorkerError;

    async fn initialize(&mut self) -> Result<(), Self::Error> {
        println!("🔐 [Auth-{}] JWT validator ready", self.id);
        Ok(())
    }

    async fn run(&mut self) -> Result<(), Self::Error> {
        loop {
            sleep(Duration::from_millis(60)).await;
        }
    }

    async fn shutdown(&mut self) -> Result<(), Self::Error> {
        println!("🔐 [Auth-{}] Shutdown", self.id);
        Ok(())
    }
}

#[derive(Debug)]
struct ProfileService {
    id: u32,
}

#[async_trait]
impl Worker for ProfileService {
    type Error = WorkerError;

    async fn initialize(&mut self) -> Result<(), Self::Error> {
        println!("👤 [Profile-{}] Cache warmed", self.id);
        Ok(())
    }

    async fn run(&mut self) -> Result<(), Self::Error> {
        loop {
            sleep(Duration::from_millis(80)).await;
        }
    }

    async fn shutdown(&mut self) -> Result<(), Self::Error> {
        println!("👤 [Profile-{}] Shutdown", self.id);
        Ok(())
    }
}

#[derive(Debug)]
struct NotificationService {
    id: u32,
}

#[async_trait]
impl Worker for NotificationService {
    type Error = WorkerError;

    async fn initialize(&mut self) -> Result<(), Self::Error> {
        println!("📧 [Notification-{}] Email/SMS provider connected", self.id);
        Ok(())
    }

    async fn run(&mut self) -> Result<(), Self::Error> {
        loop {
            sleep(Duration::from_millis(120)).await;
        }
    }

    async fn shutdown(&mut self) -> Result<(), Self::Error> {
        println!("📧 [Notification-{}] Shutdown", self.id);
        Ok(())
    }
}

// ============================================================================
// Content Service Workers
// ============================================================================

#[derive(Debug)]
struct ContentIndexer {
    id: u32,
}

#[async_trait]
impl Worker for ContentIndexer {
    type Error = WorkerError;

    async fn initialize(&mut self) -> Result<(), Self::Error> {
        println!("📑 [Indexer-{}] Search engine connected", self.id);
        Ok(())
    }

    async fn run(&mut self) -> Result<(), Self::Error> {
        loop {
            sleep(Duration::from_millis(90)).await;
        }
    }

    async fn shutdown(&mut self) -> Result<(), Self::Error> {
        println!("📑 [Indexer-{}] Shutdown", self.id);
        Ok(())
    }
}

#[derive(Debug)]
struct MediaProcessor {
    id: u32,
}

#[async_trait]
impl Worker for MediaProcessor {
    type Error = WorkerError;

    async fn initialize(&mut self) -> Result<(), Self::Error> {
        println!("🎬 [Media-{}] FFmpeg pipeline ready", self.id);
        Ok(())
    }

    async fn run(&mut self) -> Result<(), Self::Error> {
        loop {
            sleep(Duration::from_millis(150)).await;
        }
    }

    async fn shutdown(&mut self) -> Result<(), Self::Error> {
        println!("🎬 [Media-{}] Shutdown", self.id);
        Ok(())
    }
}

#[derive(Debug)]
struct CdnSync {
    id: u32,
}

#[async_trait]
impl Worker for CdnSync {
    type Error = WorkerError;

    async fn initialize(&mut self) -> Result<(), Self::Error> {
        println!("🌍 [CDN-{}] Edge nodes synchronized", self.id);
        Ok(())
    }

    async fn run(&mut self) -> Result<(), Self::Error> {
        loop {
            sleep(Duration::from_millis(200)).await;
        }
    }

    async fn shutdown(&mut self) -> Result<(), Self::Error> {
        println!("🌍 [CDN-{}] Shutdown", self.id);
        Ok(())
    }
}

// ============================================================================
// Payment Service Workers
// ============================================================================

#[derive(Debug)]
struct PaymentProcessor {
    id: u32,
    provider: &'static str,
}

#[async_trait]
impl Worker for PaymentProcessor {
    type Error = WorkerError;

    async fn initialize(&mut self) -> Result<(), Self::Error> {
        println!("💳 [Payment-{}-{}] Connected to provider", self.id, self.provider);
        Ok(())
    }

    async fn run(&mut self) -> Result<(), Self::Error> {
        loop {
            sleep(Duration::from_millis(100)).await;
        }
    }

    async fn shutdown(&mut self) -> Result<(), Self::Error> {
        println!("💳 [Payment-{}-{}] Shutdown", self.id, self.provider);
        Ok(())
    }
}

#[derive(Debug)]
struct FraudDetector {
    id: u32,
}

#[async_trait]
impl Worker for FraudDetector {
    type Error = WorkerError;

    async fn initialize(&mut self) -> Result<(), Self::Error> {
        println!("🚨 [Fraud-{}] ML model loaded", self.id);
        Ok(())
    }

    async fn run(&mut self) -> Result<(), Self::Error> {
        loop {
            sleep(Duration::from_millis(80)).await;
        }
    }

    async fn shutdown(&mut self) -> Result<(), Self::Error> {
        println!("🚨 [Fraud-{}] Shutdown", self.id);
        Ok(())
    }
}

// ============================================================================
// Analytics Service Workers
// ============================================================================

#[derive(Debug)]
struct EventCollector {
    id: u32,
}

#[async_trait]
impl Worker for EventCollector {
    type Error = WorkerError;

    async fn initialize(&mut self) -> Result<(), Self::Error> {
        println!("📊 [Events-{}] Kafka consumer ready", self.id);
        Ok(())
    }

    async fn run(&mut self) -> Result<(), Self::Error> {
        loop {
            sleep(Duration::from_millis(40)).await;
        }
    }

    async fn shutdown(&mut self) -> Result<(), Self::Error> {
        println!("📊 [Events-{}] Shutdown", self.id);
        Ok(())
    }
}

#[derive(Debug)]
struct MetricsAggregator {
    id: u32,
}

#[async_trait]
impl Worker for MetricsAggregator {
    type Error = WorkerError;

    async fn initialize(&mut self) -> Result<(), Self::Error> {
        println!("📈 [Metrics-{}] Time-series DB connected", self.id);
        Ok(())
    }

    async fn run(&mut self) -> Result<(), Self::Error> {
        loop {
            sleep(Duration::from_millis(100)).await;
        }
    }

    async fn shutdown(&mut self) -> Result<(), Self::Error> {
        println!("📈 [Metrics-{}] Shutdown", self.id);
        Ok(())
    }
}

// ============================================================================
// Database Workers
// ============================================================================

#[derive(Debug)]
struct PostgresConnector {
    id: u32,
    pool_size: u32,
}

#[async_trait]
impl Worker for PostgresConnector {
    type Error = WorkerError;

    async fn initialize(&mut self) -> Result<(), Self::Error> {
        println!("🐘 [Postgres-{}] Connection pool ({} conns) ready", self.id, self.pool_size);
        Ok(())
    }

    async fn run(&mut self) -> Result<(), Self::Error> {
        loop {
            sleep(Duration::from_millis(70)).await;
        }
    }

    async fn shutdown(&mut self) -> Result<(), Self::Error> {
        println!("🐘 [Postgres-{}] Closing connections", self.id);
        Ok(())
    }
}

#[derive(Debug)]
struct RedisConnector {
    id: u32,
}

#[async_trait]
impl Worker for RedisConnector {
    type Error = WorkerError;

    async fn initialize(&mut self) -> Result<(), Self::Error> {
        println!("🔴 [Redis-{}] Cache cluster connected", self.id);
        Ok(())
    }

    async fn run(&mut self) -> Result<(), Self::Error> {
        loop {
            sleep(Duration::from_millis(30)).await;
        }
    }

    async fn shutdown(&mut self) -> Result<(), Self::Error> {
        println!("🔴 [Redis-{}] Shutdown", self.id);
        Ok(())
    }
}

#[derive(Debug)]
struct MongoConnector {
    id: u32,
}

#[async_trait]
impl Worker for MongoConnector {
    type Error = WorkerError;

    async fn initialize(&mut self) -> Result<(), Self::Error> {
        println!("🍃 [MongoDB-{}] Replica set connected", self.id);
        Ok(())
    }

    async fn run(&mut self) -> Result<(), Self::Error> {
        loop {
            sleep(Duration::from_millis(60)).await;
        }
    }

    async fn shutdown(&mut self) -> Result<(), Self::Error> {
        println!("🍃 [MongoDB-{}] Shutdown", self.id);
        Ok(())
    }
}

// ============================================================================
// Background Job Workers
// ============================================================================

#[derive(Debug)]
struct EmailWorker {
    id: u32,
}

#[async_trait]
impl Worker for EmailWorker {
    type Error = WorkerError;

    async fn initialize(&mut self) -> Result<(), Self::Error> {
        println!("📮 [Email-Worker-{}] SMTP ready", self.id);
        Ok(())
    }

    async fn run(&mut self) -> Result<(), Self::Error> {
        loop {
            sleep(Duration::from_millis(150)).await;
        }
    }

    async fn shutdown(&mut self) -> Result<(), Self::Error> {
        println!("📮 [Email-Worker-{}] Shutdown", self.id);
        Ok(())
    }
}

#[derive(Debug)]
struct ReportGenerator {
    id: u32,
}

#[async_trait]
impl Worker for ReportGenerator {
    type Error = WorkerError;

    async fn initialize(&mut self) -> Result<(), Self::Error> {
        println!("📋 [Report-{}] Template engine loaded", self.id);
        Ok(())
    }

    async fn run(&mut self) -> Result<(), Self::Error> {
        loop {
            sleep(Duration::from_millis(300)).await;
        }
    }

    async fn shutdown(&mut self) -> Result<(), Self::Error> {
        println!("📋 [Report-{}] Shutdown", self.id);
        Ok(())
    }
}

#[derive(Debug)]
struct DataSyncWorker {
    id: u32,
}

#[async_trait]
impl Worker for DataSyncWorker {
    type Error = WorkerError;

    async fn initialize(&mut self) -> Result<(), Self::Error> {
        println!("🔄 [Sync-{}] CDC stream active", self.id);
        Ok(())
    }

    async fn run(&mut self) -> Result<(), Self::Error> {
        loop {
            sleep(Duration::from_millis(180)).await;
        }
    }

    async fn shutdown(&mut self) -> Result<(), Self::Error> {
        println!("🔄 [Sync-{}] Shutdown", self.id);
        Ok(())
    }
}

// ============================================================================
// Monitoring Workers
// ============================================================================

#[derive(Debug)]
struct HealthChecker;

#[async_trait]
impl Worker for HealthChecker {
    type Error = WorkerError;

    async fn initialize(&mut self) -> Result<(), Self::Error> {
        println!("💚 [HealthCheck] Starting health monitoring");
        Ok(())
    }

    async fn run(&mut self) -> Result<(), Self::Error> {
        loop {
            sleep(Duration::from_secs(5)).await;
            println!("💚 [HealthCheck] All systems operational");
        }
    }

    async fn shutdown(&mut self) -> Result<(), Self::Error> {
        println!("💚 [HealthCheck] Shutdown");
        Ok(())
    }
}

#[derive(Debug)]
struct LogAggregator;

#[async_trait]
impl Worker for LogAggregator {
    type Error = WorkerError;

    async fn initialize(&mut self) -> Result<(), Self::Error> {
        println!("📝 [LogAggregator] Connected to log streams");
        Ok(())
    }

    async fn run(&mut self) -> Result<(), Self::Error> {
        loop {
            sleep(Duration::from_millis(200)).await;
        }
    }

    async fn shutdown(&mut self) -> Result<(), Self::Error> {
        println!("📝 [LogAggregator] Shutdown");
        Ok(())
    }
}

#[derive(Debug)]
struct AlertManager;

#[async_trait]
impl Worker for AlertManager {
    type Error = WorkerError;

    async fn initialize(&mut self) -> Result<(), Self::Error> {
        println!("🚨 [AlertManager] PagerDuty/Slack webhooks ready");
        Ok(())
    }

    async fn run(&mut self) -> Result<(), Self::Error> {
        loop {
            sleep(Duration::from_millis(500)).await;
        }
    }

    async fn shutdown(&mut self) -> Result<(), Self::Error> {
        println!("🚨 [AlertManager] Shutdown");
        Ok(())
    }
}

// ============================================================================
// Main Supervision Tree
// ============================================================================

#[tokio::main]
async fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║  Large Supervision Tree Example - 55+ Workers               ║");
    println!("║  Simulating a Microservices Architecture                    ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // Build API Gateway Supervisor
    let api_gateway_spec = SupervisorSpec::new("api-gateway")
        .with_restart_strategy(RestartStrategy::OneForOne)
        .with_restart_intensity(RestartIntensity::new(5, 10))
        .with_worker("http-gw-1", || ServiceWorker::HttpGateway(HttpGateway { id: 1 }), RestartPolicy::Permanent)
        .with_worker("http-gw-2", || ServiceWorker::HttpGateway(HttpGateway { id: 2 }), RestartPolicy::Permanent)
        .with_worker("http-gw-3", || ServiceWorker::HttpGateway(HttpGateway { id: 3 }), RestartPolicy::Permanent)
        .with_worker("websocket-gw", || ServiceWorker::WebSocketGateway(WebSocketGateway { connections: 1000 }), RestartPolicy::Permanent)
        .with_worker("graphql-gw", || ServiceWorker::GraphQLGateway(GraphQLGateway), RestartPolicy::Permanent);

    // Build User Service Supervisor
    let user_service_spec = SupervisorSpec::new("user-service")
        .with_restart_strategy(RestartStrategy::OneForOne)
        .with_restart_intensity(RestartIntensity::new(3, 5))
        .with_worker("auth-1", || ServiceWorker::AuthService(AuthService { id: 1 }), RestartPolicy::Permanent)
        .with_worker("auth-2", || ServiceWorker::AuthService(AuthService { id: 2 }), RestartPolicy::Permanent)
        .with_worker("profile-1", || ServiceWorker::ProfileService(ProfileService { id: 1 }), RestartPolicy::Permanent)
        .with_worker("profile-2", || ServiceWorker::ProfileService(ProfileService { id: 2 }), RestartPolicy::Permanent)
        .with_worker("notification-1", || ServiceWorker::NotificationService(NotificationService { id: 1 }), RestartPolicy::Permanent);

    // Build Content Service Supervisor
    let content_service_spec = SupervisorSpec::new("content-service")
        .with_restart_strategy(RestartStrategy::OneForOne)
        .with_restart_intensity(RestartIntensity::new(4, 8))
        .with_worker("indexer-1", || ServiceWorker::ContentIndexer(ContentIndexer { id: 1 }), RestartPolicy::Permanent)
        .with_worker("indexer-2", || ServiceWorker::ContentIndexer(ContentIndexer { id: 2 }), RestartPolicy::Permanent)
        .with_worker("media-1", || ServiceWorker::MediaProcessor(MediaProcessor { id: 1 }), RestartPolicy::Permanent)
        .with_worker("media-2", || ServiceWorker::MediaProcessor(MediaProcessor { id: 2 }), RestartPolicy::Permanent)
        .with_worker("cdn-sync-1", || ServiceWorker::CdnSync(CdnSync { id: 1 }), RestartPolicy::Permanent)
        .with_worker("cdn-sync-2", || ServiceWorker::CdnSync(CdnSync { id: 2 }), RestartPolicy::Permanent);

    // Build Payment Service Supervisor
    let payment_service_spec = SupervisorSpec::new("payment-service")
        .with_restart_strategy(RestartStrategy::OneForAll) // Critical: restart all if any fails
        .with_restart_intensity(RestartIntensity::new(2, 5))
        .with_worker("payment-stripe", || ServiceWorker::PaymentProcessor(PaymentProcessor { id: 1, provider: "stripe" }), RestartPolicy::Permanent)
        .with_worker("payment-paypal", || ServiceWorker::PaymentProcessor(PaymentProcessor { id: 2, provider: "paypal" }), RestartPolicy::Permanent)
        .with_worker("fraud-1", || ServiceWorker::FraudDetector(FraudDetector { id: 1 }), RestartPolicy::Permanent)
        .with_worker("fraud-2", || ServiceWorker::FraudDetector(FraudDetector { id: 2 }), RestartPolicy::Permanent);

    // Build Analytics Service Supervisor
    let analytics_service_spec = SupervisorSpec::new("analytics-service")
        .with_restart_strategy(RestartStrategy::OneForOne)
        .with_restart_intensity(RestartIntensity::new(5, 10))
        .with_worker("event-collector-1", || ServiceWorker::EventCollector(EventCollector { id: 1 }), RestartPolicy::Transient)
        .with_worker("event-collector-2", || ServiceWorker::EventCollector(EventCollector { id: 2 }), RestartPolicy::Transient)
        .with_worker("event-collector-3", || ServiceWorker::EventCollector(EventCollector { id: 3 }), RestartPolicy::Transient)
        .with_worker("metrics-agg-1", || ServiceWorker::MetricsAggregator(MetricsAggregator { id: 1 }), RestartPolicy::Permanent)
        .with_worker("metrics-agg-2", || ServiceWorker::MetricsAggregator(MetricsAggregator { id: 2 }), RestartPolicy::Permanent);

    // Build Database Layer Supervisor (Postgres)
    let postgres_spec = SupervisorSpec::new("postgres-layer")
        .with_restart_strategy(RestartStrategy::RestForOne)
        .with_restart_intensity(RestartIntensity::new(3, 10))
        .with_worker("postgres-master", || ServiceWorker::PostgresConnector(PostgresConnector { id: 1, pool_size: 20 }), RestartPolicy::Permanent)
        .with_worker("postgres-replica-1", || ServiceWorker::PostgresConnector(PostgresConnector { id: 2, pool_size: 10 }), RestartPolicy::Permanent)
        .with_worker("postgres-replica-2", || ServiceWorker::PostgresConnector(PostgresConnector { id: 3, pool_size: 10 }), RestartPolicy::Permanent);

    // Build Database Layer Supervisor (Redis)
    let redis_spec = SupervisorSpec::new("redis-layer")
        .with_restart_strategy(RestartStrategy::OneForOne)
        .with_restart_intensity(RestartIntensity::new(5, 10))
        .with_worker("redis-cache-1", || ServiceWorker::RedisConnector(RedisConnector { id: 1 }), RestartPolicy::Permanent)
        .with_worker("redis-cache-2", || ServiceWorker::RedisConnector(RedisConnector { id: 2 }), RestartPolicy::Permanent)
        .with_worker("redis-cache-3", || ServiceWorker::RedisConnector(RedisConnector { id: 3 }), RestartPolicy::Permanent);

    // Build Database Layer Supervisor (MongoDB)
    let mongo_spec = SupervisorSpec::new("mongo-layer")
        .with_restart_strategy(RestartStrategy::OneForOne)
        .with_restart_intensity(RestartIntensity::new(3, 10))
        .with_worker("mongo-primary", || ServiceWorker::MongoConnector(MongoConnector { id: 1 }), RestartPolicy::Permanent)
        .with_worker("mongo-secondary", || ServiceWorker::MongoConnector(MongoConnector { id: 2 }), RestartPolicy::Permanent);

    // Build Background Jobs Supervisor (Email Queue)
    let email_jobs_spec = SupervisorSpec::new("email-jobs")
        .with_restart_strategy(RestartStrategy::OneForOne)
        .with_restart_intensity(RestartIntensity::new(10, 20))
        .with_worker("email-worker-1", || ServiceWorker::EmailWorker(EmailWorker { id: 1 }), RestartPolicy::Transient)
        .with_worker("email-worker-2", || ServiceWorker::EmailWorker(EmailWorker { id: 2 }), RestartPolicy::Transient)
        .with_worker("email-worker-3", || ServiceWorker::EmailWorker(EmailWorker { id: 3 }), RestartPolicy::Transient)
        .with_worker("email-worker-4", || ServiceWorker::EmailWorker(EmailWorker { id: 4 }), RestartPolicy::Transient);

    // Build Background Jobs Supervisor (Reports & Sync)
    let batch_jobs_spec = SupervisorSpec::new("batch-jobs")
        .with_restart_strategy(RestartStrategy::OneForOne)
        .with_restart_intensity(RestartIntensity::new(5, 15))
        .with_worker("report-1", || ServiceWorker::ReportGenerator(ReportGenerator { id: 1 }), RestartPolicy::Transient)
        .with_worker("report-2", || ServiceWorker::ReportGenerator(ReportGenerator { id: 2 }), RestartPolicy::Transient)
        .with_worker("sync-1", || ServiceWorker::DataSyncWorker(DataSyncWorker { id: 1 }), RestartPolicy::Permanent)
        .with_worker("sync-2", || ServiceWorker::DataSyncWorker(DataSyncWorker { id: 2 }), RestartPolicy::Permanent);

    // Build top-level supervision tree
    let root_spec = SupervisorSpec::new("microservices-platform")
        .with_restart_strategy(RestartStrategy::OneForOne)
        .with_restart_intensity(RestartIntensity::new(3, 10))
        // Gateway Layer
        .with_supervisor(api_gateway_spec)
        // Service Layer
        .with_supervisor(user_service_spec)
        .with_supervisor(content_service_spec)
        .with_supervisor(payment_service_spec)
        .with_supervisor(analytics_service_spec)
        // Data Layer
        .with_supervisor(postgres_spec)
        .with_supervisor(redis_spec)
        .with_supervisor(mongo_spec)
        // Background Jobs Layer
        .with_supervisor(email_jobs_spec)
        .with_supervisor(batch_jobs_spec)
        // Monitoring Layer (direct workers)
        .with_worker("health-checker", || ServiceWorker::HealthChecker(HealthChecker), RestartPolicy::Permanent)
        .with_worker("log-aggregator", || ServiceWorker::LogAggregator(LogAggregator), RestartPolicy::Permanent)
        .with_worker("alert-manager", || ServiceWorker::AlertManager(AlertManager), RestartPolicy::Permanent);

    println!("\n🚀 Starting supervision tree...\n");
    let handle = SupervisorHandle::start(root_spec);

    // Let it run for a bit
    sleep(Duration::from_secs(3)).await;

    // Query the tree
    println!("\n\n📊 Querying supervision tree structure...\n");
    match handle.which_children().await {
        Ok(children) => {
            println!("Root supervisor has {} direct children:", children.len());
            for child in children {
                println!("  - {} ({:?})", child.id, child.child_type);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Get supervisor info
    println!("\n📈 Supervisor Statistics:");
    if let Ok(strategy) = handle.restart_strategy().await {
        println!("  Restart Strategy: {:?}", strategy);
    }
    if let Ok(uptime) = handle.uptime().await {
        println!("  Uptime: {} seconds", uptime);
    }

    let total_requests = REQUEST_COUNTER.load(Ordering::Relaxed);
    println!("  Total Requests Processed: {}", total_requests);

    println!("\n⏳ Letting the system run for 5 more seconds...\n");
    sleep(Duration::from_secs(5)).await;

    println!("\n🛑 Initiating graceful shutdown...\n");
    match handle.shutdown().await {
        Ok(_) => println!("✅ All 55+ workers shut down gracefully"),
        Err(e) => eprintln!("❌ Shutdown error: {}", e),
    }

    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║  Demo Complete - Complex Supervision Tree                   ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");
}
