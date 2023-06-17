/*
 * Copyright 2023 Stanislav Mikhailov (xavetar)
 *
 * Licensed under the Creative Commons Zero v1.0 Universal (CC0) License.
 * You may obtain a copy of the License at
 *
 *     http://creativecommons.org/publicdomain/zero/1.0/
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the CC0 license is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// It is strictly forbidden to use the from_code method, respected Microsoft decided to define
// duplicate codes, but by defining different constant names, therefore, when using these codes,
// it is mandatory to use through from_name, and not from_code. Otherwise, it may cause undefined
// behavior or an unknown exception. Because one code corresponds to several constants.

use super::{RawError};

#[derive(Clone, Debug, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SMB {
    ERROR_SMB_NO_PREAUTH_INTEGRITY_HASH_OVERLAP,
    ERROR_SMB_BAD_CLUSTER_DIALECT,
    ERROR_SMB_NO_SIGNING_ALGORITHM_OVERLAP,
}

impl SMB {
    pub fn code(&self) -> u32 {
        return match self {
            SMB::ERROR_SMB_NO_PREAUTH_INTEGRITY_HASH_OVERLAP => 0xC05D0000 as u32,
            SMB::ERROR_SMB_BAD_CLUSTER_DIALECT => 0xC05D0001 as u32,
            SMB::ERROR_SMB_NO_SIGNING_ALGORITHM_OVERLAP => 0xC05D0002 as u32,
        }
    }

    pub fn error(&self) -> RawError {
        return match self {
            SMB::ERROR_SMB_NO_PREAUTH_INTEGRITY_HASH_OVERLAP => RawError::Kind(SMB::ERROR_SMB_NO_PREAUTH_INTEGRITY_HASH_OVERLAP),
            SMB::ERROR_SMB_BAD_CLUSTER_DIALECT => RawError::Kind(SMB::ERROR_SMB_BAD_CLUSTER_DIALECT),
            SMB::ERROR_SMB_NO_SIGNING_ALGORITHM_OVERLAP => RawError::Kind(SMB::ERROR_SMB_NO_SIGNING_ALGORITHM_OVERLAP),
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            SMB::ERROR_SMB_NO_PREAUTH_INTEGRITY_HASH_OVERLAP => "Failed to negotiate a preauthentication integrity hash function.",
            SMB::ERROR_SMB_BAD_CLUSTER_DIALECT => "The current cluster functional level does not support this SMB dialect.",
            SMB::ERROR_SMB_NO_SIGNING_ALGORITHM_OVERLAP => "Failed to negotiate a signing hash function.",
        }
    }

    pub fn from_name(name: &str) -> SMB {
        return match name {
            "ERROR_SMB_NO_PREAUTH_INTEGRITY_HASH_OVERLAP" => SMB::ERROR_SMB_NO_PREAUTH_INTEGRITY_HASH_OVERLAP,
            "ERROR_SMB_BAD_CLUSTER_DIALECT" => SMB::ERROR_SMB_BAD_CLUSTER_DIALECT,
            "ERROR_SMB_NO_SIGNING_ALGORITHM_OVERLAP" => SMB::ERROR_SMB_NO_SIGNING_ALGORITHM_OVERLAP,
        }
    }
    pub fn from_code(code: u32) -> SMB {
        return match code {
            0xC05D0000 => SMB::ERROR_SMB_NO_PREAUTH_INTEGRITY_HASH_OVERLAP,
            0xC05D0001 => SMB::ERROR_SMB_BAD_CLUSTER_DIALECT,
            0xC05D0002 => SMB::ERROR_SMB_NO_SIGNING_ALGORITHM_OVERLAP,
        }
    }
}
