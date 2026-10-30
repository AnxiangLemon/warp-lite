//! Stub MCP surface for builds without the `mcp_client` feature (e.g. warp_lite).
//! Reuses non-rmcp MCP types/files where possible; managers are no-op stubs.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use uuid::Uuid;
use warp_core::ui::Icon;
use warpui::{AppContext, Entity, ModelContext, SingletonEntity};

use crate::cloud_object::model::generic_string_model::StringModel;
use crate::cloud_object::model::json_model::JsonModel;
use crate::cloud_object::{
    CloudObjectUuid, GenericStringObjectFormat, GenericStringObjectUniqueKey, JsonObjectType,
    Revision,
};
use crate::server::sync_queue::QueueItem;

#[path = "mcp/templatable.rs"]
pub mod templatable;
#[path = "mcp/templatable_installation.rs"]
pub mod templatable_installation;
#[path = "mcp/parsing.rs"]
pub mod parsing;

pub use cloud_object_models::{
    CLIServer, CloudMCPServer, CloudMCPServerModel, JSONMCPServer, JSONTransportType, MCPServer,
    MCPServerState, ServerSentEvents, StaticEnvVar, StaticHeader, TransportType,
};
pub use parsing::{ParsedTemplatableMCPServerResult, prettify_json, resolve_json};
pub(crate) use parsing::normalize_mcp_json;
pub use templatable::{
    CloudTemplatableMCPServer, CloudTemplatableMCPServerModel, GalleryData, JsonTemplate,
    TemplatableMCPServer, TemplateVariable,
};
pub use templatable_installation::{
    TemplatableMCPServerInstallation, VariableType, VariableValue,
};

pub mod file_based_manager {
    pub use super::{FileBasedMCPManager, FileBasedMCPManagerEvent};
}

pub mod file_mcp_watcher {
    pub use super::{FileMCPWatcher, FileMCPWatcherEvent};
}

pub mod gallery {
    pub use super::{MCPGalleryManager, MCPGalleryManagerEvent};
}

pub mod templatable_manager {
    pub use super::{
        FigmaMcpStatus, McpIntegration, TemplatableMCPServerManager,
        TemplatableMCPServerManagerEvent,
    };
}

pub mod builtin {
    use uuid::Uuid;

    use super::{TemplatableMCPServer, TemplatableMCPServerInstallation};
    use crate::auth::credentials::Credentials;

    pub const FACTORY_MCP_INSTALLATION_UUID: Uuid =
        Uuid::from_u128(0xfac70a11_a55e_4bde_9c3a_1c0ffee0f001);
    pub const FACTORY_MCP_SERVER_NAME: &str = "warp-factory";

    pub fn builtin_bearer_token(_credentials: &Credentials) -> Option<String> {
        None
    }

    pub fn factory_mcp_installation(_bearer_token: &str) -> TemplatableMCPServerInstallation {
        TemplatableMCPServerInstallation::new(
            FACTORY_MCP_INSTALLATION_UUID,
            TemplatableMCPServer::default(),
            Default::default(),
        )
    }
}

impl CloudObjectUuid for MCPServer {
    fn uuid(&self) -> uuid::Uuid {
        self.uuid
    }
}

impl StringModel for MCPServer {
    type CloudObjectType = CloudMCPServer;

    fn model_type_name(&self) -> &'static str {
        "MCP server"
    }

    fn should_enforce_revisions() -> bool {
        true
    }

    fn model_format() -> GenericStringObjectFormat {
        GenericStringObjectFormat::Json(JsonObjectType::MCPServer)
    }

    fn should_show_activity_toasts() -> bool {
        true
    }

    fn warn_if_unsaved_at_quit() -> bool {
        true
    }

    fn display_name(&self) -> String {
        self.name.clone()
    }

    fn update_object_queue_item(
        &self,
        revision_ts: Option<Revision>,
        object: &Self::CloudObjectType,
    ) -> QueueItem {
        QueueItem::UpdateMCPServer {
            model: object.model().clone().into(),
            id: object.id,
            revision: revision_ts.or_else(|| object.metadata.revision.clone()),
        }
    }

    fn uniqueness_key(&self) -> Option<GenericStringObjectUniqueKey> {
        None
    }
}

