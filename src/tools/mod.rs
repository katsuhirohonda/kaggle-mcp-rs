pub mod auth;

use rmcp::prelude::*;
use crate::client::KaggleClient;
use std::sync::Arc;

pub fn register_tools(server: &mut Server<impl ToolProvider>) {
    let client = Arc::new(KaggleClient::new());
    
    // Register authentication tools
    auth::register_auth_tools(server, client.clone());
}