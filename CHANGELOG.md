## [0.2.0] - 2022-06-16

### Added

- Get ed25519program data `sysvar` 
-  parse ed25519program data into the public key, message hash and signature 
- Unpack the PDA data  
- Deserialize instruction_data 
- Process the instruction 
- Rewrite the data into the PDA account
- Add support for the outcome of an instruction with `VerifyOutcome` 
- Add owner of a program 
- Add methods to add, modify and fetch the store
- Add reusable types for both the client and program



### Fixed

- Instruction for signature verification should contain the message hash and signature of the message hash 