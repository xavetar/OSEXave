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
pub enum USERMODE_SDBUS {
    ERROR_IO_PREEMPTED,
}

impl USERMODE_SDBUS {
    pub fn code(&self) -> u32 {
        return match self {
            USERMODE_SDBUS::ERROR_IO_PREEMPTED => 0x89010001 as u32,
        }
    }

    pub fn error(&self) -> RawError {
        return match self {
            USERMODE_SDBUS::ERROR_IO_PREEMPTED => RawError::Kind(USERMODE_SDBUS::ERROR_IO_PREEMPTED),
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            USERMODE_SDBUS::ERROR_IO_PREEMPTED => "The operation was preempted by a higher priority operation. It must be resumed later.",
        }
    }

    pub fn from_name(name: &str) -> USERMODE_SDBUS {
        return match name {
            "ERROR_IO_PREEMPTED" => USERMODE_SDBUS::ERROR_IO_PREEMPTED,
        }
    }
    pub fn from_code(code: u32) -> USERMODE_SDBUS {
        return match code {
            0x89010001 => USERMODE_SDBUS::ERROR_IO_PREEMPTED,
        }
    }
}
