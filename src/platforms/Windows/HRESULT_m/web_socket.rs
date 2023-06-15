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

pub enum WEB_SOCKET {
    E_INVALID_PROTOCOL_OPERATION = 0x83760001,
    E_INVALID_PROTOCOL_FORMAT = 0x83760002,
    E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED = 0x83760003,
    E_SUBPROTOCOL_NOT_SUPPORTED = 0x83760004,
    E_PROTOCOL_VERSION_NOT_SUPPORTED = 0x83760005,
}

impl WEB_SOCKET {
    pub fn description(&self) -> &'static str {
        match self {
            WEB_SOCKET::E_INVALID_PROTOCOL_OPERATION => "Invalid operation performed by the protocol.",
            WEB_SOCKET::E_INVALID_PROTOCOL_FORMAT => "Invalid data format for the specific protocol operation.",
            WEB_SOCKET::E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED => "Protocol extensions are not supported.",
            WEB_SOCKET::E_SUBPROTOCOL_NOT_SUPPORTED => "Subprotocol is not supported.",
            WEB_SOCKET::E_PROTOCOL_VERSION_NOT_SUPPORTED => "Incorrect protocol version.",
        }
    }
}