// JsonModel for MCPServer is provided by cloud_object_models.

#[derive(Default)]
pub struct FileBasedMCPManager {}

impl FileBasedMCPManager {
    pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
        Self {}
    }

    pub fn get_servers_for_working_directory(
        &self,
        _cwd: &Path,
        _app: &AppContext,
    ) -> Vec<&TemplatableMCPServerInstallation> {
        vec![]
    }

    pub fn file_based_servers(&self) -> Vec<&TemplatableMCPServerInstallation> {
        vec![]
    }

    pub fn get_installation_by_uuid(
        &self,
        _uuid: Uuid,
    ) -> Option<&TemplatableMCPServerInstallation> {
        None
    }

    pub fn directory_paths_for_installation_and_provider(
        &self,
        _uuid: Uuid,
        _provider: MCPProvider,
    ) -> Vec<PathBuf> {
        vec![]
    }
}

impl Entity for FileBasedMCPManager {
    type Event = FileBasedMCPManagerEvent;
}

impl SingletonEntity for FileBasedMCPManager {}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum FileBasedMCPManagerEvent {
    ServersChanged,
    ConfigDiagnosticChanged,
    SpawnServers {
        installations: Vec<TemplatableMCPServerInstallation>,
    },
    DespawnServers {
        installation_uuids: Vec<Uuid>,
    },
    PurgeCredentials {
        installation_hashes: Vec<u64>,
    },
    CloudEnvMcpScanComplete {
        repo_path: PathBuf,
        detected_servers: Vec<()>,
        wait_server_uuids: Vec<Uuid>,
    },
}

#[derive(Default)]
pub struct FileMCPWatcher {}

impl FileMCPWatcher {
    pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
        Self {}
    }
}

impl Entity for FileMCPWatcher {
    type Event = FileMCPWatcherEvent;
}

impl SingletonEntity for FileMCPWatcher {}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum FileMCPWatcherEvent {
    CloudEnvMcpScanComplete { repo_path: PathBuf },
}

#[derive(Default)]
pub struct TemplatableMCPServerManager {}

impl TemplatableMCPServerManager {
    pub fn new(
        _locally_installed_servers: HashMap<Uuid, TemplatableMCPServerInstallation>,
        _running_server_uuids: Vec<Uuid>,
        _running_legacy_servers: &[Uuid],
        _ctx: &mut ModelContext<Self>,
    ) -> Self {
        Self {}
    }

    pub fn get_all_cloud_synced_mcp_servers(_ctx: &AppContext) -> HashMap<Uuid, String> {
        HashMap::new()
    }

    pub fn get_all_runnable_mcp_servers(_ctx: &AppContext) -> Vec<(Uuid, String)> {
        vec![]
    }

    pub fn get_mcp_name(_uuid: &Uuid, _app: &AppContext) -> Option<String> {
        None
    }

    pub fn get_active_file_based_servers(
        &self,
        _cwd: &Path,
        _app: &AppContext,
    ) -> HashMap<Uuid, &TemplatableMCPServerInfo> {
        HashMap::new()
    }

    pub fn get_active_cli_spawned_servers(&self) -> HashMap<Uuid, &TemplatableMCPServerInfo> {
        HashMap::new()
    }

    pub fn get_active_builtin_servers(&self) -> HashMap<Uuid, &TemplatableMCPServerInfo> {
        HashMap::new()
    }

    pub fn get_active_templatable_servers(&self) -> HashMap<Uuid, &TemplatableMCPServerInfo> {
        HashMap::new()
    }

    pub fn resources(&self) -> impl Iterator<Item = &Resource> {
        std::iter::empty()
    }

