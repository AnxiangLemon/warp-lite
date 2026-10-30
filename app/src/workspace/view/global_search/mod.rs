#[cfg(not(feature = "warp_lite"))]
use warp_ripgrep::search::Submatch;
#[cfg(not(feature = "warp_lite"))]
use warp_util::local_or_remote_path::LocalOrRemotePath;

#[cfg(not(feature = "warp_lite"))]
pub struct SearchConfig {
    pub use_regex: bool,
    pub use_case_sensitivity: bool,
}

/// A single global search match: one line in one file, which may live on
/// the local filesystem or on a remote host.
#[derive(Clone, Debug)]
#[cfg(not(feature = "warp_lite"))]
pub struct GlobalSearchMatch {
    pub location: LocalOrRemotePath,
    pub line_number: u32,
    /// Original 1-based character column in the file. This is captured
    /// before display-only whitespace trimming so opening a result navigates
    /// to the correct location.
    pub column_num: Option<usize>,
    pub line_text: String,
    pub submatches: Vec<Submatch>,
}

#[cfg_attr(not(target_family = "wasm"), path = "model.rs")]
#[cfg_attr(target_family = "wasm", path = "model_wasm.rs")]
#[cfg(not(feature = "warp_lite"))]
pub mod model;
#[cfg_attr(feature = "warp_lite", path = "view_lite.rs")]
pub mod view;
