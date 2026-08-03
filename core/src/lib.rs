//! A parser for EMF (Enhanced Metafile) binaries and a converter to SVG,
//! conforming to the [MS-EMF] specification. EMF+ records embedded in
//! comment records ([MS-EMFPLUS]) are parsed as well.
//!
//! # Usage
//!
//! [`EMFConverter`](converter::EMFConverter) takes both an EMF player and
//! a WMF player. The WMF player is only invoked when the input file turns
//! out to be WMF rather than EMF, so the SVG player implementations
//! shipped by each crate can be reused:
//!
//! ```no_run
//! let emf_data = std::fs::read("input.emf").expect("failed to read file");
//!
//! let emf_player = emf_core::converter::SVGPlayer::new();
//! let wmf_player = wmf_core::converter::SVGPlayer::new();
//! let converter = emf_core::converter::EMFConverter::new(
//!     emf_data.as_slice(),
//!     emf_player,
//!     wmf_player,
//! );
//!
//! let svg = converter.run().expect("failed to convert");
//! ```
//!
//! Output formats other than SVG can be produced by implementing the
//! [`Player`](converter::Player) trait.
//!
//! # Attribution
//!
//! Portions of the API documentation in this crate are adapted from the
//! [MS-EMF] and [MS-EMFPLUS] Open Specifications documentation,
//! © Microsoft Corporation, and are used under the Intellectual Property
//! Rights Notice for Open Specifications Documentation. The MS-EMF and
//! MS-EMFPLUS specifications are covered by the
//! [Microsoft Open Specification Promise][OSP].
//!
//! [MS-EMF]: https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-emf/91c257d7-c39d-4a36-9b1f-63e3f73d30ca
//! [MS-EMFPLUS]: https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-emfplus/5f92c789-64f2-46b5-9ed4-15a9bb0946c6
//! [OSP]: https://go.microsoft.com/fwlink/?LinkId=214445

#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::enum_variant_names,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::similar_names,
    clippy::too_many_lines,
    clippy::unreadable_literal,
    clippy::upper_case_acronyms,
    clippy::used_underscore_binding,
    clippy::wildcard_imports,
    non_camel_case_types,
    non_snake_case
)]
#![no_std]

#[macro_use]
extern crate alloc;

#[cfg(feature = "tracing")]
#[macro_use]
extern crate tracing;

#[cfg(not(feature = "tracing"))]
#[macro_use]
mod tracing {
    #[macro_export]
    macro_rules! debug {
        ($($arg:tt)+) => {};
    }

    #[macro_export]
    macro_rules! info {
        ($($arg:tt)+) => {};
    }

    #[macro_export]
    macro_rules! warn {
        ($($arg:tt)+) => {};
    }

    #[macro_export]
    macro_rules! error {
        ($($arg:tt)+) => {};
    }
}

pub mod converter;
pub mod parser;

mod imports {
    pub use alloc::{
        borrow::{Cow, ToOwned},
        boxed::Box,
        collections::{BTreeMap, BTreeSet, VecDeque},
        str,
        string::{String, ToString},
        vec::Vec,
    };
}

pub use embedded_io::Read;
