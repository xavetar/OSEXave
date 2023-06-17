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
pub enum XAUDIO2 {
    XAUDIO2_E_INVALID_CALL,
    XAUDIO2_E_XMA_DECODER_ERROR,
    XAUDIO2_E_XAPO_CREATION_FAILED,
    XAUDIO2_E_DEVICE_INVALIDATED,
}

impl XAUDIO2 {
    pub fn code(&self) -> u32 {
        return match self {
            XAUDIO2::XAUDIO2_E_INVALID_CALL => 0x88960001 as u32,
            XAUDIO2::XAUDIO2_E_XMA_DECODER_ERROR => 0x88960002 as u32,
            XAUDIO2::XAUDIO2_E_XAPO_CREATION_FAILED => 0x88960003 as u32,
            XAUDIO2::XAUDIO2_E_DEVICE_INVALIDATED => 0x88960004 as u32,
        }
    }

    pub fn error(&self) -> RawError {
        return match self {
            XAUDIO2::XAUDIO2_E_INVALID_CALL => RawError::Kind(XAUDIO2::XAUDIO2_E_INVALID_CALL),
            XAUDIO2::XAUDIO2_E_XMA_DECODER_ERROR => RawError::Kind(XAUDIO2::XAUDIO2_E_XMA_DECODER_ERROR),
            XAUDIO2::XAUDIO2_E_XAPO_CREATION_FAILED => RawError::Kind(XAUDIO2::XAUDIO2_E_XAPO_CREATION_FAILED),
            XAUDIO2::XAUDIO2_E_DEVICE_INVALIDATED => RawError::Kind(XAUDIO2::XAUDIO2_E_DEVICE_INVALIDATED),
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            XAUDIO2::XAUDIO2_E_INVALID_CALL => "An API call or one of its arguments was illegal.",
            XAUDIO2::XAUDIO2_E_XMA_DECODER_ERROR => "The XMA hardware suffered an unrecoverable error.",
            XAUDIO2::XAUDIO2_E_XAPO_CREATION_FAILED => "Failed to instantiate an effect.",
            XAUDIO2::XAUDIO2_E_DEVICE_INVALIDATED => "An audio device became unusable (unplugged, etc).",
        }
    }

    pub fn from_name(name: &str) -> XAUDIO2 {
        return match name {
            "XAUDIO2_E_INVALID_CALL" => XAUDIO2::XAUDIO2_E_INVALID_CALL,
            "XAUDIO2_E_XMA_DECODER_ERROR" => XAUDIO2::XAUDIO2_E_XMA_DECODER_ERROR,
            "XAUDIO2_E_XAPO_CREATION_FAILED" => XAUDIO2::XAUDIO2_E_XAPO_CREATION_FAILED,
            "XAUDIO2_E_DEVICE_INVALIDATED" => XAUDIO2::XAUDIO2_E_DEVICE_INVALIDATED,
        }
    }
    pub fn from_code(code: u32) -> XAUDIO2 {
        return match code {
            0x88960001 => XAUDIO2::XAUDIO2_E_INVALID_CALL,
            0x88960002 => XAUDIO2::XAUDIO2_E_XMA_DECODER_ERROR,
            0x88960003 => XAUDIO2::XAUDIO2_E_XAPO_CREATION_FAILED,
            0x88960004 => XAUDIO2::XAUDIO2_E_DEVICE_INVALIDATED,
        }
    }
}
