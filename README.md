## ContractVerify

ContractVerify is a repository containing a Solana program and a client whose aim is to provide the tools needed for a developer to add support for verifying the contents of a document agreed upon by more than one party.

This is similar to how two parties would visit a lawyer/advocate and the lawyer/advocate would draft the contract and thereafter all the parties would agree on the contract and append their signatures after paying a fee to the lawyer/advocate.

This contracts eliminates the need for a lawyer and uses smart contracts to archive binding agreements.

The pseudocode of the program:

1. A party drafts the digital contract and runs it through a Blake3 hash.
2. The party uploads the Blake3 hash of the document to a PDA account on the Solana blockchain. Uploading only the Blake3 hash of the contract ensures privacy while still retaining the integrity of the contract. Dapps can just verify the contract by running the Blake3 hash of the contract locally on the users Dapp.
3. The party sends the digital contract and the PDA account to the Dapps of the other participants
4. The other participants read the hash from the PDA account and compare it to the hash of the sent contract
5. The participants sign the contract if the hashes match and then send the transaction to the blockchain
6. Each transaction is verified by fetching the contents of the first instruction which is the `Ed25519Verify` program using `solana_program::sysvar::...` and parses the data. If the hashes and signatures match the data is recorded on the blockchain

#### Current Progress

- [x] Read contents of the Ed25519 program and verify that they correspond with the current Solana instruction
- [x] Parse the `instruction_data` to a `ContractInstruction`
- [x] Add a Blake3 message hash to the PDA data
- [x]  Add the public key of owner of the contract to the `PDA` data
- [x] Add the public key  and signature of a signing participant to the store
- [x]  Ability to reinitialize the Blake3 hash of a message and render all previous signatures void by filling them with zeroes
- [ ] Method to set the minimum number of signatures required for the contact to be valid
- [ ] Method to set the public keys that are allowed to participate in signing the contract
- [ ] Client library to simplify the parsing of documents like images and PDFs as bytes and then signing those bytes with Blake3 hash
- [ ] Client library to park and send the PDA account and contract to other participants
- [ ] Client library to deserialize PDA data to extract the message hash from the blockchain and compare it to the Blake3 hash of the contract send to the Dapp.







LICENSE

The contents of this repository are licensed under MPL-2.0



#### CONTRIBUTING

All contributions are to be made in accordance to the Rust Code of Conduct.