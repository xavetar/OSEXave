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

pub enum DIRECT3D11_DEBUG {
    APOERR_ALREADY_INITIALIZED = 0x887D0001,
    APOERR_NOT_INITIALIZED = 0x887D0002,
    APOERR_FORMAT_NOT_SUPPORTED = 0x887D0003,
    APOERR_INVALID_APO_CLSID = 0x887D0004,
    APOERR_BUFFERS_OVERLAP = 0x887D0005,
    APOERR_ALREADY_UNLOCKED = 0x887D0006,
    APOERR_NUM_CONNECTIONS_INVALID = 0x887D0007,
    APOERR_INVALID_OUTPUT_MAXFRAMECOUNT = 0x887D0008,
    APOERR_INVALID_CONNECTION_FORMAT = 0x887D0009,
    APOERR_APO_LOCKED = 0x887D000A,
    APOERR_INVALID_COEFFCOUNT = 0x887D000B,
    APOERR_INVALID_COEFFICIENT = 0x887D000C,
    APOERR_INVALID_CURVE_PARAM = 0x887D000D,
}

impl DIRECT3D11_DEBUG {
    pub fn description(&self) -> &'static str {
        match self {
            DIRECT3D11_DEBUG::APOERR_ALREADY_INITIALIZED => "The object has already been initialized.",
            DIRECT3D11_DEBUG::APOERR_NOT_INITIALIZED => "Object/structure is not initialized.",
            DIRECT3D11_DEBUG::APOERR_FORMAT_NOT_SUPPORTED => "A pin supporting the format cannot be found.",
            DIRECT3D11_DEBUG::APOERR_INVALID_APO_CLSID => "Invalid CLSID in an APO Initialization structure.",
            DIRECT3D11_DEBUG::APOERR_BUFFERS_OVERLAP => "Buffers overlap on an APO that does not accept in-place buffers.",
            DIRECT3D11_DEBUG::APOERR_ALREADY_UNLOCKED => "APO is already in an unlocked state.",
            DIRECT3D11_DEBUG::APOERR_NUM_CONNECTIONS_INVALID => "Number of input or output connections not valid on this APO.",
            DIRECT3D11_DEBUG::APOERR_INVALID_OUTPUT_MAXFRAMECOUNT => "Output maxFrameCount not large enough.",
            DIRECT3D11_DEBUG::APOERR_INVALID_CONNECTION_FORMAT => "Invalid connection format for this operation.",
            DIRECT3D11_DEBUG::APOERR_APO_LOCKED => "APO is locked ready to process and can not be changed.",
            DIRECT3D11_DEBUG::APOERR_INVALID_COEFFCOUNT => "Invalid coefficient count.",
            DIRECT3D11_DEBUG::APOERR_INVALID_COEFFICIENT => "Invalid coefficient.",
            DIRECT3D11_DEBUG::APOERR_INVALID_CURVE_PARAM => "An invalid curve parameter was specified.",
        }
    }
}
