//! Side length of a chunk.

/// Side length of a chunk, in meters.
pub const CHUNK_SIZE: i32 = 512;

/// Side length of a chunk, in meters, for coordinate arithmetic.
#[expect(clippy::as_conversions, reason = "exact within f64")]
pub const CHUNK_SIZE_F64: f64 = CHUNK_SIZE as f64;
