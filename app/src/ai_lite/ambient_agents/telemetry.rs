#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HandoffEntryPoint {
    #[default]
    Unknown,
}

pub struct CloudAgentTelemetryEvent;
pub struct CloudModeEntryPoint;
pub struct HandoffSurface;
