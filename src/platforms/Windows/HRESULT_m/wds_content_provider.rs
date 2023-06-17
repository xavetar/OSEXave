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
pub enum WDS_CONTENT_PROVIDER {
    WDSCP_E_INVALID_CONFIGURATION,
    WDSCP_E_NOT_A_DIRECTORY,
    WDSCP_E_CONFIG_STRING_REQUIRED,
}

impl WDS_CONTENT_PROVIDER {
    pub fn code(&self) -> u32 {
        return match self {
            WDS_CONTENT_PROVIDER::WDSCP_E_INVALID_CONFIGURATION => 0xC1250100 as u32,
            WDS_CONTENT_PROVIDER::WDSCP_E_NOT_A_DIRECTORY => 0xC1250101 as u32,
            WDS_CONTENT_PROVIDER::WDSCP_E_CONFIG_STRING_REQUIRED => 0xC1250102 as u32,
        }
    }

    pub fn error(&self) -> RawError {
        return match self {
            WDS_CONTENT_PROVIDER::WDSCP_E_INVALID_CONFIGURATION => RawError::Kind(WDS_CONTENT_PROVIDER::WDSCP_E_INVALID_CONFIGURATION),
            WDS_CONTENT_PROVIDER::WDSCP_E_NOT_A_DIRECTORY => RawError::Kind(WDS_CONTENT_PROVIDER::WDSCP_E_NOT_A_DIRECTORY),
            WDS_CONTENT_PROVIDER::WDSCP_E_CONFIG_STRING_REQUIRED => RawError::Kind(WDS_CONTENT_PROVIDER::WDSCP_E_CONFIG_STRING_REQUIRED),
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            WDS_CONTENT_PROVIDER::WDSCP_E_INVALID_CONFIGURATION => "The configuration string was invalid or empty.",
            WDS_CONTENT_PROVIDER::WDSCP_E_NOT_A_DIRECTORY => "The path specified in the configuration string was not a directory.",
            WDS_CONTENT_PROVIDER::WDSCP_E_CONFIG_STRING_REQUIRED => "The WDS Content Provider requires that a configuration string be provided.",
        }
    }

    pub fn from_name(name: &str) -> WDS_CONTENT_PROVIDER {
        return match name {
            "WDSCP_E_INVALID_CONFIGURATION" => WDS_CONTENT_PROVIDER::WDSCP_E_INVALID_CONFIGURATION,
            "WDSCP_E_NOT_A_DIRECTORY" => WDS_CONTENT_PROVIDER::WDSCP_E_NOT_A_DIRECTORY,
            "WDSCP_E_CONFIG_STRING_REQUIRED" => WDS_CONTENT_PROVIDER::WDSCP_E_CONFIG_STRING_REQUIRED,
        }
    }
    pub fn from_code(code: u32) -> WDS_CONTENT_PROVIDER {
        return match code {
            0xC1250100 => WDS_CONTENT_PROVIDER::WDSCP_E_INVALID_CONFIGURATION,
            0xC1250101 => WDS_CONTENT_PROVIDER::WDSCP_E_NOT_A_DIRECTORY,
            0xC1250102 => WDS_CONTENT_PROVIDER::WDSCP_E_CONFIG_STRING_REQUIRED,
        }
    }
}
