#![cfg_attr(feature = "bench", feature(test))]
#![cfg_attr(feature = "no_std", no_std)]

extern crate alloc;

#[cfg(feature = "no_std")]
mod std {
    pub mod collections {
        pub use alloc::collections::{BTreeMap as HashMap, BTreeSet as HashSet};
    }
}

pub mod compile;
pub mod ctype;
pub mod immut_vec;
pub mod matcher;
pub mod tree;

pub use compile::PosixRegexBuilder;
pub use matcher::PosixRegex;
