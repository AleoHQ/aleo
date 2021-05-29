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

use crate::{Payload, Record, RecordError};

use aleo_account::ViewKey;
use rand::Rng;
use snarkvm_algorithms::EncryptionScheme;
use snarkvm_dpc::{
    base_dpc::instantiated::Components,
    DPCComponents,
    EncryptedRecord as EncryptedRecordNative,
    RecordEncryption,
    SystemParameters,
};
use snarkvm_utilities::{to_bytes, FromBytes, ToBytes};
use std::str::FromStr;

pub struct EncryptedRecord(Vec<u8>);

impl EncryptedRecord {
    pub fn new(data: Vec<u8>) -> EncryptedRecord {
        EncryptedRecord(data)
    }

    pub fn from_native(encrypted_record_native: EncryptedRecordNative<Components>) -> Result<Self, RecordError> {
        Ok(Self(to_bytes![encrypted_record_native]?))
    }

    pub fn to_native(&self) -> Result<EncryptedRecordNative<Components>, RecordError> {
        let encrypted_record_native: EncryptedRecordNative<Components> = FromBytes::read(&self.0[..])?;
        Ok(encrypted_record_native)
    }
}

pub(crate) type EncryptionRandomness =
    <<Components as DPCComponents>::AccountEncryption as EncryptionScheme>::Randomness;

impl Record {
    /// Encrypt the given vector of records and returns
    /// 1. Encryption randomness.
    /// 2. Encrypted record
    pub fn encrypt<R: Rng>(&self, rng: &mut R) -> Result<(EncryptionRandomness, EncryptedRecord), RecordError> {
        let system_parameters = SystemParameters::<Components>::load()?;

        let record_native = self.to_native()?;

        let (encryption_randomness, encrypted_record_native) =
            RecordEncryption::<Components>::encrypt_record(&system_parameters, &record_native, rng)?;

        let encrypted_record = EncryptedRecord::from_native(encrypted_record_native)?;

        Ok((encryption_randomness, encrypted_record))
    }

    /// Decrypt and reconstruct the encrypted record
    pub fn decrypt(view_key: &ViewKey, encrypted_record: &EncryptedRecord) -> Result<Record, RecordError> {
        let system_parameters = SystemParameters::<Components>::load()?;

        let record_native = RecordEncryption::<Components>::decrypt_record(
            &system_parameters,
            &view_key.view_key,
            &encrypted_record.to_native()?,
        )?;

        let mut record_bytes = vec![];
        record_native.write(&mut record_bytes)?;

        Ok(Record::read(&record_bytes[..])?)
    }
}

impl FromStr for EncryptedRecord {
    type Err = RecordError;

    fn from_str(encrypted_record: &str) -> Result<Self, Self::Err> {
        let encrypted_record = hex::decode(encrypted_record)?;

        Ok(Self::new(encrypted_record))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aleo_account::*;

    use rand::{rngs::StdRng, Rng, SeedableRng};
    use snarkvm_algorithms::traits::CRH;
    use snarkvm_dpc::{
        base_dpc::instantiated::{Components, ProgramVerificationKeyCRH, SerialNumberNonce as SerialNumberNonceCRH},
        NoopProgramSNARKParameters,
        SystemParameters,
    };
    use snarkvm_utilities::{bytes::ToBytes, to_bytes};

    pub(crate) const ITERATIONS: usize = 5;

    #[test]
    fn test_record_encryption() {
        let mut rng = &mut StdRng::from_entropy();

        for _ in 0..ITERATIONS {
            // Load system parameters for the ledger, commitment schemes, CRH, and the
            // "always-accept" program.
            let system_parameters = SystemParameters::<Components>::load().unwrap();
            let program_snark_pp = NoopProgramSNARKParameters::<Components>::load().unwrap();

            let program_snark_vk_bytes = to_bytes![
                ProgramVerificationKeyCRH::hash(
                    &system_parameters.program_verification_key_crh,
                    &to_bytes![program_snark_pp.verification_key].unwrap()
                )
                .unwrap()
            ]
            .unwrap();

            for _ in 0..ITERATIONS {
                let dummy_private_key = PrivateKey::new(rng).unwrap();
                let owner = Address::from(&dummy_private_key).unwrap();

                let value = rng.gen();
                let payload: [u8; 32] = rng.gen();

                let serial_number_nonce_input: [u8; 32] = rng.gen();
                let serial_number_nonce =
                    SerialNumberNonceCRH::hash(&system_parameters.serial_number_nonce, &serial_number_nonce_input)
                        .unwrap();

                let given_record = Record::new()
                    .owner(owner)
                    .value(value)
                    .payload(Payload::from_bytes(&payload))
                    .birth_program_id(program_snark_vk_bytes.clone())
                    .death_program_id(program_snark_vk_bytes.clone())
                    .serial_number_nonce(serial_number_nonce)
                    .calculate_commitment(Some(rng))
                    .build()
                    .unwrap();

                // Encrypt the record
                let (_, encryped_record) = given_record.encrypt(&mut rng).unwrap();
                let view_key = ViewKey::from(&dummy_private_key).unwrap();

                // Decrypt the record
                let decrypted_record = Record::decrypt(&view_key, &encryped_record).unwrap();

                assert_eq!(given_record, decrypted_record);
            }
        }
    }
}
