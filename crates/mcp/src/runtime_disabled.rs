use cloud_object_models::TransportType;
use simple_logger::SimpleLogger;
use uuid::Uuid;

use crate::TemplatableMCPServerInfo;
use crate::oauth::AuthContext;

pub fn error_to_user_message(error: &rmcp::RmcpError) -> String {
    error.to_string()
}

/// Fails without launching a server when MCP transports are disabled.
#[allow(clippy::result_large_err)]
pub async fn spawn_server(
    server_name: String,
    description: Option<String>,
    uuid: Uuid,
    transport_type: TransportType,
    logger: SimpleLogger,
    auth_context: Option<AuthContext>,
) -> Result<TemplatableMCPServerInfo, rmcp::RmcpError> {
    drop((description, uuid, transport_type, logger, auth_context));
    Err(rmcp::RmcpError::transport_creation::<()>(format!(
        "MCP transports are disabled in this build; server '{server_name}' was not started"
    )))
}
