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
pub enum DIRECT3D12 {
    D3D12_ERROR_ADAPTER_NOT_FOUND,
    D3D12_ERROR_DRIVER_VERSION_MISMATCH,
    D3D12_ERROR_INVALID_REDIST,
}

impl DIRECT3D12 {
    pub fn code(&self) -> u32 {
        return match self {
            DIRECT3D12::D3D12_ERROR_ADAPTER_NOT_FOUND => 0x887E0001 as u32,
            DIRECT3D12::D3D12_ERROR_DRIVER_VERSION_MISMATCH => 0x887E0002 as u32,
            DIRECT3D12::D3D12_ERROR_INVALID_REDIST => 0x887E0003 as u32,
        }
    }

    pub fn error(&self) -> RawError {
        return match self {
            DIRECT3D12::D3D12_ERROR_ADAPTER_NOT_FOUND => RawError::Kind(DIRECT3D12::D3D12_ERROR_ADAPTER_NOT_FOUND),
            DIRECT3D12::D3D12_ERROR_DRIVER_VERSION_MISMATCH => RawError::Kind(DIRECT3D12::D3D12_ERROR_DRIVER_VERSION_MISMATCH),
            DIRECT3D12::D3D12_ERROR_INVALID_REDIST => RawError::Kind(DIRECT3D12::D3D12_ERROR_INVALID_REDIST),
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            DIRECT3D12::D3D12_ERROR_ADAPTER_NOT_FOUND => "The blob provided does not match the adapter that the device was created on.",
            DIRECT3D12::D3D12_ERROR_DRIVER_VERSION_MISMATCH => "The blob provided was created for a different version of the driver, and must be re-created.",
            DIRECT3D12::D3D12_ERROR_INVALID_REDIST => "The D3D12 SDK version configuration of the host exe is invalid.",
        }
    }

    pub fn from_name(name: &str) -> DIRECT3D12 {
        return match name {
            "D3D12_ERROR_ADAPTER_NOT_FOUND" => DIRECT3D12::D3D12_ERROR_ADAPTER_NOT_FOUND,
            "D3D12_ERROR_DRIVER_VERSION_MISMATCH" => DIRECT3D12::D3D12_ERROR_DRIVER_VERSION_MISMATCH,
            "D3D12_ERROR_INVALID_REDIST" => DIRECT3D12::D3D12_ERROR_INVALID_REDIST,
        }
    }
    pub fn from_code(code: u32) -> DIRECT3D12 {
        return match code {
            0x887E0001 => DIRECT3D12::D3D12_ERROR_ADAPTER_NOT_FOUND,
            0x887E0002 => DIRECT3D12::D3D12_ERROR_DRIVER_VERSION_MISMATCH,
            0x887E0003 => DIRECT3D12::D3D12_ERROR_INVALID_REDIST,
        }
    }
}
