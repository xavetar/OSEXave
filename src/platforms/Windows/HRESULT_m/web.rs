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
pub enum WEB {
    WEB_E_UNSUPPORTED_FORMAT,
    WEB_E_INVALID_XML,
    WEB_E_MISSING_REQUIRED_ELEMENT,
    WEB_E_MISSING_REQUIRED_ATTRIBUTE,
    WEB_E_UNEXPECTED_CONTENT,
    WEB_E_RESOURCE_TOO_LARGE,
    WEB_E_INVALID_JSON_STRING,
    WEB_E_INVALID_JSON_NUMBER,
    WEB_E_JSON_VALUE_NOT_FOUND,
}

impl WEB {
    pub fn code(&self) -> u32 {
        return match self {
            WEB::WEB_E_UNSUPPORTED_FORMAT => 0x83750001 as u32,
            WEB::WEB_E_INVALID_XML => 0x83750002 as u32,
            WEB::WEB_E_MISSING_REQUIRED_ELEMENT => 0x83750003 as u32,
            WEB::WEB_E_MISSING_REQUIRED_ATTRIBUTE => 0x83750004 as u32,
            WEB::WEB_E_UNEXPECTED_CONTENT => 0x83750005 as u32,
            WEB::WEB_E_RESOURCE_TOO_LARGE => 0x83750006 as u32,
            WEB::WEB_E_INVALID_JSON_STRING => 0x83750007 as u32,
            WEB::WEB_E_INVALID_JSON_NUMBER => 0x83750008 as u32,
            WEB::WEB_E_JSON_VALUE_NOT_FOUND => 0x83750009 as u32,
        }
    }

    pub fn error(&self) -> RawError {
        return match self {
            WEB::WEB_E_UNSUPPORTED_FORMAT => RawError::Kind(WEB::WEB_E_UNSUPPORTED_FORMAT),
            WEB::WEB_E_INVALID_XML => RawError::Kind(WEB::WEB_E_INVALID_XML),
            WEB::WEB_E_MISSING_REQUIRED_ELEMENT => RawError::Kind(WEB::WEB_E_MISSING_REQUIRED_ELEMENT),
            WEB::WEB_E_MISSING_REQUIRED_ATTRIBUTE => RawError::Kind(WEB::WEB_E_MISSING_REQUIRED_ATTRIBUTE),
            WEB::WEB_E_UNEXPECTED_CONTENT => RawError::Kind(WEB::WEB_E_UNEXPECTED_CONTENT),
            WEB::WEB_E_RESOURCE_TOO_LARGE => RawError::Kind(WEB::WEB_E_RESOURCE_TOO_LARGE),
            WEB::WEB_E_INVALID_JSON_STRING => RawError::Kind(WEB::WEB_E_INVALID_JSON_STRING),
            WEB::WEB_E_INVALID_JSON_NUMBER => RawError::Kind(WEB::WEB_E_INVALID_JSON_NUMBER),
            WEB::WEB_E_JSON_VALUE_NOT_FOUND => RawError::Kind(WEB::WEB_E_JSON_VALUE_NOT_FOUND),
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            WEB::WEB_E_UNSUPPORTED_FORMAT => "Unsupported format.",
            WEB::WEB_E_INVALID_XML => "Invalid XML.",
            WEB::WEB_E_MISSING_REQUIRED_ELEMENT => "Missing required element.",
            WEB::WEB_E_MISSING_REQUIRED_ATTRIBUTE => "Missing required attribute.",
            WEB::WEB_E_UNEXPECTED_CONTENT => "Unexpected content.",
            WEB::WEB_E_RESOURCE_TOO_LARGE => "Resource too large.",
            WEB::WEB_E_INVALID_JSON_STRING => "Invalid JSON string.",
            WEB::WEB_E_INVALID_JSON_NUMBER => "Invalid JSON number.",
            WEB::WEB_E_JSON_VALUE_NOT_FOUND => "JSON value not found.",
        }
    }

    pub fn from_name(name: &str) -> WEB {
        return match name {
            "WEB_E_UNSUPPORTED_FORMAT" => WEB::WEB_E_UNSUPPORTED_FORMAT,
            "WEB_E_INVALID_XML" => WEB::WEB_E_INVALID_XML,
            "WEB_E_MISSING_REQUIRED_ELEMENT" => WEB::WEB_E_MISSING_REQUIRED_ELEMENT,
            "WEB_E_MISSING_REQUIRED_ATTRIBUTE" => WEB::WEB_E_MISSING_REQUIRED_ATTRIBUTE,
            "WEB_E_UNEXPECTED_CONTENT" => WEB::WEB_E_UNEXPECTED_CONTENT,
            "WEB_E_RESOURCE_TOO_LARGE" => WEB::WEB_E_RESOURCE_TOO_LARGE,
            "WEB_E_INVALID_JSON_STRING" => WEB::WEB_E_INVALID_JSON_STRING,
            "WEB_E_INVALID_JSON_NUMBER" => WEB::WEB_E_INVALID_JSON_NUMBER,
            "WEB_E_JSON_VALUE_NOT_FOUND" => WEB::WEB_E_JSON_VALUE_NOT_FOUND,
        }
    }
    pub fn from_code(code: u32) -> WEB {
        return match code {
            0x83750001 => WEB::WEB_E_UNSUPPORTED_FORMAT,
            0x83750002 => WEB::WEB_E_INVALID_XML,
            0x83750003 => WEB::WEB_E_MISSING_REQUIRED_ELEMENT,
            0x83750004 => WEB::WEB_E_MISSING_REQUIRED_ATTRIBUTE,
            0x83750005 => WEB::WEB_E_UNEXPECTED_CONTENT,
            0x83750006 => WEB::WEB_E_RESOURCE_TOO_LARGE,
            0x83750007 => WEB::WEB_E_INVALID_JSON_STRING,
            0x83750008 => WEB::WEB_E_INVALID_JSON_NUMBER,
            0x83750009 => WEB::WEB_E_JSON_VALUE_NOT_FOUND,
        }
    }
}
