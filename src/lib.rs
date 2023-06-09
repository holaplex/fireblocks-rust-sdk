//!

#![deny(
    clippy::disallowed_methods,
    clippy::suspicious,
    clippy::style,
    missing_debug_implementations,
    missing_copy_implementations
)]
#![warn(clippy::pedantic, clippy::cargo)]

mod client;
pub mod objects;
mod signer;
pub use client::{Client, FbArgs};
