// © Wasmium Network Developers

// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at https://mozilla.org/MPL/2.0/.
// © Wasmium Network Developers

// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at https://mozilla.org/MPL/2.0/.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! `CommonTypes` is a crate used to create and transform data
//! on the Solana blockchain for the `ContractVerify` program and client

/// The store for the message signers and message hash
mod store;
pub use store::*;

/// Commons reusable types
mod common;
pub use common::*;

/// Outcome of the verification program
mod outcome;
pub use outcome::*;

/// Instructions for contract processing
mod instructions;
pub use instructions::*;
