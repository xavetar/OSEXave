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
pub enum DIRECT3D10 {
    D3D10_ERROR_TOO_MANY_UNIQUE_STATE_OBJECTS,
    D3D10_ERROR_FILE_NOT_FOUND,
}

impl DIRECT3D10 {
    pub fn code(&self) -> u32 {
        return match self {
            DIRECT3D10::D3D10_ERROR_TOO_MANY_UNIQUE_STATE_OBJECTS => 0x88790001 as u32,
            DIRECT3D10::D3D10_ERROR_FILE_NOT_FOUND => 0x88790002 as u32,
        }
    }

    pub fn error(&self) -> RawError {
        return match self {
            DIRECT3D10::D3D10_ERROR_TOO_MANY_UNIQUE_STATE_OBJECTS => RawError::Kind(DIRECT3D10::D3D10_ERROR_TOO_MANY_UNIQUE_STATE_OBJECTS),
            DIRECT3D10::D3D10_ERROR_FILE_NOT_FOUND => RawError::Kind(DIRECT3D10::D3D10_ERROR_FILE_NOT_FOUND),
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            DIRECT3D10::D3D10_ERROR_TOO_MANY_UNIQUE_STATE_OBJECTS => "The application has exceeded the maximum number of unique state objects per Direct3D device.",
            DIRECT3D10::D3D10_ERROR_FILE_NOT_FOUND => "The specified file was not found.",
        }
    }

    pub fn from_name(name: &str) -> DIRECT3D10 {
        return match name {
            "D3D10_ERROR_TOO_MANY_UNIQUE_STATE_OBJECTS" => DIRECT3D10::D3D10_ERROR_TOO_MANY_UNIQUE_STATE_OBJECTS,
            "D3D10_ERROR_FILE_NOT_FOUND" => DIRECT3D10::D3D10_ERROR_FILE_NOT_FOUND,
        }
    }
    pub fn from_code(code: u32) -> DIRECT3D10 {
        return match code {
            0x88790001 => DIRECT3D10::D3D10_ERROR_TOO_MANY_UNIQUE_STATE_OBJECTS,
            0x88790002 => DIRECT3D10::D3D10_ERROR_FILE_NOT_FOUND,
        }
    }
}
