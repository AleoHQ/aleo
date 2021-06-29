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

use crate::{PrivateKey, ViewKeyError};
use aleo_network::Network;

use snarkvm_algorithms::traits::SignatureScheme;
use snarkvm_dpc::{
    account::AccountViewKey,
    testnet1::{parameters::SystemParameters},
    traits::DPCComponents,
};
use snarkvm_utilities::{
    bytes::{FromBytes, ToBytes},
    to_bytes,
};

use rand::{CryptoRng, Rng};
use std::{fmt, str::FromStr};

#[derive(Debug)]
pub struct ViewKey<N: Network> {
    pub view_key: AccountViewKey<N::Components>,
}

impl<N: Network> ViewKey<N> {
    pub fn from(private_key: &PrivateKey<N>) -> Result<Self, ViewKeyError> {
        let parameters = SystemParameters::<N::Components>::load()?;
        let view_key = AccountViewKey::<N::Components>::from_private_key(
            &parameters.account_signature,
            &parameters.account_commitment,
            &private_key.private_key,
        )?;
        Ok(Self { view_key })
    }

    /// Sign message with the view key.
    pub fn sign<R: Rng + CryptoRng>(&self, message: &[u8], rng: &mut R) -> Result<Signature<N>, ViewKeyError> {
        let parameters = SystemParameters::<N::Components>::load()?;

        let signature = parameters
            .account_encryption
            .sign(&self.view_key.decryption_key, message, rng)?;

        Ok(Signature(signature))
    }
}

impl<N: Network> FromStr for ViewKey<N> {
    type Err = ViewKeyError;

    fn from_str(view_key: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            view_key: AccountViewKey::<N::Components>::from_str(view_key)?,
        })
    }
}

impl<N: Network> fmt::Display for ViewKey<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.view_key.to_string())
    }
}

/// An account view key signature.
pub struct Signature<N: Network>(pub <<N::Components as DPCComponents>::AccountEncryption as SignatureScheme>::Output);

impl<N: Network> FromStr for Signature<N> {
    type Err = ViewKeyError;

    fn from_str(signature: &str) -> Result<Self, Self::Err> {
        let signature_bytes = hex::decode(signature)?;
        let signature: <<N::Components as DPCComponents>::AccountEncryption as SignatureScheme>::Output =
            FromBytes::read(&signature_bytes[..])?;

        Ok(Self(signature))
    }
}

impl<N: Network> fmt::Display for Signature<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            hex::encode(to_bytes![self.0].expect("failed to convert to bytes"))
        )
    }
}
