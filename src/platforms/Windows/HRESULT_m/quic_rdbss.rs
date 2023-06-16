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

pub enum QUIC_RDBSS {
    ERROR_QUIC_HANDSHAKE_FAILURE = 0x80410000,
    ERROR_QUIC_VER_NEG_FAILURE = 0x80410001,
    ERROR_QUIC_USER_CANCELED = 0x80410002,
    ERROR_QUIC_INTERNAL_ERROR = 0x80410003,
    ERROR_QUIC_PROTOCOL_VIOLATION = 0x80410004,
    ERROR_QUIC_CONNECTION_IDLE = 0x80410005,
    ERROR_QUIC_CONNECTION_TIMEOUT = 0x80410006,
    ERROR_QUIC_ALPN_NEG_FAILURE = 0x80410007,
}

impl QUIC_RDBSS {
    pub fn description(&self) -> &'static str {
        match self {
            QUIC_RDBSS::ERROR_QUIC_HANDSHAKE_FAILURE => "The QUIC connection handshake failed.",
            QUIC_RDBSS::ERROR_QUIC_VER_NEG_FAILURE => "The QUIC connection failed to negotiate a compatible protocol version.",
            QUIC_RDBSS::ERROR_QUIC_USER_CANCELED => "The QUIC connection was canceled by the user.",
            QUIC_RDBSS::ERROR_QUIC_INTERNAL_ERROR => "The QUIC connection encountered an internal error.",
            QUIC_RDBSS::ERROR_QUIC_PROTOCOL_VIOLATION => "The QUIC connection encountered a protocol violation.",
            QUIC_RDBSS::ERROR_QUIC_CONNECTION_IDLE => "The QUIC connection was idle.",
            QUIC_RDBSS::ERROR_QUIC_CONNECTION_TIMEOUT => "The QUIC connection timed out while trying to contact the peer.",
            QUIC_RDBSS::ERROR_QUIC_ALPN_NEG_FAILURE => "The QUIC connection failed to negotiate a compatible ALPN.",
        }
    }
}
