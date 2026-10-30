//! Re-exports folder cloud-object types. The CloudModelType impl lives in
//! `crate::cloud_object::folder` so warp_lite can compile without this module.

pub use crate::cloud_object::folder::{CloudFolder, CloudFolderModel, FolderId};
