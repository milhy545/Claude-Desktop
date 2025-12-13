use crate::mcp::McpServer;
use crate::system::SystemOps;
use std::sync::Arc;
use tokio::sync::RwLock;

// Globální stav aplikace
pub struct AppState {
    pub session: RwLock<Option<String>>,
    pub mcp_servers: RwLock<Vec<McpServer>>,
    // Abstrakce pro systémové operace (I/O, Process)
    pub sys: Arc<dyn SystemOps>,
}
