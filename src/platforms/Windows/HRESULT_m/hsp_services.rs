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
pub enum HSP_SERVICES {
    HSP_E_ERROR_MASK,
    HSP_E_INTERNAL_ERROR,
    HSP_BS_ERROR_MASK,
    HSP_BS_INTERNAL_ERROR,
}

impl HSP_SERVICES {
    pub fn code(&self) -> u32 {
        return match self {
            HSP_SERVICES::HSP_E_ERROR_MASK => 0x81280000 as u32,
            HSP_SERVICES::HSP_E_INTERNAL_ERROR => 0x81280FFF as u32,
            HSP_SERVICES::HSP_BS_ERROR_MASK => 0x81281000 as u32,
            HSP_SERVICES::HSP_BS_INTERNAL_ERROR => 0x812810FF as u32,
        }
    }

    pub fn error(&self) -> RawError {
        return match self {
            HSP_SERVICES::HSP_E_ERROR_MASK => RawError::Kind(HSP_SERVICES::HSP_E_ERROR_MASK),
            HSP_SERVICES::HSP_E_INTERNAL_ERROR => RawError::Kind(HSP_SERVICES::HSP_E_INTERNAL_ERROR),
            HSP_SERVICES::HSP_BS_ERROR_MASK => RawError::Kind(HSP_SERVICES::HSP_BS_ERROR_MASK),
            HSP_SERVICES::HSP_BS_INTERNAL_ERROR => RawError::Kind(HSP_SERVICES::HSP_BS_INTERNAL_ERROR),
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            HSP_SERVICES::HSP_E_ERROR_MASK => "This is an error mask to convert HSP hardware errors to Win errors.",
            HSP_SERVICES::HSP_E_INTERNAL_ERROR => "Catastrophic internal failure in the HSP hardware.",
            HSP_SERVICES::HSP_BS_ERROR_MASK => "This is an error mask to convert HSP base services errors to Win errors.",
            HSP_SERVICES::HSP_BS_INTERNAL_ERROR => "Catastrophic internal failure in the HSP base services.",
        }
    }

    pub fn from_name(name: &str) -> HSP_SERVICES {
        return match name {
            "HSP_E_ERROR_MASK" => HSP_SERVICES::HSP_E_ERROR_MASK,
            "HSP_E_INTERNAL_ERROR" => HSP_SERVICES::HSP_E_INTERNAL_ERROR,
            "HSP_BS_ERROR_MASK" => HSP_SERVICES::HSP_BS_ERROR_MASK,
            "HSP_BS_INTERNAL_ERROR" => HSP_SERVICES::HSP_BS_INTERNAL_ERROR,
        }
    }
    pub fn from_code(code: u32) -> HSP_SERVICES {
        return match code {
            0x81280000 => HSP_SERVICES::HSP_E_ERROR_MASK,
            0x81280FFF => HSP_SERVICES::HSP_E_INTERNAL_ERROR,
            0x81281000 => HSP_SERVICES::HSP_BS_ERROR_MASK,
            0x812810FF => HSP_SERVICES::HSP_BS_INTERNAL_ERROR,
        }
    }
}
