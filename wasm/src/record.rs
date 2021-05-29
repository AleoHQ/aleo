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

use aleo_account::ViewKey;
use aleo_record::{helpers::encrypt::EncryptedRecord, Record as RecordNative};

use rand::{rngs::StdRng, SeedableRng};
use snarkvm_utilities::{to_bytes, ToBytes};
use std::str::FromStr;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Record {
    pub(crate) record: RecordNative,
}

#[wasm_bindgen]
impl Record {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let rng = &mut StdRng::from_entropy();
        let record = RecordNative::dummy(rng).unwrap();

        Self { record }
    }

    #[wasm_bindgen]
    pub fn from_string(record: &str) -> Self {
        let record = RecordNative::from_str(record).unwrap();

        Self { record }
    }

    #[wasm_bindgen]
    pub fn to_string(&self) -> String {
        format!("Record {{ record: {} }}", self.record)
    }

    #[wasm_bindgen]
    pub fn decrypt(encrypted_record: &str, view_key: &str) -> Self {
        let view_key = ViewKey::from_str(view_key).unwrap();
        let encrypted_record = EncryptedRecord::from_str(encrypted_record).unwrap();
        let record = RecordNative::decrypt(&view_key, &encrypted_record).unwrap();

        Self { record }
    }

    #[wasm_bindgen]
    pub fn owner(&self) -> String {
        self.record.owner().to_string()
    }

    #[wasm_bindgen]
    pub fn is_dummy(&self) -> bool {
        self.record.is_dummy()
    }

    #[wasm_bindgen]
    pub fn value(&self) -> u64 {
        self.record.value()
    }

    #[wasm_bindgen]
    pub fn payload(&self) -> String {
        hex::encode(to_bytes![self.record.payload()].unwrap())
    }

    #[wasm_bindgen]
    pub fn birth_program_id(&self) -> String {
        hex::encode(to_bytes![self.record.birth_program_id()])
    }

    #[wasm_bindgen]
    pub fn death_program_id(&self) -> String {
        hex::encode(to_bytes![self.record.death_program_id()])
    }

    #[wasm_bindgen]
    pub fn serial_number_nonce(&self) -> String {
        hex::encode(to_bytes![self.record.serial_number_nonce()])
    }

    #[wasm_bindgen]
    pub fn commitment(&self) -> String {
        hex::encode(to_bytes![self.record.commitment()])
    }

    #[wasm_bindgen]
    pub fn commitment_randomness(&self) -> String {
        hex::encode(to_bytes![self.record.commitment_randomness()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    pub fn new_test() {
        let _dummy_record = Record::new();
    }

    #[wasm_bindgen_test]
    pub fn record_to_string_test() {
        let record = Record::from_string(TEST_RECORD);

        println!("{} == {}", TEST_RECORD, record.record.to_string());
        assert_eq!(TEST_RECORD, record.record.to_string());
    }
}
