mod types;

pub use types::{ColorMap, DecorationStateEvent, IndentDelta, TextSlice};

#[cfg(feature = "runtime")]
mod runtime;
#[cfg(feature = "runtime")]
pub use runtime::SyntaxTreeState;

#[cfg(not(feature = "runtime"))]
mod disabled;
#[cfg(not(feature = "runtime"))]
pub use disabled::SyntaxTreeState;
