//! Distributed supervision via TCP/Unix sockets
//! 
//! Allows supervisors to run in separate processes (local) or on remote machines (network).
//! Commands are serialized with bincode for minimal overhead.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream, UnixListener, UnixStream};

use crate::{ChildInfo as SupervisorChildInfo, ChildType, RestartPolicy, SupervisorHandle, Worker};

/// Remote supervisor address
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SupervisorAddress {
    /// TCP socket address (host:port)
    Tcp(String),
    /// Unix domain socket path
    Unix(String),
}

/// Commands that can be sent to a remote supervisor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RemoteCommand {
    /// Request supervisor to shut down gracefully
    Shutdown,
    /// Get list of all children
    WhichChildren,
    /// Terminate a specific child
    TerminateChild { id: String },
    /// Get supervisor status
    Status,
}

/// Responses from remote supervisor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RemoteResponse {
    /// Command executed successfully
    Ok,
    /// List of child IDs
    Children(Vec<ChildInfo>),
    /// Supervisor status information
    Status(SupervisorStatus),
    /// Error occurred
    Error(String),
}

/// Information about a child process (simplified for serialization)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChildInfo {
    pub id: String,
    pub child_type: ChildType,
    pub restart_policy: Option<RestartPolicy>,
}

impl From<SupervisorChildInfo> for ChildInfo {
    fn from(info: SupervisorChildInfo) -> Self {
        Self {
            id: info.id,
            child_type: info.child_type,
            restart_policy: info.restart_policy,
        }
    }
}

/// Status information about a supervisor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupervisorStatus {
    pub name: String,
    pub children_count: usize,
    pub restart_strategy: String,
    pub uptime_secs: u64,
}

/// Handle to communicate with a remote supervisor
pub struct RemoteSupervisorHandle {
    address: SupervisorAddress,
}

impl RemoteSupervisorHandle {
    /// Create a new handle to a remote supervisor
    pub fn new(address: SupervisorAddress) -> Self {
        Self { address }
    }

    /// Connect to a TCP supervisor
    pub async fn connect_tcp(addr: impl Into<String>) -> Result<Self, DistributedError> {
        let address = SupervisorAddress::Tcp(addr.into());
        Ok(Self { address })
    }

    /// Connect to a Unix socket supervisor
    pub async fn connect_unix(path: impl Into<String>) -> Result<Self, DistributedError> {
        let address = SupervisorAddress::Unix(path.into());
        Ok(Self { address })
    }

    /// Send a command to the remote supervisor
    pub async fn send_command(&self, cmd: RemoteCommand) -> Result<RemoteResponse, DistributedError> {
        match &self.address {
            SupervisorAddress::Tcp(addr) => {
                let mut stream = TcpStream::connect(addr).await?;
                send_message(&mut stream, &cmd).await?;
                receive_message(&mut stream).await
            }
            SupervisorAddress::Unix(path) => {
                let mut stream = UnixStream::connect(path).await?;
                send_message(&mut stream, &cmd).await?;
                receive_message(&mut stream).await
            }
        }
    }

    /// Shutdown the remote supervisor
    pub async fn shutdown(&self) -> Result<(), DistributedError> {
        self.send_command(RemoteCommand::Shutdown).await?;
        Ok(())
    }

    /// Get list of children from remote supervisor
    pub async fn which_children(&self) -> Result<Vec<ChildInfo>, DistributedError> {
        match self.send_command(RemoteCommand::WhichChildren).await? {
            RemoteResponse::Children(children) => Ok(children),
            RemoteResponse::Error(e) => Err(DistributedError::RemoteError(e)),
            _ => Err(DistributedError::UnexpectedResponse),
        }
    }

    /// Terminate a child on the remote supervisor
    pub async fn terminate_child(&self, id: &str) -> Result<(), DistributedError> {
        match self.send_command(RemoteCommand::TerminateChild { id: id.to_string() }).await? {
            RemoteResponse::Ok => Ok(()),
            RemoteResponse::Error(e) => Err(DistributedError::RemoteError(e)),
            _ => Err(DistributedError::UnexpectedResponse),
        }
    }

    /// Get status from remote supervisor
    pub async fn status(&self) -> Result<SupervisorStatus, DistributedError> {
        match self.send_command(RemoteCommand::Status).await? {
            RemoteResponse::Status(status) => Ok(status),
            RemoteResponse::Error(e) => Err(DistributedError::RemoteError(e)),
            _ => Err(DistributedError::UnexpectedResponse),
        }
    }
}

/// Server that wraps a SupervisorHandle and accepts remote commands
pub struct SupervisorServer<W: Worker> {
    handle: Arc<SupervisorHandle<W>>,
}

impl<W: Worker> SupervisorServer<W> {
    /// Create a new supervisor server wrapping a SupervisorHandle
    pub fn new(handle: SupervisorHandle<W>) -> Self {
        Self {
            handle: Arc::new(handle),
        }
    }

