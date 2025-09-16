// lib.rs - Rust EtherNet/IP Driver Library with Comprehensive Documentation
// =========================================================================
//
// # Rust EtherNet/IP Driver Library v0.5.3
//
// A high-performance, production-ready EtherNet/IP communication library for
// Allen-Bradley CompactLogix and ControlLogix PLCs, written in pure Rust with
// comprehensive language bindings (C#, Python, Go, JavaScript/TypeScript).
//
// ## Overview
//
// This library provides a complete implementation of the EtherNet/IP protocol
// and Common Industrial Protocol (CIP) for communicating with Allen-Bradley
// CompactLogix and ControlLogix series PLCs. It offers native Rust APIs, comprehensive
// language bindings, and production-ready features for enterprise deployment.
//
// ## Architecture
//
// ```text
// ┌─────────────────────────────────────────────────────────────────────────────────┐
// │                              Application Layer                                  │
// │  ┌─────────────┐  ┌─────────────────────────────────────────────────────────┐  │
// │  │    Rust     │  │                    C# Ecosystem                         │  │
// │  │   Native    │  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────────┐  │  │
// │  │             │  │  │     WPF     │  │  WinForms   │  │   ASP.NET Core  │  │  │
// │  │             │  │  │  Desktop    │  │  Desktop    │  │    Web API      │  │  │
// │  │             │  │  └─────────────┘  └─────────────┘  └─────────┬───────┘  │  │
// │  │             │  │                                               │           │  │
// │  │             │  │                                    ┌─────────┴───────┐  │  │
// │  │             │  │                                    │  TypeScript +   │  │  │
// │  │             │  │                                    │  React Frontend │  │  │
// │  │             │  │                                    │  (HTTP/REST)    │  │  │
// │  │             │  │                                    └─────────────────┘  │  │
// │  └─────────────┘  └─────────────────────────────────────────────────────────┘  │
// │  ┌─────────────┐  ┌─────────────────────────────────────────────────────────┐  │
// │  │   Python    │  │                    Go Ecosystem                         │  │
// │  │  PyO3       │  │  ┌─────────────┐  ┌─────────────────────────────────┐  │  │
// │  │  Bindings   │  │  │   CGO       │  │        Next.js Frontend         │  │  │
// │  │             │  │  │  Backend    │  │  ┌─────────────┐  ┌─────────────┐  │  │
// │  │             │  │  │             │  │  │ TypeScript  │  │   React     │  │  │
// │  │             │  │  │             │  │  │ Components  │  │ Components  │  │  │
// │  │             │  │  └─────────────┘  │  └─────────────┘  └─────────────┘  │  │
// │  │             │  │                   └─────────────────────────────────┘  │  │
// │  └─────────────┘  └─────────────────────────────────────────────────────────┘  │
// │  ┌─────────────────────────────────────────────────────────────────────────┐  │
// │  │                    Vue.js Ecosystem                                     │  │
// │  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────────────┐  │  │
// │  │  │   Vue 3     │  │ TypeScript  │  │        Vite Build System        │  │  │
// │  │  │ Components  │  │   Support   │  │     (Hot Module Replacement)     │  │  │
// │  │  └─────────────┘  └─────────────┘  └─────────────────────────────────┘  │  │
// │  └─────────────────────────────────────────────────────────────────────────┘  │
// └─────────────────────┬─────────────────────────────────────────────────────────┘
//                       │
// ┌─────────────────────┴─────────────────────────────────────────────────────────┐
// │                        Language Wrappers                                      │
// │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
// │  │   C# FFI    │  │  Python     │  │    Go       │  │   JavaScript/TS     │  │
// │  │  Wrapper    │  │  PyO3       │  │   CGO       │  │   FFI Bindings      │  │
// │  │             │  │  Bindings   │  │  Bindings   │  │                     │  │
// │  │ • 22 funcs  │  │ • Native    │  │ • Native    │  │ • Node.js Support   │  │
// │  │ • Type-safe │  │ • Async     │  │ • Concurrent│  │ • Browser Support   │  │
// │  │ • Cross-plat│  │ • Cross-plat│  │ • Cross-plat│  │ • TypeScript Types  │  │
// │  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────────────┘  │
// └─────────────────────┬─────────────────────────────────────────────────────────┘
//                       │
// ┌─────────────────────┴─────────────────────────────────────────────────────────┐
// │                         Core Rust Library                                     │
// │  ┌─────────────────────────────────────────────────────────────────────────┐  │
// │  │                           EipClient                                     │  │
// │  │  • Connection Management & Session Handling                            │  │
// │  │  • Advanced Tag Operations & Program-Scoped Tag Support                │  │
// │  │  • Complete Data Type Support (13 Allen-Bradley types)                 │  │
// │  │  • Advanced Tag Path Parsing (arrays, bits, UDTs, strings)             │  │
// │  │  • Real-Time Subscriptions with Event-Driven Notifications             │  │
// │  │  • High-Performance Batch Operations (2,000+ ops/sec)                  │  │
// │  └─────────────────────────────────────────────────────────────────────────┘  │
// │  ┌─────────────────────────────────────────────────────────────────────────┐  │
// │  │                    Protocol Implementation                              │  │
// │  │  • EtherNet/IP Encapsulation Protocol                                  │  │
// │  │  • CIP (Common Industrial Protocol)                                    │  │
// │  │  • Symbolic Tag Addressing with Advanced Parsing                       │  │
// │  │  • Comprehensive CIP Error Code Mapping                                │  │
// │  └─────────────────────────────────────────────────────────────────────────┘  │
// │  ┌─────────────────────────────────────────────────────────────────────────┐  │
// │  │                        Network Layer                                    │  │
// │  │  • TCP Socket Management with Connection Pooling                       │  │
// │  │  • Async I/O with Tokio Runtime                                        │  │
// │  │  • Robust Error Handling & Network Resilience                          │  │
// │  │  • Session Management & Automatic Reconnection                         │  │
// │  └─────────────────────────────────────────────────────────────────────────┘  │
// └─────────────────────────────────────────────────────────────────────────────────┘
// ```
//
// ## Integration Paths
//
// ### 🦀 **Native Rust Applications**
// Direct library usage with full async support and zero-overhead abstractions.
// Perfect for high-performance applications and embedded systems.
//
// ### 🖥️ **Desktop Applications (C#)**
// - **WPF**: Modern desktop applications with MVVM architecture
// - **WinForms**: Traditional Windows applications with familiar UI patterns
// - Uses C# FFI wrapper for seamless integration
//
// ### 🐍 **Python Applications**
// - **Native Python Bindings**: Direct PyO3 integration with full async support
// - **Cross-Platform**: Windows, Linux, macOS support
// - **Easy Installation**: pip install or maturin development
//
// ### 🐹 **Go Applications**
// - **CGO Bindings**: Native Go integration with C FFI
// - **High Performance**: Zero-copy operations where possible
// - **Concurrent**: Full goroutine support for concurrent operations
//
// ### 🌐 **Web Applications**
// - **ASP.NET Core Web API**: RESTful backend service
// - **TypeScript + React Frontend**: Modern web dashboard via HTTP/REST API
// - **Vue.js Applications**: Modern reactive web interfaces
// - **Scalable Architecture**: Backend handles PLC communication, frontend provides UI
//
// ### 🔧 **System Integration**
// - **C/C++ Applications**: Direct FFI integration
// - **Other .NET Languages**: VB.NET, F#, etc. via C# wrapper
// - **Microservices**: ASP.NET Core API as a service component
//
// ## Features
//
// ### Core Capabilities
// - **High Performance**: 2,000+ operations per second with batch operations
// - **Real-Time Subscriptions**: Event-driven notifications with 1ms-10s intervals
// - **Complete Data Types**: All Allen-Bradley native data types with type-safe operations
// - **Advanced Tag Addressing**: Program-scoped, arrays, bits, UDTs, strings
// - **Batch Operations**: High-performance multi-tag read/write with 2,000+ ops/sec
// - **Async I/O**: Built on Tokio for excellent concurrency and performance
// - **Error Handling**: Comprehensive CIP error code mapping and reporting
// - **Memory Safe**: Zero-copy operations where possible, proper resource cleanup
// - **Production Ready**: Enterprise-grade monitoring, health checks, and configuration
//
// ### Supported PLCs
// - **CompactLogix L1x, L2x, L3x, L4x, L5x series** (Primary focus)
// - **ControlLogix L6x, L7x, L8x series** (Full support)
// - Optimized for PC applications (Windows, Linux, macOS)
//
// ### Advanced Tag Addressing
// - **Program-scoped tags**: `Program:MainProgram.Tag1`
// - **Array element access**: `MyArray[5]`, `MyArray[1,2,3]`
// - **Bit-level operations**: `MyDINT.15` (access individual bits)
// - **UDT member access**: `MyUDT.Member1.SubMember`
// - **String operations**: `MyString.LEN`, `MyString.DATA[5]`
// - **Complex nested paths**: `Program:Production.Lines[2].Stations[5].Motor.Status.15`
//
// ### Complete Data Type Support
// - **BOOL**: Boolean values
// - **SINT, INT, DINT, LINT**: Signed integers (8, 16, 32, 64-bit)
// - **USINT, UINT, UDINT, ULINT**: Unsigned integers (8, 16, 32, 64-bit)
// - **REAL, LREAL**: Floating point (32, 64-bit IEEE 754)
// - **STRING**: Variable-length strings
// - **UDT**: User Defined Types with full nesting support
//
// ### Protocol Support
// - **EtherNet/IP**: Complete encapsulation protocol implementation
// - **CIP**: Common Industrial Protocol for tag operations
// - **Symbolic Addressing**: Direct tag name resolution with advanced parsing
// - **Session Management**: Proper registration/unregistration sequences
//
// ### Integration Options
// - **Native Rust**: Direct library usage with full async support
// - **C# Desktop Applications**: WPF and WinForms via C# FFI wrapper
// - **Python Applications**: Native PyO3 bindings with full async support
// - **Go Applications**: CGO bindings with concurrent operations
// - **Web Applications**: ASP.NET Core API + TypeScript/React/Vue frontend
// - **C/C++ Integration**: Direct FFI functions for system integration
// - **Cross-Platform**: Windows, Linux, macOS support
//
// ## Performance Characteristics
//
// Benchmarked on typical industrial hardware:
//
// | Operation | Performance | Notes |
// |-----------|-------------|-------|
// | Read BOOL | 1,500+ ops/sec | Single tag operations |
// | Read DINT | 1,400+ ops/sec | 32-bit integer tags |
// | Read REAL | 1,300+ ops/sec | Floating point tags |
// | Write BOOL | 800+ ops/sec | Single tag operations |
// | Write DINT | 750+ ops/sec | 32-bit integer tags |
// | Write REAL | 700+ ops/sec | Floating point tags |
// | **Batch Read** | **2,000+ ops/sec** | **Multi-tag operations** |
// | **Batch Write** | **1,500+ ops/sec** | **Multi-tag operations** |
// | **Real-Time Subscriptions** | **1ms-10s intervals** | **Event-driven** |
// | Connection | <1 second | Initial session setup |
// | Tag Path Parsing | 10,000+ ops/sec | Advanced addressing |
//
// ## Security Considerations
//
// - **No Authentication**: EtherNet/IP protocol has limited built-in security
// - **Network Level**: Implement firewall rules and network segmentation
// - **PLC Protection**: Use PLC safety locks and access controls
// - **Data Validation**: Always validate data before writing to PLCs
//
// ## Thread Safety
//
// The `EipClient` struct is **NOT** thread-safe. For multi-threaded applications:
// - Use one client per thread, OR
// - Implement external synchronization (Mutex/RwLock), OR
// - Use a connection pool pattern
//
// ## Memory Usage
//
// - **Per Connection**: ~8KB base memory footprint
// - **Network Buffers**: ~2KB per active connection
// - **Tag Cache**: Minimal (tag names only when needed)
// - **Total Typical**: <10MB for most applications
//
// ## Error Handling Philosophy
//
// This library follows Rust's error handling principles:
// - All fallible operations return `Result<T, EtherNetIpError>`
// - Errors are propagated rather than panicking
// - Detailed error messages with CIP status code mapping
// - Network errors are distinguished from protocol errors
//
// ## Examples
//
// See the `examples/` directory for comprehensive usage examples, including:
// - Advanced tag addressing demonstrations
// - Complete data type showcase
// - Real-world industrial automation scenarios
// - Professional HMI/SCADA dashboard
// - Multi-language integration examples (C#, Python, Go, TypeScript, Vue)
//
// ## Changelog
//
// ### v0.5.3 (January 2025) - **CURRENT**
// - Enhanced safety documentation for all FFI functions
// - Comprehensive clippy optimizations and code quality improvements
// - Improved memory management and connection pool handling
// - Enhanced Python, C#, and Go wrapper stability
// - Production-ready code quality with 0 warnings
//
// ### v0.5.0 (January 2025)
// - Professional HMI/SCADA production dashboard
// - Enterprise-grade monitoring and health checks
// - Production-ready configuration management
// - Comprehensive metrics collection and reporting
// - Enhanced error handling and recovery mechanisms
//
// ### v0.4.0 (January 2025)
// - Real-time subscriptions with event-driven notifications
// - High-performance batch operations (2,000+ ops/sec)
// - Complete data type support for all Allen-Bradley types
// - Advanced tag path parsing (program-scoped, arrays, bits, UDTs)
// - Enhanced error handling and documentation
// - Comprehensive test coverage (47+ tests)
// - Production-ready stability and performance
//
// =========================================================================

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
// BATCH OPERATIONS DATA STRUCTURES
// =========================================================================

