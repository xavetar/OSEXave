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
pub enum BCD {
    ERROR_BCD_NOT_ALL_ENTRIES_IMPORTED,
    ERROR_BCD_NOT_ALL_ENTRIES_SYNCHRONIZED,
    ERROR_BCD_TOO_MANY_ELEMENTS,
}

impl BCD {
    pub fn code(&self) -> u32 {
        return match self {
            BCD::ERROR_BCD_NOT_ALL_ENTRIES_IMPORTED => 0x80390001 as u32,
            BCD::ERROR_BCD_NOT_ALL_ENTRIES_SYNCHRONIZED => 0x80390003 as u32,
            BCD::ERROR_BCD_TOO_MANY_ELEMENTS => 0xC0390002 as u32,
        }
    }

    pub fn error(&self) -> RawError {
        return match self {
            BCD::ERROR_BCD_NOT_ALL_ENTRIES_IMPORTED => RawError::Kind(BCD::ERROR_BCD_NOT_ALL_ENTRIES_IMPORTED),
            BCD::ERROR_BCD_NOT_ALL_ENTRIES_SYNCHRONIZED => RawError::Kind(BCD::ERROR_BCD_NOT_ALL_ENTRIES_SYNCHRONIZED),
            BCD::ERROR_BCD_TOO_MANY_ELEMENTS => RawError::Kind(BCD::ERROR_BCD_TOO_MANY_ELEMENTS),
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            BCD::ERROR_BCD_NOT_ALL_ENTRIES_IMPORTED => "Some BCD entries were not imported correctly from the BCD store.",
            BCD::ERROR_BCD_NOT_ALL_ENTRIES_SYNCHRONIZED => "Some BCD entries were not synchronized correctly with the firmware.",
            BCD::ERROR_BCD_TOO_MANY_ELEMENTS => "Entries enumerated have exceeded the allowed threshold.",
        }
    }

    pub fn from_name(name: &str) -> BCD {
        return match name {
            "ERROR_BCD_NOT_ALL_ENTRIES_IMPORTED" => BCD::ERROR_BCD_NOT_ALL_ENTRIES_IMPORTED,
            "ERROR_BCD_NOT_ALL_ENTRIES_SYNCHRONIZED" => BCD::ERROR_BCD_NOT_ALL_ENTRIES_SYNCHRONIZED,
            "ERROR_BCD_TOO_MANY_ELEMENTS" => BCD::ERROR_BCD_TOO_MANY_ELEMENTS,
        }
    }
    pub fn from_code(code: u32) -> BCD {
        return match code {
            0x80390001 => BCD::ERROR_BCD_NOT_ALL_ENTRIES_IMPORTED,
            0x80390003 => BCD::ERROR_BCD_NOT_ALL_ENTRIES_SYNCHRONIZED,
            0xC0390002 => BCD::ERROR_BCD_TOO_MANY_ELEMENTS,
        }
    }
}
