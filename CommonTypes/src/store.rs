// © Wasmium Network Developers

// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at https://mozilla.org/MPL/2.0/.
// © Wasmium Network Developers

// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at https://mozilla.org/MPL/2.0/.


use crate::*;
use borsh::{BorshDeserialize, BorshSerialize};
use core::fmt;

/// Stores the message hash and signers information
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, BorshSerialize, BorshDeserialize)]
pub struct HashStore {
    message_hash: Blake3Hash,
    signers: Vec<SignerData>,
}

impl HashStore {
    /// Create a new instance of the store
    pub fn new() -> Self {
        HashStore {
            message_hash: PublicKey::default(),
            signers: Vec::default(),
        }
    }

    /// Add the message hash
    pub fn add_message_hash(&mut self, hash: Blake3Hash) -> &mut Self {
        self.message_hash = hash;

        self
    }

    /// Add a new signer
    pub fn add_signer(&mut self, signer_data: SignerData) -> &mut Self {
        self.signers.push(signer_data);

        self
    }

    /// Verify the current message hash and the user provided message hash match
    pub fn verify(&self, user_hash: Blake3Hash) -> VerifyOutcome {
        if self.message_hash != user_hash {
            VerifyOutcome::InvalidMessageHash
        } else {
            VerifyOutcome::SignatureVerified
        }
    }

    /// Get the message hash stored
    pub fn message_hash(&self) -> Blake3Hash {
        self.message_hash
    }

    /// Get the current signers
    pub fn signers(&self) -> &Vec<SignerData> {
        self.signers.as_ref()
    }
}

impl fmt::Debug for HashStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HashStore")
            .field(
                "message_hash",
                &bs58::encode(&self.message_hash).into_string(),
            )
            .field("signers", &self.signers)
            .finish()
    }
}

impl Default for HashStore {
    fn default() -> Self {
        HashStore::new()
    }
}

/// The public key and the signature of a party participating in the
/// signing of the message
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, BorshSerialize, BorshDeserialize)]
pub struct SignerData {
    public_key: PublicKey,
    signature: Signature,
}

impl fmt::Debug for SignerData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SignerData")
            .field("public_key", &bs58::encode(&self.public_key).into_string())
            .field("signature", &bs58::encode(&self.public_key).into_string())
            .finish()
    }
}
