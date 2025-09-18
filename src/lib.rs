// lib.rs - Rust EtherNet/IP Driver Library with Port Routing Support
// =========================================================================
//
// # Rust EtherNet/IP Driver Library v0.5.3 + Port Routing
//
// A high-performance, production-ready EtherNet/IP communication library for
// Allen-Bradley CompactLogix and ControlLogix PLCs, written in pure Rust with
// comprehensive language bindings (C#, Python, Go, JavaScript/TypeScript).
// Enhanced with port routing support for connecting to processors in different slots.
//
// ## Port Routing Features
//
// - **PortSegment Support**: Connect to processors in specific slots via backplane
// - **Runtime Configuration**: Change connection paths during execution
// - **Allen-Bradley Compatible**: CIP-compliant Unconnected Send routing
// - **Zero Breaking Changes**: Existing code continues to work unchanged

use crate::udt::UdtManager;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;
use tokio::time::{timeout, Duration, Instant};

pub mod config; // Production-ready configuration management
pub mod error;
pub mod ffi;
pub mod monitoring; // Enterprise-grade monitoring and health checks
pub mod plc_manager;
#[cfg(feature = "python")]
pub mod python;
pub mod subscription;
pub mod tag_manager;
pub mod tag_path;
pub mod tag_subscription; // Real-time subscription management
pub mod udt;
pub mod version;

// Re-export commonly used items
pub use config::{
    ConnectionConfig, LoggingConfig, MonitoringConfig, PerformanceConfig, PlcSpecificConfig,
    ProductionConfig, SecurityConfig,
};
pub use error::{EtherNetIpError, Result};
pub use monitoring::{
    ConnectionMetrics, ErrorMetrics, HealthMetrics, HealthStatus, MonitoringMetrics,
    OperationMetrics, PerformanceMetrics, ProductionMonitor,
};
pub use plc_manager::{PlcConfig, PlcConnection, PlcManager};
pub use subscription::{SubscriptionManager, SubscriptionOptions, TagSubscription};
pub use tag_manager::{TagCache, TagManager, TagMetadata, TagPermissions, TagScope};
pub use tag_path::TagPath;
pub use tag_subscription::{
    SubscriptionManager as RealTimeSubscriptionManager,
    SubscriptionOptions as RealTimeSubscriptionOptions, TagSubscription as RealTimeSubscription,
};
pub use udt::{UdtDefinition, UdtMember};

// Static runtime and client management for FFI
lazy_static! {
    /// Global Tokio runtime for handling async operations in FFI context
    static ref RUNTIME: Runtime = Runtime::new().unwrap();

    /// Global storage for EipClient instances, indexed by client ID
    static ref CLIENTS: Mutex<HashMap<i32, EipClient>> = Mutex::new(HashMap::new());

    /// Counter for generating unique client IDs
    static ref NEXT_ID: Mutex<i32> = Mutex::new(1);
}

// =========================================================================
// PORT ROUTING IMPLEMENTATION
// =========================================================================

/// Port routing segment for connecting to PLCs in different slots
///
/// This structure represents a CIP path segment that specifies how to route
/// messages through Allen-Bradley backplanes to reach processors in specific slots.
/// Essential for configurations where network cards and processors are in different slots.
///
/// # Examples
///
/// ```rust
/// use rust_ethernet_ip::PortSegment;
///
/// // Connect to processor in slot 1 (most common)
/// let routing = PortSegment::processor_in_slot(1);
///
/// // Connect to processor in slot 3 via port 2
/// let routing = PortSegment::new(2, vec![3]);
///
/// // Multi-hop routing (advanced configurations)
/// let routing = PortSegment::new(1, vec![0, 2, 5]);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortSegment {
    /// Port number (typically 1 for backplane, 2 for Ethernet)
    pub port: u16,
    /// Link address path (slot numbers to traverse)
    pub link: Vec<u8>,
}

impl Default for PortSegment {
    fn default() -> Self {
        Self::processor_in_slot(1)
    }
}

impl PortSegment {
    /// Creates a new port segment with specified port and link path
    ///
    /// # Arguments
    ///
    /// * `port` - Port number (1 = backplane, 2 = Ethernet)
    /// * `link` - Vector of slot numbers to traverse
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_ethernet_ip::PortSegment;
    ///
    /// // Simple routing to slot 1
    /// let segment = PortSegment::new(1, vec![1]);
    ///
    /// // Multi-hop routing through multiple slots
    /// let segment = PortSegment::new(1, vec![0, 2, 5]);
    /// ```
    pub fn new(port: u16, link: Vec<u8>) -> Self {
        Self { port, link }
    }

    /// Creates routing for a processor in a specific slot (most common use case)
    ///
    /// This is the standard constructor for connecting to CompactLogix processors
    /// where the network card is in slot 0 and the processor is in another slot.
    ///
    /// # Arguments
    ///
    /// * `slot` - Slot number of the target processor (typically 1-16)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_ethernet_ip::PortSegment;
    ///
    /// // Connect to L72 processor in slot 1 (most common)
    /// let routing = PortSegment::processor_in_slot(1);
    ///
    /// // Connect to processor in slot 3
    /// let routing = PortSegment::processor_in_slot(3);
    /// ```
    pub fn processor_in_slot(slot: u8) -> Self {
        Self {
            port: 1,        // Backplane port
            link: vec![slot], // Target slot
        }
    }

    /// Creates routing for Ethernet-based communication (port 2)
    ///
    /// Used for connecting through Ethernet modules or bridges.
    ///
    /// # Arguments
    ///
    /// * `link` - Vector of addresses to traverse
    pub fn ethernet_routing(link: Vec<u8>) -> Self {
        Self { port: 2, link }
    }

    /// Creates direct backplane routing (port 1, slot 0)
    ///
    /// Used for connecting directly to the network card's processor
    /// or when no routing is needed.
    pub fn direct() -> Self {
        Self {
            port: 1,
            link: vec![0],
        }
    }

    /// Encodes the port segment for CIP transmission
    ///
    /// Converts the port segment into the binary format required by
    /// the CIP Unconnected Send service for routing messages.
    ///
    /// # Returns
    ///
    /// Vector of bytes representing the encoded port segment
    pub fn encode(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(2 + self.link.len());
        
        // Port segment format:
        // [0x01] = Port segment identifier
        // [port] = Port number (1 byte, even though port is u16)
        // [link_size] = Number of link addresses
        // [link_addresses...] = Link path addresses
        
        bytes.push(0x01); // Port segment identifier
        bytes.push(self.port as u8); // Port number (typically 1 for backplane)
        bytes.push(self.link.len() as u8); // Link path size
        bytes.extend_from_slice(&self.link); // Link addresses
        
        // Pad to even length for CIP word alignment
        if bytes.len() % 2 != 0 {
            bytes.push(0x00);
        }
        
        bytes
    }

    /// Gets a human-readable description of this port segment
    ///
    /// # Returns
    ///
    /// String describing the routing path
    pub fn description(&self) -> String {
        if self.link.len() == 1 && self.port == 1 {
            format!("Processor in slot {}", self.link[0])
        } else if self.port == 2 {
            format!("Ethernet routing via {:?}", self.link)
        } else {
            format!("Port {} routing via {:?}", self.port, self.link)
        }
    }

    /// Validates that this port segment is reasonable
    ///
    /// # Returns
    ///
    /// `Ok(())` if valid, `Err` with description if invalid
    pub fn validate(&self) -> std::result::Result<(), String> {
        if self.link.is_empty() {
            return Err("Link path cannot be empty".to_string());
        }
        
        if self.link.len() > 8 {
            return Err("Link path too long (max 8 hops)".to_string());
        }
        
        if self.port == 0 || self.port > 255 {
            return Err("Port number must be 1-255".to_string());
        }
        
        for &slot in &self.link {
            if slot > 31 {
                return Err(format!("Slot number {} too high (max 31)", slot));
            }
        }
        
        Ok(())
    }
}

// =========================================================================
// BATCH OPERATIONS DATA STRUCTURES  
// =========================================================================

/// Represents a single operation in a batch request
#[derive(Debug, Clone)]
pub enum BatchOperation {
    /// Read operation for a specific tag
    Read { tag_name: String },

    /// Write operation for a specific tag with a value
    Write { tag_name: String, value: PlcValue },
}

/// Result of a single operation in a batch request
#[derive(Debug, Clone)]
pub struct BatchResult {
    /// The original operation that was executed
    pub operation: BatchOperation,

    /// The result of the operation
    pub result: std::result::Result<Option<PlcValue>, BatchError>,

    /// Execution time for this specific operation (in microseconds)
    pub execution_time_us: u64,
}

/// Specific error types that can occur during batch operations
#[derive(Debug, Clone)]
pub enum BatchError {
    /// Tag was not found in the PLC
    TagNotFound(String),

    /// Data type mismatch between expected and actual
    DataTypeMismatch { expected: String, actual: String },

    /// Network communication error
    NetworkError(String),

    /// CIP protocol error with status code
    CipError { status: u8, message: String },

    /// Tag name parsing error
    TagPathError(String),

    /// Value serialization/deserialization error
    SerializationError(String),

    /// Operation timeout
    Timeout,

    /// Generic error for unexpected issues
    Other(String),
}

impl std::fmt::Display for BatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BatchError::TagNotFound(tag) => write!(f, "Tag not found: {}", tag),
            BatchError::DataTypeMismatch { expected, actual } => {
                write!(
                    f,
                    "Data type mismatch: expected {}, got {}",
                    expected, actual
                )
            }
            BatchError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            BatchError::CipError { status, message } => {
                write!(f, "CIP error (0x{:02X}): {}", status, message)
            }
            BatchError::TagPathError(msg) => write!(f, "Tag path error: {}", msg),
            BatchError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            BatchError::Timeout => write!(f, "Operation timeout"),
            BatchError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for BatchError {}

/// Configuration for batch operations
#[derive(Debug, Clone)]
pub struct BatchConfig {
    /// Maximum number of operations to include in a single CIP packet
    pub max_operations_per_packet: usize,

    /// Maximum packet size in bytes for batch operations
    pub max_packet_size: usize,

    /// Timeout for individual batch packets (in milliseconds)
    pub packet_timeout_ms: u64,

    /// Whether to continue processing other operations if one fails
    pub continue_on_error: bool,

    /// Whether to optimize packet packing by grouping similar operations
    pub optimize_packet_packing: bool,
}

impl Default for BatchConfig {
    fn default() -> Self {
        Self {
            max_operations_per_packet: 20,
            max_packet_size: 504, // Conservative default for maximum compatibility
            packet_timeout_ms: 3000,
            continue_on_error: true,
            optimize_packet_packing: true,
        }
    }
}

/// Connected session information for Class 3 explicit messaging
#[derive(Debug, Clone)]
pub struct ConnectedSession {
    /// Connection ID assigned by the PLC
    pub connection_id: u32,

    /// Our connection ID (originator -> target)
    pub o_to_t_connection_id: u32,

    /// PLC's connection ID (target -> originator)
    pub t_to_o_connection_id: u32,

    /// Connection serial number for this session
    pub connection_serial: u16,

    /// Originator vendor ID (our vendor ID)
    pub originator_vendor_id: u16,

    /// Originator serial number (our serial number)
    pub originator_serial: u32,

    /// Connection timeout multiplier
    pub timeout_multiplier: u8,

    /// Requested Packet Interval (RPI) in microseconds
    pub rpi: u32,

    /// Connection parameters for O->T direction
    pub o_to_t_params: ConnectionParameters,

    /// Connection parameters for T->O direction
    pub t_to_o_params: ConnectionParameters,

    /// Timestamp when connection was established
    pub established_at: Instant,

    /// Whether this connection is currently active
    pub is_active: bool,

