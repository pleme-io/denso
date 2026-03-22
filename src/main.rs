use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(name = "denso", about = "Wireless ADB manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Discover devices on the network via mDNS
    Discover,
    /// Pair with a device using a pairing code
    Pair,
    /// Connect to a wireless ADB device
    Connect,
}

/// A wireless ADB device connection.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeviceConnection {
    pub serial: String,
    pub host: String,
    pub port: u16,
    pub connected: bool,
    pub last_seen: String,
}

/// Manages a collection of wireless ADB device connections.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConnectionManager {
    pub connections: Vec<DeviceConnection>,
}

impl ConnectionManager {
    /// Create a new empty connection manager.
    #[must_use]
    pub fn new() -> Self {
        Self {
            connections: Vec::new(),
        }
    }

    /// Add or update a device connection. If a connection with the same
    /// serial already exists, it is replaced.
    pub fn add(&mut self, connection: DeviceConnection) {
        if let Some(existing) = self.connections.iter_mut().find(|c| c.serial == connection.serial)
        {
            *existing = connection;
        } else {
            self.connections.push(connection);
        }
    }

    /// Remove a device connection by serial. Returns true if removed.
    pub fn remove(&mut self, serial: &str) -> bool {
        let len_before = self.connections.len();
        self.connections.retain(|c| c.serial != serial);
        self.connections.len() < len_before
    }

    /// Find a device connection by serial.
    #[must_use]
    pub fn find(&self, serial: &str) -> Option<&DeviceConnection> {
        self.connections.iter().find(|c| c.serial == serial)
    }

    /// Count the number of currently connected devices.
    #[must_use]
    pub fn connected_count(&self) -> usize {
        self.connections.iter().filter(|c| c.connected).count()
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Discover => {
            println!("denso: discovering devices");
        }
        Commands::Pair => {
            println!("denso: pairing with device");
        }
        Commands::Connect => {
            println!("denso: connecting to device");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_device(serial: &str, connected: bool) -> DeviceConnection {
        DeviceConnection {
            serial: serial.to_string(),
            host: "192.168.1.100".to_string(),
            port: 5555,
            connected,
            last_seen: "2026-03-22T00:00:00Z".to_string(),
        }
    }

    #[test]
    fn add_connection() {
        let mut mgr = ConnectionManager::new();
        mgr.add(test_device("pixel7-abc", true));
        assert_eq!(mgr.connections.len(), 1);
        assert_eq!(mgr.connections[0].serial, "pixel7-abc");
    }

    #[test]
    fn remove_connection() {
        let mut mgr = ConnectionManager::new();
        mgr.add(test_device("pixel7-abc", true));
        mgr.add(test_device("pixel8-xyz", false));
        assert!(mgr.remove("pixel7-abc"));
        assert_eq!(mgr.connections.len(), 1);
        assert!(!mgr.remove("nonexistent"));
    }

    #[test]
    fn find_by_serial() {
        let mut mgr = ConnectionManager::new();
        mgr.add(test_device("pixel7-abc", true));
        mgr.add(test_device("pixel8-xyz", false));
        let found = mgr.find("pixel8-xyz");
        assert!(found.is_some());
        assert_eq!(found.unwrap().serial, "pixel8-xyz");
        assert!(mgr.find("nonexistent").is_none());
    }

    #[test]
    fn connected_count() {
        let mut mgr = ConnectionManager::new();
        mgr.add(test_device("a", true));
        mgr.add(test_device("b", false));
        mgr.add(test_device("c", true));
        assert_eq!(mgr.connected_count(), 2);
    }

    #[test]
    fn add_duplicate_serial_updates() {
        let mut mgr = ConnectionManager::new();
        mgr.add(test_device("pixel7-abc", false));
        assert!(!mgr.find("pixel7-abc").unwrap().connected);

        mgr.add(DeviceConnection {
            serial: "pixel7-abc".to_string(),
            host: "192.168.1.200".to_string(),
            port: 5556,
            connected: true,
            last_seen: "2026-03-22T01:00:00Z".to_string(),
        });
        assert_eq!(mgr.connections.len(), 1);
        let updated = mgr.find("pixel7-abc").unwrap();
        assert!(updated.connected);
        assert_eq!(updated.host, "192.168.1.200");
        assert_eq!(updated.port, 5556);
    }
}
