use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::mpsc::SyncSender;

use ai::workspace::WorkspaceMetadata;
use lsp::supported_servers::LSPServerType;
use serde::{Deserialize, Serialize};
use warpui::{Entity, ModelContext, SingletonEntity};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnablementState {
    Yes,
    No,
    #[default]
    Suggested,
}

#[cfg(feature = "local_fs")]
pub enum LspTask {
    Install {
        file_path: PathBuf,
        repo_root: PathBuf,
        server_type: LSPServerType,
    },
    Spawn {
        file_path: PathBuf,
    },
}

#[cfg(not(feature = "local_fs"))]
pub enum LspTask {
    Spawn {
        file_path: PathBuf,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LspRepoStatus {
    Ready,
    Enabled,
    CheckingForInstallation,
    DisabledAndInstalled { server_type: LSPServerType },
    DisabledAndNotInstalled { server_type: LSPServerType },
    Installing { server_type: LSPServerType },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LSPInstallationStatus {
    Installed,
    NotInstalled,
    Checking,
    Installing,
}

impl LspRepoStatus {
    pub fn from_installation_status(
        status: &LSPInstallationStatus,
        server_type: LSPServerType,
    ) -> Self {
        match status {
            LSPInstallationStatus::Installed => Self::DisabledAndInstalled { server_type },
            LSPInstallationStatus::NotInstalled => Self::DisabledAndNotInstalled { server_type },
            LSPInstallationStatus::Checking => Self::CheckingForInstallation,
            LSPInstallationStatus::Installing => Self::Installing { server_type },
        }
    }
}

pub enum LSPEnablementResultForFile {
    Enabled,
    UnsupportedLanguage,
    LSPNotEnabled { root_name: Option<String> },
}

#[derive(Clone, Debug)]
pub enum PersistedWorkspaceEvent {
    InstallStatusUpdate {
        server_type: LSPServerType,
        status: LSPInstallationStatus,
    },
    InstallationSucceeded,
    InstallationFailed,
    AvailableServersDetected {
        workspace_path: PathBuf,
        servers: Vec<LSPServerType>,
    },
    WorkspaceAdded { path: PathBuf },
}

struct Workspace {
    metadata: WorkspaceMetadata,
}

pub struct PersistedWorkspace {
    workspaces: HashMap<PathBuf, Workspace>,
}

impl PersistedWorkspace {
    pub fn new(
        _persistence_writer: SyncSender<crate::persistence::ModelEvent>,
        _ctx: &mut ModelContext<Self>,
    ) -> Self {
        Self {
            workspaces: HashMap::new(),
        }
    }

    pub fn workspaces(&self) -> impl Iterator<Item = WorkspaceMetadata> + '_ {
        self.workspaces
            .values()
            .map(|workspace| workspace.metadata.clone())
    }

    pub fn root_for_workspace<'a>(&self, _path: &'a Path) -> Option<&'a Path> {
        None
    }

    pub fn execute_lsp_task(&mut self, _task: LspTask, _ctx: &mut ModelContext<Self>) {}

    pub fn enable_lsp_server_for_path(
        &mut self,
        _path: &Path,
        _server_type: LSPServerType,
    ) {
    }

    pub fn disable_lsp_server_for_path(
        &mut self,
        _path: &Path,
        _server_type: LSPServerType,
    ) {
    }

    pub fn detect_available_servers_for_workspaces(&mut self, _ctx: &mut ModelContext<Self>) {}

    pub fn detect_lsp_workspace_status(
        &mut self,
        _path: PathBuf,
        _ctx: &mut ModelContext<Self>,
    ) -> LspRepoStatus {
        LspRepoStatus::CheckingForInstallation
    }

    pub fn has_enabled_lsp_server_for_file_path(
        &self,
        _path: &Path,
    ) -> LSPEnablementResultForFile {
        LSPEnablementResultForFile::UnsupportedLanguage
    }

    pub fn user_added_workspace(&mut self, _path: PathBuf, _ctx: &mut ModelContext<Self>) {}
}

impl Entity for PersistedWorkspace {
    type Event = PersistedWorkspaceEvent;
}

impl SingletonEntity for PersistedWorkspace {}