    /// Start listening on a Unix socket
    pub async fn listen_unix(self, path: impl AsRef<std::path::Path>) -> Result<(), DistributedError> {
        let path = path.as_ref();
        let _ = std::fs::remove_file(path); // Clean up old socket

        let listener = UnixListener::bind(path)?;
        println!("[Server] Listening on Unix socket: {}", path.display());

        loop {
            let (mut stream, _) = listener.accept().await?;
            let handle = Arc::clone(&self.handle);

            tokio::spawn(async move {
                if let Err(e) = Self::handle_connection(&mut stream, handle).await {
                    eprintln!("[Server] Connection error: {}", e);
                }
            });
        }
    }

    /// Start listening on a TCP socket
    pub async fn listen_tcp(self, addr: impl AsRef<str>) -> Result<(), DistributedError> {
        let listener = TcpListener::bind(addr.as_ref()).await?;
        println!("[Server] Listening on TCP: {}", addr.as_ref());

        loop {
            let (mut stream, peer) = listener.accept().await?;
            println!("[Server] Connection from: {:?}", peer);
            let handle = Arc::clone(&self.handle);

            tokio::spawn(async move {
                if let Err(e) = Self::handle_connection(&mut stream, handle).await {
                    eprintln!("[Server] Connection error: {}", e);
                }
            });
        }
    }

    async fn handle_connection<S>(
        stream: &mut S,
        handle: Arc<SupervisorHandle<W>>,
    ) -> Result<(), DistributedError>
    where
        S: AsyncReadExt + AsyncWriteExt + Unpin,
    {
        let command: RemoteCommand = receive_message(stream).await?;
        let response = Self::process_command(command, &handle).await;
        send_message(stream, &response).await?;
        Ok(())
    }

    async fn process_command(
        command: RemoteCommand,
        handle: &SupervisorHandle<W>,
    ) -> RemoteResponse {
        match command {
            RemoteCommand::Shutdown => {
                match handle.shutdown().await {
                    Ok(()) => RemoteResponse::Ok,
                    Err(e) => RemoteResponse::Error(e.to_string()),
                }
            }
            RemoteCommand::WhichChildren => {
                match handle.which_children().await {
                    Ok(children) => {
                        let children: Vec<ChildInfo> = children.into_iter().map(Into::into).collect();
                        RemoteResponse::Children(children)
                    }
                    Err(e) => RemoteResponse::Error(e.to_string()),
                }
            }
            RemoteCommand::TerminateChild { id } => {
                match handle.terminate_child(&id).await {
                    Ok(()) => RemoteResponse::Ok,
                    Err(e) => RemoteResponse::Error(e.to_string()),
                }
            }
            RemoteCommand::Status => {
                RemoteResponse::Status(SupervisorStatus {
                    name: handle.name().to_string(),
                    children_count: handle.which_children().await.map(|c| c.len()).unwrap_or(0),
                    restart_strategy: "OneForOne".to_string(), // TODO: expose from handle
                    uptime_secs: 0, // TODO: track uptime
                })
            }
        }
    }
}

/// Send a message over a stream (length-prefixed bincode)
async fn send_message<S, T>(stream: &mut S, msg: &T) -> Result<(), DistributedError>
where
    S: AsyncWriteExt + Unpin,
    T: Serialize,
{
    let encoded = bincode::serialize(msg)?;
    let len = encoded.len() as u32;
    
    stream.write_all(&len.to_be_bytes()).await?;
    stream.write_all(&encoded).await?;
    stream.flush().await?;
    
    Ok(())
}

/// Receive a message from a stream (length-prefixed bincode)
async fn receive_message<S, T>(stream: &mut S) -> Result<T, DistributedError>
where
    S: AsyncReadExt + Unpin,
    T: for<'de> Deserialize<'de>,
{
    let mut len_bytes = [0u8; 4];
    stream.read_exact(&mut len_bytes).await?;
    let len = u32::from_be_bytes(len_bytes) as usize;
    
    if len > 10_000_000 {
        return Err(DistributedError::MessageTooLarge(len));
    }
    
    let mut buffer = vec![0u8; len];
    stream.read_exact(&mut buffer).await?;
    
    let decoded = bincode::deserialize(&buffer)?;
    Ok(decoded)
}

/// Errors that can occur in distributed operations
#[derive(Debug)]
pub enum DistributedError {
    Io(std::io::Error),
    Serialization(bincode::Error),
    RemoteError(String),
    UnexpectedResponse,
    MessageTooLarge(usize),
}

impl fmt::Display for DistributedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DistributedError::Io(e) => write!(f, "IO error: {}", e),
            DistributedError::Serialization(e) => write!(f, "Serialization error: {}", e),
            DistributedError::RemoteError(e) => write!(f, "Remote error: {}", e),
            DistributedError::UnexpectedResponse => write!(f, "Unexpected response from remote"),
            DistributedError::MessageTooLarge(size) => write!(f, "Message too large: {} bytes", size),
        }
    }
}

impl std::error::Error for DistributedError {}

impl From<std::io::Error> for DistributedError {
    fn from(e: std::io::Error) -> Self {
        DistributedError::Io(e)
    }
}

impl From<bincode::Error> for DistributedError {
    fn from(e: bincode::Error) -> Self {
        DistributedError::Serialization(e)
    }
}
