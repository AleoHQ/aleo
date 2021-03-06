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

use crate::{Record, RecordError};
use aleo_account::Address;
use aleo_environment::Environment;

use snarkvm_algorithms::traits::{CommitmentScheme, CRH};
use snarkvm_dpc::{
    testnet1::parameters::{NoopProgramSNARKParameters, SystemParameters},
    DPCComponents,
    RecordScheme,
};
use snarkvm_utilities::{to_bytes, variable_length_integer::variable_length_integer, FromBytes, ToBytes, UniformRand};

use once_cell::sync::OnceCell;
use rand::{CryptoRng, Rng};

/// A builder struct for the record data type.
#[derive(Derivative)]
#[derivative(Default(bound = "E: Environment"), Debug(bound = "E: Environment"))]
pub struct RecordBuilder<E: Environment> {
    pub(crate) owner: OnceCell<Address>,
    pub(crate) is_dummy: OnceCell<bool>,
    pub(crate) value: OnceCell<u64>,
    pub(crate) payload: OnceCell<<Record<E> as RecordScheme>::Payload>,

    pub(crate) birth_program_id: OnceCell<Vec<u8>>,
    pub(crate) death_program_id: OnceCell<Vec<u8>>,

    pub(crate) serial_number_nonce: OnceCell<<Record<E> as RecordScheme>::SerialNumberNonce>,
    pub(crate) commitment: OnceCell<<Record<E> as RecordScheme>::Commitment>,
    pub(crate) commitment_randomness: OnceCell<<Record<E> as RecordScheme>::CommitmentRandomness>,

    pub(crate) errors: Vec<RecordError>,
}

impl<E: Environment> RecordBuilder<E> {
    ///
    /// Returns a new record builder.
    /// To return a record and consume the record builder, call the `.build()` method.
    ///
    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    ///
    /// Returns a new record builder and sets field `owner: Address`.
    ///
    pub fn owner<A: Into<Address>>(mut self, owner: A) -> Self {
        let owner = owner.into();
        let owner_string = format!("owner: {}", owner);
        if self.owner.set(owner).is_err() {
            self.errors.push(RecordError::DuplicateArgument(owner_string));
        }
        self
    }

    ///
    /// Returns a new record builder and sets field `value: u64`.
    ///
    pub fn value(mut self, value: u64) -> Self {
        if self.value.set(value).is_err() {
            self.errors
                .push(RecordError::DuplicateArgument(format!("value: {}", value)))
        }
        self
    }

    ///
    /// Returns a new record builder and sets field `payload: Payload`.
    ///
    pub fn payload(mut self, payload: <Record<E> as RecordScheme>::Payload) -> Self {
        let payload_string = hex::encode(&to_bytes![payload].unwrap()[..]);
        if self.payload.set(payload).is_err() {
            self.errors
                .push(RecordError::DuplicateArgument(format!("payload: {}", payload_string)))
        }
        self
    }

    ///
    /// Returns a new record builder and sets field `birth_program_id: Vec<u8>`.
    ///
    pub fn birth_program_id(mut self, birth_program_id: Vec<u8>) -> Self {
        let birth_program_id_string = hex::encode(&to_bytes![birth_program_id].unwrap()[..]);
        if self.birth_program_id.set(birth_program_id).is_err() {
            self.errors.push(RecordError::DuplicateArgument(format!(
                "birth_program_id: {}",
                birth_program_id_string
            )))
        }
        self
    }

    ///
    /// Returns a new record builder and sets field `death_program_id: Vec<u8>`.
    ///
    pub fn death_program_id(mut self, death_program_id: Vec<u8>) -> Self {
        let death_program_id_string = hex::encode(&to_bytes![death_program_id].unwrap()[..]);
        if self.death_program_id.set(death_program_id).is_err() {
            self.errors.push(RecordError::DuplicateArgument(format!(
                "death_program_id: {}",
                death_program_id_string
            )))
        }
        self
    }

    ///
    /// Returns a new record builder and sets field `serial_number_nonce: SerialNumberNonce`.
    ///
    pub fn serial_number_nonce(mut self, serial_number_nonce: <Record<E> as RecordScheme>::SerialNumberNonce) -> Self {
        if self.serial_number_nonce.set(serial_number_nonce).is_err() {
            self.errors.push(RecordError::DuplicateArgument(format!(
                "serial_number_nonce: {}",
                hex::encode(&to_bytes![serial_number_nonce].unwrap()[..])
            )))
        }
        self
    }

    ///
    /// Returns a new record builder and computes the field `serial_number_nonce: SerialNumberNonce`
    /// from the given inputs.
    ///
    pub fn calculate_serial_number_nonce<R: Rng>(
        mut self,
        parameters: SystemParameters<E::Components>,
        index: u8,
        joint_serial_numbers: Vec<u8>,
        rng: &mut R,
    ) -> Self {
        // Sample randomness sn_randomness for the CRH input.
        let sn_randomness: [u8; 32] = rng.gen();

        let crh_input = to_bytes![index, sn_randomness, joint_serial_numbers].unwrap();
        let sn_nonce =
            <E::Components as DPCComponents>::SerialNumberNonceCRH::hash(&parameters.serial_number_nonce, &crh_input)
                .unwrap();

        self.serial_number_nonce(sn_nonce)
    }

    ///
    /// Returns a new record builder and sets field `commitment: RecordCommitment`.
    ///
    pub fn commitment(mut self, commitment: <Record<E> as RecordScheme>::Commitment) -> Self {
        let commitment_string = hex::encode(&to_bytes![commitment].unwrap()[..]);
        if self.commitment.set(commitment).is_err() {
            self.errors.push(RecordError::DuplicateArgument(format!(
                "commitment: {}",
                commitment_string
            )))
        }
        self
    }

