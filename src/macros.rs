//! Convenience macros for building supervision trees

/// Implement the Worker trait with minimal boilerplate for the run method only
///
/// # Examples
///
/// ```ignore
/// use ash_flare::impl_worker;
///
/// struct MyWorker;
///
/// impl_worker! {
///     MyWorker, std::io::Error => {
///         loop {
///             // do work
///             tokio::time::sleep(Duration::from_secs(1)).await;
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! impl_worker {
    ($worker:ty, $error:ty => $run_body:block) => {
        #[async_trait::async_trait]
        impl $crate::Worker for $worker {
            type Error = $error;

            async fn run(&mut self) -> Result<(), Self::Error> $run_body
        }
    };
}

/// Build a supervision tree with a declarative syntax
///
/// # Examples
///
/// ```ignore
/// use ash_flare::supervision_tree;
///
/// let spec = supervision_tree! {
///     name: "app",
///     strategy: OneForOne,
///     intensity: (5, 10), // max_restarts, within_seconds
///     workers: [
///         ("worker-1", || MyWorker::new(), Permanent),
///         ("worker-2", || MyWorker::new(), Transient),
///     ],
///     supervisors: []
/// };
/// ```
#[macro_export]
macro_rules! supervision_tree {
    (
        name: $name:expr,
        strategy: $strategy:ident,
        intensity: ($max:expr, $secs:expr),
        workers: [ $(($id:expr, $factory:expr, $policy:ident)),* $(,)? ],
        supervisors: [ $($sup:expr),* $(,)? ]
    ) => {
        {
            let spec = $crate::SupervisorSpec::new($name)
                .with_restart_strategy($crate::RestartStrategy::$strategy)
                .with_restart_intensity($crate::RestartIntensity {
                    max_restarts: $max,
                    within_seconds: $secs,
                })
                $(
                    .with_worker($id, $factory, $crate::RestartPolicy::$policy)
                )*
                $(
                    .with_supervisor($sup)
                )*;
            spec
        }
    };
    (
        name: $name:expr,
        strategy: $strategy:ident,
        workers: [ $(($id:expr, $factory:expr, $policy:ident)),* $(,)? ],
        supervisors: [ $($sup:expr),* $(,)? ]
    ) => {
        {
            let spec = $crate::SupervisorSpec::new($name)
                .with_restart_strategy($crate::RestartStrategy::$strategy)
                $(
                    .with_worker($id, $factory, $crate::RestartPolicy::$policy)
                )*
                $(
                    .with_supervisor($sup)
                )*;
            spec
        }
    };
}

/// Start a distributed supervisor server with TCP or Unix socket
///
/// # Examples
///
/// ```ignore
/// use ash_flare::serve_supervisor;
///
/// // TCP server
/// serve_supervisor!(tcp, handle, "127.0.0.1:8080");
///
/// // Unix socket server
/// serve_supervisor!(unix, handle, "/tmp/supervisor.sock");
/// ```
#[macro_export]
macro_rules! serve_supervisor {
    (tcp, $handle:expr, $addr:expr) => {{
        let server = $crate::distributed::SupervisorServer::new($handle);
        tokio::spawn(async move { server.listen_tcp($addr).await })
    }};
    (unix, $handle:expr, $path:expr) => {{
        let server = $crate::distributed::SupervisorServer::new($handle);
        tokio::spawn(async move { server.listen_unix($path).await })
    }};
}

/// Connect to a remote supervisor via TCP or Unix socket
///
/// # Examples
///
/// ```ignore
/// use ash_flare::connect_supervisor;
///
/// // Connect via TCP
/// let remote = connect_supervisor!(tcp, "127.0.0.1:8080").await?;
///
/// // Connect via Unix socket
/// let remote = connect_supervisor!(unix, "/tmp/supervisor.sock").await?;
/// ```
#[macro_export]
macro_rules! connect_supervisor {
    (tcp, $addr:expr) => {
        $crate::distributed::RemoteSupervisorHandle::connect_tcp($addr)
    };
    (unix, $path:expr) => {
        $crate::distributed::RemoteSupervisorHandle::connect_unix($path)
    };
}

/// Create and start a distributed supervision system with server and client
///
/// # Examples
///
/// ```ignore
/// use ash_flare::distributed_system;
///
/// distributed_system! {
///     server: tcp @ "127.0.0.1:8080" => {
///         name: "remote-app",
///         strategy: OneForOne,
///         workers: [
///             ("worker-1", || MyWorker::new(), Permanent),
///         ],
///         supervisors: []
///     }
/// }
/// ```
#[macro_export]
macro_rules! distributed_system {
    (
        server: tcp @ $addr:expr => {
            name: $name:expr,
            strategy: $strategy:ident,
            workers: [ $(($id:expr, $factory:expr, $policy:ident)),* $(,)? ],
            supervisors: [ $($sup:expr),* $(,)? ]
        }
    ) => {
        {
            let spec = supervision_tree! {
                name: $name,
                strategy: $strategy,
                workers: [ $(($id, $factory, $policy)),* ],
                supervisors: [ $($sup),* ]
            };
            let handle = $crate::SupervisorHandle::start(spec);
            let server = $crate::distributed::SupervisorServer::new(handle.clone());
            let server_task = tokio::spawn(async move {
                server.listen_tcp($addr).await
            });
            (handle, server_task)
        }
    };
    (
        server: unix @ $path:expr => {
            name: $name:expr,
            strategy: $strategy:ident,
            workers: [ $(($id:expr, $factory:expr, $policy:ident)),* $(,)? ],
            supervisors: [ $($sup:expr),* $(,)? ]
        }
    ) => {
        {
            let spec = supervision_tree! {
                name: $name,
                strategy: $strategy,
                workers: [ $(($id, $factory, $policy)),* ],
                supervisors: [ $($sup),* ]
            };
            let handle = $crate::SupervisorHandle::start(spec);
            let server = $crate::distributed::SupervisorServer::new(handle.clone());
            let server_task = tokio::spawn(async move {
                server.listen_unix($path).await
            });
            (handle, server_task)
        }
    };
}