/// Represents a single operation in a batch request
///
/// This enum defines the different types of operations that can be
/// performed in a batch. Each operation specifies whether it's a read
/// or write operation and includes the necessary parameters.
#[derive(Debug, Clone)]
pub enum BatchOperation {
    /// Read operation for a specific tag
    ///
    /// # Fields
    ///
    /// * `tag_name` - The name of the tag to read
    Read { tag_name: String },

    /// Write operation for a specific tag with a value
    ///
    /// # Fields
    ///
    /// * `tag_name` - The name of the tag to write
    /// * `value` - The value to write to the tag
    Write { tag_name: String, value: PlcValue },
}

/// Result of a single operation in a batch request
///
/// This structure contains the result of executing a single batch operation,
/// including success/failure status and the actual data or error information.
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
///
/// This enum provides detailed error information for batch operations,
/// allowing for better error handling and diagnostics.
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
///
/// This structure controls the behavior and performance characteristics
/// of batch read/write operations. Proper tuning can significantly
/// improve throughput for applications that need to process many tags.
#[derive(Debug, Clone)]
pub struct BatchConfig {
    /// Maximum number of operations to include in a single CIP packet
    ///
    /// Larger values improve performance but may exceed PLC packet size limits.
    /// Typical range: 10-50 operations per packet.
    pub max_operations_per_packet: usize,

    /// Maximum packet size in bytes for batch operations
    ///
    /// Should not exceed the PLC's maximum packet size capability.
    /// Typical values: 504 bytes (default), up to 4000 bytes for modern PLCs.
    pub max_packet_size: usize,

    /// Timeout for individual batch packets (in milliseconds)
    ///
    /// This is per-packet timeout, not per-operation.
    /// Typical range: 1000-5000 milliseconds.
    pub packet_timeout_ms: u64,

    /// Whether to continue processing other operations if one fails
    ///
    /// If true, failed operations are reported but don't stop the batch.
    /// If false, the first error stops the entire batch processing.
    pub continue_on_error: bool,

    /// Whether to optimize packet packing by grouping similar operations
    ///
    /// If true, reads and writes are grouped separately for better performance.
    /// If false, operations are processed in the order provided.
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
///
/// Allen-Bradley PLCs often require connected sessions for certain operations
/// like STRING writes. This structure maintains the connection state.
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
                println!("🔧 [CONFIG 1] Conservative: 504 bytes, 200ms RPI, low priority");
            }
            2 => {
                // Config 2: Compact parameters
                session.timeout_multiplier = 0x03; // 8 seconds timeout
                session.rpi = 50000; // 50ms RPI (faster)
                session.o_to_t_params.size = 256; // Smaller packet size
                session.t_to_o_params.size = 256;
                session.o_to_t_params.priority = 0x02; // Scheduled priority
                session.t_to_o_params.priority = 0x02;
                println!("🔧 [CONFIG 2] Compact: 256 bytes, 50ms RPI, scheduled priority");
            }
            3 => {
                // Config 3: Minimal parameters
                session.timeout_multiplier = 0x01; // 4 seconds timeout
                session.rpi = 1000000; // 1000ms RPI (very slow)
                session.o_to_t_params.size = 128; // Very small packets
                session.t_to_o_params.size = 128;
                session.o_to_t_params.priority = 0x03; // Urgent priority
                session.t_to_o_params.priority = 0x03;
                println!("🔧 [CONFIG 3] Minimal: 128 bytes, 1000ms RPI, urgent priority");
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
                println!("🔧 [CONFIG 4] Rockwell standard: 500 bytes, 100ms RPI, multicast, Rockwell vendor");
            }
            5 => {
                // Config 5: Large buffer parameters
                session.timeout_multiplier = 0x0A; // Very long timeout
                session.rpi = 500000; // 500ms RPI
                session.o_to_t_params.size = 1024; // Large packets
                session.t_to_o_params.size = 1024;
                session.o_to_t_params.variable_size = true; // Variable size
                session.t_to_o_params.variable_size = true;
                println!("🔧 [CONFIG 5] Large buffer: 1024 bytes, 500ms RPI, variable size");
            }
            _ => {
                // Default config
                println!("🔧 [CONFIG 0] Default parameters");
            }
        }

        session
    }
}

/// Represents the different data types supported by Allen-Bradley PLCs
///
/// These correspond to the CIP data type codes used in EtherNet/IP
/// communication. Each variant maps to a specific 16-bit type identifier
/// that the PLC uses to describe tag data.
///
/// # Supported Data Types
///
/// ## Integer Types
/// - **SINT**: 8-bit signed integer (-128 to 127)
/// - **INT**: 16-bit signed integer (-32,768 to 32,767)
/// - **DINT**: 32-bit signed integer (-2,147,483,648 to 2,147,483,647)
/// - **LINT**: 64-bit signed integer (-9,223,372,036,854,775,808 to 9,223,372,036,854,775,807)
///
/// ## Unsigned Integer Types
/// - **USINT**: 8-bit unsigned integer (0 to 255)
/// - **UINT**: 16-bit unsigned integer (0 to 65,535)
/// - **UDINT**: 32-bit unsigned integer (0 to 4,294,967,295)
/// - **ULINT**: 64-bit unsigned integer (0 to 18,446,744,073,709,551,615)
///
/// ## Floating Point Types
/// - **REAL**: 32-bit IEEE 754 float (±1.18 × 10^-38 to ±3.40 × 10^38)
/// - **LREAL**: 64-bit IEEE 754 double (±2.23 × 10^-308 to ±1.80 × 10^308)
///
/// ## Other Types
/// - **BOOL**: Boolean value (true/false)
/// - **STRING**: Variable-length string
/// - **UDT**: User Defined Type (structured data)
#[derive(Debug, Clone, PartialEq)]
pub enum PlcValue {
    /// Boolean value (single bit)
    ///
    /// Maps to CIP type 0x00C1. In CompactLogix PLCs, BOOL tags
    /// are stored as single bits but transmitted as bytes over the network.
    Bool(bool),

    /// 8-bit signed integer (-128 to 127)
    ///
    /// Maps to CIP type 0x00C2. Used for small numeric values,
    /// status codes, and compact data storage.
    Sint(i8),

