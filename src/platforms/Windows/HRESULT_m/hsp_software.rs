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
pub enum HSP_SOFTWARE {
    HSP_DRV_ERROR_MASK,
    HSP_DRV_INTERNAL_ERROR,
    HSP_BASE_ERROR_MASK,
    HSP_BASE_INTERNAL_ERROR,
    HSP_KSP_ERROR_MASK,
    HSP_KSP_DEVICE_NOT_READY,
    HSP_KSP_INVALID_PROVIDER_HANDLE,
    HSP_KSP_INVALID_KEY_HANDLE,
    HSP_KSP_INVALID_PARAMETER,
    HSP_KSP_BUFFER_TOO_SMALL,
    HSP_KSP_NOT_SUPPORTED,
    HSP_KSP_INVALID_DATA,
    HSP_KSP_INVALID_FLAGS,
    HSP_KSP_ALGORITHM_NOT_SUPPORTED,
    HSP_KSP_KEY_ALREADY_FINALIZED,
    HSP_KSP_KEY_NOT_FINALIZED,
    HSP_KSP_INVALID_KEY_TYPE,
    HSP_KSP_NO_MEMORY,
    HSP_KSP_PARAMETER_NOT_SET,
    HSP_KSP_KEY_EXISTS,
    HSP_KSP_KEY_MISSING,
    HSP_KSP_KEY_LOAD_FAIL,
    HSP_KSP_NO_MORE_ITEMS,
    HSP_KSP_INTERNAL_ERROR,
}

impl HSP_SOFTWARE {
    pub fn code(&self) -> u32 {
        return match self {
            HSP_SOFTWARE::HSP_DRV_ERROR_MASK => 0x81290000 as u32,
            HSP_SOFTWARE::HSP_DRV_INTERNAL_ERROR => 0x812900FF as u32,
            HSP_SOFTWARE::HSP_BASE_ERROR_MASK => 0x81290100 as u32,
            HSP_SOFTWARE::HSP_BASE_INTERNAL_ERROR => 0x812901FF as u32,
            HSP_SOFTWARE::HSP_KSP_ERROR_MASK => 0x81290200 as u32,
            HSP_SOFTWARE::HSP_KSP_DEVICE_NOT_READY => 0x81290201 as u32,
            HSP_SOFTWARE::HSP_KSP_INVALID_PROVIDER_HANDLE => 0x81290202 as u32,
            HSP_SOFTWARE::HSP_KSP_INVALID_KEY_HANDLE => 0x81290203 as u32,
            HSP_SOFTWARE::HSP_KSP_INVALID_PARAMETER => 0x81290204 as u32,
            HSP_SOFTWARE::HSP_KSP_BUFFER_TOO_SMALL => 0x81290205 as u32,
            HSP_SOFTWARE::HSP_KSP_NOT_SUPPORTED => 0x81290206 as u32,
            HSP_SOFTWARE::HSP_KSP_INVALID_DATA => 0x81290207 as u32,
            HSP_SOFTWARE::HSP_KSP_INVALID_FLAGS => 0x81290208 as u32,
            HSP_SOFTWARE::HSP_KSP_ALGORITHM_NOT_SUPPORTED => 0x81290209 as u32,
            HSP_SOFTWARE::HSP_KSP_KEY_ALREADY_FINALIZED => 0x8129020A as u32,
            HSP_SOFTWARE::HSP_KSP_KEY_NOT_FINALIZED => 0x8129020B as u32,
            HSP_SOFTWARE::HSP_KSP_INVALID_KEY_TYPE => 0x8129020C as u32,
            HSP_SOFTWARE::HSP_KSP_NO_MEMORY => 0x81290210 as u32,
            HSP_SOFTWARE::HSP_KSP_PARAMETER_NOT_SET => 0x81290211 as u32,
            HSP_SOFTWARE::HSP_KSP_KEY_EXISTS => 0x81290215 as u32,
            HSP_SOFTWARE::HSP_KSP_KEY_MISSING => 0x81290216 as u32,
            HSP_SOFTWARE::HSP_KSP_KEY_LOAD_FAIL => 0x81290217 as u32,
            HSP_SOFTWARE::HSP_KSP_NO_MORE_ITEMS => 0x81290218 as u32,
            HSP_SOFTWARE::HSP_KSP_INTERNAL_ERROR => 0x812902FF as u32,
        }
    }

