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

use crate::{Address, PrivateKey, ViewKey};
use aleo_network::Testnet1;

use rand::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;
use std::str::FromStr;

#[test]
pub fn private_key_test() {
    let rng = &mut ChaChaRng::seed_from_u64(1231275789u64);
    let private_key = PrivateKey::<Testnet1>::new(rng);
    assert!(private_key.is_ok());

    let expected_private_key = "APrivateKey1tvv5YV1dipNiku2My8jMkqpqCyYKvR5Jq4y2mtjw7s77Zpn";
    let candidate_private_key = private_key.unwrap().to_string();

    println!("{} == {}", expected_private_key, candidate_private_key);
    assert_eq!(expected_private_key, candidate_private_key);
}

#[test]
pub fn view_key_test() {
    let private_key = PrivateKey::<Testnet1>::from_str("APrivateKey1tvv5YV1dipNiku2My8jMkqpqCyYKvR5Jq4y2mtjw7s77Zpn").unwrap();
    let view_key = ViewKey::<Testnet1>::from(&private_key);
    assert!(view_key.is_ok());

    let expected_view_key = "AViewKey1m8gvywHKHKfUzZiLiLoHedcdHEjKwo5TWo6efz8gK7wF";
    let candidate_view_key = view_key.unwrap().to_string();

    println!("{} == {}", expected_view_key, candidate_view_key);
    assert_eq!(expected_view_key, candidate_view_key);
}

#[test]
pub fn address_test() {
    let private_key = PrivateKey::<Testnet1>::from_str("APrivateKey1tvv5YV1dipNiku2My8jMkqpqCyYKvR5Jq4y2mtjw7s77Zpn").unwrap();
    let address = Address::<Testnet1>::from(&private_key);
    assert!(address.is_ok());

    let expected_address = "aleo1faksgtpmculyzt6tgaq26fe4fgdjtwualyljjvfn2q6k42ydegzspfz9uh";
    let candidate_address = address.unwrap().to_string();

    println!("{} == {}", expected_address, candidate_address);
    assert_eq!(expected_address, candidate_address);
}

#[test]
pub fn view_key_signature_test() {
    let rng = &mut ChaChaRng::seed_from_u64(1231275789u64);
    let private_key = PrivateKey::<Testnet1>::new(rng);
    assert!(private_key.is_ok());

    let view_key = ViewKey::from(&private_key.unwrap());
    assert!(view_key.is_ok());

    let message: [u8; 32] = rng.gen();

    let signature = view_key.unwrap().sign(&message, rng);
    assert!(signature.is_ok());

    let expected_signature = "672cfc66d9d2c018fbac1a9d8245d3f1ed5ab2485031e64eaeeaf0c09c8cab03a4474f5d8f9f7108cce355c8c4509e3c625b78ccc63a0f203bf81a3493ce7c03";
    let candidate_signature = signature.unwrap().to_string();

    println!("{} == {}", expected_signature, candidate_signature);
    assert_eq!(expected_signature, candidate_signature);
}

#[test]
pub fn view_key_signature_verification_test() {
    let rng = &mut ChaChaRng::seed_from_u64(1231275789u64);
    let private_key = PrivateKey::<Testnet1>::new(rng);
    assert!(private_key.is_ok());

    let view_key = ViewKey::from(&private_key.unwrap()).unwrap();
    let address = Address::from_view_key(&view_key).unwrap();

    let message: [u8; 32] = rng.gen();

    let signature = view_key.sign(&message, rng);
    assert!(signature.is_ok());

    let verification = address.verify(&message, &signature.unwrap());
    assert!(verification.is_ok());
    assert!(verification.unwrap())
}

#[test]
pub fn view_key_signature_failed_verification_test() {
    let rng = &mut ChaChaRng::seed_from_u64(1231275789u64);
    let private_key = PrivateKey::<Testnet1>::new(rng);
    assert!(private_key.is_ok());

    let view_key = ViewKey::from(&private_key.unwrap()).unwrap();
    let address = Address::from_view_key(&view_key).unwrap();

    let message: [u8; 32] = rng.gen();
    let bad_message: [u8; 32] = rng.gen();

    let signature = view_key.sign(&message, rng);
    assert!(signature.is_ok());

    let verification = address.verify(&bad_message, &signature.unwrap());
    assert!(verification.is_ok());
    assert!(!verification.unwrap())
}
