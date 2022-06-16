use common_types::{ContractInstruction, HashStore, PublicKey, SignerData, VerifyOutcome};
use solana_program::{instruction::Instruction, msg};
use zeroed_store::{StoreResult, ZeroedStore};

pub const PUBLIC_KEY_OFFSET: usize = 16;
pub const SIGNATURE_KEY_OFFSET: usize = 48;
pub const MESSAGE_DATA_OFFSET: usize = 112;

#[derive(Debug)]
pub struct ProcessInstruction {
    signer: PublicKey,
    store: ZeroedStore<HashStore>,
    instruction: ContractInstruction,
}

impl ProcessInstruction {
    pub fn unpack(bytes: &[u8]) -> StoreResult<Self> {
        let store = ZeroedStore::<HashStore>::unpack(bytes)?;

        Ok(ProcessInstruction {
            signer: PublicKey::default(),
            store,
            instruction: ContractInstruction::default(),
        })
    }

    pub fn add_signer(&mut self, signer: PublicKey) -> &mut Self {
        self.signer = signer;

        self
    }

    pub fn add_instruction(&mut self, instruction: ContractInstruction) -> &mut Self {
        self.instruction = instruction;

        self
    }
    pub fn process(&mut self, sysvar_instruction: &Instruction) -> VerifyOutcome {
        let public_key = &sysvar_instruction.data[PUBLIC_KEY_OFFSET..PUBLIC_KEY_OFFSET + 32];
        let signature = &sysvar_instruction.data[SIGNATURE_KEY_OFFSET..SIGNATURE_KEY_OFFSET + 64];
        let data = &sysvar_instruction.data[MESSAGE_DATA_OFFSET..MESSAGE_DATA_OFFSET + 32];

        let parsed_public_key: [u8; 32] = match data.try_into() {
            Ok(value) => value,
            Err(_) => {
                msg!("Error Converting Public Key bytes to a `[u8; 32]` array",);
                panic!() //TODO Covert to a usable error
            }
        };
        let parsed_hash: [u8; 32] = match data.try_into() {
            Ok(value) => value,
            Err(_) => {
                msg!("Error Converting Message bytes to a `[u8; 32]` array",);
                panic!() //TODO Covert to a usable error
            }
        };
        let parsed_signature: [u8; 64] = match signature.try_into() {
            Ok(value) => value,
            Err(_) => {
                msg!("Error Converting Signature bytes to a `[u8; 64]` array",);
                panic!() //TODO Covert to a usable error
            }
        };
        let blake3_hash: blake3::Hash = parsed_hash.into();

        msg!("PK: {:?}", &public_key);
        msg!("SIG: {:?}", &signature);
        msg!("BLAKE3HASH: {:?}", &blake3_hash.to_hex().as_str());

        match self.instruction {
            ContractInstruction::UnInitialized => VerifyOutcome::InvalidMessageHash,
            ContractInstruction::AddContractHash(message_hash) => {
                let compared = self.compare_hashes(blake3_hash, message_hash);

                if !compared {
                    VerifyOutcome::InvalidMessageHash
                } else {
                    self.store.data.add_owner(self.signer);
                    self.store.data.add_message_hash(message_hash);

                    VerifyOutcome::AddedHash
                }
            }
            ContractInstruction::AddContractHashZeroData(message_hash) => {
                let compared = self.compare_hashes(blake3_hash, message_hash);

                if !compared {
                    return VerifyOutcome::InvalidMessageHash;
                }

                if self.store.data.owner() == self.signer {
                    VerifyOutcome::InvalidNewOwner
                } else {
                    self.store.data.reinitialize(message_hash);

                    VerifyOutcome::ReInitialized
                }
            }
            ContractInstruction::AddSignature {
                message_hash,
                user_signature,
            } => {
                let compared = self.compare_hashes(blake3_hash, message_hash);

                if !compared {
                    VerifyOutcome::InvalidMessageHash
                } else {
                    if parsed_signature == user_signature {
                        let mut signer_data = SignerData::new();
                        signer_data
                            .add_public_key(parsed_public_key)
                            .add_signature(parsed_signature);

                        self.store.data.add_signer(signer_data);
                        VerifyOutcome::SignatureVerified
                    } else {
                        VerifyOutcome::InvalidSignature
                    }
                }
            }
        }
    }

    pub fn pack(&self, buffer: &mut [u8]) -> zeroed_store::StoreResult<usize> {
        self.store.pack(buffer)
    }

    pub fn compare_hashes(&self, sysvar_hash: blake3::Hash, user_hash: [u8; 32]) -> bool {
        let blake3_hash_input: blake3::Hash = user_hash.into();
        msg!("USER_HASH: {}", blake3_hash_input.to_hex().as_str());

        // Compares with constant time equality
        if sysvar_hash == blake3_hash_input {
            true
        } else {
            false
        }
    }
}
