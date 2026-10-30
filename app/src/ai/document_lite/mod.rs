//! Minimal AI document module for warp_lite builds.
//!
//! Provides in-memory plan document tracking without Warp Drive / notebook editor dependencies.

pub mod ai_document_model;
pub(in crate::ai) mod plan_publication;

pub use ai::document::{AIDocumentId, AIDocumentVersion, DEFAULT_PLANNING_DOCUMENT_TITLE};
