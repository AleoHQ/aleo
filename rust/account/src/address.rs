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

use crate::{
    view_key::{Signature, ViewKey},
    AddressError,
    PrivateKey,
};
use aleo_network::Network;

use snarkvm_algorithms::traits::SignatureScheme;
use snarkvm_dpc::{
    account::AccountAddress,
    testnet1::{parameters::SystemParameters},
};
use snarkvm_utilities::bytes::{FromBytes, ToBytes};

use std::{
    fmt,
    io::{Read, Result as IoResult, Write},
    str::FromStr,
};

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct Address<N: Network> {
    pub address: AccountAddress<N::Components>,
}

impl<N: Network> Address<N> {
    pub fn from(private_key: &PrivateKey<N>) -> Result<Self, AddressError> {
        let parameters = SystemParameters::<N::Components>::load()?;
        let address = AccountAddress::<N::Components>::from_private_key(
            &parameters.account_signature,
            &parameters.account_commitment,
            &parameters.account_encryption,
            &private_key.private_key,
        )?;
        Ok(Self { address })
    }

    pub fn from_view_key(view_key: &ViewKey<N>) -> Result<Self, AddressError> {
        let parameters = SystemParameters::<N::Components>::load()?;
        let address = AccountAddress::<N::Components>::from_view_key(&parameters.account_encryption, &view_key.view_key)?;
        Ok(Self { address })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut output = vec![];
        self.address.write(&mut output).expect("serialization to bytes failed");
        output
    }

    /// Verify a signature signed by the view key
    /// Returns `true` if the signature is verified correctly. Otherwise, returns `false`.
    pub fn verify(&self, message: &[u8], signature: &Signature<N>) -> Result<bool, AddressError> {
        let parameters = SystemParameters::<N::Components>::load()?;

        Ok(parameters
            .account_encryption
            .verify(&self.address.encryption_key, message, &signature.0)?)
    }
}

impl<N: Network> FromStr for Address<N> {
    type Err = AddressError;

    fn from_str(address: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            address: AccountAddress::<N::Components>::from_str(address)?,
        })
    }
}

impl<N: Network> fmt::Display for Address<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.address.to_string())
    }
}

impl<N: Network> ToBytes for Address<N> {
    fn write<W: Write>(&self, mut writer: W) -> IoResult<()> {
        self.address.write(&mut writer)
    }
}

impl<N: Network> FromBytes for Address<N> {
    /// Reads in an account address buffer.
    #[inline]
    fn read<R: Read>(mut reader: R) -> IoResult<Self> {
        Ok(Self {
            address: AccountAddress::<N::Components>::read(&mut reader)?,
        })
    }
}
