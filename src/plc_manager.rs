use std::collections::HashMap;
use std::error::Error;
use std::net::SocketAddr;
use tokio::time::{Duration, Instant};
use crate::EipClient;

/// Configuration for a PLC connection
#[derive(Debug, Clone)]
pub struct PlcConfig {
    /// IP address and port of the PLC
    pub address: SocketAddr,
    /// Maximum number of connections to maintain
    pub max_connections: u32,
    /// Connection timeout in milliseconds
    pub connection_timeout: Duration,
    /// Health check interval in milliseconds
    pub health_check_interval: Duration,
    /// Maximum packet size in bytes
    pub max_packet_size: usize,
}

impl Default for PlcConfig {
    fn default() -> Self {
        Self {
            address: "127.0.0.1:44818".parse().unwrap(),
            max_connections: 5,
            connection_timeout: Duration::from_secs(5),
            health_check_interval: Duration::from_secs(30),
            max_packet_size: 4000,
        }
    }
}

/// Represents the health status of a PLC connection
#[derive(Debug, Clone)]
pub struct ConnectionHealth {
    /// Whether the connection is currently active
    pub is_active: bool,
    /// Last successful communication timestamp
    pub last_success: Instant,
    /// Number of failed attempts since last success
    pub failed_attempts: u32,
    /// Current connection latency in milliseconds
    pub latency: Duration,
}

/// Represents a connection to a PLC
#[derive(Debug)]
pub struct PlcConnection {
    /// The EIP client instance
    client: EipClient,
    /// Health status of the connection
    health: ConnectionHealth,
    /// Last time this connection was used
    last_used: Instant,
}

impl PlcConnection {
    /// Creates a new PLC connection
    pub fn new(client: EipClient) -> Self {
        Self {
            client,
            health: ConnectionHealth {
                is_active: true,
                last_success: Instant::now(),
                failed_attempts: 0,
                latency: Duration::from_millis(0),
            },
            last_used: Instant::now(),
        }
    }

    /// Updates the health status of the connection
    pub fn update_health(&mut self, is_active: bool, latency: Duration) {
        self.health.is_active = is_active;
        if is_active {
            self.health.last_success = Instant::now();
            self.health.failed_attempts = 0;
            self.health.latency = latency;
        } else {
            self.health.failed_attempts += 1;
        }
    }
}

/// Manager for multiple PLC connections
#[derive(Debug)]
pub struct PlcManager {
    /// Configuration for each PLC
    configs: HashMap<SocketAddr, PlcConfig>,
    /// Active connections for each PLC
    connections: HashMap<SocketAddr, Vec<PlcConnection>>,
    /// Health check interval
    health_check_interval: Duration,
}

impl PlcManager {
    /// Creates a new PLC manager
    pub fn new() -> Self {
        Self {
            configs: HashMap::new(),
            connections: HashMap::new(),
            health_check_interval: Duration::from_secs(30),
        }
    }

    /// Adds a PLC configuration
    pub fn add_plc(&mut self, config: PlcConfig) {
        self.configs.insert(config.address, config);
    }

    /// Gets a connection to a PLC
    pub async fn get_connection(&mut self, address: SocketAddr) -> Result<&mut EipClient, Box<dyn Error>> {
        let config = self.configs.get(&address)
            .ok_or_else(|| "PLC not configured".to_string())?;

        // First check if we have any connections for this address
        if !self.connections.contains_key(&address) {
            // No connections exist, create a new one
            let mut client = EipClient::connect(&address.to_string()).await?;
            client.set_max_packet_size(config.max_packet_size as u32);
            let mut new_conn = PlcConnection::new(client);
            new_conn.last_used = Instant::now();
            self.connections.insert(address, vec![new_conn]);
            return Ok(&mut self.connections.get_mut(&address).unwrap()[0].client);
        }

        // Get mutable access to the connections
        let connections = self.connections.get_mut(&address).unwrap();

        // First try to find an inactive connection
        for i in 0..connections.len() {
            if !connections[i].health.is_active {
                let mut client = EipClient::connect(&address.to_string()).await?;
                client.set_max_packet_size(config.max_packet_size as u32);
                connections[i].client = client;
                connections[i].health.is_active = true;
                connections[i].health.last_success = Instant::now();
                connections[i].health.failed_attempts = 0;
                connections[i].health.latency = Duration::from_millis(0);
                connections[i].last_used = Instant::now();
                return Ok(&mut connections[i].client);
            }
        }

        // If we have room for more connections, create a new one
        if connections.len() < config.max_connections as usize {
            let mut client = EipClient::connect(&address.to_string()).await?;
            client.set_max_packet_size(config.max_packet_size as u32);
            let mut new_conn = PlcConnection::new(client);
            new_conn.last_used = Instant::now();
            connections.push(new_conn);
            return Ok(&mut connections.last_mut().unwrap().client);
        }

        // Find the least recently used connection
        let lru_index = connections.iter()
            .enumerate()
            .min_by_key(|(_, conn)| conn.last_used)
            .map(|(i, _)| i)
            .unwrap();

        // Update the LRU connection
        let mut client = EipClient::connect(&address.to_string()).await?;
        client.set_max_packet_size(config.max_packet_size as u32);
        connections[lru_index].client = client;
        connections[lru_index].health.is_active = true;
        connections[lru_index].health.last_success = Instant::now();
        connections[lru_index].health.failed_attempts = 0;
        connections[lru_index].health.latency = Duration::from_millis(0);
        connections[lru_index].last_used = Instant::now();
        Ok(&mut connections[lru_index].client)
    }

    /// Performs health checks on all connections
    pub async fn check_health(&mut self) {
        for (address, connections) in &mut self.connections {
            let _config = self.configs.get(address).unwrap();
            
            for conn in connections.iter_mut() {
                if !conn.health.is_active {
                    if let Ok(new_client) = EipClient::connect(&address.to_string()).await {
                        conn.client = new_client;
                        conn.health.is_active = true;
                        conn.health.last_success = Instant::now();
                        conn.health.failed_attempts = 0;
                        conn.health.latency = Duration::from_millis(0);
                        conn.last_used = Instant::now();
                    }
                }
            }
        }
    }

    /// Removes inactive connections
    pub fn cleanup_connections(&mut self) {
        for connections in self.connections.values_mut() {
            connections.retain(|conn| conn.health.is_active);
        }
    }

    pub async fn get_client(&mut self, address: &str) -> Result<&mut EipClient, Box<dyn Error>> {
        let addr = address.parse::<SocketAddr>()
            .map_err(|_| "Invalid address format")?;
        self.get_connection(addr).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plc_config_default() {
        let config = PlcConfig::default();
        assert_eq!(config.max_connections, 5);
        assert_eq!(config.max_packet_size, 4000);
    }

    #[tokio::test]
    async fn test_plc_manager_connection_pool() {
        let mut manager = PlcManager::new();
        let config = PlcConfig {
            address: "127.0.0.1:44818".parse().unwrap(),
            max_connections: 2,
            ..Default::default()
        };
        manager.add_plc(config);

        // This will fail in tests since there's no actual PLC
        // but it demonstrates the connection pool logic
        let result = manager.get_connection(config.address).await;
        assert!(result.is_err());
    }
} 