    /// Sequence counter for connected messages (increments with each message)
    pub sequence_count: u16,
}

/// Connection parameters for EtherNet/IP connections
#[derive(Debug, Clone)]
pub struct ConnectionParameters {
    /// Connection size in bytes
    pub size: u16,

    /// Connection type (0x02 = Point-to-point, 0x01 = Multicast)
    pub connection_type: u8,

    /// Priority (0x00 = Low, 0x01 = High, 0x02 = Scheduled, 0x03 = Urgent)
    pub priority: u8,

    /// Variable size flag
    pub variable_size: bool,
}

impl Default for ConnectionParameters {
    fn default() -> Self {
        Self {
            size: 500,             // 500 bytes default
            connection_type: 0x02, // Point-to-point
            priority: 0x01,        // High priority
            variable_size: false,
        }
    }
}

impl ConnectedSession {
    /// Creates a new connected session with default parameters
    pub fn new(connection_serial: u16) -> Self {
        Self {
            connection_id: 0,
            o_to_t_connection_id: 0,
            t_to_o_connection_id: 0,
            connection_serial,
            originator_vendor_id: 0x1337,  // Custom vendor ID
            originator_serial: 0x12345678, // Custom serial number
            timeout_multiplier: 0x05,      // 32 seconds timeout
            rpi: 100000,                   // 100ms RPI
            o_to_t_params: ConnectionParameters::default(),
            t_to_o_params: ConnectionParameters::default(),
            established_at: Instant::now(),
            is_active: false,
            sequence_count: 0,
        }
    }

    /// Creates a connected session with alternative parameters for different PLCs
    pub fn with_config(connection_serial: u16, config_id: u8) -> Self {
        let mut session = Self::new(connection_serial);

        match config_id {
            1 => {
                // Config 1: Conservative Allen-Bradley parameters
                session.timeout_multiplier = 0x07; // 256 seconds timeout
                session.rpi = 200000; // 200ms RPI (slower)
                session.o_to_t_params.size = 504; // Standard packet size
                session.t_to_o_params.size = 504;
                session.o_to_t_params.priority = 0x00; // Low priority
                session.t_to_o_params.priority = 0x00;
            }
            2 => {
                // Config 2: Compact parameters
                session.timeout_multiplier = 0x03; // 8 seconds timeout
                session.rpi = 50000; // 50ms RPI (faster)
                session.o_to_t_params.size = 256; // Smaller packet size
                session.t_to_o_params.size = 256;
                session.o_to_t_params.priority = 0x02; // Scheduled priority
                session.t_to_o_params.priority = 0x02;
            }
            3 => {
                // Config 3: Minimal parameters
                session.timeout_multiplier = 0x01; // 4 seconds timeout
                session.rpi = 1000000; // 1000ms RPI (very slow)
                session.o_to_t_params.size = 128; // Very small packets
                session.t_to_o_params.size = 128;
                session.o_to_t_params.priority = 0x03; // Urgent priority
                session.t_to_o_params.priority = 0x03;
            }
            4 => {
                // Config 4: Standard Rockwell parameters (from documentation)
                session.timeout_multiplier = 0x05; // 32 seconds timeout
                session.rpi = 100000; // 100ms RPI
                session.o_to_t_params.size = 500; // Standard size
                session.t_to_o_params.size = 500;
                session.o_to_t_params.connection_type = 0x01; // Multicast
                session.t_to_o_params.connection_type = 0x01;
                session.originator_vendor_id = 0x001D; // Rockwell vendor ID
            }
            5 => {
                // Config 5: Large buffer parameters
                session.timeout_multiplier = 0x0A; // Very long timeout
                session.rpi = 500000; // 500ms RPI
                session.o_to_t_params.size = 1024; // Large packets
                session.t_to_o_params.size = 1024;
                session.o_to_t_params.variable_size = true; // Variable size
                session.t_to_o_params.variable_size = true;
            }
            _ => {
                // Default config
            }
        }

        session
    }
}

/// Represents the different data types supported by Allen-Bradley PLCs
#[derive(Debug, Clone, PartialEq)]
pub enum PlcValue {
    /// Boolean value (single bit)
    Bool(bool),
    /// 8-bit signed integer (-128 to 127)
    Sint(i8),
    /// 16-bit signed integer (-32,768 to 32,767)
    Int(i16),
    /// 32-bit signed integer (-2,147,483,648 to 2,147,483,647)
    Dint(i32),
    /// 64-bit signed integer
    Lint(i64),
    /// 8-bit unsigned integer (0 to 255)
    Usint(u8),
    /// 16-bit unsigned integer (0 to 65,535)
    Uint(u16),
    /// 32-bit unsigned integer (0 to 4,294,967,295)
    Udint(u32),
    /// 64-bit unsigned integer
    Ulint(u64),
    /// 32-bit IEEE 754 floating point number
    Real(f32),
    /// 64-bit IEEE 754 floating point number
    Lreal(f64),
    /// String value
    String(String),
    /// User Defined Type instance
    Udt(HashMap<String, PlcValue>),
}

impl PlcValue {
    /// Converts the PLC value to its byte representation for network transmission
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            PlcValue::Bool(val) => vec![if *val { 0xFF } else { 0x00 }],
            PlcValue::Sint(val) => val.to_le_bytes().to_vec(),
            PlcValue::Int(val) => val.to_le_bytes().to_vec(),
            PlcValue::Dint(val) => val.to_le_bytes().to_vec(),
            PlcValue::Lint(val) => val.to_le_bytes().to_vec(),
            PlcValue::Usint(val) => val.to_le_bytes().to_vec(),
            PlcValue::Uint(val) => val.to_le_bytes().to_vec(),
            PlcValue::Udint(val) => val.to_le_bytes().to_vec(),
            PlcValue::Ulint(val) => val.to_le_bytes().to_vec(),
            PlcValue::Real(val) => val.to_le_bytes().to_vec(),
            PlcValue::Lreal(val) => val.to_le_bytes().to_vec(),
            PlcValue::String(val) => {
                let mut bytes = Vec::new();
                let length = val.len().min(82) as u32;
                bytes.extend_from_slice(&length.to_le_bytes());
                let string_bytes = val.as_bytes();
                let data_len = string_bytes.len().min(82);
                bytes.extend_from_slice(&string_bytes[..data_len]);
                bytes
            }
            PlcValue::Udt(_) => {
                // UDT serialization is handled by the UdtManager
                vec![]
            }
        }
    }

    /// Returns the CIP data type code for this value
    pub fn get_data_type(&self) -> u16 {
        match self {
            PlcValue::Bool(_) => 0x00C1,   // BOOL
            PlcValue::Sint(_) => 0x00C2,   // SINT (signed char)
            PlcValue::Int(_) => 0x00C3,    // INT (short)
            PlcValue::Dint(_) => 0x00C4,   // DINT (int)
            PlcValue::Lint(_) => 0x00C5,   // LINT (long long)
            PlcValue::Usint(_) => 0x00C6,  // USINT (unsigned char)
            PlcValue::Uint(_) => 0x00C7,   // UINT (unsigned short)
            PlcValue::Udint(_) => 0x00C8,  // UDINT (unsigned int)
            PlcValue::Ulint(_) => 0x00C9,  // ULINT (unsigned long long)
            PlcValue::Real(_) => 0x00CA,   // REAL (float)
            PlcValue::Lreal(_) => 0x00CB,  // LREAL (double)
            PlcValue::String(_) => 0x02A0, // Allen-Bradley STRING type (matches PLC read responses)
            PlcValue::Udt(_) => 0x00A0,    // UDT placeholder
        }
    }
}

/// High-performance EtherNet/IP client for PLC communication with port routing support
///
/// Enhanced version of EipClient that includes PortSegment routing capabilities
/// for connecting to processors in different slots through Allen-Bradley backplanes.
///
/// # Port Routing Examples
///
/// ```rust,no_run
/// use rust_ethernet_ip::{EipClient, PortSegment, PlcValue};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
///     // Connect to processor in slot 1 (L72 processor)
///     let mut client = EipClient::connect_with_path(
///         "192.168.1.3:44818",
///         Some(PortSegment::processor_in_slot(1))
///     ).await?;
///
///     // Read/write operations work normally - routing is automatic
///     let value = client.read_tag("Rust_Real[0]").await?;
///     client.write_tag("SetPoint", PlcValue::Dint(1500)).await?;
///
///     // Change routing at runtime if needed
///     client.set_connection_path(PortSegment::processor_in_slot(3));
///
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct EipClient {
    /// TCP stream for network communication
    stream: Arc<Mutex<TcpStream>>,
    /// Session handle for the connection
    session_handle: u32,
    /// Connection ID for the session
    _connection_id: u32,
    /// Tag manager for handling tag operations
    tag_manager: Arc<Mutex<TagManager>>,
    /// UDT manager for handling UDT operations
    udt_manager: Arc<Mutex<UdtManager>>,
    /// Whether the client is connected
    _connected: Arc<AtomicBool>,
    /// Maximum packet size for communication
    max_packet_size: u32,
    /// Last activity timestamp
    last_activity: Arc<Mutex<Instant>>,
    /// Session timeout duration
    _session_timeout: Duration,
    /// Configuration for batch operations
    batch_config: BatchConfig,
    /// Connected session management for Class 3 operations
    connected_sessions: Arc<Mutex<HashMap<String, ConnectedSession>>>,
    /// Connection sequence counter
    connection_sequence: Arc<Mutex<u32>>,
    /// Active tag subscriptions
    subscriptions: Arc<Mutex<Vec<TagSubscription>>>,
    /// PORT ROUTING: Connection path for routing messages through backplanes
    connection_path: Option<PortSegment>,
}

impl EipClient {
    /// Creates a new EipClient with direct connection (no port routing)
    ///
    /// This is the original constructor that connects directly to the PLC
    /// without any port routing. Compatible with existing code.
    ///
    /// # Arguments
    ///
    /// * `addr` - IP address and port (e.g., "192.168.1.100:44818")
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use rust_ethernet_ip::EipClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    ///     let mut client = EipClient::new("192.168.1.100:44818").await?;
    ///     // Use client...
    ///     Ok(())
    /// }
    /// ```
    pub async fn new(addr: &str) -> Result<Self> {
        let addr = addr
            .parse::<SocketAddr>()
            .map_err(|e| EtherNetIpError::Protocol(format!("Invalid address format: {}", e)))?;
        let stream = TcpStream::connect(addr).await?;
        let mut client = Self {
            stream: Arc::new(Mutex::new(stream)),
            session_handle: 0,
            _connection_id: 0,
            tag_manager: Arc::new(Mutex::new(TagManager::new())),
            udt_manager: Arc::new(Mutex::new(UdtManager::new())),
            _connected: Arc::new(AtomicBool::new(false)),
            max_packet_size: 4000,
            last_activity: Arc::new(Mutex::new(Instant::now())),
            _session_timeout: Duration::from_secs(120),
            batch_config: BatchConfig::default(),
            connected_sessions: Arc::new(Mutex::new(HashMap::new())),
            connection_sequence: Arc::new(Mutex::new(1)),
            subscriptions: Arc::new(Mutex::new(Vec::new())),
            connection_path: None, // No routing by default
        };
        client.register_session().await?;
        Ok(client)
    }

    /// Creates a new EipClient with port routing support
    ///
    /// This constructor allows specifying a connection path for routing messages
    /// through Allen-Bradley backplanes to reach processors in different slots.
    ///
    /// # Arguments
    ///
    /// * `addr` - IP address and port (e.g., "192.168.1.3:44818")
    /// * `connection_path` - Optional port routing configuration
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use rust_ethernet_ip::{EipClient, PortSegment};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    ///     // Connect to L72 processor in slot 1
    ///     let mut client = EipClient::connect_with_path(
    ///         "192.168.1.3:44818",
    ///         Some(PortSegment::processor_in_slot(1))
    ///     ).await?;
    ///     
    ///     // Now all read/write operations will be routed to slot 1
    ///     let value = client.read_tag("MyTag").await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn connect_with_path(addr: &str, connection_path: Option<PortSegment>) -> Result<Self> {
        let addr = addr
            .parse::<SocketAddr>()
            .map_err(|e| EtherNetIpError::Protocol(format!("Invalid address format: {}", e)))?;
        
