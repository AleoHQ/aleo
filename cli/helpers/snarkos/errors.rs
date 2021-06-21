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

use uuid::Uuid;

#[derive(Debug, Error)]
pub enum RpcRequestError {
    #[error("Deserialization error: {}", _0)]
    DeserializationError(serde_json::error::Error),

    #[error("{}", _0)]
    JsonRPC(String),

    #[error("{}", _0)]
    Message(String),

    #[error("Request id {} does not match response id {}", _0, _1)]
    RequestIdMismatch(Uuid, Uuid),

    #[error("{}", _0)]
    Reqwest(reqwest::Error),

    #[error(transparent)]
    Std(std::io::Error),
}

impl From<reqwest::Error> for RpcRequestError {
    fn from(error: reqwest::Error) -> Self {
        RpcRequestError::Reqwest(error)
    }
}
