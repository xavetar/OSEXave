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
pub enum USERMODE_VOLSNAP {
    ERROR_VOLSNAP_BOOTFILE_NOT_VALID,
    ERROR_VOLSNAP_ACTIVATION_TIMEOUT,
    ERROR_VOLSNAP_NO_BYPASSIO_WITH_SNAPSHOT,
}

impl USERMODE_VOLSNAP {
    pub fn code(&self) -> u32 {
        return match self {
            USERMODE_VOLSNAP::ERROR_VOLSNAP_BOOTFILE_NOT_VALID => 0x80820001 as u32,
            USERMODE_VOLSNAP::ERROR_VOLSNAP_ACTIVATION_TIMEOUT => 0x80820002 as u32,
            USERMODE_VOLSNAP::ERROR_VOLSNAP_NO_BYPASSIO_WITH_SNAPSHOT => 0x80820003 as u32,
        }
    }

    pub fn error(&self) -> RawError {
        return match self {
            USERMODE_VOLSNAP::ERROR_VOLSNAP_BOOTFILE_NOT_VALID => RawError::Kind(USERMODE_VOLSNAP::ERROR_VOLSNAP_BOOTFILE_NOT_VALID),
            USERMODE_VOLSNAP::ERROR_VOLSNAP_ACTIVATION_TIMEOUT => RawError::Kind(USERMODE_VOLSNAP::ERROR_VOLSNAP_ACTIVATION_TIMEOUT),
            USERMODE_VOLSNAP::ERROR_VOLSNAP_NO_BYPASSIO_WITH_SNAPSHOT => RawError::Kind(USERMODE_VOLSNAP::ERROR_VOLSNAP_NO_BYPASSIO_WITH_SNAPSHOT),
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            USERMODE_VOLSNAP::ERROR_VOLSNAP_BOOTFILE_NOT_VALID => "The bootfile is too small to support persistent snapshots.",
            USERMODE_VOLSNAP::ERROR_VOLSNAP_ACTIVATION_TIMEOUT => "Activation of persistent snapshots on this volume took longer than was allowed.",
            USERMODE_VOLSNAP::ERROR_VOLSNAP_NO_BYPASSIO_WITH_SNAPSHOT => "BypassIO cannot be enabled while a volume snapshot exists.",
        }
    }

    pub fn from_name(name: &str) -> USERMODE_VOLSNAP {
        return match name {
            "ERROR_VOLSNAP_BOOTFILE_NOT_VALID" => USERMODE_VOLSNAP::ERROR_VOLSNAP_BOOTFILE_NOT_VALID,
            "ERROR_VOLSNAP_ACTIVATION_TIMEOUT" => USERMODE_VOLSNAP::ERROR_VOLSNAP_ACTIVATION_TIMEOUT,
            "ERROR_VOLSNAP_NO_BYPASSIO_WITH_SNAPSHOT" => USERMODE_VOLSNAP::ERROR_VOLSNAP_NO_BYPASSIO_WITH_SNAPSHOT,
        }
    }
    pub fn from_code(code: u32) -> USERMODE_VOLSNAP {
        return match code {
            0x80820001 => USERMODE_VOLSNAP::ERROR_VOLSNAP_BOOTFILE_NOT_VALID,
            0x80820002 => USERMODE_VOLSNAP::ERROR_VOLSNAP_ACTIVATION_TIMEOUT,
            0x80820003 => USERMODE_VOLSNAP::ERROR_VOLSNAP_NO_BYPASSIO_WITH_SNAPSHOT,
        }
    }
}
