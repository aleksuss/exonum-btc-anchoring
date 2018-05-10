// Copyright 2018 The Exonum Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// pub mod payload;

use bitcoin::util::hash;

use std::ops::Deref;

#[derive(Debug, Clone, Copy, Index, From, Into, LowerHex, PartialEq, Display)]
pub struct Sha256dHash {
    inner: hash::Sha256dHash
}

impl Deref for Sha256dHash {
    type Target = hash::Sha256dHash;

    fn deref(&self) -> &hash::Sha256dHash {
        &self.inner
    }
}