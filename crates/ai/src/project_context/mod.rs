#[cfg(feature = "runtime")]
cfg_if::cfg_if! {
    if #[cfg(feature = "local_fs")] {
        mod global_rules;
        pub(crate) use global_rules::GlobalRules;
    } else {
        mod dummy_global_rules;
        pub(crate) use dummy_global_rules::GlobalRules;
    }
}
#[cfg(feature = "runtime")]
pub mod model;
mod types;

#[cfg(not(feature = "runtime"))]
pub mod model {
    pub use super::types::ProjectRulePath;
}