    pub fn tools(&self) -> impl Iterator<Item = &Tool> {
        std::iter::empty()
    }

    pub fn get_all_templatable_mcp_servers(&self) -> Vec<&TemplatableMCPServer> {
        vec![]
    }

    pub fn get_templatable_mcp_server(&self, _uuid: Uuid) -> Option<&TemplatableMCPServer> {
        None
    }

    pub fn get_installation_by_template_uuid(
        &self,
        _template_uuid: Uuid,
    ) -> Option<&TemplatableMCPServerInstallation> {
        None
    }

    pub fn get_installed_server(
        &self,
        _installation_uuid: &Uuid,
    ) -> Option<&TemplatableMCPServerInstallation> {
        None
    }

    pub fn get_installed_templatable_servers(
        &self,
    ) -> &HashMap<Uuid, TemplatableMCPServerInstallation> {
        static EMPTY: std::sync::OnceLock<HashMap<Uuid, TemplatableMCPServerInstallation>> =
            std::sync::OnceLock::new();
        EMPTY.get_or_init(HashMap::new)
    }

    pub fn get_figma_installation_uuid(&self) -> Option<Uuid> {
        None
    }

    pub fn get_template_uuid(&self, _installation_uuid: Uuid) -> Option<Uuid> {
        None
    }

    pub fn is_server_active_or_pending(&self, _uuid: Uuid) -> bool {
        false
    }

    pub fn get_server_state(&self, _installation_uuid: Uuid) -> Option<MCPServerState> {
        None
    }

    pub fn get_server_error_message(&self, _installation_uuid: Uuid) -> Option<&str> {
        None
    }

    pub fn tool_input_schema(
        &self,
        _installation_id: Option<Uuid>,
        _tool_name: &str,
    ) -> Option<std::sync::Arc<serde_json::Map<String, serde_json::Value>>> {
        None
    }

    pub fn server_from_tool(&self, _tool: String) -> Option<&Uuid> {
        None
    }

    pub fn server_from_resource(&self, _name: &str, _uri: Option<&str>) -> Option<&Uuid> {
        None
    }

    pub fn spawn_server(&mut self, _installation_uuid: Uuid, _ctx: &mut ModelContext<Self>) {}

    pub fn spawn_cli_ephemeral_server(
        &mut self,
        _installation: TemplatableMCPServerInstallation,
        _ctx: &mut ModelContext<Self>,
    ) {
    }

    pub fn sync_builtin_servers(&mut self, _force_respawn: bool, _ctx: &mut ModelContext<Self>) {}

    pub fn get_figma_mcp_status(&self) -> FigmaMcpStatus {
        FigmaMcpStatus::NotInstalled
    }

    pub fn install_figma_from_gallery(&mut self, _ctx: &mut ModelContext<Self>) {}

    pub fn enable_figma_mcp(&mut self, _ctx: &mut ModelContext<Self>) {}

    pub fn handle_oauth_callback(&mut self, _url: &url::Url) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn is_mcp_server_running(&self, _integration: McpIntegration) -> bool {
        false
    }

    pub fn extract_server_info<T: Eq + std::hash::Hash>(
        &self,
        _template_fn: fn(&TemplatableMCPServer) -> Option<T>,
        _installation_fn: fn(&TemplatableMCPServerInstallation) -> Option<T>,
        _app: &AppContext,
    ) -> HashSet<T> {
        HashSet::new()
    }
}

impl Entity for TemplatableMCPServerManager {
    type Event = TemplatableMCPServerManagerEvent;
}