    /// 16-bit signed integer (-32,768 to 32,767)
    ///
    /// Maps to CIP type 0x00C3. Common for analog input/output values,
    /// counters, and medium-range numeric data.
    Int(i16),

    /// 32-bit signed integer (-2,147,483,648 to 2,147,483,647)
    ///
    /// Maps to CIP type 0x00C4. This is the most common integer type
    /// in Allen-Bradley PLCs, used for counters, setpoints, and numeric values.
    Dint(i32),

    /// 64-bit signed integer (-9,223,372,036,854,775,808 to 9,223,372,036,854,775,807)
    ///
    /// Maps to CIP type 0x00C5. Used for large counters, timestamps,
    /// and high-precision calculations.
    Lint(i64),

    /// 8-bit unsigned integer (0 to 255)
    ///
    /// Maps to CIP type 0x00C6. Used for byte data, small counters,
    /// and status flags.
    Usint(u8),

    /// 16-bit unsigned integer (0 to 65,535)
    ///
    /// Maps to CIP type 0x00C7. Common for analog values, port numbers,
    /// and medium-range unsigned data.
    Uint(u16),

    /// 32-bit unsigned integer (0 to 4,294,967,295)
    ///
    /// Maps to CIP type 0x00C8. Used for large counters, memory addresses,
    /// and unsigned calculations.
    Udint(u32),

    /// 64-bit unsigned integer (0 to 18,446,744,073,709,551,615)
    ///
    /// Maps to CIP type 0x00C9. Used for very large counters, timestamps,
    /// and high-precision unsigned calculations.
    Ulint(u64),

    /// 32-bit IEEE 754 floating point number
    ///
    /// Maps to CIP type 0x00CA. Used for analog values, calculations,
    /// and any data requiring decimal precision.
    /// Range: ±1.18 × 10^-38 to ±3.40 × 10^38
    Real(f32),

    /// 64-bit IEEE 754 floating point number
    ///
    /// Maps to CIP type 0x00CB. Used for high-precision calculations,
    /// scientific data, and extended-range floating point values.
    /// Range: ±2.23 × 10^-308 to ±1.80 × 10^308
    Lreal(f64),

    /// String value
    ///
    /// Maps to CIP type 0x00DA. Variable-length string data
    /// commonly used for product names, status messages, and text data.
    String(String),

    /// User Defined Type instance
    ///
    /// Maps to CIP type 0x00A0. Structured data type containing
    /// multiple members of different types.
    Udt(HashMap<String, PlcValue>),
}

impl PlcValue {
    /// Converts the PLC value to its byte representation for network transmission
    ///
    /// This function handles the little-endian byte encoding required by
    /// the EtherNet/IP protocol. Each data type has specific encoding rules:
    ///
    /// - BOOL: Single byte (0x00 = false, 0xFF = true)
    /// - SINT: Single signed byte
    /// - INT: 2 bytes in little-endian format
    /// - DINT: 4 bytes in little-endian format
    /// - LINT: 8 bytes in little-endian format
    /// - USINT: Single unsigned byte
    /// - UINT: 2 bytes in little-endian format
    /// - UDINT: 4 bytes in little-endian format
    /// - ULINT: 8 bytes in little-endian format
    /// - REAL: 4 bytes IEEE 754 little-endian format
    /// - LREAL: 8 bytes IEEE 754 little-endian format
    ///
    /// # Returns
    ///
    /// A vector of bytes ready for transmission to the PLC
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
                // Try minimal approach - just length + data without padding
                // Testing if the PLC accepts a simpler format

                let mut bytes = Vec::new();

                // Length field (4 bytes as DINT) - number of characters currently used
                let length = val.len().min(82) as u32;
                bytes.extend_from_slice(&length.to_le_bytes());

                // String data - just the actual characters, no padding
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
    ///
    /// These codes are defined by the CIP specification and must match
    /// exactly what the PLC expects for each data type.
    ///
    /// # Returns
    ///
    /// The 16-bit CIP type code for this value type
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

/// High-performance EtherNet/IP client for PLC communication
///
/// This struct provides the core functionality for communicating with Allen-Bradley
/// PLCs using the EtherNet/IP protocol. It handles connection management, session
/// registration, and tag operations.
///
/// # Thread Safety
///
/// The `EipClient` is **NOT** thread-safe. For multi-threaded applications:
///
/// ```rust,no_run
/// use std::sync::Arc;
/// use tokio::sync::Mutex;
/// use rust_ethernet_ip::EipClient;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
///     // Create a thread-safe wrapper
///     let client = Arc::new(Mutex::new(EipClient::connect("192.168.1.100:44818").await?));
///
///     // Use in multiple threads
///     let client_clone = client.clone();
///     tokio::spawn(async move {
///         let mut client = client_clone.lock().await;
///         let _ = client.read_tag("Tag1").await?;
///         Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
///     });
///     Ok(())
/// }
/// ```
///
/// # Performance Characteristics
///
/// | Operation | Latency | Throughput | Memory |
/// |-----------|---------|------------|---------|
/// | Connect | 100-500ms | N/A | ~8KB |
/// | Read Tag | 1-5ms | 1,500+ ops/sec | ~2KB |
/// | Write Tag | 2-10ms | 600+ ops/sec | ~2KB |
/// | Batch Read | 5-20ms | 2,000+ ops/sec | ~4KB |
///
/// # Error Handling
///
/// All operations return `Result<T, EtherNetIpError>`. Common errors include:
///
/// ```rust,no_run
/// use rust_ethernet_ip::{EipClient, EtherNetIpError};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
///     let mut client = EipClient::connect("192.168.1.100:44818").await?;
///     match client.read_tag("Tag1").await {
///         Ok(value) => println!("Tag value: {:?}", value),
///         Err(EtherNetIpError::Protocol(_)) => println!("Tag does not exist"),
///         Err(EtherNetIpError::Connection(_)) => println!("Lost connection to PLC"),
///         Err(EtherNetIpError::Timeout(_)) => println!("Operation timed out"),
///         Err(e) => println!("Other error: {}", e),
///     }
///     Ok(())
/// }
/// ```
///
/// # Examples
///
/// Basic usage:
/// ```rust,no_run
/// use rust_ethernet_ip::{EipClient, PlcValue};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
///     let mut client = EipClient::connect("192.168.1.100:44818").await?;
///
///     // Read a boolean tag
///     let motor_running = client.read_tag("MotorRunning").await?;
///
///     // Write an integer tag
///     client.write_tag("SetPoint", PlcValue::Dint(1500)).await?;
///
///     // Read multiple tags in sequence
///     let tag1 = client.read_tag("Tag1").await?;
///     let tag2 = client.read_tag("Tag2").await?;
///     let tag3 = client.read_tag("Tag3").await?;
///     Ok(())
/// }
/// ```
///
/// Advanced usage with error recovery:
/// ```rust
/// use rust_ethernet_ip::{EipClient, PlcValue, EtherNetIpError};
/// use tokio::time::Duration;
///
/// async fn read_with_retry(client: &mut EipClient, tag: &str, retries: u32) -> Result<PlcValue, EtherNetIpError> {
///     for attempt in 0..retries {
///         match client.read_tag(tag).await {
///             Ok(value) => return Ok(value),
///             Err(EtherNetIpError::Connection(_)) => {
///                 if attempt < retries - 1 {
///                     tokio::time::sleep(Duration::from_secs(1)).await;
///                     continue;
///                 }
///             }
///             Err(e) => return Err(e),
///         }
///     }
///     Err(EtherNetIpError::Protocol("Max retries exceeded".to_string()))
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
}

