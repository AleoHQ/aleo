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

use snarkvm_algorithms::CRHError;
use snarkvm_dpc::{AccountError, DPCError, TransactionError as DPCTransactionError};

use anyhow::Error as AnyhowError;

#[derive(Debug, Error)]
pub enum TransactionError {
    #[error("{}", _0)]
    AccountError(#[from] AccountError),

    #[error("{}", _0)]
    AnyhowError(#[from] AnyhowError),

    #[error("Failed to build Transaction data type. See console logs for error")]
    BuilderError,

    #[error("{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[error("{}", _0)]
    CRHError(#[from] CRHError),

    #[error("{}", _0)]
    DPCError(#[from] DPCError),

    #[error("{}", _0)]
    DPCTransactionError(#[from] DPCTransactionError),

    #[error("Attempted to set transaction builder argument {} twice", _0)]
    DuplicateArgument(String),

    #[error("Missing Transaction field: {}", _0)]
    MissingField(String),
}

impl From<std::io::Error> for TransactionError {
    fn from(error: std::io::Error) -> Self {
        TransactionError::Crate("std::io", format!("{:?}", error))
    }
}