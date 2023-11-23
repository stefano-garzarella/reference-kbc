// Support using this crate without the standard library
#![cfg_attr(not(feature = "std"), no_std)]
// As long as there is a memory allocator, we can still use this crate
// without the rest of the standard library by using the `alloc` crate
#[cfg(feature = "alloc")]
extern crate alloc;

/// A facade around all the types we need from the `std`, `core`, and `alloc`
/// crates. This avoids elaborate import wrangling having to happen in every
/// module.
mod lib {
    mod core {
        #[cfg(not(feature = "std"))]
        pub use core::*;
        #[cfg(feature = "std")]
        pub use std::*;
    }

    mod alloc {
        #[cfg(feature = "std")]
        pub use std::*;

        #[cfg(all(feature = "alloc", not(feature = "std")))]
        pub use ::alloc::*;
    }

    // alloc modules (re-exported by `std` when have the standard library)
    pub use self::alloc::{
        boxed::Box,
        string::{String, ToString},
        vec,
        vec::Vec,
    };
    // core modules (re-exported by `std` when have the standard library)
    pub use self::core::{
        fmt::{self, Debug, Display},
        num::TryFromIntError,
    };
}

pub mod client_proxy;
pub mod client_registration;
pub mod client_session;

#[derive(Debug)]
pub enum KBCError {
    // Errors related to client_session
    CS(client_session::CSError),
    // Errors related to client_registration
    CR(client_registration::CRError),
    // Errors related to client_proxy
    CP(client_proxy::CPError),
}