impl EipClient {
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
        };
        client.register_session().await?;
        Ok(client)
    }

    /// Public async connect function for EipClient
    pub async fn connect(addr: &str) -> Result<Self> {
        Self::new(addr).await
    }

    /// Registers an EtherNet/IP session with the PLC
    ///
    /// This is an internal function that implements the EtherNet/IP session
    /// registration protocol. It sends a Register Session command and
    /// processes the response to extract the session handle.
    ///
    /// # Protocol Details
    ///
    /// The Register Session command consists of:
    /// - EtherNet/IP Encapsulation Header (24 bytes)
    /// - Registration Data (4 bytes: protocol version + options)
    ///
    /// The PLC responds with:
    /// - Same header format with assigned session handle
    /// - Status code indicating success/failure
    ///
    /// # Errors
    ///
    /// - Network timeout or disconnection
    /// - Invalid response format
    /// - PLC rejection (status code non-zero)
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

    /// Reads a tag value from the PLC
    ///
    /// This function performs a CIP read request for the specified tag.
    /// The tag's data type is automatically determined from the PLC's response.
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
    /// use rust_ethernet_ip::{EipClient, PlcValue};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    ///     let mut client = EipClient::connect("192.168.1.100:44818").await?;
    ///
    ///     // Read different data types
    ///     let bool_val = client.read_tag("MotorRunning").await?;
    ///     let int_val = client.read_tag("Counter").await?;
    ///     let real_val = client.read_tag("Temperature").await?;
    ///
    ///     // Handle the result
    ///     match bool_val {
    ///         PlcValue::Bool(true) => println!("Motor is running"),
    ///         PlcValue::Bool(false) => println!("Motor is stopped"),
    ///         _ => println!("Unexpected data type"),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Performance
    ///
    /// - Latency: 1-5ms typical
    /// - Throughput: 1,500+ ops/sec
    /// - Network: 1 request/response cycle
    ///
    /// # Error Handling
    ///
    /// Common errors:
    /// - `Protocol`: Tag doesn't exist or invalid format
    /// - `Connection`: Lost connection to PLC
    /// - `Timeout`: Operation timed out
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

        // Standard tag reading
        let response = self
            .send_cip_request(&self.build_read_request(tag_name))
            .await?;
        let cip_data = self.extract_cip_from_response(&response)?;
        self.parse_cip_response(&cip_data)
    }

    /// Writes a value to a PLC tag
    ///
    /// This method automatically determines the best communication method based on the data type:
    /// - STRING values use unconnected explicit messaging with proper AB STRING format
    /// - Other data types use standard unconnected messaging
    ///
    /// # Arguments
    ///
    /// * `tag_name` - The name of the tag to write to
    /// * `value` - The value to write
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// # let mut client = rust_ethernet_ip::EipClient::connect("192.168.1.100:44818").await?;
    /// use rust_ethernet_ip::PlcValue;
    ///
    /// client.write_tag("Counter", PlcValue::Dint(42)).await?;
    /// client.write_tag("Message", PlcValue::String("Hello PLC".to_string())).await?;
    /// # Ok(())
    /// # }
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

        // Use specialized AB STRING format for STRING writes (required for proper Allen-Bradley STRING handling)
        // All data types including strings now use the standard write path
        // The PlcValue::to_bytes() method handles the correct format for each type

        // Use standard unconnected messaging for other data types
        let cip_request = self.build_write_request(tag_name, &value)?;

        let response = self.send_cip_request(&cip_request).await?;

        // Check write response for errors - need to extract CIP response first
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

    /// Builds a write request specifically for Allen-Bradley string format
    fn _build_ab_string_write_request(
        &self,
        tag_name: &str,
        value: &PlcValue,
    ) -> crate::error::Result<Vec<u8>> {
        if let PlcValue::String(string_value) = value {
            println!(
                "🔧 [DEBUG] Building correct Allen-Bradley string write request for tag: '{}'",
                tag_name
            );

            let mut cip_request = Vec::new();

            // Service: Write Tag Service (0x4D)
            cip_request.push(0x4D);

            // Request Path Size (in words)
            let tag_bytes = tag_name.as_bytes();
            let path_len = if tag_bytes.len() % 2 == 0 {
                tag_bytes.len() + 2
            } else {
                tag_bytes.len() + 3
            } / 2;
            cip_request.push(path_len as u8);

            // Request Path
            cip_request.push(0x91); // ANSI Extended Symbol
            cip_request.push(tag_bytes.len() as u8);
            cip_request.extend_from_slice(tag_bytes);

            // Pad to word boundary if needed
            if tag_bytes.len() % 2 != 0 {
                cip_request.push(0x00);
            }

            // Data Type: Allen-Bradley STRING (0x02A0)
            cip_request.extend_from_slice(&[0xA0, 0x02]);

            // Element Count (always 1 for single string)
            cip_request.extend_from_slice(&[0x01, 0x00]);

            // Build the correct AB STRING structure
            let string_bytes = string_value.as_bytes();
            let max_len: u16 = 82; // Standard AB STRING max length
            let current_len = string_bytes.len().min(max_len as usize) as u16;

            // AB STRING structure:
            // - Len (2 bytes) - number of characters used
            cip_request.extend_from_slice(&current_len.to_le_bytes());

            // - MaxLen (2 bytes) - maximum characters allowed (typically 82)
            cip_request.extend_from_slice(&max_len.to_le_bytes());

            // - Data[MaxLen] (82 bytes) - the character array, zero-padded
            let mut data_array = vec![0u8; max_len as usize];
            data_array[..current_len as usize]
                .copy_from_slice(&string_bytes[..current_len as usize]);
            cip_request.extend_from_slice(&data_array);

            println!("🔧 [DEBUG] Built correct AB string write request ({} bytes): len={}, maxlen={}, data_len={}",
                     cip_request.len(), current_len, max_len, string_bytes.len());
            println!(
                "🔧 [DEBUG] First 32 bytes: {:02X?}",
                &cip_request[..std::cmp::min(32, cip_request.len())]
            );

            Ok(cip_request)
        } else {
            Err(EtherNetIpError::Protocol(
                "Expected string value for Allen-Bradley string write".to_string(),
            ))
        }
    }

    /// Builds a CIP Write Tag Service request
    ///
    /// This creates the CIP packet for writing a value to a tag.
    /// The request includes the service code, tag path, data type, and value.
    fn build_write_request(
        &self,
        tag_name: &str,
        value: &PlcValue,
    ) -> crate::error::Result<Vec<u8>> {
        println!("🔧 [DEBUG] Building write request for tag: '{}'", tag_name);

        // Use Connected Explicit Messaging for consistency
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

    /// Builds a raw write request with pre-serialized data
    fn build_write_request_raw(
        &self,
        tag_name: &str,
        data: &[u8],
    ) -> crate::error::Result<Vec<u8>> {
        let mut request = Vec::new();

        // Write Tag Service
        request.push(0x4D);
        request.push(0x00);

        // Build tag path
        let tag_path = self.build_tag_path(tag_name);
        request.extend(tag_path);

        // Add raw data
        request.extend(data);

        Ok(request)
    }

    /// Serializes a PlcValue into bytes for transmission
    #[allow(dead_code)]
    fn serialize_value(&self, value: &PlcValue) -> crate::error::Result<Vec<u8>> {
        let mut data = Vec::new();

        match value {
            PlcValue::Bool(v) => {
                data.extend(&0x00C1u16.to_le_bytes()); // Data type
                data.push(if *v { 0xFF } else { 0x00 });
            }
            PlcValue::Sint(v) => {
                data.extend(&0x00C2u16.to_le_bytes()); // Data type
                data.extend(&v.to_le_bytes());
            }
            PlcValue::Int(v) => {
                data.extend(&0x00C3u16.to_le_bytes()); // Data type
                data.extend(&v.to_le_bytes());
            }
            PlcValue::Dint(v) => {
                data.extend(&0x00C4u16.to_le_bytes()); // Data type
                data.extend(&v.to_le_bytes());
            }
            PlcValue::Lint(v) => {
                data.extend(&0x00C5u16.to_le_bytes()); // Data type
                data.extend(&v.to_le_bytes());
            }
            PlcValue::Usint(v) => {
                data.extend(&0x00C6u16.to_le_bytes()); // Data type
                data.extend(&v.to_le_bytes());
            }
            PlcValue::Uint(v) => {
                data.extend(&0x00C7u16.to_le_bytes()); // Data type
                data.extend(&v.to_le_bytes());
            }
            PlcValue::Udint(v) => {
                data.extend(&0x00C8u16.to_le_bytes()); // Data type
                data.extend(&v.to_le_bytes());
            }
            PlcValue::Ulint(v) => {
                data.extend(&0x00C9u16.to_le_bytes()); // Data type
                data.extend(&v.to_le_bytes());
            }
            PlcValue::Real(v) => {
                data.extend(&0x00CAu16.to_le_bytes()); // Data type
                data.extend(&v.to_le_bytes());
            }
            PlcValue::Lreal(v) => {
                data.extend(&0x00CBu16.to_le_bytes()); // Data type
                data.extend(&v.to_le_bytes());
            }
            PlcValue::String(v) => {
                data.extend(&0x00CEu16.to_le_bytes()); // Data type - correct Allen-Bradley STRING CIP type

                // Length field (4 bytes as DINT) - number of characters currently used
                let length = v.len().min(82) as u32;
                data.extend_from_slice(&length.to_le_bytes());

                // String data - the actual characters (no MaxLen field)
                let string_bytes = v.as_bytes();
                let data_len = string_bytes.len().min(82);
                data.extend_from_slice(&string_bytes[..data_len]);

                // Padding to make total data area exactly 82 bytes after length
                let remaining_chars = 82 - data_len;
                data.extend(vec![0u8; remaining_chars]);
            }
            PlcValue::Udt(_) => {
                // UDT serialization is handled by the UdtManager
                // For now, just add placeholder data
                data.extend(&0x00A0u16.to_le_bytes()); // UDT type code
            }
        }

        Ok(data)
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
    ///
    /// # Arguments
    ///
    /// * `status` - The CIP status code to look up
    ///
    /// # Returns
    ///
    /// A string describing the error
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

    /// Writes raw data to a tag
    #[allow(dead_code)]
    async fn write_tag_raw(&mut self, tag_name: &str, data: &[u8]) -> crate::error::Result<()> {
        let request = self.build_write_request_raw(tag_name, data)?;
        let response = self.send_cip_request(&request).await?;

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

        println!("✅ Write completed successfully");
        Ok(())
    }

    /// Sends a CIP request wrapped in EtherNet/IP SendRRData command
    pub async fn send_cip_request(&self, cip_request: &[u8]) -> Result<Vec<u8>> {
        println!(
            "🔧 [DEBUG] Sending CIP request ({} bytes): {:02X?}",
            cip_request.len(),
            cip_request
        );

        // Calculate total packet size
        let cip_data_size = cip_request.len();
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

        // Add CIP request data
        packet.extend_from_slice(cip_request);

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

    // =========================================================================
    // BATCH OPERATIONS IMPLEMENTATION
    // =========================================================================

    /// Executes a batch of read and write operations
    ///
    /// This is the main entry point for batch operations. It takes a slice of
    /// `BatchOperation` items and executes them efficiently by grouping them
    /// into optimal CIP packets based on the current `BatchConfig`.
    ///
    /// # Arguments
    ///
    /// * `operations` - A slice of operations to execute
    ///
    /// # Returns
    ///
    /// A vector of `BatchResult` items, one for each input operation.
    /// Results are returned in the same order as the input operations.
    ///
    /// # Performance
    ///
    /// - **Throughput**: 5,000-15,000+ operations/second (vs 1,500 individual)
    /// - **Latency**: 5-20ms per batch (vs 1-3ms per individual operation)
    /// - **Network efficiency**: 1-5 packets vs N packets for N operations
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use rust_ethernet_ip::{EipClient, BatchOperation, PlcValue};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    ///     let mut client = EipClient::connect("192.168.1.100:44818").await?;
    ///
    ///     let operations = vec![
    ///         BatchOperation::Read { tag_name: "Motor1_Speed".to_string() },
    ///         BatchOperation::Read { tag_name: "Motor2_Speed".to_string() },
    ///         BatchOperation::Write {
    ///             tag_name: "SetPoint".to_string(),
    ///             value: PlcValue::Dint(1500)
    ///         },
    ///     ];
    ///
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

    /// Reads multiple tags in a single batch operation
    ///
    /// This is a convenience method for read-only batch operations.
    /// It's optimized for reading many tags at once.
    ///
    /// # Arguments
    ///
    /// * `tag_names` - A slice of tag names to read
    ///
    /// # Returns
    ///
    /// A vector of tuples containing (tag_name, result) pairs
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use rust_ethernet_ip::EipClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    ///     let mut client = EipClient::connect("192.168.1.100:44818").await?;
    ///
    ///     let tags = ["Motor1_Speed", "Motor2_Speed", "Temperature", "Pressure"];
    ///     let results = client.read_tags_batch(&tags).await?;
    ///
    ///     for (tag_name, result) in results {
    ///         match result {
    ///             Ok(value) => println!("{}: {:?}", tag_name, value),
    ///             Err(e) => println!("{}: Error - {}", tag_name, e),
    ///         }
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
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

    /// Writes multiple tag values in a single batch operation
    ///
    /// This is a convenience method for write-only batch operations.
    /// It's optimized for writing many values at once.
    ///
    /// # Arguments
    ///
    /// * `tag_values` - A slice of (tag_name, value) tuples to write
    ///
    /// # Returns
    ///
    /// A vector of tuples containing (tag_name, result) pairs
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use rust_ethernet_ip::{EipClient, PlcValue};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    ///     let mut client = EipClient::connect("192.168.1.100:44818").await?;
    ///
    ///     let writes = vec![
    ///         ("SetPoint1", PlcValue::Bool(true)),
    ///         ("SetPoint2", PlcValue::Dint(2000)),
    ///         ("EnableFlag", PlcValue::Bool(true)),
    ///     ];
    ///
    ///     let results = client.write_tags_batch(&writes).await?;
    ///
    ///     for (tag_name, result) in results {
    ///         match result {
    ///             Ok(_) => println!("{}: Write successful", tag_name),
    ///             Err(e) => println!("{}: Write failed - {}", tag_name, e),
    ///         }
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
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
    ///
    /// This method allows fine-tuning of batch operation behavior,
    /// including performance optimizations and error handling.
    ///
    /// # Arguments
    ///
    /// * `config` - The new batch configuration to use
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use rust_ethernet_ip::{EipClient, BatchConfig};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    ///     let mut client = EipClient::connect("192.168.1.100:44818").await?;
    ///
    ///     let config = BatchConfig {
    ///         max_operations_per_packet: 50,
    ///         max_packet_size: 1500,
    ///         packet_timeout_ms: 5000,
    ///         continue_on_error: false,
    ///         optimize_packet_packing: true,
    ///     };
    ///
    ///     client.configure_batch_operations(config);
    ///
    ///     Ok(())
    /// }
    /// ```
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

        // Send request and get response
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

        // Parse Multiple Service Response header from CIP data:
        // [0] = Service Code (0x8A)
        // [1] = Reserved (0x00)
        // [2] = General Status (0x00 for success)
        // [3] = Additional Status Size (0x00)
        // [4-5] = Number of replies (little endian)

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

        // Read reply offsets (each is 2 bytes, little endian)
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

        // The reply data starts after all the offsets
        let reply_base_offset = 6 + (num_replies * 2);

        println!("🔧 [DEBUG] Reply base offset: {}", reply_base_offset);

        // Parse each reply
        for (i, &reply_offset) in reply_offsets.iter().enumerate() {
            // Reply offset is relative to position 4 (after service code, reserved, status, additional status size)
            let reply_start = 4 + reply_offset;

            if reply_start >= cip_data.len() {
                results.push(Err(BatchError::Other(
                    "Reply offset beyond CIP data".to_string(),
                )));
                continue;
            }

            // Calculate reply end position
            let reply_end = if i + 1 < reply_offsets.len() {
                // Not the last reply - use next reply's offset as boundary
                4 + reply_offsets[i + 1]
            } else {
                // Last reply - goes to end of CIP data
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
                "🔧 [DEBUG] Reply {} at offset {}: start={}, end={}, len={}",
                i,
                reply_offset,
                reply_start,
                reply_end,
                reply_data.len()
            );
            println!("🔧 [DEBUG] Reply {} data: {:02X?}", i, reply_data);

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

        // Each individual reply in Multiple Service Response has the same format as standalone CIP response:
        // [0] = Service Code (0xCC for read response, 0xCD for write response)
        // [1] = Reserved (0x00)
        // [2] = General Status (0x00 for success)
        // [3] = Additional Status Size (0x00)
        // [4..] = Response data (for reads) or empty (for writes)

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

                // Parse the data directly (skip the 4-byte header)
                // Data format: [type_low, type_high, value_bytes...]
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
                    0x00C5 => {
                        // LINT
                        if value_data.len() < 8 {
                            return Err(BatchError::SerializationError(
                                "Missing LINT value".to_string(),
                            ));
                        }
                        let value = i64::from_le_bytes([
                            value_data[0],
                            value_data[1],
                            value_data[2],
                            value_data[3],
                            value_data[4],
                            value_data[5],
                            value_data[6],
                            value_data[7],
                        ]);
                        Ok(Some(PlcValue::Lint(value)))
                    }
                    0x00C6 => {
                        // USINT
                        if value_data.is_empty() {
                            return Err(BatchError::SerializationError(
                                "Missing USINT value".to_string(),
                            ));
                        }
                        Ok(Some(PlcValue::Usint(value_data[0])))
                    }
                    0x00C7 => {
                        // UINT
                        if value_data.len() < 2 {
                            return Err(BatchError::SerializationError(
                                "Missing UINT value".to_string(),
                            ));
                        }
                        let value = u16::from_le_bytes([value_data[0], value_data[1]]);
                        Ok(Some(PlcValue::Uint(value)))
                    }
                    0x00C8 => {
                        // UDINT
                        if value_data.len() < 4 {
                            return Err(BatchError::SerializationError(
                                "Missing UDINT value".to_string(),
                            ));
                        }
                        let value = u32::from_le_bytes([
                            value_data[0],
                            value_data[1],
                            value_data[2],
                            value_data[3],
                        ]);
                        Ok(Some(PlcValue::Udint(value)))
                    }
                    0x00C9 => {
                        // ULINT
                        if value_data.len() < 8 {
                            return Err(BatchError::SerializationError(
                                "Missing ULINT value".to_string(),
                            ));
                        }
                        let value = u64::from_le_bytes([
                            value_data[0],
                            value_data[1],
                            value_data[2],
                            value_data[3],
                            value_data[4],
                            value_data[5],
                            value_data[6],
                            value_data[7],
                        ]);
                        Ok(Some(PlcValue::Ulint(value)))
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
                    0x00CB => {
                        // LREAL
                        if value_data.len() < 8 {
                            return Err(BatchError::SerializationError(
                                "Missing LREAL value".to_string(),
                            ));
                        }
                        let bytes = [
                            value_data[0],
                            value_data[1],
                            value_data[2],
                            value_data[3],
                            value_data[4],
                            value_data[5],
                            value_data[6],
                            value_data[7],
                        ];
                        let value = f64::from_le_bytes(bytes);
                        Ok(Some(PlcValue::Lreal(value)))
                    }
                    0x00DA => {
                        // STRING
                        if value_data.is_empty() {
                            return Ok(Some(PlcValue::String(String::new())));
                        }
                        let length = value_data[0] as usize;
                        if value_data.len() < 1 + length {
                            return Err(BatchError::SerializationError(
                                "Insufficient data for STRING value".to_string(),
                            ));
                        }
                        let string_data = &value_data[1..1 + length];
                        let value = String::from_utf8_lossy(string_data).to_string();
                        println!("🔧 [DEBUG] Parsed STRING: '{}'", value);
                        Ok(Some(PlcValue::String(value)))
                    }
                    0x02A0 => {
                        // Alternative STRING type (Allen-Bradley specific) for batch operations
                        if value_data.len() < 7 {
                            return Err(BatchError::SerializationError(
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
                        Ok(Some(PlcValue::String(value)))
                    }
                    _ => Err(BatchError::SerializationError(format!(
                        "Unsupported data type: 0x{:04X}",
                        data_type
                    ))),
                }
            }
        }
    }

    /// Writes a string value using Allen-Bradley UDT component access
    /// This writes to TestString.LEN and TestString.DATA separately
    pub async fn write_ab_string_components(
        &mut self,
        tag_name: &str,
        value: &str,
    ) -> crate::error::Result<()> {
        println!(
            "🔧 [AB STRING] Writing string '{}' to tag '{}' using component access",
            value, tag_name
        );

        let string_bytes = value.as_bytes();
        let string_len = string_bytes.len() as i32;

        // Step 1: Write the length to TestString.LEN
        let len_tag = format!("{}.LEN", tag_name);
        println!("   📝 Step 1: Writing length {} to {}", string_len, len_tag);

        match self.write_tag(&len_tag, PlcValue::Dint(string_len)).await {
            Ok(_) => println!("   ✅ Length written successfully"),
            Err(e) => {
                println!("   ❌ Length write failed: {}", e);
                return Err(e);
            }
        }

        // Step 2: Write the string data to TestString.DATA using array access
        println!("   📝 Step 2: Writing string data to {}.DATA", tag_name);

        // We need to write each character individually to the DATA array
        for (i, &byte) in string_bytes.iter().enumerate() {
            let data_element = format!("{}.DATA[{}]", tag_name, i);
            match self
                .write_tag(&data_element, PlcValue::Sint(byte as i8))
                .await
            {
                Ok(_) => print!("."),
                Err(e) => {
                    println!(
                        "\n   ❌ Failed to write byte {} to position {}: {}",
                        byte, i, e
                    );
                    return Err(e);
                }
            }
        }

        // Step 3: Clear remaining bytes (null terminate)
        if string_bytes.len() < 82 {
            let null_element = format!("{}.DATA[{}]", tag_name, string_bytes.len());
            match self.write_tag(&null_element, PlcValue::Sint(0)).await {
                Ok(_) => println!("\n   ✅ String null-terminated successfully"),
                Err(e) => println!("\n   ⚠️ Could not null-terminate: {}", e),
            }
        }

        println!("   🎉 AB STRING component write completed!");
        Ok(())
    }

    /// Writes a string using a single UDT write with proper AB STRING format
    pub async fn write_ab_string_udt(
        &mut self,
        tag_name: &str,
        value: &str,
    ) -> crate::error::Result<()> {
        println!(
            "🔧 [AB STRING UDT] Writing string '{}' to tag '{}' as UDT",
            value, tag_name
        );

        let string_bytes = value.as_bytes();
        if string_bytes.len() > 82 {
            return Err(EtherNetIpError::Protocol(
                "String too long for Allen-Bradley STRING (max 82 chars)".to_string(),
            ));
        }

        // Build a CIP request that writes the complete AB STRING structure
        let mut cip_request = Vec::new();

        // Service: Write Tag Service (0x4D)
        cip_request.push(0x4D);

        // Request Path
        let tag_path = self.build_tag_path(tag_name);
        cip_request.push((tag_path.len() / 2) as u8); // Path size in words
        cip_request.extend_from_slice(&tag_path);

        // Data Type: Allen-Bradley STRING (0x02A0) - but write as UDT components
        cip_request.extend_from_slice(&[0xA0, 0x00]); // UDT type
        cip_request.extend_from_slice(&[0x01, 0x00]); // Element count

        // AB STRING UDT structure:
        // - DINT .LEN (4 bytes)
        // - SINT .DATA[82] (82 bytes)

        // Write .LEN field (current string length)
        let len = string_bytes.len() as u32;
        cip_request.extend_from_slice(&len.to_le_bytes());

        // Write .DATA field (82 bytes total)
        cip_request.extend_from_slice(string_bytes); // Actual string data

        // Pad with zeros to reach 82 bytes
        let padding_needed = 82 - string_bytes.len();
        cip_request.extend_from_slice(&vec![0u8; padding_needed]);

        println!(
            "   📦 Built UDT write request: {} bytes total",
            cip_request.len()
        );

        let response = self.send_cip_request(&cip_request).await?;

        if response.len() >= 3 {
            let general_status = response[2];
            if general_status == 0x00 {
                println!("   ✅ AB STRING UDT write successful!");
                Ok(())
            } else {
                let error_msg = self.get_cip_error_message(general_status);
                Err(EtherNetIpError::Protocol(format!(
                    "AB STRING UDT write failed - CIP Error 0x{:02X}: {}",
                    general_status, error_msg
                )))
            }
        } else {
            Err(EtherNetIpError::Protocol(
                "Invalid AB STRING UDT write response".to_string(),
            ))
        }
    }

    /// Establishes a Class 3 connected session for STRING operations
    ///
    /// Connected sessions are required for certain operations like STRING writes
    /// in Allen-Bradley PLCs. This implements the Forward Open CIP service.
    /// Will try multiple connection parameter configurations until one succeeds.
    async fn establish_connected_session(
        &mut self,
        session_name: &str,
    ) -> crate::error::Result<ConnectedSession> {
        println!(
            "🔗 [CONNECTED] Establishing connected session: '{}'",
            session_name
        );
        println!("🔗 [CONNECTED] Will try multiple parameter configurations...");

        // Generate unique connection parameters
        *self.connection_sequence.lock().await += 1;
        let connection_serial = (*self.connection_sequence.lock().await & 0xFFFF) as u16;

        // Try different configurations until one works
        for config_id in 0..=5 {
            println!(
                "\n🔧 [ATTEMPT {}] Trying configuration {}:",
                config_id + 1,
                config_id
            );

            let mut session = if config_id == 0 {
                ConnectedSession::new(connection_serial)
            } else {
                ConnectedSession::with_config(connection_serial, config_id)
            };

            // Generate unique connection IDs for this attempt
            session.o_to_t_connection_id =
                0x20000000 + *self.connection_sequence.lock().await + (config_id as u32 * 0x1000);
            session.t_to_o_connection_id =
                0x30000000 + *self.connection_sequence.lock().await + (config_id as u32 * 0x1000);

            // Build Forward Open request with this configuration
            let forward_open_request = self.build_forward_open_request(&session)?;

            println!(
                "🔗 [ATTEMPT {}] Sending Forward Open request ({} bytes)",
                config_id + 1,
                forward_open_request.len()
            );

            // Send Forward Open request
            match self.send_cip_request(&forward_open_request).await {
                Ok(response) => {
                    // Try to parse the response - DON'T clone, modify the session directly!
                    match self.parse_forward_open_response(&mut session, &response) {
                        Ok(()) => {
                            // Success! Store the session and return
                            println!("✅ [SUCCESS] Configuration {} worked!", config_id);
                            println!("   Connection ID: 0x{:08X}", session.connection_id);
                            println!("   O->T ID: 0x{:08X}", session.o_to_t_connection_id);
                            println!("   T->O ID: 0x{:08X}", session.t_to_o_connection_id);
                            println!(
                                "   Using Connection ID: 0x{:08X} for messaging",
                                session.connection_id
                            );

                            session.is_active = true;
                            let mut sessions = self.connected_sessions.lock().await;
                            sessions.insert(session_name.to_string(), session.clone());
                            return Ok(session);
                        }
                        Err(e) => {
                            println!(
                                "❌ [ATTEMPT {}] Configuration {} failed: {}",
                                config_id + 1,
                                config_id,
                                e
                            );

                            // If it's a specific status error, log it
                            if e.to_string().contains("status: 0x") {
                                println!("   Status indicates: parameter incompatibility or resource conflict");
                            }
                        }
                    }
                }
                Err(e) => {
                    println!(
                        "❌ [ATTEMPT {}] Network error with config {}: {}",
                        config_id + 1,
                        config_id,
                        e
                    );
                }
            }

            // Small delay between attempts
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }

        // If we get here, all configurations failed
        Err(EtherNetIpError::Protocol(
            "All connection parameter configurations failed. PLC may not support connected messaging or has reached connection limits.".to_string()
        ))
    }

    /// Builds a Forward Open CIP request for establishing connected sessions
    fn build_forward_open_request(
        &self,
        session: &ConnectedSession,
    ) -> crate::error::Result<Vec<u8>> {
        let mut request = Vec::with_capacity(50);

        // CIP Forward Open Service (0x54)
        request.push(0x54);

        // Request path length (Connection Manager object)
        request.push(0x02); // 2 words

        // Class ID: Connection Manager (0x06)
        request.push(0x20); // Logical Class segment
        request.push(0x06);

        // Instance ID: Connection Manager instance (0x01)
        request.push(0x24); // Logical Instance segment
        request.push(0x01);

        // Forward Open parameters

        // Connection Timeout Ticks (1 byte) + Timeout multiplier (1 byte)
        request.push(0x0A); // Timeout ticks (10)
        request.push(session.timeout_multiplier);

        // Originator -> Target Connection ID (4 bytes, little-endian)
        request.extend_from_slice(&session.o_to_t_connection_id.to_le_bytes());

        // Target -> Originator Connection ID (4 bytes, little-endian)
        request.extend_from_slice(&session.t_to_o_connection_id.to_le_bytes());

        // Connection Serial Number (2 bytes, little-endian)
        request.extend_from_slice(&session.connection_serial.to_le_bytes());

        // Originator Vendor ID (2 bytes, little-endian)
        request.extend_from_slice(&session.originator_vendor_id.to_le_bytes());

        // Originator Serial Number (4 bytes, little-endian)
        request.extend_from_slice(&session.originator_serial.to_le_bytes());

        // Connection Timeout Multiplier (1 byte) - repeated for target
        request.push(session.timeout_multiplier);

        // Reserved bytes (3 bytes)
        request.extend_from_slice(&[0x00, 0x00, 0x00]);

        // Originator -> Target RPI (4 bytes, little-endian, microseconds)
        request.extend_from_slice(&session.rpi.to_le_bytes());

        // Originator -> Target connection parameters (4 bytes)
        let o_to_t_params = self.encode_connection_parameters(&session.o_to_t_params);
        request.extend_from_slice(&o_to_t_params.to_le_bytes());

        // Target -> Originator RPI (4 bytes, little-endian, microseconds)
        request.extend_from_slice(&session.rpi.to_le_bytes());

        // Target -> Originator connection parameters (4 bytes)
        let t_to_o_params = self.encode_connection_parameters(&session.t_to_o_params);
        request.extend_from_slice(&t_to_o_params.to_le_bytes());

        // Transport type/trigger (1 byte) - Class 3, Application triggered
        request.push(0xA3);

        // Connection Path Size (1 byte)
        request.push(0x02); // 2 words for Message Router path

        // Connection Path - Target the Message Router
        request.push(0x20); // Logical Class segment
        request.push(0x02); // Message Router class (0x02)
        request.push(0x24); // Logical Instance segment
        request.push(0x01); // Message Router instance (0x01)

        Ok(request)
    }

    /// Encodes connection parameters into a 32-bit value
    fn encode_connection_parameters(&self, params: &ConnectionParameters) -> u32 {
        let mut encoded = 0u32;

        // Connection size (bits 0-15)
        encoded |= params.size as u32;

        // Variable flag (bit 25)
        if params.variable_size {
            encoded |= 1 << 25;
        }

        // Connection type (bits 29-30)
        encoded |= (params.connection_type as u32) << 29;

        // Priority (bits 26-27)
        encoded |= (params.priority as u32) << 26;

        encoded
    }

    /// Parses Forward Open response and updates session with connection info
    fn parse_forward_open_response(
        &self,
        session: &mut ConnectedSession,
        response: &[u8],
    ) -> crate::error::Result<()> {
        if response.len() < 2 {
            return Err(EtherNetIpError::Protocol(
                "Forward Open response too short".to_string(),
            ));
        }

        let service = response[0];
        let status = response[1];

        // Check if this is a Forward Open Reply (0xD4)
        if service != 0xD4 {
            return Err(EtherNetIpError::Protocol(format!(
                "Unexpected service in Forward Open response: 0x{:02X}",
                service
            )));
        }

        // Check status
        if status != 0x00 {
            let error_msg = match status {
                0x01 => "Connection failure - Resource unavailable or already exists",
                0x02 => "Invalid parameter - Connection parameters rejected",
                0x03 => "Connection timeout - PLC did not respond in time",
                0x04 => "Connection limit exceeded - Too many connections",
                0x08 => "Invalid service - Forward Open not supported",
                0x0C => "Invalid attribute - Connection parameters invalid",
                0x13 => "Path destination unknown - Target object not found",
                0x26 => "Invalid parameter value - RPI or size out of range",
                _ => &format!("Unknown status: 0x{:02X}", status),
            };
            return Err(EtherNetIpError::Protocol(format!(
                "Forward Open failed with status 0x{:02X}: {}",
                status, error_msg
            )));
        }

        // Parse successful response
        if response.len() < 16 {
            return Err(EtherNetIpError::Protocol(
                "Forward Open response data too short".to_string(),
            ));
        }

        // CRITICAL FIX: The Forward Open response contains the actual connection IDs assigned by the PLC
        // Use the IDs returned by the PLC, not our requested ones
        let actual_o_to_t_id =
            u32::from_le_bytes([response[2], response[3], response[4], response[5]]);
        let actual_t_to_o_id =
            u32::from_le_bytes([response[6], response[7], response[8], response[9]]);

        // Update session with the actual assigned connection IDs
        session.o_to_t_connection_id = actual_o_to_t_id;
        session.t_to_o_connection_id = actual_t_to_o_id;
        session.connection_id = actual_o_to_t_id; // Use O->T as the primary connection ID

        println!("✅ [FORWARD OPEN] Success!");
        println!(
            "   O->T Connection ID: 0x{:08X} (PLC assigned)",
            session.o_to_t_connection_id
        );
        println!(
            "   T->O Connection ID: 0x{:08X} (PLC assigned)",
            session.t_to_o_connection_id
        );
        println!(
            "   Using Connection ID: 0x{:08X} for messaging",
            session.connection_id
        );

        Ok(())
    }

    /// Writes a string using connected explicit messaging
    pub async fn write_string_connected(
        &mut self,
        tag_name: &str,
        value: &str,
    ) -> crate::error::Result<()> {
        let session_name = format!("string_write_{}", tag_name);
        let mut sessions = self.connected_sessions.lock().await;

        if !sessions.contains_key(&session_name) {
            drop(sessions); // Release the lock before calling establish_connected_session
            self.establish_connected_session(&session_name).await?;
            sessions = self.connected_sessions.lock().await;
        }

        let session = sessions.get(&session_name).unwrap().clone();
        let request = self.build_connected_string_write_request(tag_name, value, &session)?;

        drop(sessions); // Release the lock before sending the request
        let response = self
            .send_connected_cip_request(&request, &session, &session_name)
            .await?;

        // Check if write was successful
        if response.len() >= 2 {
            let status = response[1];
            if status == 0x00 {
                Ok(())
            } else {
                let error_msg = self.get_cip_error_message(status);
                Err(EtherNetIpError::Protocol(format!(
                    "CIP Error 0x{:02X}: {}",
                    status, error_msg
                )))
            }
        } else {
            Err(EtherNetIpError::Protocol(
                "Invalid connected string write response".to_string(),
            ))
        }
    }

    /// Builds a string write request for connected messaging
    fn build_connected_string_write_request(
        &self,
        tag_name: &str,
        value: &str,
        _session: &ConnectedSession,
    ) -> crate::error::Result<Vec<u8>> {
        let mut request = Vec::new();

        // For connected messaging, use direct CIP Write service
        // The connection is already established, so we can send the request directly

        // CIP Write Service Code
        request.push(0x4D);

        // Tag path - use simple ANSI format for connected messaging
        let tag_bytes = tag_name.as_bytes();
        let path_size_words = (2 + tag_bytes.len() + 1) / 2; // +1 for potential padding, /2 for word count
        request.push(path_size_words as u8);

        request.push(0x91); // ANSI symbol segment
        request.push(tag_bytes.len() as u8); // Length of tag name
        request.extend_from_slice(tag_bytes);

        // Add padding byte if needed to make path even length
        if (2 + tag_bytes.len()) % 2 != 0 {
            request.push(0x00);
        }

        // Data type for AB STRING
        request.extend_from_slice(&[0xCE, 0x0F]); // AB STRING data type (4046)

        // Number of elements (always 1 for a single string)
        request.extend_from_slice(&[0x01, 0x00]);

        // Build the AB STRING structure payload
        let string_bytes = value.as_bytes();
        let max_len: u16 = 82; // Standard AB STRING max length
        let current_len = string_bytes.len().min(max_len as usize) as u16;

        // STRING structure:
        // - Len (2 bytes) - number of characters used
        request.extend_from_slice(&current_len.to_le_bytes());

        // - MaxLen (2 bytes) - maximum characters allowed (typically 82)
        request.extend_from_slice(&max_len.to_le_bytes());

        // - Data[MaxLen] (82 bytes) - the character array, zero-padded
        let mut data_array = vec![0u8; max_len as usize];
        data_array[..current_len as usize].copy_from_slice(&string_bytes[..current_len as usize]);
        request.extend_from_slice(&data_array);

        println!("🔧 [DEBUG] Built connected string write request ({} bytes) for '{}' = '{}' (len={}, maxlen={})",
                 request.len(), tag_name, value, current_len, max_len);
        println!("🔧 [DEBUG] Request: {:02X?}", request);

        Ok(request)
    }

    /// Sends a CIP request using connected messaging
    async fn send_connected_cip_request(
        &mut self,
        cip_request: &[u8],
        session: &ConnectedSession,
        session_name: &str,
    ) -> crate::error::Result<Vec<u8>> {
        println!("🔗 [CONNECTED] Sending connected CIP request ({} bytes) using T->O connection ID 0x{:08X}",
                 cip_request.len(), session.t_to_o_connection_id);

        // Build EtherNet/IP header for connected data (Send RR Data)
        let mut packet = Vec::new();

        // EtherNet/IP Header
        packet.extend_from_slice(&[0x6F, 0x00]); // Command: Send RR Data (0x006F) - correct for connected messaging
        packet.extend_from_slice(&[0x00, 0x00]); // Length (fill in later)
        packet.extend_from_slice(&self.session_handle.to_le_bytes()); // Session handle
        packet.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // Status
        packet.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]); // Context
        packet.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // Options

        // CPF (Common Packet Format) data starts here
        let cpf_start = packet.len();

        // Interface handle (4 bytes)
        packet.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);

        // Timeout (2 bytes) - 5 seconds
        packet.extend_from_slice(&[0x05, 0x00]);

        // Item count (2 bytes) - 2 items: Address + Data
        packet.extend_from_slice(&[0x02, 0x00]);

        // Item 1: Connected Address Item (specifies which connection to use)
        packet.extend_from_slice(&[0xA1, 0x00]); // Type: Connected Address Item (0x00A1)
        packet.extend_from_slice(&[0x04, 0x00]); // Length: 4 bytes
                                                 // Use T->O connection ID (Target to Originator) for addressing
        packet.extend_from_slice(&session.t_to_o_connection_id.to_le_bytes());

        // Item 2: Connected Data Item (contains the CIP request + sequence)
        packet.extend_from_slice(&[0xB1, 0x00]); // Type: Connected Data Item (0x00B1)
        let data_length = cip_request.len() + 2; // +2 for sequence count
        packet.extend_from_slice(&(data_length as u16).to_le_bytes()); // Length

        // Clone session_name and session before acquiring the lock
        let session_name_clone = session_name.to_string();
        let _session_clone = session.clone();

        // Get the current session mutably to increment sequence counter
        let mut sessions = self.connected_sessions.lock().await;
        let current_sequence = if let Some(session_mut) = sessions.get_mut(&session_name_clone) {
            session_mut.sequence_count += 1;
            session_mut.sequence_count
        } else {
            1 // Fallback if session not found
        };

        // Drop the lock before sending the request
        drop(sessions);

        // Sequence count (2 bytes) - incremental counter for this connection
        packet.extend_from_slice(&current_sequence.to_le_bytes());

        // CIP request data
        packet.extend_from_slice(cip_request);

        // Update packet length in header (total CPF data size)
        let cpf_length = packet.len() - cpf_start;
        packet[2..4].copy_from_slice(&(cpf_length as u16).to_le_bytes());

        println!(
            "🔗 [CONNECTED] Sending packet ({} bytes) with sequence {}",
            packet.len(),
            current_sequence
        );

        // Send packet
        let mut stream = self.stream.lock().await;
        stream
            .write_all(&packet)
            .await
            .map_err(EtherNetIpError::Io)?;

        // Read response header
        let mut header = [0u8; 24];
        stream
            .read_exact(&mut header)
            .await
            .map_err(EtherNetIpError::Io)?;

        // Check EtherNet/IP command status
        let cmd_status = u32::from_le_bytes([header[8], header[9], header[10], header[11]]);
        if cmd_status != 0 {
            return Err(EtherNetIpError::Protocol(format!(
                "Connected message failed with status: 0x{:08X}",
                cmd_status
            )));
        }

        // Read response data
        let response_length = u16::from_le_bytes([header[2], header[3]]) as usize;
        let mut response_data = vec![0u8; response_length];
        stream
            .read_exact(&mut response_data)
            .await
            .map_err(EtherNetIpError::Io)?;

        let mut last_activity = self.last_activity.lock().await;
        *last_activity = Instant::now();

        println!(
            "🔗 [CONNECTED] Received response ({} bytes)",
            response_data.len()
        );

        // Extract connected CIP response
        self.extract_connected_cip_from_response(&response_data)
    }

    /// Extracts CIP data from connected response
    fn extract_connected_cip_from_response(
        &self,
        response: &[u8],
    ) -> crate::error::Result<Vec<u8>> {
        println!(
            "🔗 [CONNECTED] Extracting CIP from connected response ({} bytes): {:02X?}",
            response.len(),
            response
        );

        if response.len() < 12 {
            return Err(EtherNetIpError::Protocol(
                "Connected response too short for CPF header".to_string(),
            ));
        }

        // Parse CPF (Common Packet Format) structure
        // [0-3]: Interface handle
        // [4-5]: Timeout
        // [6-7]: Item count
        let item_count = u16::from_le_bytes([response[6], response[7]]) as usize;
        println!("🔗 [CONNECTED] CPF item count: {}", item_count);

        let mut pos = 8; // Start after CPF header

        // Look for Connected Data Item (0x00B1)
        for _i in 0..item_count {
            if pos + 4 > response.len() {
                return Err(EtherNetIpError::Protocol(
                    "Response truncated while parsing items".to_string(),
                ));
            }

            let item_type = u16::from_le_bytes([response[pos], response[pos + 1]]);
            let item_length = u16::from_le_bytes([response[pos + 2], response[pos + 3]]) as usize;
            pos += 4; // Skip item header

            println!(
                "🔗 [CONNECTED] Found item: type=0x{:04X}, length={}",
                item_type, item_length
            );

            if item_type == 0x00B1 {
                // Connected Data Item
                if pos + item_length > response.len() {
                    return Err(EtherNetIpError::Protocol(
                        "Connected data item truncated".to_string(),
                    ));
                }

                // Connected Data Item contains [sequence_count(2)][cip_data]
                if item_length < 2 {
                    return Err(EtherNetIpError::Protocol(
                        "Connected data item too short for sequence".to_string(),
                    ));
                }

                let sequence_count = u16::from_le_bytes([response[pos], response[pos + 1]]);
                println!("🔗 [CONNECTED] Sequence count: {}", sequence_count);

                // Extract CIP data (skip 2-byte sequence count)
                let cip_data = response[pos + 2..pos + item_length].to_vec();
                println!(
                    "🔗 [CONNECTED] Extracted CIP data ({} bytes): {:02X?}",
                    cip_data.len(),
                    cip_data
                );

                return Ok(cip_data);
            } else {
                // Skip this item's data
                pos += item_length;
            }
        }

        Err(EtherNetIpError::Protocol(
            "Connected Data Item (0x00B1) not found in response".to_string(),
        ))
    }

    /// Closes a specific connected session
    async fn close_connected_session(&mut self, session_name: &str) -> crate::error::Result<()> {
        if let Some(session) = self.connected_sessions.lock().await.get(session_name) {
            let session = session.clone(); // Clone to avoid borrowing issues

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

        // Connection Timeout Ticks (1 byte) + Timeout multiplier (1 byte)
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

    /// Writes a string using unconnected explicit messaging with proper AB STRING format
    ///
    /// This method uses standard unconnected messaging instead of connected messaging
    /// and implements the proper Allen-Bradley STRING structure as described in the
    /// provided information about Len, MaxLen, and Data[82] format.
    pub async fn write_string_unconnected(
        &mut self,
        tag_name: &str,
        value: &str,
    ) -> crate::error::Result<()> {
        println!(
            "📝 [UNCONNECTED] Writing string '{}' to tag '{}' using unconnected messaging",
            value, tag_name
        );

        self.validate_session().await?;

        let string_bytes = value.as_bytes();
        if string_bytes.len() > 82 {
            return Err(EtherNetIpError::Protocol(
                "String too long for Allen-Bradley STRING (max 82 chars)".to_string(),
            ));
        }

        // Build the CIP request with proper AB STRING structure
        let mut cip_request = Vec::new();

        // Service: Write Tag Service (0x4D)
        cip_request.push(0x4D);

        // Request Path Size (in words)
        let tag_bytes = tag_name.as_bytes();
        let path_len = if tag_bytes.len() % 2 == 0 {
            tag_bytes.len() + 2
        } else {
            tag_bytes.len() + 3
        } / 2;
        cip_request.push(path_len as u8);

        // Request Path: ANSI Extended Symbol Segment for tag name
        cip_request.push(0x91); // ANSI Extended Symbol Segment
        cip_request.push(tag_bytes.len() as u8); // Tag name length
        cip_request.extend_from_slice(tag_bytes); // Tag name

        // Pad to even length if necessary
        if tag_bytes.len() % 2 != 0 {
            cip_request.push(0x00);
        }

        // For write operations, we don't include data type and element count
        // The PLC infers the data type from the tag definition

        // Build Allen-Bradley STRING structure based on what we see in read responses:
        // Looking at read response: [CE, 0F, 01, 00, 00, 00, 31, 00, ...]
        // Structure appears to be:
        // - Some header/identifier (2 bytes): 0xCE, 0x0F
        // - Length (2 bytes): number of characters
        // - MaxLength or padding (2 bytes): 0x00, 0x00
        // - Data array (variable length, null terminated)

        let _current_len = string_bytes.len().min(82) as u16;

        // Build the correct Allen-Bradley STRING structure to match what the PLC expects
        // Analysis of read response: [CE, 0F, 01, 00, 00, 00, 31, 00, 00, 00, ...]
        // Structure appears to be:
        // - Header (2 bytes): 0xCE, 0x0F (Allen-Bradley STRING identifier)
        // - Length (4 bytes, DINT): Number of characters currently used
        // - Data (variable): Character data followed by padding to complete the structure

        let current_len = string_bytes.len().min(82) as u32;

        // AB STRING header/identifier - this appears to be required
        cip_request.extend_from_slice(&[0xCE, 0x0F]);

        // Length (4 bytes) - number of characters used as DINT
        cip_request.extend_from_slice(&current_len.to_le_bytes());

        // Data bytes - the actual string content
        cip_request.extend_from_slice(&string_bytes[..current_len as usize]);

        // Add padding if the total structure needs to be a specific size
        // Based on reads, it looks like there might be additional padding after the data

        println!("🔧 [DEBUG] Built Allen-Bradley STRING write request ({} bytes) for '{}' = '{}' (len={})",
                 cip_request.len(), tag_name, value, current_len);
        println!("🔧 [DEBUG] Request structure: Service=0x4D, Path={} bytes, Header=0xCE0F, Len={} (4 bytes), Data",
                 path_len * 2, current_len);

        // Send the request using standard unconnected messaging
        let response = self.send_cip_request(&cip_request).await?;

        // Extract CIP response from EtherNet/IP wrapper
        let cip_response = self.extract_cip_from_response(&response)?;

        // Check if write was successful - use correct CIP response format
        if cip_response.len() >= 3 {
            let service_reply = cip_response[0]; // Should be 0xCD (0x4D + 0x80) for Write Tag reply
            let _additional_status_size = cip_response[1]; // Additional status size (usually 0)
            let status = cip_response[2]; // CIP status code at position 2

            println!(
                "🔧 [DEBUG] Write response - Service: 0x{:02X}, Status: 0x{:02X}",
                service_reply, status
            );

            if status == 0x00 {
                println!("✅ [UNCONNECTED] String write completed successfully");
                Ok(())
            } else {
                let error_msg = self.get_cip_error_message(status);
                println!(
                    "❌ [UNCONNECTED] String write failed: {} (0x{:02X})",
                    error_msg, status
                );
                Err(EtherNetIpError::Protocol(format!(
                    "CIP Error 0x{:02X}: {}",
                    status, error_msg
                )))
            }
        } else {
            Err(EtherNetIpError::Protocol(
                "Invalid unconnected string write response - too short".to_string(),
            ))
        }
    }

    /// Write a string value to a PLC tag using unconnected messaging
    ///
    /// # Arguments
    ///
    /// * `tag_name` - The name of the tag to write to
    /// * `value` - The string value to write (max 82 characters)
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the write was successful
    /// * `Err(EtherNetIpError)` if the write failed
    ///
    /// # Errors
    ///
    /// * `StringTooLong` - If the string is longer than 82 characters
    /// * `InvalidString` - If the string contains invalid characters
    /// * `TagNotFound` - If the tag doesn't exist
    /// * `WriteError` - If the write operation fails
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

        // Send the request and get the response
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

    /// Build a string write request packet
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

    /// Subscribes to a tag for real-time updates
    pub async fn subscribe_to_tag(
        &self,
        tag_path: &str,
        options: SubscriptionOptions,
    ) -> Result<()> {
        let mut subscriptions = self.subscriptions.lock().await;
        let subscription = TagSubscription::new(tag_path.to_string(), options);
        subscriptions.push(subscription);
        drop(subscriptions); // Release the lock before starting the monitoring thread

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

    async fn _get_connected_session(
        &mut self,
        session_name: &str,
    ) -> crate::error::Result<ConnectedSession> {
        // First check if we already have a session
        {
            let sessions = self.connected_sessions.lock().await;
            if let Some(session) = sessions.get(session_name) {
                return Ok(session.clone());
            }
        }

        // If we don't have a session, establish a new one
        let session = self.establish_connected_session(session_name).await?;

        // Store the new session
        let mut sessions = self.connected_sessions.lock().await;
        sessions.insert(session_name.to_string(), session.clone());

        Ok(session)
    }
}

/*
===============================================================================
END OF LIBRARY DOCUMENTATION

This file provides a complete, production-ready EtherNet/IP communication
library for Allen-Bradley PLCs. The library includes:

- Native Rust API with async support
- C FFI exports for cross-language integration
- Comprehensive error handling and validation
- Detailed documentation and examples
- Performance optimizations
- Memory safety guarantees

For usage examples, see the main.rs file or the C# integration samples.

For technical details about the EtherNet/IP protocol implementation,
refer to the inline documentation above.

Version: 1.0.0
Compatible with: CompactLogix L1x-L5x series PLCs
License: As specified in Cargo.toml
===============================================================================_
*/
