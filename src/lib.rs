//! Source distribution of the Clipper2 polygon clipping library.
//!
//! This crate compiles Clipper2 from source and exports paths for downstream crates.
//!
//! # Usage
//!
//! Add this crate as a build dependency:
//!
//! ```toml
//! [build-dependencies]
//! clipper2-src = "2"
//! ```
//!
//! In your `build.rs`, access the include path via the `DEP_CLIPPER2_INCLUDE` environment variable:
//!
//! ```ignore
//! let clipper2_include = std::env::var("DEP_CLIPPER2_INCLUDE").unwrap();
//! ```
//!
//! # Version
//!
//! This crate version tracks Clipper2 releases (e.g., clipper2-src 2.0.1 = Clipper2 v2.0.1).

/// Clipper2 version bundled with this crate.
pub const CLIPPER2_VERSION: &str = "2.0.1";
