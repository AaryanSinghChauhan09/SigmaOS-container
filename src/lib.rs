//! SigmaOS-container — container runtime model.
//!
//! Models container lifecycle, namespaces, cgroups, images,
//! and OCI-compatible runtime abstractions. Deterministic
//! and std-only (no actual container execution).

pub mod container;
pub mod runtime;
pub mod image;
pub mod namespace;
pub mod cgroup;

pub use container::{Container, ContainerState, ContainerConfig};
pub use runtime::RuntimeManager;
pub use image::{ContainerImage, ImageManager};
pub use namespace::{Namespace, NamespaceType};
pub use cgroup::{Cgroup, CgroupLimits};