    pub fn error(&self) -> RawError {
        return match self {
            HSP_SOFTWARE::HSP_DRV_ERROR_MASK => RawError::Kind(HSP_SOFTWARE::HSP_DRV_ERROR_MASK),
            HSP_SOFTWARE::HSP_DRV_INTERNAL_ERROR => RawError::Kind(HSP_SOFTWARE::HSP_DRV_INTERNAL_ERROR),
            HSP_SOFTWARE::HSP_BASE_ERROR_MASK => RawError::Kind(HSP_SOFTWARE::HSP_BASE_ERROR_MASK),
            HSP_SOFTWARE::HSP_BASE_INTERNAL_ERROR => RawError::Kind(HSP_SOFTWARE::HSP_BASE_INTERNAL_ERROR),
            HSP_SOFTWARE::HSP_KSP_ERROR_MASK => RawError::Kind(HSP_SOFTWARE::HSP_KSP_ERROR_MASK),
            HSP_SOFTWARE::HSP_KSP_DEVICE_NOT_READY => RawError::Kind(HSP_SOFTWARE::HSP_KSP_DEVICE_NOT_READY),
            HSP_SOFTWARE::HSP_KSP_INVALID_PROVIDER_HANDLE => RawError::Kind(HSP_SOFTWARE::HSP_KSP_INVALID_PROVIDER_HANDLE),
            HSP_SOFTWARE::HSP_KSP_INVALID_KEY_HANDLE => RawError::Kind(HSP_SOFTWARE::HSP_KSP_INVALID_KEY_HANDLE),
            HSP_SOFTWARE::HSP_KSP_INVALID_PARAMETER => RawError::Kind(HSP_SOFTWARE::HSP_KSP_INVALID_PARAMETER),
            HSP_SOFTWARE::HSP_KSP_BUFFER_TOO_SMALL => RawError::Kind(HSP_SOFTWARE::HSP_KSP_BUFFER_TOO_SMALL),
            HSP_SOFTWARE::HSP_KSP_NOT_SUPPORTED => RawError::Kind(HSP_SOFTWARE::HSP_KSP_NOT_SUPPORTED),
            HSP_SOFTWARE::HSP_KSP_INVALID_DATA => RawError::Kind(HSP_SOFTWARE::HSP_KSP_INVALID_DATA),
            HSP_SOFTWARE::HSP_KSP_INVALID_FLAGS => RawError::Kind(HSP_SOFTWARE::HSP_KSP_INVALID_FLAGS),
            HSP_SOFTWARE::HSP_KSP_ALGORITHM_NOT_SUPPORTED => RawError::Kind(HSP_SOFTWARE::HSP_KSP_ALGORITHM_NOT_SUPPORTED),
            HSP_SOFTWARE::HSP_KSP_KEY_ALREADY_FINALIZED => RawError::Kind(HSP_SOFTWARE::HSP_KSP_KEY_ALREADY_FINALIZED),
            HSP_SOFTWARE::HSP_KSP_KEY_NOT_FINALIZED => RawError::Kind(HSP_SOFTWARE::HSP_KSP_KEY_NOT_FINALIZED),
            HSP_SOFTWARE::HSP_KSP_INVALID_KEY_TYPE => RawError::Kind(HSP_SOFTWARE::HSP_KSP_INVALID_KEY_TYPE),
            HSP_SOFTWARE::HSP_KSP_NO_MEMORY => RawError::Kind(HSP_SOFTWARE::HSP_KSP_NO_MEMORY),
            HSP_SOFTWARE::HSP_KSP_PARAMETER_NOT_SET => RawError::Kind(HSP_SOFTWARE::HSP_KSP_PARAMETER_NOT_SET),
            HSP_SOFTWARE::HSP_KSP_KEY_EXISTS => RawError::Kind(HSP_SOFTWARE::HSP_KSP_KEY_EXISTS),
            HSP_SOFTWARE::HSP_KSP_KEY_MISSING => RawError::Kind(HSP_SOFTWARE::HSP_KSP_KEY_MISSING),
            HSP_SOFTWARE::HSP_KSP_KEY_LOAD_FAIL => RawError::Kind(HSP_SOFTWARE::HSP_KSP_KEY_LOAD_FAIL),
            HSP_SOFTWARE::HSP_KSP_NO_MORE_ITEMS => RawError::Kind(HSP_SOFTWARE::HSP_KSP_NO_MORE_ITEMS),
            HSP_SOFTWARE::HSP_KSP_INTERNAL_ERROR => RawError::Kind(HSP_SOFTWARE::HSP_KSP_INTERNAL_ERROR),
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            HSP_SOFTWARE::HSP_DRV_ERROR_MASK => "This is an error mask to convert HSP driver errors to Win errors.",
            HSP_SOFTWARE::HSP_DRV_INTERNAL_ERROR => "Catastrophic internal failure in the HSP driver.",
            HSP_SOFTWARE::HSP_BASE_ERROR_MASK => "This is an error mask to convert HSP base class errors to Win errors.",
            HSP_SOFTWARE::HSP_BASE_INTERNAL_ERROR => "Catastrophic internal failure in the HSP base class.",
            HSP_SOFTWARE::HSP_KSP_ERROR_MASK => "This is an error mask to convert HSP KSP errors to Win errors.",
            HSP_SOFTWARE::HSP_KSP_DEVICE_NOT_READY => "The Pluton processor is currently not ready for use.",
            HSP_SOFTWARE::HSP_KSP_INVALID_PROVIDER_HANDLE => "The handle to the HSP KSP is invalid.",
            HSP_SOFTWARE::HSP_KSP_INVALID_KEY_HANDLE => "The handle to a key stored by the HSP KSP is invalid.",
            HSP_SOFTWARE::HSP_KSP_INVALID_PARAMETER => "A parameter to the HSP KSP was invalid.",
            HSP_SOFTWARE::HSP_KSP_BUFFER_TOO_SMALL => "The supplied buffer is too small.",
            HSP_SOFTWARE::HSP_KSP_NOT_SUPPORTED => "The requested operation is not supported.",
            HSP_SOFTWARE::HSP_KSP_INVALID_DATA => "The provided data is invalid.",
            HSP_SOFTWARE::HSP_KSP_INVALID_FLAGS => "The provided flags are invalid.",
            HSP_SOFTWARE::HSP_KSP_ALGORITHM_NOT_SUPPORTED => "The algorithm identifier is not supported.",
            HSP_SOFTWARE::HSP_KSP_KEY_ALREADY_FINALIZED => "The key has already been finalized.",
            HSP_SOFTWARE::HSP_KSP_KEY_NOT_FINALIZED => "The key has not been finalized.",
            HSP_SOFTWARE::HSP_KSP_INVALID_KEY_TYPE => "The key does not support the requested operation.",
            HSP_SOFTWARE::HSP_KSP_NO_MEMORY => "There is not enough memory for the operation.",
            HSP_SOFTWARE::HSP_KSP_PARAMETER_NOT_SET => "The parameter has not been set and has no default value.",
            HSP_SOFTWARE::HSP_KSP_KEY_EXISTS => "Key object already exists.",
            HSP_SOFTWARE::HSP_KSP_KEY_MISSING => "The requsted key object does not exist.",
            HSP_SOFTWARE::HSP_KSP_KEY_LOAD_FAIL => "Failed to load the requested key.",
            HSP_SOFTWARE::HSP_KSP_NO_MORE_ITEMS => "No more data is available.",
            HSP_SOFTWARE::HSP_KSP_INTERNAL_ERROR => "Catastrophic internal failure in the HSP KSP.",
        }
    }

