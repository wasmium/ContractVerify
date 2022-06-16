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
    /// A new hash has been added and the previous signers removed
    ReInitialized,
    /// Added Blake3 Message Hash to the store
    AddedHash,
    /// The signature provided by the Solana Ed25519Verify program does not
    /// match the signature provided by the `ContractInstruction`
    InvalidSignature,
    /// The Account trying to re-initialize the PDA account data storage is not the same
    /// as the account that initialized the data
    InvalidNewOwner,
}
