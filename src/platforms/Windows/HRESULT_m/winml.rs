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
pub enum WINML {
    WINML_ERR_INVALID_DEVICE,
    WINML_ERR_INVALID_BINDING,
    WINML_ERR_VALUE_NOTFOUND,
    WINML_ERR_SIZE_MISMATCH,
}

impl WINML {
    pub fn code(&self) -> u32 {
        return match self {
            WINML::WINML_ERR_INVALID_DEVICE => 0x88900001 as u32,
            WINML::WINML_ERR_INVALID_BINDING => 0x88900002 as u32,
            WINML::WINML_ERR_VALUE_NOTFOUND => 0x88900003 as u32,
            WINML::WINML_ERR_SIZE_MISMATCH => 0x88900004 as u32,
        }
    }

    pub fn error(&self) -> RawError {
        return match self {
            WINML::WINML_ERR_INVALID_DEVICE => RawError::Kind(WINML::WINML_ERR_INVALID_DEVICE),
            WINML::WINML_ERR_INVALID_BINDING => RawError::Kind(WINML::WINML_ERR_INVALID_BINDING),
            WINML::WINML_ERR_VALUE_NOTFOUND => RawError::Kind(WINML::WINML_ERR_VALUE_NOTFOUND),
            WINML::WINML_ERR_SIZE_MISMATCH => RawError::Kind(WINML::WINML_ERR_SIZE_MISMATCH),
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            WINML::WINML_ERR_INVALID_DEVICE => "The device is invalid or does not support machine learning.",
            WINML::WINML_ERR_INVALID_BINDING => "The binding is incomplete or does not match the input/output description.",
            WINML::WINML_ERR_VALUE_NOTFOUND => "An attempt was made to bind an unknown input or output.",
            WINML::WINML_ERR_SIZE_MISMATCH => "The size of the buffer provided for a bound variable is invalid.",
        }
    }

    pub fn from_name(name: &str) -> WINML {
        return match name {
            "WINML_ERR_INVALID_DEVICE" => WINML::WINML_ERR_INVALID_DEVICE,
            "WINML_ERR_INVALID_BINDING" => WINML::WINML_ERR_INVALID_BINDING,
            "WINML_ERR_VALUE_NOTFOUND" => WINML::WINML_ERR_VALUE_NOTFOUND,
            "WINML_ERR_SIZE_MISMATCH" => WINML::WINML_ERR_SIZE_MISMATCH,
        }
    }
    pub fn from_code(code: u32) -> WINML {
        return match code {
            0x88900001 => WINML::WINML_ERR_INVALID_DEVICE,
            0x88900002 => WINML::WINML_ERR_INVALID_BINDING,
            0x88900003 => WINML::WINML_ERR_VALUE_NOTFOUND,
            0x88900004 => WINML::WINML_ERR_SIZE_MISMATCH,
        }
    }
}
