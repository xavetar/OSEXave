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
pub enum QUIC {
    ERROR_QUIC_HANDSHAKE_FAILURE,
    ERROR_QUIC_VER_NEG_FAILURE,
    ERROR_QUIC_USER_CANCELED,
    ERROR_QUIC_INTERNAL_ERROR,
    ERROR_QUIC_PROTOCOL_VIOLATION,
    ERROR_QUIC_CONNECTION_IDLE,
    ERROR_QUIC_CONNECTION_TIMEOUT,
    ERROR_QUIC_ALPN_NEG_FAILURE,
}

impl QUIC {
    pub fn code(&self) -> u32 {
        return match self {
            QUIC::ERROR_QUIC_HANDSHAKE_FAILURE => 0x80410000 as u32,
            QUIC::ERROR_QUIC_VER_NEG_FAILURE => 0x80410001 as u32,
            QUIC::ERROR_QUIC_USER_CANCELED => 0x80410002 as u32,
            QUIC::ERROR_QUIC_INTERNAL_ERROR => 0x80410003 as u32,
            QUIC::ERROR_QUIC_PROTOCOL_VIOLATION => 0x80410004 as u32,
            QUIC::ERROR_QUIC_CONNECTION_IDLE => 0x80410005 as u32,
            QUIC::ERROR_QUIC_CONNECTION_TIMEOUT => 0x80410006 as u32,
            QUIC::ERROR_QUIC_ALPN_NEG_FAILURE => 0x80410007 as u32,
        }
    }

    pub fn error(&self) -> RawError {
        return match self {
            QUIC::ERROR_QUIC_HANDSHAKE_FAILURE => RawError::Kind(QUIC::ERROR_QUIC_HANDSHAKE_FAILURE),
            QUIC::ERROR_QUIC_VER_NEG_FAILURE => RawError::Kind(QUIC::ERROR_QUIC_VER_NEG_FAILURE),
            QUIC::ERROR_QUIC_USER_CANCELED => RawError::Kind(QUIC::ERROR_QUIC_USER_CANCELED),
            QUIC::ERROR_QUIC_INTERNAL_ERROR => RawError::Kind(QUIC::ERROR_QUIC_INTERNAL_ERROR),
            QUIC::ERROR_QUIC_PROTOCOL_VIOLATION => RawError::Kind(QUIC::ERROR_QUIC_PROTOCOL_VIOLATION),
            QUIC::ERROR_QUIC_CONNECTION_IDLE => RawError::Kind(QUIC::ERROR_QUIC_CONNECTION_IDLE),
            QUIC::ERROR_QUIC_CONNECTION_TIMEOUT => RawError::Kind(QUIC::ERROR_QUIC_CONNECTION_TIMEOUT),
            QUIC::ERROR_QUIC_ALPN_NEG_FAILURE => RawError::Kind(QUIC::ERROR_QUIC_ALPN_NEG_FAILURE),
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            QUIC::ERROR_QUIC_HANDSHAKE_FAILURE => "The QUIC connection handshake failed.",
            QUIC::ERROR_QUIC_VER_NEG_FAILURE => "The QUIC connection failed to negotiate a compatible protocol version.",
            QUIC::ERROR_QUIC_USER_CANCELED => "The QUIC connection was canceled by the user.",
            QUIC::ERROR_QUIC_INTERNAL_ERROR => "The QUIC connection encountered an internal error.",
            QUIC::ERROR_QUIC_PROTOCOL_VIOLATION => "The QUIC connection encountered a protocol violation.",
            QUIC::ERROR_QUIC_CONNECTION_IDLE => "The QUIC connection was idle.",
            QUIC::ERROR_QUIC_CONNECTION_TIMEOUT => "The QUIC connection timed out while trying to contact the peer.",
            QUIC::ERROR_QUIC_ALPN_NEG_FAILURE => "The QUIC connection failed to negotiate a compatible ALPN.",
        }
    }

    pub fn from_name(name: &str) -> QUIC {
        return match name {
            "ERROR_QUIC_HANDSHAKE_FAILURE" => QUIC::ERROR_QUIC_HANDSHAKE_FAILURE,
            "ERROR_QUIC_VER_NEG_FAILURE" => QUIC::ERROR_QUIC_VER_NEG_FAILURE,
            "ERROR_QUIC_USER_CANCELED" => QUIC::ERROR_QUIC_USER_CANCELED,
            "ERROR_QUIC_INTERNAL_ERROR" => QUIC::ERROR_QUIC_INTERNAL_ERROR,
            "ERROR_QUIC_PROTOCOL_VIOLATION" => QUIC::ERROR_QUIC_PROTOCOL_VIOLATION,
            "ERROR_QUIC_CONNECTION_IDLE" => QUIC::ERROR_QUIC_CONNECTION_IDLE,
            "ERROR_QUIC_CONNECTION_TIMEOUT" => QUIC::ERROR_QUIC_CONNECTION_TIMEOUT,
            "ERROR_QUIC_ALPN_NEG_FAILURE" => QUIC::ERROR_QUIC_ALPN_NEG_FAILURE,
        }
    }
    pub fn from_code(code: u32) -> QUIC {
        return match code {
            0x80410000 => QUIC::ERROR_QUIC_HANDSHAKE_FAILURE,
            0x80410001 => QUIC::ERROR_QUIC_VER_NEG_FAILURE,
            0x80410002 => QUIC::ERROR_QUIC_USER_CANCELED,
            0x80410003 => QUIC::ERROR_QUIC_INTERNAL_ERROR,
            0x80410004 => QUIC::ERROR_QUIC_PROTOCOL_VIOLATION,
            0x80410005 => QUIC::ERROR_QUIC_CONNECTION_IDLE,
            0x80410006 => QUIC::ERROR_QUIC_CONNECTION_TIMEOUT,
            0x80410007 => QUIC::ERROR_QUIC_ALPN_NEG_FAILURE,
        }
    }
}
