#![forbid(unsafe_code)]
#![cfg_attr(
    not(test),
    deny(
        clippy::expect_used,
        clippy::indexing_slicing,
        clippy::panic,
        clippy::unreachable,
        clippy::unwrap_used
    )
)]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

mod escape;
pub mod parser;
pub mod structs;
pub mod to_md;
