pub struct RemoteChildLaunchConfig;



pub fn prepare_remote_child_launch() {}



pub struct CloudAgentStartupAuthFlow;

pub struct CloudAgentStartupBlocker;

pub struct CloudAgentStartupFailure;

pub struct CloudAgentStartupIssue;



pub struct CloudAgentStartupPresentation;

pub fn classify_cloud_agent_startup_error(_error: &str) -> CloudAgentStartupFailure {

    CloudAgentStartupFailure

}



pub fn should_disable_snapshot(_app: &warpui::AppContext) -> bool {

    true

}