    ///
    /// Returns a new record builder and sets field `commitment_randomness: CommitmentRandomness`.
    ///
    pub fn commitment_randomness(
        mut self,
        commitment_randomness: <Record<E> as RecordScheme>::CommitmentRandomness,
    ) -> Self {
        let commitment_randomness_string = hex::encode(&to_bytes![commitment_randomness].unwrap()[..]);
        if self.commitment_randomness.set(commitment_randomness).is_err() {
            self.errors.push(RecordError::DuplicateArgument(format!(
                "commitment_randomness: {}",
                commitment_randomness_string
            )))
        }
        self
    }

    ///
    /// Returns a new record builder and calculates a new `commitment_randomness: CommitmentRandomness`.
    ///
    pub fn calculate_commitment_randomness<R: Rng + CryptoRng>(self, rng: &mut R) -> Self {
        // Sample new commitment randomness.
        let commitment_randomness =
            <<E::Components as DPCComponents>::RecordCommitment as CommitmentScheme>::Randomness::rand(rng);

        self.commitment_randomness(commitment_randomness)
    }

    ///
    /// Returns a `Record` and consumes the record builder.
    /// Returns an error if fields are missing or error are encountered while building.
    ///
    pub fn build(self) -> Result<Record<E>, RecordError> {
        // Return error.
        if !self.errors.is_empty() {
            // Print out all errors

            for err in self.errors {
                println!("BuilderError: {:?}", err);
            }

            // Return builder fail.
            return Err(RecordError::BuilderError);
        }

        // Get owner
        let owner = match self.owner.get() {
            Some(value) => value,
            None => return Err(RecordError::MissingField("owner".to_string())),
        };

        // Get value
        let value = match self.value.get() {
            Some(value) => value,
            None => return Err(RecordError::MissingField("value".to_string())),
        };

        // Get payload
        let payload = match self.payload.get() {
            Some(payload) => payload,
            None => return Err(RecordError::MissingField("payload".to_string())),
        };

        // Get birth_program_id
        let birth_program_id = match self.birth_program_id.get() {
            Some(birth_program_id) => birth_program_id,
            None => return Err(RecordError::MissingField("birth_program_id".to_string())),
        };

        // Get death_program_id
        let death_program_id = match self.death_program_id.get() {
            Some(death_program_id) => death_program_id,
            None => return Err(RecordError::MissingField("death_program_id".to_string())),
        };

        // Get serial_number_nonce
        let serial_number_nonce = match self.serial_number_nonce.get() {
            Some(serial_number_nonce) => serial_number_nonce,
            None => return Err(RecordError::MissingField("serial_number_nonce".to_string())),
        };

        // Get noop_program_id.
        let system_parameters = SystemParameters::<E::Components>::load()?;
        let program_snark_pp = NoopProgramSNARKParameters::<E::Components>::load()?;

        let noop_program_id = to_bytes![
            <E::Components as DPCComponents>::ProgramVerificationKeyCRH::hash(
                &system_parameters.program_verification_key_crh,
                &to_bytes![program_snark_pp.verification_key].unwrap()
            )
            .unwrap()
        ]
        .unwrap();

        // Derive is_dummy
        let is_dummy = (*value == 0)
            && (*payload == <Record<E> as RecordScheme>::Payload::default())
            && (*birth_program_id == noop_program_id)
            && (*death_program_id == noop_program_id);

        // Get commitment_randomness
        let commitment_randomness = match self.commitment_randomness.get() {
            Some(commitment_randomness) => commitment_randomness,
            None => return Err(RecordError::MissingRandomness),
        };

        // Total = 32 + 1 + 8 + 32 + 48 + 48 + 32 = 201 bytes
        let commitment_input = to_bytes![
            owner.to_bytes(),    // 256 bits = 32 bytes
            is_dummy,            // 1 bit = 1 byte
            value,               // 64 bits = 8 bytes
            payload,             // 256 bits = 32 bytes
            birth_program_id,    // 384 bits = 48 bytes
            death_program_id,    // 384 bits = 48 bytes
            serial_number_nonce  // 256 bits = 32 bytes
        ]
        .unwrap();

        // Derive commitment
        let commitment = <E::Components as DPCComponents>::RecordCommitment::commit(
            &system_parameters.record_commitment,
            &commitment_input,
            commitment_randomness,
        )?;

        // Check the given record commitment is valid if the user provided it.
        if let Some(given_commitment) = self.commitment.get() {
            if *given_commitment != commitment {
                return Err(RecordError::InvalidCommitment);
            }
        }

        // Serialize record
        let mut record_bytes = Vec::new();
        owner.write(&mut record_bytes)?;

        is_dummy.write(&mut record_bytes)?;
        value.write(&mut record_bytes)?;
        payload.write(&mut record_bytes)?;

        variable_length_integer(birth_program_id.len() as u64).write(&mut record_bytes)?;
        birth_program_id.write(&mut record_bytes)?;

        variable_length_integer(death_program_id.len() as u64).write(&mut record_bytes)?;
        death_program_id.write(&mut record_bytes)?;

        serial_number_nonce.write(&mut record_bytes)?;
        commitment.write(&mut record_bytes)?;
        commitment_randomness.write(&mut record_bytes)?;

        // Build record.
        Ok(Record {
            record: FromBytes::read(&to_bytes![record_bytes]?[..])?,
        })
    }
}
