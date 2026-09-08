//! Reader for the subset of TIFF the source files use.
//!
//! - Depends only on `std` and `weezl`
//! - Knows nothing of terrain, chunks or coordinates

pub use tiff_bytes::*;
pub use tiff_entry::*;
pub use tiff_tag::*;
pub use tiff_tags::*;

mod tiff_bytes;
mod tiff_entry;
#[cfg(test)]
pub(crate) mod tiff_fixture;
mod tiff_tag;
mod tiff_tags;
