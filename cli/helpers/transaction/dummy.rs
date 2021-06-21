// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the Aleo library.

// The Aleo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Aleo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Aleo library. If not, see <https://www.gnu.org/licenses/>.

use crate::helpers::transaction::delegate_transaction;
use snarkos_storage::{mem::MemDb, Ledger};
use snarkvm_algorithms::traits::CRH;
use snarkvm_dpc::{
    account::{AccountAddress, AccountPrivateKey},
    testnet1::{
        instantiated::{CommitmentMerkleParameters, Components, InstantiatedDPC, Tx},
        parameters::PublicParameters,
        payload::Payload,
        record::Record,
    },
    traits::{DPCComponents, DPCScheme},
};
use snarkvm_utilities::{to_bytes, ToBytes};

pub type MerkleTreeLedger = Ledger<Tx, CommitmentMerkleParameters, MemDb>;

use rand::{CryptoRng, Rng};

/// Returns a transaction constructed with dummy records.
pub fn new_dummy_transaction<R: Rng + CryptoRng>(
    network_id: u8,
    rng: &mut R,
) -> anyhow::Result<(Tx, Vec<Record<Components>>)> {
    let parameters = PublicParameters::<Components>::load(false).unwrap();

    let spender = AccountPrivateKey::<Components>::new(
        &parameters.system_parameters.account_signature,
        &parameters.system_parameters.account_commitment,
        rng,
    )?;

    let new_recipient_private_key = AccountPrivateKey::<Components>::new(
        &parameters.system_parameters.account_signature,
        &parameters.system_parameters.account_commitment,
        rng,
    )?;
    let new_recipient = AccountAddress::<Components>::from_private_key(
        parameters.account_signature_parameters(),
        parameters.account_commitment_parameters(),
        parameters.account_encryption_parameters(),
        &new_recipient_private_key,
    )?;

    let noop_program_id = to_bytes![
        parameters
            .system_parameters
            .program_verification_key_crh
            .hash(&to_bytes![parameters.noop_program_snark_parameters.verification_key]?)?
    ]?;

    let mut old_records = vec![];
    let old_account_private_keys = vec![spender.clone(); Components::NUM_INPUT_RECORDS];

    while old_records.len() < Components::NUM_INPUT_RECORDS {
        let sn_randomness: [u8; 32] = rng.gen();
        let old_sn_nonce = parameters.system_parameters.serial_number_nonce.hash(&sn_randomness)?;

        let address = AccountAddress::<Components>::from_private_key(
            parameters.account_signature_parameters(),
            parameters.account_commitment_parameters(),
            parameters.account_encryption_parameters(),
            &spender,
        )?;

        let dummy_record = InstantiatedDPC::generate_record(
            &parameters.system_parameters,
            old_sn_nonce,
            address,
            true, // The input record is dummy
            0,
            Payload::default(),
            noop_program_id.clone(),
            noop_program_id.clone(),
            rng,
        )?;

        old_records.push(dummy_record);
    }

    assert_eq!(old_records.len(), Components::NUM_INPUT_RECORDS);

    let new_record_owners = vec![new_recipient; Components::NUM_OUTPUT_RECORDS];
    let new_is_dummy_flags = vec![true; Components::NUM_OUTPUT_RECORDS];
    let new_values = vec![0; Components::NUM_OUTPUT_RECORDS];
    let new_birth_program_ids = vec![noop_program_id.clone(); Components::NUM_OUTPUT_RECORDS];
    let new_death_program_ids = vec![noop_program_id; Components::NUM_OUTPUT_RECORDS];
    let new_payloads = vec![Payload::default(); Components::NUM_OUTPUT_RECORDS];

    // Generate a random memo
    let memo = rng.gen();

    // Generate transaction

    // Offline execution to generate a DPC transaction
    let execute_context = <InstantiatedDPC as DPCScheme<MerkleTreeLedger>>::execute_offline(
        parameters.system_parameters,
        old_records,
        old_account_private_keys,
        new_record_owners,
        &new_is_dummy_flags,
        &new_values,
        new_payloads,
        new_birth_program_ids,
        new_death_program_ids,
        memo,
        network_id,
        rng,
    )?;

    // Delegate online phase of transaction generation

    let random_path: u16 = rng.gen();

    let mut path = std::env::current_dir()?;
    path.push(format!("storage_db_{}", random_path));

    let ledger = MerkleTreeLedger::open_at_path(&path).unwrap();

    let (transaction, new_records) = delegate_transaction(execute_context, &ledger, rng)?;

    drop(ledger);

    Ok((transaction, new_records))
}