        // Validate connection path if provided
        if let Some(ref path) = connection_path {
            if let Err(e) = path.validate() {
                return Err(EtherNetIpError::Protocol(format!("Invalid connection path: {}", e)));
            }
        }
        
        let stream = TcpStream::connect(addr).await?;
        let mut client = Self {
            stream: Arc::new(Mutex::new(stream)),
            session_handle: 0,
            _connection_id: 0,
            tag_manager: Arc::new(Mutex::new(TagManager::new())),
            udt_manager: Arc::new(Mutex::new(UdtManager::new())),
            _connected: Arc::new(AtomicBool::new(false)),
            max_packet_size: 4000,
            last_activity: Arc::new(Mutex::new(Instant::now())),
            _session_timeout: Duration::from_secs(120),
            batch_config: BatchConfig::default(),
            connected_sessions: Arc::new(Mutex::new(HashMap::new())),
            connection_sequence: Arc::new(Mutex::new(1)),
            subscriptions: Arc::new(Mutex::new(Vec::new())),
            connection_path, // Set the routing path
        };
        client.register_session().await?;
        
        if let Some(ref path) = client.connection_path {
            println!("🛤️ [PORT ROUTING] Connection established with routing: {}", path.description());
        }
        
        Ok(client)
    }

    /// Public async connect function for EipClient (maintains compatibility)
    pub async fn connect(addr: &str) -> Result<Self> {
        Self::new(addr).await
    }

    /// Sets the connection path for port routing
    ///
    /// Changes the routing path for all subsequent operations. This allows
    /// runtime reconfiguration of which processor slot to communicate with.
    ///
    /// # Arguments
    ///
    /// * `connection_path` - The new port routing configuration
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use rust_ethernet_ip::{EipClient, PortSegment};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    ///     let mut client = EipClient::connect("192.168.1.3:44818").await?;
    ///     
    ///     // Initially no routing - talks to network card
    ///     let value1 = client.read_tag("NetworkCardTag").await?;
    ///     
    ///     // Switch to processor in slot 1
    ///     client.set_connection_path(PortSegment::processor_in_slot(1));
    ///     let value2 = client.read_tag("ProcessorTag").await?;
    ///     
    ///     // Switch to processor in slot 3
    ///     client.set_connection_path(PortSegment::processor_in_slot(3));
    ///     let value3 = client.read_tag("OtherProcessorTag").await?;
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub fn set_connection_path(&mut self, connection_path: PortSegment) {
        if let Err(e) = connection_path.validate() {
            eprintln!("⚠️ [PORT ROUTING] Warning: Invalid connection path: {}", e);
        }
        
        println!("🛤️ [PORT ROUTING] Switching to: {}", connection_path.description());
        self.connection_path = Some(connection_path);
    }

    /// Gets the current connection path
    ///
    /// # Returns
    ///
    /// Reference to the current port routing configuration, or `None` if no routing is configured
    pub fn connection_path(&self) -> Option<&PortSegment> {
        self.connection_path.as_ref()
    }

    /// Clears the connection path (disables port routing)
    ///
    /// After calling this method, all communications will go directly to the
    /// network card without any routing.
    pub fn clear_connection_path(&mut self) {
        if self.connection_path.is_some() {
            println!("🛤️ [PORT ROUTING] Clearing connection path - switching to direct communication");
        }
        self.connection_path = None;
    }

    /// Registers an EtherNet/IP session with the PLC
    async fn register_session(&mut self) -> crate::error::Result<()> {
        println!("🔌 [DEBUG] Starting session registration...");
        let packet: [u8; 28] = [
            0x65, 0x00, // Command: Register Session (0x0065)
            0x04, 0x00, // Length: 4 bytes
            0x00, 0x00, 0x00, 0x00, // Session Handle: 0 (will be assigned)
            0x00, 0x00, 0x00, 0x00, // Status: 0
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Sender Context (8 bytes)
            0x00, 0x00, 0x00, 0x00, // Options: 0
            0x01, 0x00, // Protocol Version: 1
            0x00, 0x00, // Option Flags: 0
        ];

        println!(
            "📤 [DEBUG] Sending Register Session packet: {:02X?}",
            packet
        );
        self.stream
            .lock()
            .await
            .write_all(&packet)
            .await
            .map_err(|e| {
                println!("❌ [DEBUG] Failed to send Register Session packet: {}", e);
                EtherNetIpError::Io(e)
            })?;

        let mut buf = [0u8; 1024];
        println!("⏳ [DEBUG] Waiting for Register Session response...");
        let n = match timeout(
            Duration::from_secs(5),
            self.stream.lock().await.read(&mut buf),
        )
        .await
        {
            Ok(Ok(n)) => {
                println!("📥 [DEBUG] Received {} bytes in response", n);
                n
            }
            Ok(Err(e)) => {
                println!("❌ [DEBUG] Error reading response: {}", e);
                return Err(EtherNetIpError::Io(e));
            }
            Err(_) => {
                println!("⏰ [DEBUG] Timeout waiting for response");
                return Err(EtherNetIpError::Timeout(Duration::from_secs(5)));
            }
        };

        if n < 28 {
            println!("❌ [DEBUG] Response too short: {} bytes (expected 28)", n);
            return Err(EtherNetIpError::Protocol("Response too short".to_string()));
        }

        // Extract session handle from response
        self.session_handle = u32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]);
        println!("🔑 [DEBUG] Session handle: 0x{:08X}", self.session_handle);

        // Check status
        let status = u32::from_le_bytes([buf[8], buf[9], buf[10], buf[11]]);
        println!("📊 [DEBUG] Status code: 0x{:08X}", status);

        if status != 0 {
            println!(
                "❌ [DEBUG] Session registration failed with status: 0x{:08X}",
                status
            );
            return Err(EtherNetIpError::Protocol(format!(
                "Session registration failed with status: 0x{:08X}",
                status
            )));
        }

        println!("✅ [DEBUG] Session registration successful");
        Ok(())
    }

    /// Sets the maximum packet size for communication
    pub fn set_max_packet_size(&mut self, size: u32) {
        self.max_packet_size = size.min(4000);
    }

    /// Discovers all tags in the PLC (including hierarchical UDT members)
    pub async fn discover_tags(&mut self) -> crate::error::Result<()> {
        let response = self
            .send_cip_request(&self.build_list_tags_request())
            .await?;
        let tags = {
            let tag_manager = self.tag_manager.lock().await;
            tag_manager.parse_tag_list(&response)?
        };

        println!("[DEBUG] Initial tag discovery found {} tags", tags.len());

        // Perform recursive drill-down discovery (similar to TypeScript implementation)
        let hierarchical_tags = {
            let tag_manager = self.tag_manager.lock().await;
            tag_manager.drill_down_tags(&tags).await?
        };

        println!(
            "[DEBUG] After drill-down: {} total tags discovered",
            hierarchical_tags.len()
        );

        {
            let tag_manager = self.tag_manager.lock().await;
            let mut cache = tag_manager.cache.write().unwrap();
            for (name, metadata) in hierarchical_tags {
                cache.insert(name, metadata);
            }
        }
        Ok(())
    }

    /// Discovers UDT members for a specific structure
    pub async fn discover_udt_members(
        &mut self,
        udt_name: &str,
    ) -> crate::error::Result<Vec<(String, TagMetadata)>> {
        // Build CIP request to get UDT definition
        let cip_request = {
            let tag_manager = self.tag_manager.lock().await;
            tag_manager.build_udt_definition_request(udt_name)?
        };

        // Send the request
        let response = self.send_cip_request(&cip_request).await?;

        // Parse the UDT definition from response
        let definition = {
            let tag_manager = self.tag_manager.lock().await;
            tag_manager.parse_udt_definition_response(&response, udt_name)?
        };

        // Cache the definition
        {
            let tag_manager = self.tag_manager.lock().await;
            let mut definitions = tag_manager.udt_definitions.write().unwrap();
            definitions.insert(udt_name.to_string(), definition.clone());
        }

        // Create member metadata
        let mut members = Vec::new();
        for member in &definition.members {
            let member_name = member.name.clone();
            let full_name = format!("{}.{}", udt_name, member_name);

            let metadata = TagMetadata {
                data_type: member.data_type,
                scope: TagScope::Controller,
                permissions: TagPermissions {
                    readable: true,
                    writable: true,
                },
                is_array: false,
                dimensions: Vec::new(),
                last_access: std::time::Instant::now(),
                size: member.size,
                array_info: None,
                last_updated: std::time::Instant::now(),
            };

            members.push((full_name, metadata));
        }

        Ok(members)
    }

    /// Gets cached UDT definition
    pub async fn get_udt_definition(&self, udt_name: &str) -> Option<UdtDefinition> {
        let tag_manager = self.tag_manager.lock().await;
        tag_manager.get_udt_definition_cached(udt_name)
    }

    /// Lists all cached UDT definitions
    pub async fn list_udt_definitions(&self) -> Vec<String> {
        let tag_manager = self.tag_manager.lock().await;
        tag_manager.list_udt_definitions()
    }

    /// Discovers hierarchical tags by drilling down into structures and UDTs
    async fn discover_hierarchical_tags(
        &mut self,
        base_tags: &[(String, TagMetadata)],
    ) -> crate::error::Result<Vec<(String, TagMetadata)>> {
        let mut all_tags = Vec::new();
        let mut tag_names = std::collections::HashSet::new();

        // Add base tags first
        for (name, metadata) in base_tags {
            all_tags.push((name.clone(), metadata.clone()));
            tag_names.insert(name.clone());
        }

        // Process each tag for hierarchical discovery
        for (name, metadata) in base_tags {
            if metadata.is_structure() && !metadata.is_array {
                // This is a structure/UDT, try to discover its members
                if let Ok(members) = self.discover_udt_members(name).await {
                    for (member_name, member_metadata) in members {
                        let full_name = format!("{}.{}", name, member_name);
                        if !tag_names.contains(&full_name) {
                            all_tags.push((full_name.clone(), member_metadata.clone()));
                            tag_names.insert(full_name.clone());

                            // Recursively discover nested structures
                            if member_metadata.is_structure() && !member_metadata.is_array {
                                if let Ok(nested_members) =
                                    self.discover_udt_members(&full_name).await
                                {
                                    for (nested_name, nested_metadata) in nested_members {
                                        let nested_full_name =
                                            format!("{}.{}", full_name, nested_name);
                                        if !tag_names.contains(&nested_full_name) {
                                            all_tags
                                                .push((nested_full_name.clone(), nested_metadata));
                                            tag_names.insert(nested_full_name);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        println!(
            "[DEBUG] Discovered {} total tags (including hierarchical)",
            all_tags.len()
        );
        Ok(all_tags)
    }

    /// Gets metadata for a tag
    pub async fn get_tag_metadata(&self, tag_name: &str) -> Option<TagMetadata> {
        let tag_manager = self.tag_manager.lock().await;
        let cache = tag_manager.cache.read().unwrap();
        let result = cache.get(tag_name).cloned();
        result
    }

    /// Reads a tag value from the PLC with automatic port routing
    ///
    /// This function performs a CIP read request for the specified tag.
    /// If port routing is configured, the request is automatically wrapped
    /// in an Unconnected Send service to route through the backplane.
    ///
    /// # Arguments
    ///
    /// * `tag_name` - The name of the tag to read
    ///
    /// # Returns
    ///
    /// The tag's value as a `PlcValue` enum
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use rust_ethernet_ip::{EipClient, PortSegment, PlcValue};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    ///     // With port routing to slot 1
    ///     let mut client = EipClient::connect_with_path(
    ///         "192.168.1.3:44818",
    ///         Some(PortSegment::processor_in_slot(1))
    ///     ).await?;
    ///
    ///     // Read different data types - routing is automatic
    ///     let bool_val = client.read_tag("MotorRunning").await?;
    ///     let int_val = client.read_tag("Counter").await?;
    ///     let real_val = client.read_tag("Temperature").await?;
    ///     let array_val = client.read_tag("Rust_Real[5]").await?;
    ///
    ///     match bool_val {
    ///         PlcValue::Bool(true) => println!("Motor is running"),
    ///         PlcValue::Bool(false) => println!("Motor is stopped"),
    ///         _ => println!("Unexpected data type"),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn read_tag(&mut self, tag_name: &str) -> crate::error::Result<PlcValue> {
        self.validate_session().await?;
        
        // Check if we have metadata for this tag
        if let Some(metadata) = self.get_tag_metadata(tag_name).await {
            // Handle UDT tags
            if metadata.data_type == 0x00A0 {
                let data = self.read_tag_raw(tag_name).await?;
                return self
                    .udt_manager
                    .lock()
                    .await
                    .parse_udt_instance(tag_name, &data);
            }
        }

        // Standard tag reading with automatic routing
        let response = self
            .send_cip_request(&self.build_read_request(tag_name))
            .await?;
        let cip_data = self.extract_cip_from_response(&response)?;
        self.parse_cip_response(&cip_data)
    }

    /// Writes a value to a PLC tag with automatic port routing
    ///
    /// This method automatically determines the best communication method based on the data type
    /// and applies port routing if configured. All write operations are automatically routed
    /// through the specified connection path.
    ///
    /// # Arguments
    ///
    /// * `tag_name` - The name of the tag to write to
    /// * `value` - The value to write
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use rust_ethernet_ip::{EipClient, PortSegment, PlcValue};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    ///     // Connect with routing to processor in slot 1
    ///     let mut client = EipClient::connect_with_path(
    ///         "192.168.1.3:44818",
    ///         Some(PortSegment::processor_in_slot(1))
    ///     ).await?;
    ///
    ///     // Write different data types - routing is automatic
    ///     client.write_tag("Counter", PlcValue::Dint(42)).await?;
    ///     client.write_tag("SetPoint", PlcValue::Real(123.45)).await?;
    ///     client.write_tag("EnableFlag", PlcValue::Bool(true)).await?;
    ///     client.write_tag("Message", PlcValue::String("Hello PLC".to_string())).await?;
    ///
    ///     // Write to array element
    ///     client.write_tag("Rust_Real[0]", PlcValue::Real(99.99)).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn write_tag(&mut self, tag_name: &str, value: PlcValue) -> crate::error::Result<()> {
        println!(
            "📝 Writing '{}' to tag '{}'",
            match &value {
                PlcValue::String(s) => format!("\"{}\"", s),
                _ => format!("{:?}", value),
            },
            tag_name
        );

        // Build write request (routing will be applied automatically in send_cip_request)
        let cip_request = self.build_write_request(tag_name, &value)?;
        let response = self.send_cip_request(&cip_request).await?;

        // Check write response for errors
        let cip_response = self.extract_cip_from_response(&response)?;

        if cip_response.len() < 3 {
            return Err(EtherNetIpError::Protocol(
                "Write response too short".to_string(),
            ));
        }

        let service_reply = cip_response[0]; // Should be 0xCD (0x4D + 0x80) for Write Tag reply
        let general_status = cip_response[2]; // CIP status code

        println!(
            "🔧 [DEBUG] Write response - Service: 0x{:02X}, Status: 0x{:02X}",
            service_reply, general_status
        );

        if general_status != 0x00 {
            let error_msg = self.get_cip_error_message(general_status);
            println!(
                "❌ [WRITE] CIP Error: {} (0x{:02X})",
                error_msg, general_status
            );
            return Err(EtherNetIpError::Protocol(format!(
                "CIP Error 0x{:02X}: {}",
                general_status, error_msg
            )));
        }

        println!("✅ Write operation completed successfully");
        Ok(())
    }

    /// Builds a CIP Write Tag Service request
    fn build_write_request(
        &self,
        tag_name: &str,
        value: &PlcValue,
    ) -> crate::error::Result<Vec<u8>> {
        println!("🔧 [DEBUG] Building write request for tag: '{}'", tag_name);

        let mut cip_request = Vec::new();

        // Service: Write Tag Service (0x4D)
        cip_request.push(0x4D);

        // Request Path Size (in words)
        let tag_bytes = tag_name.as_bytes();
        let path_len = if tag_bytes.len() % 2 == 0 {
            tag_bytes.len() + 2
        } else {
            tag_bytes.len() + 3
        };
        cip_request.push((path_len / 2) as u8);

        // Request Path: ANSI Extended Symbol Segment for tag name
        cip_request.push(0x91); // ANSI Extended Symbol Segment
        cip_request.push(tag_bytes.len() as u8); // Tag name length
        cip_request.extend_from_slice(tag_bytes); // Tag name

        // Pad to even length if necessary
        if tag_bytes.len() % 2 != 0 {
            cip_request.push(0x00);
        }

        // Add data type and element count
        let data_type = value.get_data_type();
        let value_bytes = value.to_bytes();

        cip_request.extend_from_slice(&data_type.to_le_bytes()); // Data type
        cip_request.extend_from_slice(&[0x01, 0x00]); // Element count: 1
        cip_request.extend_from_slice(&value_bytes); // Value data

        println!(
            "🔧 [DEBUG] Built CIP write request ({} bytes): {:02X?}",
            cip_request.len(),
            cip_request
        );
        Ok(cip_request)
    }

    /// Builds a CIP Read Tag Service request
    fn build_read_request(&self, tag_name: &str) -> Vec<u8> {
        println!("🔧 [DEBUG] Building read request for tag: '{}'", tag_name);

        let mut cip_request = Vec::new();

        // Service: Read Tag Service (0x4C)
        cip_request.push(0x4C);

        // Build the path based on tag name format
        let path = self.build_tag_path(tag_name);

        // Request Path Size (in words)
        cip_request.push((path.len() / 2) as u8);

        // Request Path
        cip_request.extend_from_slice(&path);

        // Element count (little-endian)
        cip_request.extend_from_slice(&[0x01, 0x00]); // Read 1 element

        println!(
            "🔧 [DEBUG] Built CIP read request ({} bytes): {:02X?}",
            cip_request.len(),
            cip_request
        );

        cip_request
    }

    /// Builds the correct path for a tag name
    fn build_tag_path(&self, tag_name: &str) -> Vec<u8> {
        let mut path = Vec::new();

        if tag_name.starts_with("Program:") {
            // Handle program tags: Program:ProgramName.TagName
            let parts: Vec<&str> = tag_name.splitn(2, ':').collect();
            if parts.len() == 2 {
                let program_and_tag = parts[1];
                let program_parts: Vec<&str> = program_and_tag.splitn(2, '.').collect();

                if program_parts.len() == 2 {
                    let program_name = program_parts[0];
                    let tag_name = program_parts[1];

                    // Build path: Program segment + program name + tag segment + tag name
                    path.push(0x91); // ANSI Extended Symbol Segment
                    path.push(program_name.len() as u8);
                    path.extend_from_slice(program_name.as_bytes());

                    // Pad to even length if necessary
                    if program_name.len() % 2 != 0 {
                        path.push(0x00);
                    }

                    // Add tag segment
                    path.push(0x91); // ANSI Extended Symbol Segment
                    path.push(tag_name.len() as u8);
                    path.extend_from_slice(tag_name.as_bytes());

                    // Pad to even length if necessary
                    if tag_name.len() % 2 != 0 {
                        path.push(0x00);
                    }
                } else {
                    // Fallback to simple tag name
                    path.extend_from_slice(&self.build_simple_tag_path(tag_name));
                }
            } else {
                // Fallback to simple tag name
                path.extend_from_slice(&self.build_simple_tag_path(tag_name));
            }
        } else {
            // Handle simple tag names
            path.extend_from_slice(&self.build_simple_tag_path(tag_name));
        }

        path
    }

    /// Builds a simple tag path (no program prefix)
    fn build_simple_tag_path(&self, tag_name: &str) -> Vec<u8> {
        let mut path = Vec::new();
        path.push(0x91); // ANSI Extended Symbol Segment
        path.push(tag_name.len() as u8);
        path.extend_from_slice(tag_name.as_bytes());

        // Pad to even length if necessary
        if tag_name.len() % 2 != 0 {
            path.push(0x00);
        }

        path
    }

    pub fn build_list_tags_request(&self) -> Vec<u8> {
        println!("🔧 [DEBUG] Building list tags request");

        // Build path array for Symbol Object Class (0x6B)
        let path_array = vec![
            // Class segment: Symbol Object Class (0x6B)
            0x20, // Class segment identifier
            0x6B, // Symbol Object Class
            // Instance segment: Start at Instance 0
            0x25, // Instance segment identifier with 0x00
            0x00, 0x00, 0x00,
        ];

        // Request data: 2 Attributes - Attribute 1 and Attribute 2
        let request_data = vec![0x02, 0x00, 0x01, 0x00, 0x02, 0x00];

        // Build CIP Message Router request
        let mut cip_request = Vec::new();

        // Service: Get Instance Attribute List (0x55)
        cip_request.push(0x55);

        // Request Path Size (in words)
        cip_request.push((path_array.len() / 2) as u8);

        // Request Path
        cip_request.extend_from_slice(&path_array);

        // Request Data
        cip_request.extend_from_slice(&request_data);

        println!(
            "🔧 [DEBUG] Built CIP list tags request ({} bytes): {:02X?}",
            cip_request.len(),
            cip_request
        );

        cip_request
    }

    /// Gets a human-readable error message for a CIP status code
    fn get_cip_error_message(&self, status: u8) -> String {
        match status {
            0x00 => "Success".to_string(),
            0x01 => "Connection failure".to_string(),
            0x02 => "Resource unavailable".to_string(),
            0x03 => "Invalid parameter value".to_string(),
            0x04 => "Path segment error".to_string(),
            0x05 => "Path destination unknown".to_string(),
            0x06 => "Partial transfer".to_string(),
            0x07 => "Connection lost".to_string(),
            0x08 => "Service not supported".to_string(),
            0x09 => "Invalid attribute value".to_string(),
            0x0A => "Attribute list error".to_string(),
            0x0B => "Already in requested mode/state".to_string(),
            0x0C => "Object state conflict".to_string(),
            0x0D => "Object already exists".to_string(),
            0x0E => "Attribute not settable".to_string(),
            0x0F => "Privilege violation".to_string(),
            0x10 => "Device state conflict".to_string(),
            0x11 => "Reply data too large".to_string(),
            0x12 => "Fragmentation of a primitive value".to_string(),
            0x13 => "Not enough data".to_string(),
            0x14 => "Attribute not supported".to_string(),
            0x15 => "Too much data".to_string(),
            0x16 => "Object does not exist".to_string(),
            0x17 => "Service fragmentation sequence not in progress".to_string(),
            0x18 => "No stored attribute data".to_string(),
            0x19 => "Store operation failure".to_string(),
            0x1A => "Routing failure, request packet too large".to_string(),
            0x1B => "Routing failure, response packet too large".to_string(),
            0x1C => "Missing attribute list entry data".to_string(),
            0x1D => "Invalid attribute value list".to_string(),
            0x1E => "Embedded service error".to_string(),
            0x1F => "Vendor specific error".to_string(),
            0x20 => "Invalid parameter".to_string(),
            0x21 => "Write-once value or medium already written".to_string(),
            0x22 => "Invalid reply received".to_string(),
            0x23 => "Buffer overflow".to_string(),
            0x24 => "Invalid message format".to_string(),
            0x25 => "Key failure in path".to_string(),
            0x26 => "Path size invalid".to_string(),
            0x27 => "Unexpected attribute in list".to_string(),
            0x28 => "Invalid member ID".to_string(),
            0x29 => "Member not settable".to_string(),
            0x2A => "Group 2 only server general failure".to_string(),
            0x2B => "Unknown Modbus error".to_string(),
            0x2C => "Attribute not gettable".to_string(),
            _ => format!("Unknown CIP error code: 0x{:02X}", status),
        }
    }

    async fn validate_session(&mut self) -> crate::error::Result<()> {
        let time_since_activity = self.last_activity.lock().await.elapsed();

        // Send keep-alive if it's been more than 30 seconds since last activity
        if time_since_activity > Duration::from_secs(30) {
            self.send_keep_alive().await?;
        }

        Ok(())
    }

    async fn send_keep_alive(&mut self) -> crate::error::Result<()> {
        let packet = vec![
            0x6F, 0x00, // Command: SendRRData
            0x00, 0x00, // Length: 0
        ];

        let mut stream = self.stream.lock().await;
        stream.write_all(&packet).await?;
        *self.last_activity.lock().await = Instant::now();
        Ok(())
    }

    /// Checks the health of the connection
    pub async fn check_health(&self) -> bool {
        // Check if we have a valid session handle and recent activity
        self.session_handle != 0
            && self.last_activity.lock().await.elapsed() < Duration::from_secs(150)
    }

    /// Performs a more thorough health check by actually communicating with the PLC
    pub async fn check_health_detailed(&mut self) -> crate::error::Result<bool> {
        if self.session_handle == 0 {
            return Ok(false);
        }

        // Try sending a lightweight keep-alive command
        match self.send_keep_alive().await {
            Ok(()) => Ok(true),
            Err(_) => {
                // If keep-alive fails, try re-registering the session
                match self.register_session().await {
                    Ok(()) => Ok(true),
                    Err(_) => Ok(false),
                }
            }
        }
    }

    /// Reads raw data from a tag
    async fn read_tag_raw(&mut self, tag_name: &str) -> crate::error::Result<Vec<u8>> {
        let response = self
            .send_cip_request(&self.build_read_request(tag_name))
            .await?;
        self.extract_cip_from_response(&response)
    }

    /// Sends a CIP request with automatic port routing support
    ///
    /// This is the core method that handles both direct and routed communication.
    /// When a connection path is configured, the CIP request is automatically
    /// wrapped in an Unconnected Send service to route through the backplane.
    ///
    /// # Arguments
    ///
    /// * `cip_request` - The CIP request to send
    ///
    /// # Returns
    ///
    /// The response data from the PLC
    pub async fn send_cip_request(&self, cip_request: &[u8]) -> Result<Vec<u8>> {
        // Apply port routing if configured
        let final_cip_request = if let Some(ref connection_path) = self.connection_path {
            println!("🛤️ [PORT ROUTING] Wrapping request with Unconnected Send routing: {}", 
                     connection_path.description());
            self.wrap_with_unconnected_send(cip_request, connection_path)?
        } else {
            println!("🔧 [DEBUG] Sending direct CIP request ({} bytes)", cip_request.len());
            cip_request.to_vec()
        };

        println!(
            "🔧 [DEBUG] Final request ({} bytes): {:02X?}",
            final_cip_request.len(),
            &final_cip_request[..std::cmp::min(32, final_cip_request.len())]
        );

        // Calculate total packet size
        let cip_data_size = final_cip_request.len();
        let total_data_len = 4 + 2 + 2 + 8 + cip_data_size; // Interface + Timeout + Count + Items + CIP

        let mut packet = Vec::new();

        // EtherNet/IP header (24 bytes)
        packet.extend_from_slice(&[0x6F, 0x00]); // Command: Send RR Data (0x006F)
        packet.extend_from_slice(&(total_data_len as u16).to_le_bytes()); // Length
        packet.extend_from_slice(&self.session_handle.to_le_bytes()); // Session handle
        packet.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // Status
        packet.extend_from_slice(&[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]); // Context
        packet.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // Options

        // CPF (Common Packet Format) data
        packet.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // Interface handle
        packet.extend_from_slice(&[0x05, 0x00]); // Timeout (5 seconds)
        packet.extend_from_slice(&[0x02, 0x00]); // Item count: 2

        // Item 1: Null Address Item (0x0000)
        packet.extend_from_slice(&[0x00, 0x00]); // Type: Null Address
        packet.extend_from_slice(&[0x00, 0x00]); // Length: 0

        // Item 2: Unconnected Data Item (0x00B2)
        packet.extend_from_slice(&[0xB2, 0x00]); // Type: Unconnected Data
        packet.extend_from_slice(&(cip_data_size as u16).to_le_bytes()); // Length

        // Add final CIP request data (either direct or routed)
        packet.extend_from_slice(&final_cip_request);

        println!(
            "🔧 [DEBUG] Built packet ({} bytes): {:02X?}",
            packet.len(),
            &packet[..std::cmp::min(64, packet.len())]
        );

        // Send packet with timeout
        let mut stream = self.stream.lock().await;
        stream
            .write_all(&packet)
            .await
            .map_err(EtherNetIpError::Io)?;

        // Read response header with timeout
        let mut header = [0u8; 24];
        match timeout(Duration::from_secs(10), stream.read_exact(&mut header)).await {
            Ok(Ok(_)) => {}
            Ok(Err(e)) => return Err(EtherNetIpError::Io(e)),
            Err(_) => return Err(EtherNetIpError::Timeout(Duration::from_secs(10))),
        }

        // Check EtherNet/IP command status
        let cmd_status = u32::from_le_bytes([header[8], header[9], header[10], header[11]]);
        if cmd_status != 0 {
            return Err(EtherNetIpError::Protocol(format!(
                "EIP Command failed. Status: 0x{:08X}",
                cmd_status
            )));
        }

        // Parse response length
        let response_length = u16::from_le_bytes([header[2], header[3]]) as usize;
        if response_length == 0 {
            return Ok(Vec::new());
        }

        // Read response data with timeout
        let mut response_data = vec![0u8; response_length];
        match timeout(
            Duration::from_secs(10),
            stream.read_exact(&mut response_data),
        )
        .await
        {
            Ok(Ok(_)) => {}
            Ok(Err(e)) => return Err(EtherNetIpError::Io(e)),
            Err(_) => return Err(EtherNetIpError::Timeout(Duration::from_secs(10))),
        }

        // Update last activity time
        *self.last_activity.lock().await = Instant::now();

        println!(
            "🔧 [DEBUG] Received response ({} bytes): {:02X?}",
            response_data.len(),
            &response_data[..std::cmp::min(32, response_data.len())]
        );

        Ok(response_data)
    }

    /// Wraps a CIP request in an Unconnected Send service for port routing
    ///
    /// This method implements the CIP Unconnected Send service (0x52) which is used
    /// to route messages through Allen-Bradley backplanes to reach processors in
    /// different slots than the network card.
    ///
    /// # Arguments
    ///
    /// * `cip_request` - The original CIP request to wrap
    /// * `connection_path` - The port routing configuration
    ///
    /// # Returns
    ///
    /// A new CIP request wrapped with Unconnected Send routing
    fn wrap_with_unconnected_send(&self, cip_request: &[u8], connection_path: &PortSegment) -> Result<Vec<u8>> {
        let mut wrapped_request = Vec::new();
        
        // Unconnected Send Service (0x52)
        wrapped_request.push(0x52);
        
        // Request path to Message Router (Class 0x02, Instance 0x01)
        wrapped_request.push(0x02); // Path size in words
        wrapped_request.push(0x20); // Logical Class segment
        wrapped_request.push(0x02); // Message Router class
        wrapped_request.push(0x24); // Logical Instance segment  
        wrapped_request.push(0x01); // Message Router instance
        
        // Priority/Tick time (1 byte) - 0x07 = high priority, 7 tick time
        wrapped_request.push(0x07);
        
        // Timeout ticks (1 byte) - number of ticks before timeout
        wrapped_request.push(0x0A); // 10 ticks
        
        // Message request size (2 bytes, little-endian)
        wrapped_request.extend_from_slice(&(cip_request.len() as u16).to_le_bytes());
        
        // Embedded CIP request (the original request)
        wrapped_request.extend_from_slice(cip_request);
        
        // Pad to even length if necessary (CIP requirement)
        if wrapped_request.len() % 2 != 0 {
            wrapped_request.push(0x00);
        }
        
        // Connection path size (1 byte) - size in words
        let connection_path_bytes = connection_path.encode();
        wrapped_request.push((connection_path_bytes.len() / 2) as u8);
        
        // Connection path (port segment)
        wrapped_request.extend_from_slice(&connection_path_bytes);
        
        println!("🛤️ [PORT ROUTING] Created Unconnected Send wrapper:");
        println!("  - Original request: {} bytes", cip_request.len());
        println!("  - Port routing: {}", connection_path.description());
        println!("  - Connection path: {:02X?}", connection_path_bytes);
        println!("  - Final wrapped request: {} bytes", wrapped_request.len());
        
        Ok(wrapped_request)
    }

    /// Extracts CIP data from EtherNet/IP response packet
    fn extract_cip_from_response(&self, response: &[u8]) -> crate::error::Result<Vec<u8>> {
        println!(
            "🔧 [DEBUG] Extracting CIP from response ({} bytes): {:02X?}",
            response.len(),
            &response[..std::cmp::min(32, response.len())]
        );

        // Parse CPF (Common Packet Format) structure directly from response data
        // Response format: [Interface(4)] [Timeout(2)] [ItemCount(2)] [Items...]

        if response.len() < 8 {
            return Err(EtherNetIpError::Protocol(
                "Response too short for CPF header".to_string(),
            ));
        }

        // Skip interface handle (4 bytes) and timeout (2 bytes)
        let mut pos = 6;

        // Read item count
        let item_count = u16::from_le_bytes([response[pos], response[pos + 1]]);
        pos += 2;
        println!("🔧 [DEBUG] CPF item count: {}", item_count);

        // Process items
        for i in 0..item_count {
            if pos + 4 > response.len() {
                return Err(EtherNetIpError::Protocol(
                    "Response truncated while parsing items".to_string(),
                ));
            }

            let item_type = u16::from_le_bytes([response[pos], response[pos + 1]]);
            let item_length = u16::from_le_bytes([response[pos + 2], response[pos + 3]]) as usize;
            pos += 4; // Skip item header

            println!(
                "🔧 [DEBUG] Item {}: type=0x{:04X}, length={}",
                i, item_type, item_length
            );

            if item_type == 0x00B2 {
                // Unconnected Data Item
                if pos + item_length > response.len() {
                    return Err(EtherNetIpError::Protocol("Data item truncated".to_string()));
                }

                let cip_data = response[pos..pos + item_length].to_vec();
                
                // Check if this is an Unconnected Send response that needs unwrapping
                if self.connection_path.is_some() && cip_data.len() >= 4 {
                    // Check if this is an Unconnected Send reply (service 0xD2 = 0x52 + 0x80)
                    if cip_data[0] == 0xD2 {
                        println!("🛤️ [PORT ROUTING] Unwrapping Unconnected Send response");
                        return self.unwrap_unconnected_send_response(&cip_data);
                    }
                }
                
                println!(
                    "🔧 [DEBUG] Found Unconnected Data Item, extracted CIP data ({} bytes)",
                    cip_data.len()
                );
                println!(
                    "🔧 [DEBUG] CIP data bytes: {:02X?}",
                    &cip_data[..std::cmp::min(16, cip_data.len())]
                );
                return Ok(cip_data);
            } else {
                // Skip this item's data
                pos += item_length;
            }
        }

        Err(EtherNetIpError::Protocol(
            "No Unconnected Data Item (0x00B2) found in response".to_string(),
        ))
    }

    /// Unwraps an Unconnected Send response to extract the embedded CIP response
    ///
    /// When using port routing, responses come wrapped in Unconnected Send reply
    /// format. This method extracts the actual CIP response from within the wrapper.
    ///
    /// # Arguments
    ///
    /// * `wrapped_response` - The Unconnected Send response data
    ///
    /// # Returns
    ///
    /// The embedded CIP response data
    fn unwrap_unconnected_send_response(&self, wrapped_response: &[u8]) -> Result<Vec<u8>> {
        if wrapped_response.len() < 4 {
            return Err(EtherNetIpError::Protocol(
                "Unconnected Send response too short".to_string(),
            ));
        }

        // Unconnected Send Response format:
        // [0] = Service code (0xD2 = 0x52 + 0x80)
        // [1] = Reserved (0x00)
        // [2] = General status (0x00 for success)
        // [3] = Additional status size (usually 0x00)
        // [4..] = Embedded response data

        let service_code = wrapped_response[0];
        let general_status = wrapped_response[2];
        
        println!("🛤️ [PORT ROUTING] Unconnected Send response: service=0x{:02X}, status=0x{:02X}",
                 service_code, general_status);

        // Check if the Unconnected Send itself failed
        if general_status != 0x00 {
            let error_msg = self.get_cip_error_message(general_status);
            return Err(EtherNetIpError::Protocol(format!(
                "Unconnected Send failed with status 0x{:02X}: {}",
                general_status, error_msg
            )));
        }

        // Extract the embedded response (skip the 4-byte Unconnected Send header)
        if wrapped_response.len() <= 4 {
            return Err(EtherNetIpError::Protocol(
                "No embedded response data in Unconnected Send reply".to_string(),
            ));
        }

        let embedded_response = wrapped_response[4..].to_vec();
        
        println!("🛤️ [PORT ROUTING] Extracted embedded response ({} bytes): {:02X?}",
                 embedded_response.len(),
                 &embedded_response[..std::cmp::min(16, embedded_response.len())]);

        Ok(embedded_response)
    }

    /// Parses CIP response and converts to PlcValue
    fn parse_cip_response(&self, cip_response: &[u8]) -> crate::error::Result<PlcValue> {
        println!(
            "🔧 [DEBUG] Parsing CIP response ({} bytes): {:02X?}",
            cip_response.len(),
            cip_response
        );

        if cip_response.len() < 2 {
            return Err(EtherNetIpError::Protocol(
                "CIP response too short".to_string(),
            ));
        }

        let service_reply = cip_response[0]; // Should be 0xCC (0x4C + 0x80) for Read Tag reply
        let general_status = cip_response[2]; // CIP status code

        println!(
            "🔧 [DEBUG] Service reply: 0x{:02X}, Status: 0x{:02X}",
            service_reply, general_status
        );

        // Check for CIP errors
        if general_status != 0x00 {
            let error_msg = self.get_cip_error_message(general_status);
            println!(
                "🔧 [DEBUG] CIP Error - Status: 0x{:02X}, Message: {}",
                general_status, error_msg
            );
            return Err(EtherNetIpError::Protocol(format!(
                "CIP Error {}: {}",
                general_status, error_msg
            )));
        }

        // For read operations, parse the returned data
        if service_reply == 0xCC {
            // Read Tag reply
            if cip_response.len() < 6 {
                return Err(EtherNetIpError::Protocol(
                    "Read response too short for data".to_string(),
                ));
            }

            let data_type = u16::from_le_bytes([cip_response[4], cip_response[5]]);
            let value_data = &cip_response[6..];

            println!(
                "🔧 [DEBUG] Data type: 0x{:04X}, Value data ({} bytes): {:02X?}",
                data_type,
                value_data.len(),
                value_data
            );

            // Parse based on data type
            match data_type {
                0x00C1 => {
                    // BOOL
                    if value_data.is_empty() {
                        return Err(EtherNetIpError::Protocol(
                            "No data for BOOL value".to_string(),
                        ));
                    }
                    let value = value_data[0] != 0;
                    println!("🔧 [DEBUG] Parsed BOOL: {}", value);
                    Ok(PlcValue::Bool(value))
                }
                0x00C2 => {
                    // SINT
                    if value_data.is_empty() {
                        return Err(EtherNetIpError::Protocol(
                            "No data for SINT value".to_string(),
                        ));
                    }
                    let value = value_data[0] as i8;
                    println!("🔧 [DEBUG] Parsed SINT: {}", value);
                    Ok(PlcValue::Sint(value))
                }
                0x00C3 => {
                    // INT
                    if value_data.len() < 2 {
                        return Err(EtherNetIpError::Protocol(
                            "Insufficient data for INT value".to_string(),
                        ));
                    }
                    let value = i16::from_le_bytes([value_data[0], value_data[1]]);
                    println!("🔧 [DEBUG] Parsed INT: {}", value);
                    Ok(PlcValue::Int(value))
                }
                0x00C4 => {
                    // DINT
                    if value_data.len() < 4 {
                        return Err(EtherNetIpError::Protocol(
                            "Insufficient data for DINT value".to_string(),
                        ));
                    }
                    let value = i32::from_le_bytes([
                        value_data[0],
                        value_data[1],
                        value_data[2],
                        value_data[3],
                    ]);
                    println!("🔧 [DEBUG] Parsed DINT: {}", value);
                    Ok(PlcValue::Dint(value))
                }
                0x00CA => {
                    // REAL
                    if value_data.len() < 4 {
                        return Err(EtherNetIpError::Protocol(
                            "Insufficient data for REAL value".to_string(),
                        ));
                    }
                    let value = f32::from_le_bytes([
                        value_data[0],
                        value_data[1],
                        value_data[2],
                        value_data[3],
                    ]);
                    println!("🔧 [DEBUG] Parsed REAL: {}", value);
                    Ok(PlcValue::Real(value))
                }
                0x00DA => {
                    // STRING
                    if value_data.is_empty() {
                        return Ok(PlcValue::String(String::new()));
                    }
                    let length = value_data[0] as usize;
                    if value_data.len() < 1 + length {
                        return Err(EtherNetIpError::Protocol(
                            "Insufficient data for STRING value".to_string(),
                        ));
                    }
                    let string_data = &value_data[1..1 + length];
                    let value = String::from_utf8_lossy(string_data).to_string();
                    println!("🔧 [DEBUG] Parsed STRING: '{}'", value);
                    Ok(PlcValue::String(value))
                }
                0x02A0 => {
                    // Alternative STRING type (Allen-Bradley specific)
                    if value_data.len() < 7 {
                        return Err(EtherNetIpError::Protocol(
                            "Insufficient data for alternative STRING value".to_string(),
                        ));
                    }

                    // For this format, the string data starts directly at position 6
                    // We need to find the null terminator or use the full remaining length
                    let string_start = 6;
                    let string_data = &value_data[string_start..];

                    // Find null terminator or use full length
                    let string_end = string_data
                        .iter()
                        .position(|&b| b == 0)
                        .unwrap_or(string_data.len());
                    let string_bytes = &string_data[..string_end];

                    let value = String::from_utf8_lossy(string_bytes).to_string();
                    println!("🔧 [DEBUG] Parsed alternative STRING (0x02A0): '{}'", value);
                    Ok(PlcValue::String(value))
                }
                _ => {
                    println!("🔧 [DEBUG] Unknown data type: 0x{:04X}", data_type);
                    Err(EtherNetIpError::Protocol(format!(
                        "Unsupported data type: 0x{:04X}",
                        data_type
                    )))
                }
            }
        } else if service_reply == 0xCD {
            // Write Tag reply - no data to parse
            println!("🔧 [DEBUG] Write operation successful");
            Ok(PlcValue::Bool(true)) // Indicate success
        } else {
            Err(EtherNetIpError::Protocol(format!(
                "Unknown service reply: 0x{:02X}",
                service_reply
            )))
        }
    }

    /// Unregisters the EtherNet/IP session with the PLC
    pub async fn unregister_session(&mut self) -> crate::error::Result<()> {
        println!("🔌 Unregistering session and cleaning up connections...");

        // Close all connected sessions first
        let _ = self.close_all_connected_sessions().await;

        let mut packet = Vec::new();

        // EtherNet/IP header
        packet.extend_from_slice(&[0x66, 0x00]); // Command: Unregister Session
        packet.extend_from_slice(&[0x04, 0x00]); // Length: 4 bytes
        packet.extend_from_slice(&self.session_handle.to_le_bytes()); // Session handle
        packet.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // Status
        packet.extend_from_slice(&[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]); // Sender context
        packet.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // Options

        // Protocol version for unregister session
        packet.extend_from_slice(&[0x01, 0x00, 0x00, 0x00]); // Protocol version 1

        self.stream
            .lock()
            .await
            .write_all(&packet)
            .await
            .map_err(EtherNetIpError::Io)?;

        println!("✅ Session unregistered and all connections closed");
        Ok(())
    }

    // =========================================================================
    // BATCH OPERATIONS IMPLEMENTATION
    // =========================================================================

    /// Executes a batch of read and write operations with automatic port routing
    ///
    /// All operations in the batch will use the same connection path if port routing
    /// is configured. This provides high performance for multiple tag operations
    /// while maintaining proper routing.
    ///
    /// # Arguments
    ///
    /// * `operations` - A slice of operations to execute
    ///
    /// # Returns
    ///
    /// A vector of `BatchResult` items, one for each input operation
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use rust_ethernet_ip::{EipClient, PortSegment, BatchOperation, PlcValue};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    ///     // Connect with routing to processor in slot 1
    ///     let mut client = EipClient::connect_with_path(
    ///         "192.168.1.3:44818",
    ///         Some(PortSegment::processor_in_slot(1))
    ///     ).await?;
    ///
    ///     let operations = vec![
    ///         BatchOperation::Read { tag_name: "Rust_Real[0]".to_string() },
    ///         BatchOperation::Read { tag_name: "Rust_Real[1]".to_string() },
    ///         BatchOperation::Write {
    ///             tag_name: "SetPoint".to_string(),
    ///             value: PlcValue::Dint(1500)
    ///         },
    ///     ];
    ///
    ///     // All operations automatically use slot 1 routing
    ///     let results = client.execute_batch(&operations).await?;
    ///
    ///     for result in results {
    ///         match result.result {
    ///             Ok(Some(value)) => println!("Read value: {:?}", value),
    ///             Ok(None) => println!("Write successful"),
    ///             Err(e) => println!("Operation failed: {}", e),
    ///         }
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn execute_batch(
        &mut self,
        operations: &[BatchOperation],
    ) -> crate::error::Result<Vec<BatchResult>> {
        if operations.is_empty() {
            return Ok(Vec::new());
        }

        let start_time = Instant::now();
        println!(
            "🚀 [BATCH] Starting batch execution with {} operations",
            operations.len()
        );

        // Display routing info for batch operations
        if let Some(ref path) = self.connection_path {
            println!("🛤️ [BATCH] All operations will use routing: {}", path.description());
        }

        // Group operations based on configuration
        let operation_groups = if self.batch_config.optimize_packet_packing {
            self.optimize_operation_groups(operations)
        } else {
            self.sequential_operation_groups(operations)
        };

        let mut all_results = Vec::with_capacity(operations.len());

        // Execute each group
        for (group_index, group) in operation_groups.iter().enumerate() {
            println!(
                "🔧 [BATCH] Processing group {} with {} operations",
                group_index + 1,
                group.len()
            );

            match self.execute_operation_group(group).await {
                Ok(mut group_results) => {
                    all_results.append(&mut group_results);
                }
                Err(e) => {
                    if !self.batch_config.continue_on_error {
                        return Err(e);
                    }

                    // Create error results for this group
                    for op in group {
                        let error_result = BatchResult {
                            operation: op.clone(),
                            result: Err(BatchError::NetworkError(e.to_string())),
                            execution_time_us: 0,
                        };
                        all_results.push(error_result);
                    }
                }
            }
        }

        let total_time = start_time.elapsed();
        println!(
            "✅ [BATCH] Completed batch execution in {:?} - {} operations processed",
            total_time,
            all_results.len()
        );

        Ok(all_results)
    }

    /// Reads multiple tags in a single batch operation with automatic port routing
    pub async fn read_tags_batch(
        &mut self,
        tag_names: &[&str],
    ) -> crate::error::Result<Vec<(String, std::result::Result<PlcValue, BatchError>)>> {
        let operations: Vec<BatchOperation> = tag_names
            .iter()
            .map(|&name| BatchOperation::Read {
                tag_name: name.to_string(),
            })
            .collect();

        let results = self.execute_batch(&operations).await?;

        Ok(results
            .into_iter()
            .map(|result| {
                let tag_name = match &result.operation {
                    BatchOperation::Read { tag_name } => tag_name.clone(),
                    _ => unreachable!("Should only have read operations"),
                };

                let value_result = match result.result {
                    Ok(Some(value)) => Ok(value),
                    Ok(None) => Err(BatchError::Other(
                        "Unexpected None result for read operation".to_string(),
                    )),
                    Err(e) => Err(e),
                };

                (tag_name, value_result)
            })
            .collect())
    }

    /// Writes multiple tag values in a single batch operation with automatic port routing
    pub async fn write_tags_batch(
        &mut self,
        tag_values: &[(&str, PlcValue)],
    ) -> crate::error::Result<Vec<(String, std::result::Result<(), BatchError>)>> {
        let operations: Vec<BatchOperation> = tag_values
            .iter()
            .map(|(name, value)| BatchOperation::Write {
                tag_name: name.to_string(),
                value: value.clone(),
            })
            .collect();

        let results = self.execute_batch(&operations).await?;

        Ok(results
            .into_iter()
            .map(|result| {
                let tag_name = match &result.operation {
                    BatchOperation::Write { tag_name, .. } => tag_name.clone(),
                    _ => unreachable!("Should only have write operations"),
                };

                let write_result = match result.result {
                    Ok(None) => Ok(()),
                    Ok(Some(_)) => Err(BatchError::Other(
                        "Unexpected value result for write operation".to_string(),
                    )),
                    Err(e) => Err(e),
                };

                (tag_name, write_result)
            })
            .collect())
    }

    /// Configures batch operation settings
    pub fn configure_batch_operations(&mut self, config: BatchConfig) {
        self.batch_config = config;
        println!(
            "🔧 [BATCH] Updated batch configuration: max_ops={}, max_size={}, timeout={}ms",
            self.batch_config.max_operations_per_packet,
            self.batch_config.max_packet_size,
            self.batch_config.packet_timeout_ms
        );
    }

    /// Gets current batch operation configuration
    pub fn get_batch_config(&self) -> &BatchConfig {
        &self.batch_config
    }

    // =========================================================================
    // INTERNAL BATCH OPERATION HELPERS
    // =========================================================================

    /// Groups operations optimally for batch processing
    fn optimize_operation_groups(&self, operations: &[BatchOperation]) -> Vec<Vec<BatchOperation>> {
        let mut groups = Vec::new();
        let mut reads = Vec::new();
        let mut writes = Vec::new();

        // Separate reads and writes
        for op in operations {
            match op {
                BatchOperation::Read { .. } => reads.push(op.clone()),
                BatchOperation::Write { .. } => writes.push(op.clone()),
            }
        }

        // Group reads
        for chunk in reads.chunks(self.batch_config.max_operations_per_packet) {
            groups.push(chunk.to_vec());
        }

        // Group writes
        for chunk in writes.chunks(self.batch_config.max_operations_per_packet) {
            groups.push(chunk.to_vec());
        }

        groups
    }

    /// Groups operations sequentially (preserves order)
    fn sequential_operation_groups(
        &self,
        operations: &[BatchOperation],
    ) -> Vec<Vec<BatchOperation>> {
        operations
            .chunks(self.batch_config.max_operations_per_packet)
            .map(|chunk| chunk.to_vec())
            .collect()
    }

    /// Executes a single group of operations as a CIP Multiple Service Packet
    async fn execute_operation_group(
        &mut self,
        operations: &[BatchOperation],
    ) -> crate::error::Result<Vec<BatchResult>> {
        let start_time = Instant::now();
        let mut results = Vec::with_capacity(operations.len());

        // Build Multiple Service Packet request
        let cip_request = self.build_multiple_service_packet(operations)?;

        // Send request and get response (routing is handled automatically in send_cip_request)
        let response = self.send_cip_request(&cip_request).await?;

        // Parse response and create results
        let parsed_results = self.parse_multiple_service_response(&response, operations)?;

        let execution_time = start_time.elapsed();

        // Create BatchResult objects
        for (i, operation) in operations.iter().enumerate() {
            let op_execution_time = execution_time.as_micros() as u64 / operations.len() as u64;

            let result = if i < parsed_results.len() {
                match &parsed_results[i] {
                    Ok(value) => Ok(value.clone()),
                    Err(e) => Err(e.clone()),
                }
            } else {
                Err(BatchError::Other(
                    "Missing result from response".to_string(),
                ))
            };

            results.push(BatchResult {
                operation: operation.clone(),
                result,
                execution_time_us: op_execution_time,
            });
        }

        Ok(results)
    }

    /// Builds a CIP Multiple Service Packet request
    fn build_multiple_service_packet(
        &self,
        operations: &[BatchOperation],
    ) -> crate::error::Result<Vec<u8>> {
        let mut packet = Vec::with_capacity(8 + (operations.len() * 2));

        // Multiple Service Packet service code
        packet.push(0x0A);

        // Request path (2 bytes for class 0x02, instance 1)
        packet.push(0x02); // Path size in words
        packet.push(0x20); // Class segment
        packet.push(0x02); // Class 0x02 (Message Router)
        packet.push(0x24); // Instance segment
        packet.push(0x01); // Instance 1

        // Number of services
        packet.extend_from_slice(&(operations.len() as u16).to_le_bytes());

        // Calculate offset table
        let mut service_requests = Vec::with_capacity(operations.len());
        let mut current_offset = 2 + (operations.len() * 2); // Start after offset table

        for operation in operations {
            // Build individual service request
            let service_request = match operation {
                BatchOperation::Read { tag_name } => self.build_read_request(tag_name),
                BatchOperation::Write { tag_name, value } => {
                    self.build_write_request(tag_name, value)?
                }
            };

            service_requests.push(service_request);
        }

        // Add offset table
        for service_request in &service_requests {
            packet.extend_from_slice(&(current_offset as u16).to_le_bytes());
            current_offset += service_request.len();
        }

        // Add service requests
        for service_request in service_requests {
            packet.extend_from_slice(&service_request);
        }

        println!(
            "🔧 [BATCH] Built Multiple Service Packet ({} bytes, {} services)",
            packet.len(),
            operations.len()
        );

        Ok(packet)
    }

    /// Parses a Multiple Service Packet response
    fn parse_multiple_service_response(
        &self,
        response: &[u8],
        operations: &[BatchOperation],
    ) -> crate::error::Result<Vec<std::result::Result<Option<PlcValue>, BatchError>>> {
        if response.len() < 6 {
            return Err(crate::error::EtherNetIpError::Protocol(
                "Response too short for Multiple Service Packet".to_string(),
            ));
        }

        let mut results = Vec::new();

        println!(
            "🔧 [DEBUG] Raw Multiple Service Response ({} bytes): {:02X?}",
            response.len(),
            response
        );

        // First, extract the CIP data from the EtherNet/IP response
        let cip_data = match self.extract_cip_from_response(response) {
            Ok(data) => data,
            Err(e) => {
                println!("🔧 [DEBUG] Failed to extract CIP data: {}", e);
                return Err(e);
            }
        };

        println!(
            "🔧 [DEBUG] Extracted CIP data ({} bytes): {:02X?}",
            cip_data.len(),
            cip_data
        );

        if cip_data.len() < 6 {
            return Err(crate::error::EtherNetIpError::Protocol(
                "CIP data too short for Multiple Service Response".to_string(),
            ));
        }

        // Parse Multiple Service Response header from CIP data
        let service_code = cip_data[0];
        let general_status = cip_data[2];
        let num_replies = u16::from_le_bytes([cip_data[4], cip_data[5]]) as usize;

        println!(
            "🔧 [DEBUG] Multiple Service Response: service=0x{:02X}, status=0x{:02X}, replies={}",
            service_code, general_status, num_replies
        );

        if general_status != 0x00 {
            return Err(crate::error::EtherNetIpError::Protocol(format!(
                "Multiple Service Response error: 0x{:02X}",
                general_status
            )));
        }

        if num_replies != operations.len() {
            return Err(crate::error::EtherNetIpError::Protocol(format!(
                "Reply count mismatch: expected {}, got {}",
                operations.len(),
                num_replies
            )));
        }

        // Read reply offsets
        let mut reply_offsets = Vec::new();
        let mut offset = 6; // Skip header

        for _i in 0..num_replies {
            if offset + 2 > cip_data.len() {
                return Err(crate::error::EtherNetIpError::Protocol(
                    "CIP data too short for reply offsets".to_string(),
                ));
            }
            let reply_offset =
                u16::from_le_bytes([cip_data[offset], cip_data[offset + 1]]) as usize;
            reply_offsets.push(reply_offset);
            offset += 2;
        }

        println!("🔧 [DEBUG] Reply offsets: {:?}", reply_offsets);

        // Parse each reply
        for (i, &reply_offset) in reply_offsets.iter().enumerate() {
            let reply_start = 4 + reply_offset;

            if reply_start >= cip_data.len() {
                results.push(Err(BatchError::Other(
                    "Reply offset beyond CIP data".to_string(),
                )));
                continue;
            }

            let reply_end = if i + 1 < reply_offsets.len() {
                4 + reply_offsets[i + 1]
            } else {
                cip_data.len()
            };

            if reply_end > cip_data.len() || reply_start >= reply_end {
                results.push(Err(BatchError::Other(
                    "Invalid reply boundaries".to_string(),
                )));
                continue;
            }

            let reply_data = &cip_data[reply_start..reply_end];

            println!(
                "🔧 [DEBUG] Reply {} data: {:02X?}", i, reply_data
            );

            let result = self.parse_individual_reply(reply_data, &operations[i]);
            results.push(result);
        }

        Ok(results)
    }

    /// Parses an individual service reply within a Multiple Service Packet response
    fn parse_individual_reply(
        &self,
        reply_data: &[u8],
        operation: &BatchOperation,
    ) -> std::result::Result<Option<PlcValue>, BatchError> {
        if reply_data.len() < 4 {
            return Err(BatchError::SerializationError(
                "Reply too short".to_string(),
            ));
        }

        println!(
            "🔧 [DEBUG] Parsing individual reply ({} bytes): {:02X?}",
            reply_data.len(),
            reply_data
        );

        let service_code = reply_data[0];
        let general_status = reply_data[2];

        println!(
            "🔧 [DEBUG] Service code: 0x{:02X}, Status: 0x{:02X}",
            service_code, general_status
        );

        if general_status != 0x00 {
            let error_msg = self.get_cip_error_message(general_status);
            return Err(BatchError::CipError {
                status: general_status,
                message: error_msg,
            });
        }

        match operation {
            BatchOperation::Write { .. } => {
                // Write operations return no data on success
                Ok(None)
            }
            BatchOperation::Read { .. } => {
                // Read operations return data starting at offset 4
                if reply_data.len() < 6 {
                    return Err(BatchError::SerializationError(
                        "Read reply too short for data".to_string(),
                    ));
                }

                let data = &reply_data[4..];
                println!(
                    "🔧 [DEBUG] Parsing data ({} bytes): {:02X?}",
                    data.len(),
                    data
                );

                if data.len() < 2 {
                    return Err(BatchError::SerializationError(
                        "Data too short for type".to_string(),
                    ));
                }

                let data_type = u16::from_le_bytes([data[0], data[1]]);
                let value_data = &data[2..];

                println!(
                    "🔧 [DEBUG] Data type: 0x{:04X}, Value data ({} bytes): {:02X?}",
                    data_type,
                    value_data.len(),
                    value_data
                );

                // Parse based on data type
                match data_type {
                    0x00C1 => {
                        // BOOL
                        if value_data.is_empty() {
                            return Err(BatchError::SerializationError(
                                "Missing BOOL value".to_string(),
                            ));
                        }
                        Ok(Some(PlcValue::Bool(value_data[0] != 0)))
                    }
                    0x00C2 => {
                        // SINT
                        if value_data.is_empty() {
                            return Err(BatchError::SerializationError(
                                "Missing SINT value".to_string(),
                            ));
                        }
                        Ok(Some(PlcValue::Sint(value_data[0] as i8)))
                    }
                    0x00C3 => {
                        // INT
                        if value_data.len() < 2 {
                            return Err(BatchError::SerializationError(
                                "Missing INT value".to_string(),
                            ));
                        }
                        let value = i16::from_le_bytes([value_data[0], value_data[1]]);
                        Ok(Some(PlcValue::Int(value)))
                    }
                    0x00C4 => {
                        // DINT
                        if value_data.len() < 4 {
                            return Err(BatchError::SerializationError(
                                "Missing DINT value".to_string(),
                            ));
                        }
                        let value = i32::from_le_bytes([
                            value_data[0],
                            value_data[1],
                            value_data[2],
                            value_data[3],
                        ]);
                        println!("🔧 [DEBUG] Parsed DINT: {}", value);
                        Ok(Some(PlcValue::Dint(value)))
                    }
                    0x00CA => {
                        // REAL
                        if value_data.len() < 4 {
                            return Err(BatchError::SerializationError(
                                "Missing REAL value".to_string(),
                            ));
                        }
                        let bytes = [value_data[0], value_data[1], value_data[2], value_data[3]];
                        let value = f32::from_le_bytes(bytes);
                        println!("🔧 [DEBUG] Parsed REAL: {}", value);
                        Ok(Some(PlcValue::Real(value)))
                    }
                    0x00DA | 0x02A0 => {
                        // STRING types
                        if value_data.is_empty() {
                            return Ok(Some(PlcValue::String(String::new())));
                        }
                        
                        let string_value = if data_type == 0x00DA {
                            // Standard STRING format
                            let length = value_data[0] as usize;
                            if value_data.len() < 1 + length {
                                return Err(BatchError::SerializationError(
                                    "Insufficient data for STRING value".to_string(),
                                ));
                            }
                            let string_data = &value_data[1..1 + length];
                            String::from_utf8_lossy(string_data).to_string()
                        } else {
                            // Allen-Bradley specific STRING format
                            if value_data.len() < 7 {
                                return Err(BatchError::SerializationError(
                                    "Insufficient data for AB STRING value".to_string(),
                                ));
                            }
                            let string_start = 6;
                            let string_data = &value_data[string_start..];
                            let string_end = string_data
                                .iter()
                                .position(|&b| b == 0)
                                .unwrap_or(string_data.len());
                            let string_bytes = &string_data[..string_end];
                            String::from_utf8_lossy(string_bytes).to_string()
                        };
                        
                        println!("🔧 [DEBUG] Parsed STRING: '{}'", string_value);
                        Ok(Some(PlcValue::String(string_value)))
                    }
                    _ => Err(BatchError::SerializationError(format!(
                        "Unsupported data type: 0x{:04X}",
                        data_type
                    ))),
                }
            }
        }
    }

    /// Subscribes to a tag for real-time updates
    pub async fn subscribe_to_tag(
        &self,
        tag_path: &str,
        options: SubscriptionOptions,
    ) -> Result<()> {
        let mut subscriptions = self.subscriptions.lock().await;
        let subscription = TagSubscription::new(tag_path.to_string(), options);
        subscriptions.push(subscription);
        drop(subscriptions);

        let tag_path = tag_path.to_string();
        let mut client = self.clone();
        tokio::spawn(async move {
            loop {
                match client.read_tag(&tag_path).await {
                    Ok(value) => {
                        if let Err(e) = client.update_subscription(&tag_path, &value).await {
                            eprintln!("Error updating subscription: {}", e);
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading tag {}: {}", tag_path, e);
                        break;
                    }
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        });
        Ok(())
    }

    pub async fn subscribe_to_tags(&self, tags: &[(&str, SubscriptionOptions)]) -> Result<()> {
        for (tag_name, options) in tags {
            self.subscribe_to_tag(tag_name, options.clone()).await?;
        }
        Ok(())
    }

    async fn update_subscription(&self, tag_name: &str, value: &PlcValue) -> Result<()> {
        let subscriptions = self.subscriptions.lock().await;
        for subscription in subscriptions.iter() {
            if subscription.tag_path == tag_name && subscription.is_active() {
                subscription.update_value(value).await?;
            }
        }
        Ok(())
    }

    /// Builds a string write request packet
    fn build_string_write_request(
        &self,
        tag_name: &str,
        value: &str,
    ) -> crate::error::Result<Vec<u8>> {
        let mut request = Vec::new();

        // CIP Write Service (0x4D)
        request.push(0x4D);

        // Tag path
        let tag_path = self.build_tag_path(tag_name);
        request.extend_from_slice(&tag_path);

        // AB STRING data structure
        request.extend_from_slice(&(value.len() as u16).to_le_bytes()); // Len
        request.extend_from_slice(&82u16.to_le_bytes()); // MaxLen

        // Data[82] with padding
        let mut data = [0u8; 82];
        let bytes = value.as_bytes();
        data[..bytes.len()].copy_from_slice(bytes);
        request.extend_from_slice(&data);

        Ok(request)
    }

    /// Write a string value to a PLC tag using unconnected messaging
    pub async fn write_string(&mut self, tag_name: &str, value: &str) -> crate::error::Result<()> {
        // Validate string length
        if value.len() > 82 {
            return Err(crate::error::EtherNetIpError::StringTooLong {
                max_length: 82,
                actual_length: value.len(),
            });
        }

        // Validate string content (ASCII only)
        if !value.is_ascii() {
            return Err(crate::error::EtherNetIpError::InvalidString {
                reason: "String contains non-ASCII characters".to_string(),
            });
        }

        // Build the string write request
        let request = self.build_string_write_request(tag_name, value)?;

        // Send the request and get the response (routing applied automatically)
        let response = self.send_cip_request(&request).await?;

        // Parse the response
        let cip_response = self.extract_cip_from_response(&response)?;

        // Check for errors in the response
        if cip_response.len() < 2 {
            return Err(crate::error::EtherNetIpError::InvalidResponse {
                reason: "Response too short".to_string(),
            });
        }

        let status = cip_response[0];
        if status != 0 {
            return Err(crate::error::EtherNetIpError::WriteError {
                status,
                message: self.get_cip_error_message(status),
            });
        }

        Ok(())
    }

    /// Closes all connected sessions (called during disconnect)
    async fn close_all_connected_sessions(&mut self) -> crate::error::Result<()> {
        let session_names: Vec<String> = self
            .connected_sessions
            .lock()
            .await
            .keys()
            .cloned()
            .collect();

        for session_name in session_names {
            let _ = self.close_connected_session(&session_name).await; // Ignore errors during cleanup
        }

        Ok(())
    }

    /// Closes a specific connected session
    async fn close_connected_session(&mut self, session_name: &str) -> crate::error::Result<()> {
        if let Some(session) = self.connected_sessions.lock().await.get(session_name) {
            let session = session.clone();

            // Build Forward Close request
            let forward_close_request = self.build_forward_close_request(&session)?;

            // Send Forward Close request
            let _response = self.send_cip_request(&forward_close_request).await?;

            println!(
                "🔗 [CONNECTED] Session '{}' closed successfully",
                session_name
            );
        }

        // Remove session from our tracking
        let mut sessions = self.connected_sessions.lock().await;
        sessions.remove(session_name);

        Ok(())
    }

    /// Builds a Forward Close CIP request for terminating connected sessions
    fn build_forward_close_request(
        &self,
        session: &ConnectedSession,
    ) -> crate::error::Result<Vec<u8>> {
        let mut request = Vec::with_capacity(21);

        // CIP Forward Close Service (0x4E)
        request.push(0x4E);

        // Request path length (Connection Manager object)
        request.push(0x02); // 2 words

        // Class ID: Connection Manager (0x06)
        request.push(0x20); // Logical Class segment
        request.push(0x06);

        // Instance ID: Connection Manager instance (0x01)
        request.push(0x24); // Logical Instance segment
        request.push(0x01);

        // Forward Close parameters
        request.push(0x0A); // Timeout ticks (10)
        request.push(session.timeout_multiplier);

        // Connection Serial Number (2 bytes, little-endian)
        request.extend_from_slice(&session.connection_serial.to_le_bytes());

        // Originator Vendor ID (2 bytes, little-endian)
        request.extend_from_slice(&session.originator_vendor_id.to_le_bytes());

        // Originator Serial Number (4 bytes, little-endian)
        request.extend_from_slice(&session.originator_serial.to_le_bytes());

        // Connection Path Size (1 byte)
        request.push(0x02); // 2 words for Message Router path

        // Connection Path - Target the Message Router
        request.push(0x20); // Logical Class segment
        request.push(0x02); // Message Router class (0x02)
        request.push(0x24); // Logical Instance segment
        request.push(0x01); // Message Router instance (0x01)

        Ok(request)
    }
}

/*
===============================================================================
END OF LIBRARY DOCUMENTATION

This file provides a complete, production-ready EtherNet/IP communication
library for Allen-Bradley PLCs with comprehensive port routing support.

## Port Routing Features Added:

- **PortSegment Structure**: Complete CIP-compliant port routing implementation
- **Automatic Routing**: All read/write operations automatically use configured routing
- **Runtime Configuration**: Connection paths can be changed during execution
- **Zero Breaking Changes**: Existing code continues to work unchanged
- **Validation**: Connection path validation and error reporting
- **Batch Support**: All batch operations support automatic port routing

## Usage Examples:

```rust
// Connect to processor in slot 1 (most common case)
let mut client = EipClient::connect_with_path(
    "192.168.1.3:44818",
    Some(PortSegment::processor_in_slot(1))
).await?;

// All operations are automatically routed to slot 1
let value = client.read_tag("Rust_Real[0]").await?;
client.write_tag("SetPoint", PlcValue::Dint(1500)).await?;

// Change routing at runtime
client.set_connection_path(PortSegment::processor_in_slot(3));

// Batch operations also use routing automatically
let operations = vec![
    BatchOperation::Read { tag_name: "Tag1".to_string() },
    BatchOperation::Write { tag_name: "Tag2".to_string(), value: PlcValue::Dint(42) },
];
let results = client.execute_batch(&operations).await?;
```

Version: 0.5.3 + Port Routing
Compatible with: CompactLogix L1x-L5x series PLCs with backplane routing
Perfect for: Network cards in slot 0, processors in other slots
===============================================================================
*/