impl SingletonEntity for TemplatableMCPServerManager {}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum TemplatableMCPServerManagerEvent {
    StateChanged {
        uuid: Uuid,
        state: MCPServerState,
    },
    AuthenticationRequired {
        uuid: Uuid,
    },
    CredentialsChanged {
        uuid: Uuid,
    },
    ServerInstallationAdded(Uuid),
    ServerInstallationDeleted(Uuid),
    TemplatableMCPServersUpdated,
    LegacyServerConverted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FigmaMcpStatus {
    NotInstalled,
    Installed,
    Enabling,
    Running,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum McpIntegration {
    Figma,
}

#[derive(Default)]
pub struct MCPGalleryManager {}

impl MCPGalleryManager {
    pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
        Self {}
    }
}

impl Entity for MCPGalleryManager {
    type Event = MCPGalleryManagerEvent;
}

impl SingletonEntity for MCPGalleryManager {}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum MCPGalleryManagerEvent {
    Updated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum MCPProvider {
    Warp,
    Claude,
    Codex,
    Agents,
}

impl MCPProvider {
    pub fn display_name(&self) -> &str {
        match self {
            MCPProvider::Warp => "Warp",
            MCPProvider::Claude => "Claude",
            MCPProvider::Codex => "Codex",
            MCPProvider::Agents => "Other Agents",
        }
    }

    pub fn icon(&self) -> Icon {
        match self {
            MCPProvider::Warp => Icon::Agent,
            MCPProvider::Claude => Icon::ClaudeLogo,
            MCPProvider::Codex => Icon::OpenAILogo,
            MCPProvider::Agents => Icon::AiAssistant,
        }
    }

    pub fn home_config_path(&self) -> &'static Path {
        match self {
            MCPProvider::Warp => Path::new(".warp/.mcp.json"),
            MCPProvider::Claude => Path::new(".claude.json"),
            MCPProvider::Codex => Path::new(".codex/config.toml"),
            MCPProvider::Agents => Path::new(".agents/.mcp.json"),
        }
    }

    pub fn project_config_path(&self) -> &'static Path {
        match self {
            MCPProvider::Warp => Path::new(".warp/.mcp.json"),
            MCPProvider::Claude => Path::new(".mcp.json"),
            MCPProvider::Codex => Path::new(".codex/config.toml"),
            MCPProvider::Agents => Path::new(".agents/.mcp.json"),
        }
    }
}

pub(crate) fn home_config_file_path(provider: MCPProvider) -> Option<PathBuf> {
    match provider {
        MCPProvider::Warp => warp_core::paths::warp_home_mcp_config_file_path(),
        _ => dirs::home_dir().map(|home_dir| home_dir.join(provider.home_config_path())),
    }
}

pub fn mcp_provider_from_file_path(file_path: &Path) -> Option<MCPProvider> {
    for provider in MCPProvider::iter() {
        if home_config_file_path(provider)
            .as_ref()
            .is_some_and(|home_config_path| file_path == home_config_path)
        {
            return Some(provider);
        }
    }
    let mut best: Option<(MCPProvider, usize)> = None;
    for provider in MCPProvider::iter() {
        let cfg = provider.project_config_path();
        if file_path.ends_with(cfg) {
            let len = cfg.as_os_str().len();
            if best.is_none_or(|(_, best_len)| len > best_len) {
                best = Some((provider, len));
            }
        }
    }
    best.map(|(p, _)| p)
}

#[derive(Clone, Debug, Default)]
pub struct MCPServerUpdate {}

/// Stubs for MCP protocol resource/tool descriptors when `rmcp` is not linked.
#[derive(Debug, Clone, Default)]
pub struct Resource {}

#[derive(Debug, Clone, Default)]
pub struct Tool {}

#[derive(Debug, Clone, Default)]
pub struct TemplatableMCPServerInfo {
    resources: Vec<Resource>,
    tools: Vec<Tool>,
}

impl TemplatableMCPServerInfo {
    pub fn name(&self) -> &str {
        ""
    }

    pub fn description(&self) -> Option<&str> {
        None
    }

    pub fn installation_id(&self) -> Uuid {
        Uuid::nil()
    }

    pub fn resources(&self) -> &Vec<Resource> {
        &self.resources
    }

    pub fn tools(&self) -> &Vec<Tool> {
        &self.tools
    }
}