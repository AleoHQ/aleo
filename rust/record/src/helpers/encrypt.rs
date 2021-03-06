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

use crate::Record;
use aleo_environment::Environment;

use snarkvm_algorithms::EncryptionScheme;
use snarkvm_dpc::{
    testnet1::{instantiated::Components, parameters::SystemParameters, record_encryption::RecordEncryption},
    DPCComponents,
    DPCError,
};
use snarkvm_utilities::{to_bytes, FromBytes, ToBytes};

use rand::Rng;

pub(crate) type EncryptionRandomness =
    <<Components as DPCComponents>::AccountEncryption as EncryptionScheme>::Randomness;

pub(crate) struct Encrypt;

impl Encrypt {
    /// Encrypt the given record and returns tuple (encryption randomness, encrypted record).
    pub(crate) fn encrypt<R: Rng, E: Environment>(
        record: &Record<E>,
        rng: &mut R,
    ) -> Result<(EncryptionRandomness, Vec<u8>), DPCError> {
        let system_parameters = SystemParameters::<Components>::load()?;

        let (encryption_randomness, encrypted_record) = RecordEncryption::<Components>::encrypt_record(
            &system_parameters,
            &FromBytes::read(&to_bytes![record]?[..])?,
            rng,
        )?;

        Ok((encryption_randomness, to_bytes![encrypted_record]?))
    }
}
