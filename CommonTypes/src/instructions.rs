use crate::{Blake3Hash, Signature};
use borsh::{BorshDeserialize, BorshSerialize};

/// The instructions to process a contract
#[derive(Debug, BorshSerialize, BorshDeserialize, PartialEq, PartialOrd, Ord, Eq, Copy, Clone)]
pub enum ContractInstruction {
    /// The instruction is not yet initialized
    UnInitialized,
    /// The owner of the program to add a contract Blake3 hash
    AddContractHash(Blake3Hash),
    /// Reinitialize storage with new hash, removes all previous signers
    AddContractHashZeroData(Blake3Hash),
    /// Add a signature of the message to a contract
    AddSignature {
        /// The Blake3 hash of the message
        message_hash: Blake3Hash,
        /// The Ed25519 signature for the Blake3 hash of the message
        user_signature: Signature,
    },
}

impl Default for ContractInstruction {
    fn default() -> Self {
        ContractInstruction::UnInitialized
    }
}
