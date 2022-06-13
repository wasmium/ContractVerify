// © Wasmium Network Developers

// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at https://mozilla.org/MPL/2.0/.
// © Wasmium Network Developers

// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at https://mozilla.org/MPL/2.0/.


use borsh::{BorshDeserialize, BorshSerialize};

/// The outcome of the verification process
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, BorshSerialize, BorshDeserialize)]
pub enum VerifyOutcome {
    /// The Signature of the message matches the message hash provided
    /// by the program
    SignatureVerified,
    /// The Blake3 message hash provided does not match the Blake3 message hash
    /// provided by the user
    InvalidMessageHash,
}
