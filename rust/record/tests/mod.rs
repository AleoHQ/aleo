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

pub mod record;
pub mod record_builder;
pub mod record_encryption;

// The test data for records
pub const TEST_ENCRYPTED_RECORD: &str = "0841ad8eb642c4a2475e2d6c3548f445253db290842531d9b5e25effe74d3eee03c097f5273f56517fe1615100f820577619242101568ddc5da5972b7b7c1c760a6969ddc7ed39cd774a18bc15d5cf38c6d59df1d14e05add65f0e4e6a54b2c901f1580a556f9e9f8e438cdb0d92fa0da1642816eb9318c14387be499d7481950847131dbb8496d3dcc58811dfa96df2bd2ad769cb69438bb1a2657625686b140f1196bfe7a292673f8502acc9cd1ac30f0d16342759105882b3026dafa030320285daefd9fde6dc65dd33541452b43a3bf17e57cf2f147392edc8f8c65af3850020b79c96609743cbfd0b21249265c84344e1c993b480cd042e296d66c17bc7056500";
pub const TEST_RECORD: &str = "4f6d042c3bc73e412f4b4740ad27354a1b25bb9df93f29313350356aa88dca050064000000000000000000000000000000000000000000000000000000000000000000000000000000304e7ae3ef9577877ddcef8f8c5d9b5e3bf544c78c50c51213857f35c33c3502df12f0fb72a0d7c56ccd31a87dada92b00304e7ae3ef9577877ddcef8f8c5d9b5e3bf544c78c50c51213857f35c33c3502df12f0fb72a0d7c56ccd31a87dada92b00d3cb77fba8ef681178b0dd54dedae5d8c72ea496acb71a788a9bf3be8cf552082ffd75e3e518853ef2328b72b29cda505241bf26b264e85cb12556e3d4556b03e095bb02db522a7dd926a2dff52838bb541dd9cf6eb07c416deeee6f30e54a02";
pub const TEST_RECORD_OWNER: &str = "aleo1faksgtpmculyzt6tgaq26fe4fgdjtwualyljjvfn2q6k42ydegzspfz9uh";
pub const TEST_RECORD_IS_DUMMY: bool = false;
pub const TEST_RECORD_PAYLOAD: &str = "0000000000000000000000000000000000000000000000000000000000000000";
pub const TEST_RECORD_BIRTH_PROGRAM_ID: &str =
    "4e7ae3ef9577877ddcef8f8c5d9b5e3bf544c78c50c51213857f35c33c3502df12f0fb72a0d7c56ccd31a87dada92b00";
pub const TEST_RECORD_DEATH_PROGRAM_ID: &str =
    "4e7ae3ef9577877ddcef8f8c5d9b5e3bf544c78c50c51213857f35c33c3502df12f0fb72a0d7c56ccd31a87dada92b00";
pub const TEST_RECORD_SERIAL_NUMBER_NONCE: &str = "d3cb77fba8ef681178b0dd54dedae5d8c72ea496acb71a788a9bf3be8cf55208";
pub const TEST_RECORD_COMMITMENT: &str = "2ffd75e3e518853ef2328b72b29cda505241bf26b264e85cb12556e3d4556b03";
pub const TEST_RECORD_COMMITMENT_RANDOMNESS: &str = "e095bb02db522a7dd926a2dff52838bb541dd9cf6eb07c416deeee6f30e54a02";
pub const TEST_RECORD_VALUE: u64 = 100;
pub const TEST_SERIAL_NUMBER: &str = "7b5ead0c963658ed7127bcdcb916eb42397824ee82a23c8c3435a60469630410afc2550c22cb9c3b26c5899a4b8150f12b75cfcc032d21dd735b1feb5d87e50e";
pub const TEST_PRIVATE_KEY: &str = "APrivateKey1tvv5YV1dipNiku2My8jMkqpqCyYKvR5Jq4y2mtjw7s77Zpn";
