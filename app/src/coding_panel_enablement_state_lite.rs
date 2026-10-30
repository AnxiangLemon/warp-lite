#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CodingPanelEnablementState {
    Enabled,
    PendingRemoteSession,
    RemoteSession { has_remote_server: bool },
    UnsupportedSession,
    Disabled,
}

impl CodingPanelEnablementState {
    pub(crate) fn from_session_env(
        is_enabled: bool,
        is_remote: bool,
        is_unsupported_session: bool,
        has_remote_server: bool,
    ) -> Self {
        if is_remote {
            Self::RemoteSession { has_remote_server }
        } else if is_unsupported_session {
            Self::UnsupportedSession
        } else if is_enabled {
            Self::Enabled
        } else {
            Self::Disabled
        }
    }
}
