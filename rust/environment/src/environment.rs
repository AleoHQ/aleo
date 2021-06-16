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

use snarkvm_dpc::{
    testnet1::{
        instantiated::Components as Testnet1Components,
        payload::Payload as Testnet1Payload,
        BaseDPCComponents,
    },
    DPCComponents,
};
use snarkvm_utilities::{FromBytes, ToBytes};

/// The target environment for building records, and transactions.
pub trait Environment {
    type Components: DPCComponents + BaseDPCComponents;

    /// Record interface
    type Payload: FromBytes + ToBytes + Default + PartialEq;
}

/// The testnet1 environment
pub struct Testnet1;

impl Environment for Testnet1 {
    type Components = Testnet1Components;
    type Payload = Testnet1Payload;
}
