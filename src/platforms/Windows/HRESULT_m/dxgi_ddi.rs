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
pub enum DXGI_DDI {
    DXGI_DDI_ERR_WASSTILLDRAWING,
    DXGI_DDI_ERR_UNSUPPORTED,
    DXGI_DDI_ERR_NONEXCLUSIVE,
}

impl DXGI_DDI {
    pub fn code(&self) -> u32 {
        return match self {
            DXGI_DDI::DXGI_DDI_ERR_WASSTILLDRAWING => 0x887B0001 as u32,
            DXGI_DDI::DXGI_DDI_ERR_UNSUPPORTED => 0x887B0002 as u32,
            DXGI_DDI::DXGI_DDI_ERR_NONEXCLUSIVE => 0x887B0003 as u32,
        }
    }

    pub fn error(&self) -> RawError {
        return match self {
            DXGI_DDI::DXGI_DDI_ERR_WASSTILLDRAWING => RawError::Kind(DXGI_DDI::DXGI_DDI_ERR_WASSTILLDRAWING),
            DXGI_DDI::DXGI_DDI_ERR_UNSUPPORTED => RawError::Kind(DXGI_DDI::DXGI_DDI_ERR_UNSUPPORTED),
            DXGI_DDI::DXGI_DDI_ERR_NONEXCLUSIVE => RawError::Kind(DXGI_DDI::DXGI_DDI_ERR_NONEXCLUSIVE),
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            DXGI_DDI::DXGI_DDI_ERR_WASSTILLDRAWING => "The GPU was busy when the operation was requested.",
            DXGI_DDI::DXGI_DDI_ERR_UNSUPPORTED => "The driver has rejected the creation of this resource.",
            DXGI_DDI::DXGI_DDI_ERR_NONEXCLUSIVE => "The GPU counter was in use by another process or d3d device when application requested access to it.",
        }
    }

    pub fn from_name(name: &str) -> DXGI_DDI {
        return match name {
            "DXGI_DDI_ERR_WASSTILLDRAWING" => DXGI_DDI::DXGI_DDI_ERR_WASSTILLDRAWING,
            "DXGI_DDI_ERR_UNSUPPORTED" => DXGI_DDI::DXGI_DDI_ERR_UNSUPPORTED,
            "DXGI_DDI_ERR_NONEXCLUSIVE" => DXGI_DDI::DXGI_DDI_ERR_NONEXCLUSIVE,
        }
    }
    pub fn from_code(code: u32) -> DXGI_DDI {
        return match code {
            0x887B0001 => DXGI_DDI::DXGI_DDI_ERR_WASSTILLDRAWING,
            0x887B0002 => DXGI_DDI::DXGI_DDI_ERR_UNSUPPORTED,
            0x887B0003 => DXGI_DDI::DXGI_DDI_ERR_NONEXCLUSIVE,
        }
    }
}
