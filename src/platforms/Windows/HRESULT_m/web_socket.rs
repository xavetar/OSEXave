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
pub enum WEB_SOCKET {
    E_INVALID_PROTOCOL_OPERATION,
    E_INVALID_PROTOCOL_FORMAT,
    E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED,
    E_SUBPROTOCOL_NOT_SUPPORTED,
    E_PROTOCOL_VERSION_NOT_SUPPORTED,
}

impl WEB_SOCKET {
    pub fn code(&self) -> u32 {
        return match self {
            WEB_SOCKET::E_INVALID_PROTOCOL_OPERATION => 0x83760001 as u32,
            WEB_SOCKET::E_INVALID_PROTOCOL_FORMAT => 0x83760002 as u32,
            WEB_SOCKET::E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED => 0x83760003 as u32,
            WEB_SOCKET::E_SUBPROTOCOL_NOT_SUPPORTED => 0x83760004 as u32,
            WEB_SOCKET::E_PROTOCOL_VERSION_NOT_SUPPORTED => 0x83760005 as u32,
        }
    }

    pub fn error(&self) -> RawError {
        return match self {
            WEB_SOCKET::E_INVALID_PROTOCOL_OPERATION => RawError::Kind(WEB_SOCKET::E_INVALID_PROTOCOL_OPERATION),
            WEB_SOCKET::E_INVALID_PROTOCOL_FORMAT => RawError::Kind(WEB_SOCKET::E_INVALID_PROTOCOL_FORMAT),
            WEB_SOCKET::E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED => RawError::Kind(WEB_SOCKET::E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED),
            WEB_SOCKET::E_SUBPROTOCOL_NOT_SUPPORTED => RawError::Kind(WEB_SOCKET::E_SUBPROTOCOL_NOT_SUPPORTED),
            WEB_SOCKET::E_PROTOCOL_VERSION_NOT_SUPPORTED => RawError::Kind(WEB_SOCKET::E_PROTOCOL_VERSION_NOT_SUPPORTED),
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            WEB_SOCKET::E_INVALID_PROTOCOL_OPERATION => "Invalid operation performed by the protocol.",
            WEB_SOCKET::E_INVALID_PROTOCOL_FORMAT => "Invalid data format for the specific protocol operation.",
            WEB_SOCKET::E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED => "Protocol extensions are not supported.",
            WEB_SOCKET::E_SUBPROTOCOL_NOT_SUPPORTED => "Subprotocol is not supported.",
            WEB_SOCKET::E_PROTOCOL_VERSION_NOT_SUPPORTED => "Incorrect protocol version.",
        }
    }

    pub fn from_name(name: &str) -> WEB_SOCKET {
        return match name {
            "E_INVALID_PROTOCOL_OPERATION" => WEB_SOCKET::E_INVALID_PROTOCOL_OPERATION,
            "E_INVALID_PROTOCOL_FORMAT" => WEB_SOCKET::E_INVALID_PROTOCOL_FORMAT,
            "E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED" => WEB_SOCKET::E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED,
            "E_SUBPROTOCOL_NOT_SUPPORTED" => WEB_SOCKET::E_SUBPROTOCOL_NOT_SUPPORTED,
            "E_PROTOCOL_VERSION_NOT_SUPPORTED" => WEB_SOCKET::E_PROTOCOL_VERSION_NOT_SUPPORTED,
        }
    }
    pub fn from_code(code: u32) -> WEB_SOCKET {
        return match code {
            0x83760001 => WEB_SOCKET::E_INVALID_PROTOCOL_OPERATION,
            0x83760002 => WEB_SOCKET::E_INVALID_PROTOCOL_FORMAT,
            0x83760003 => WEB_SOCKET::E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED,
            0x83760004 => WEB_SOCKET::E_SUBPROTOCOL_NOT_SUPPORTED,
            0x83760005 => WEB_SOCKET::E_PROTOCOL_VERSION_NOT_SUPPORTED,
        }
    }
}