    pub fn from_name(name: &str) -> HSP_SOFTWARE {
        return match name {
            "HSP_DRV_ERROR_MASK" => HSP_SOFTWARE::HSP_DRV_ERROR_MASK,
            "HSP_DRV_INTERNAL_ERROR" => HSP_SOFTWARE::HSP_DRV_INTERNAL_ERROR,
            "HSP_BASE_ERROR_MASK" => HSP_SOFTWARE::HSP_BASE_ERROR_MASK,
            "HSP_BASE_INTERNAL_ERROR" => HSP_SOFTWARE::HSP_BASE_INTERNAL_ERROR,
            "HSP_KSP_ERROR_MASK" => HSP_SOFTWARE::HSP_KSP_ERROR_MASK,
            "HSP_KSP_DEVICE_NOT_READY" => HSP_SOFTWARE::HSP_KSP_DEVICE_NOT_READY,
            "HSP_KSP_INVALID_PROVIDER_HANDLE" => HSP_SOFTWARE::HSP_KSP_INVALID_PROVIDER_HANDLE,
            "HSP_KSP_INVALID_KEY_HANDLE" => HSP_SOFTWARE::HSP_KSP_INVALID_KEY_HANDLE,
            "HSP_KSP_INVALID_PARAMETER" => HSP_SOFTWARE::HSP_KSP_INVALID_PARAMETER,
            "HSP_KSP_BUFFER_TOO_SMALL" => HSP_SOFTWARE::HSP_KSP_BUFFER_TOO_SMALL,
            "HSP_KSP_NOT_SUPPORTED" => HSP_SOFTWARE::HSP_KSP_NOT_SUPPORTED,
            "HSP_KSP_INVALID_DATA" => HSP_SOFTWARE::HSP_KSP_INVALID_DATA,
            "HSP_KSP_INVALID_FLAGS" => HSP_SOFTWARE::HSP_KSP_INVALID_FLAGS,
            "HSP_KSP_ALGORITHM_NOT_SUPPORTED" => HSP_SOFTWARE::HSP_KSP_ALGORITHM_NOT_SUPPORTED,
            "HSP_KSP_KEY_ALREADY_FINALIZED" => HSP_SOFTWARE::HSP_KSP_KEY_ALREADY_FINALIZED,
            "HSP_KSP_KEY_NOT_FINALIZED" => HSP_SOFTWARE::HSP_KSP_KEY_NOT_FINALIZED,
            "HSP_KSP_INVALID_KEY_TYPE" => HSP_SOFTWARE::HSP_KSP_INVALID_KEY_TYPE,
            "HSP_KSP_NO_MEMORY" => HSP_SOFTWARE::HSP_KSP_NO_MEMORY,
            "HSP_KSP_PARAMETER_NOT_SET" => HSP_SOFTWARE::HSP_KSP_PARAMETER_NOT_SET,
            "HSP_KSP_KEY_EXISTS" => HSP_SOFTWARE::HSP_KSP_KEY_EXISTS,
            "HSP_KSP_KEY_MISSING" => HSP_SOFTWARE::HSP_KSP_KEY_MISSING,
            "HSP_KSP_KEY_LOAD_FAIL" => HSP_SOFTWARE::HSP_KSP_KEY_LOAD_FAIL,
            "HSP_KSP_NO_MORE_ITEMS" => HSP_SOFTWARE::HSP_KSP_NO_MORE_ITEMS,
            "HSP_KSP_INTERNAL_ERROR" => HSP_SOFTWARE::HSP_KSP_INTERNAL_ERROR,
        }
    }
    pub fn from_code(code: u32) -> HSP_SOFTWARE {
        return match code {
            0x81290000 => HSP_SOFTWARE::HSP_DRV_ERROR_MASK,
            0x812900FF => HSP_SOFTWARE::HSP_DRV_INTERNAL_ERROR,
            0x81290100 => HSP_SOFTWARE::HSP_BASE_ERROR_MASK,
            0x812901FF => HSP_SOFTWARE::HSP_BASE_INTERNAL_ERROR,
            0x81290200 => HSP_SOFTWARE::HSP_KSP_ERROR_MASK,
            0x81290201 => HSP_SOFTWARE::HSP_KSP_DEVICE_NOT_READY,
            0x81290202 => HSP_SOFTWARE::HSP_KSP_INVALID_PROVIDER_HANDLE,
            0x81290203 => HSP_SOFTWARE::HSP_KSP_INVALID_KEY_HANDLE,
            0x81290204 => HSP_SOFTWARE::HSP_KSP_INVALID_PARAMETER,
            0x81290205 => HSP_SOFTWARE::HSP_KSP_BUFFER_TOO_SMALL,
            0x81290206 => HSP_SOFTWARE::HSP_KSP_NOT_SUPPORTED,
            0x81290207 => HSP_SOFTWARE::HSP_KSP_INVALID_DATA,
            0x81290208 => HSP_SOFTWARE::HSP_KSP_INVALID_FLAGS,
            0x81290209 => HSP_SOFTWARE::HSP_KSP_ALGORITHM_NOT_SUPPORTED,
            0x8129020A => HSP_SOFTWARE::HSP_KSP_KEY_ALREADY_FINALIZED,
            0x8129020B => HSP_SOFTWARE::HSP_KSP_KEY_NOT_FINALIZED,
            0x8129020C => HSP_SOFTWARE::HSP_KSP_INVALID_KEY_TYPE,
            0x81290210 => HSP_SOFTWARE::HSP_KSP_NO_MEMORY,
            0x81290211 => HSP_SOFTWARE::HSP_KSP_PARAMETER_NOT_SET,
            0x81290215 => HSP_SOFTWARE::HSP_KSP_KEY_EXISTS,
            0x81290216 => HSP_SOFTWARE::HSP_KSP_KEY_MISSING,
            0x81290217 => HSP_SOFTWARE::HSP_KSP_KEY_LOAD_FAIL,
            0x81290218 => HSP_SOFTWARE::HSP_KSP_NO_MORE_ITEMS,
            0x812902FF => HSP_SOFTWARE::HSP_KSP_INTERNAL_ERROR,
        }
    }
}
