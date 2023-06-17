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
pub enum INPUT {
    INPUT_E_OUT_OF_ORDER,
    INPUT_E_REENTRANCY,
    INPUT_E_MULTIMODAL,
    INPUT_E_PACKET,
    INPUT_E_FRAME,
    INPUT_E_HISTORY,
    INPUT_E_DEVICE_INFO,
    INPUT_E_TRANSFORM,
    INPUT_E_DEVICE_PROPERTY,
}

impl INPUT {
    pub fn code(&self) -> u32 {
        return match self {
            INPUT::INPUT_E_OUT_OF_ORDER => 0x80400000 as u32,
            INPUT::INPUT_E_REENTRANCY => 0x80400001 as u32,
            INPUT::INPUT_E_MULTIMODAL => 0x80400002 as u32,
            INPUT::INPUT_E_PACKET => 0x80400003 as u32,
            INPUT::INPUT_E_FRAME => 0x80400004 as u32,
            INPUT::INPUT_E_HISTORY => 0x80400005 as u32,
            INPUT::INPUT_E_DEVICE_INFO => 0x80400006 as u32,
            INPUT::INPUT_E_TRANSFORM => 0x80400007 as u32,
            INPUT::INPUT_E_DEVICE_PROPERTY => 0x80400008 as u32,
        }
    }

    pub fn error(&self) -> RawError {
        return match self {
            INPUT::INPUT_E_OUT_OF_ORDER => RawError::Kind(INPUT::INPUT_E_OUT_OF_ORDER),
            INPUT::INPUT_E_REENTRANCY => RawError::Kind(INPUT::INPUT_E_REENTRANCY),
            INPUT::INPUT_E_MULTIMODAL => RawError::Kind(INPUT::INPUT_E_MULTIMODAL),
            INPUT::INPUT_E_PACKET => RawError::Kind(INPUT::INPUT_E_PACKET),
            INPUT::INPUT_E_FRAME => RawError::Kind(INPUT::INPUT_E_FRAME),
            INPUT::INPUT_E_HISTORY => RawError::Kind(INPUT::INPUT_E_HISTORY),
            INPUT::INPUT_E_DEVICE_INFO => RawError::Kind(INPUT::INPUT_E_DEVICE_INFO),
            INPUT::INPUT_E_TRANSFORM => RawError::Kind(INPUT::INPUT_E_TRANSFORM),
            INPUT::INPUT_E_DEVICE_PROPERTY => RawError::Kind(INPUT::INPUT_E_DEVICE_PROPERTY),
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            INPUT::INPUT_E_OUT_OF_ORDER => "Input data cannot be processed in the non-chronological order.",
            INPUT::INPUT_E_REENTRANCY => "Requested operation cannot be performed inside the callback or event handler.",
            INPUT::INPUT_E_MULTIMODAL => "Input cannot be processed because there is ongoing interaction with another pointer type.",
            INPUT::INPUT_E_PACKET => "One or more fields in the input packet are invalid.",
            INPUT::INPUT_E_FRAME => "Packets in the frame are inconsistent. Either pointer ids are not unique or there is a discrepancy in timestamps, frame ids, pointer types or source devices.",
            INPUT::INPUT_E_HISTORY => "The history of frames is inconsistent. Pointer ids, types, source devices don't match, or frame ids are not unique, or timestamps are out of order.",
            INPUT::INPUT_E_DEVICE_INFO => "Failed to retrieve information about the input device.",
            INPUT::INPUT_E_TRANSFORM => "Coordinate system transformation failed to transform the data.",
            INPUT::INPUT_E_DEVICE_PROPERTY => "The property is not supported or not reported correctly by the input device.",
        }
    }

    pub fn from_name(name: &str) -> INPUT {
        return match name {
            "INPUT_E_OUT_OF_ORDER" => INPUT::INPUT_E_OUT_OF_ORDER,
            "INPUT_E_REENTRANCY" => INPUT::INPUT_E_REENTRANCY,
            "INPUT_E_MULTIMODAL" => INPUT::INPUT_E_MULTIMODAL,
            "INPUT_E_PACKET" => INPUT::INPUT_E_PACKET,
            "INPUT_E_FRAME" => INPUT::INPUT_E_FRAME,
            "INPUT_E_HISTORY" => INPUT::INPUT_E_HISTORY,
            "INPUT_E_DEVICE_INFO" => INPUT::INPUT_E_DEVICE_INFO,
            "INPUT_E_TRANSFORM" => INPUT::INPUT_E_TRANSFORM,
            "INPUT_E_DEVICE_PROPERTY" => INPUT::INPUT_E_DEVICE_PROPERTY,
        }
    }
    pub fn from_code(code: u32) -> INPUT {
        return match code {
            0x80400000 => INPUT::INPUT_E_OUT_OF_ORDER,
            0x80400001 => INPUT::INPUT_E_REENTRANCY,
            0x80400002 => INPUT::INPUT_E_MULTIMODAL,
            0x80400003 => INPUT::INPUT_E_PACKET,
            0x80400004 => INPUT::INPUT_E_FRAME,
            0x80400005 => INPUT::INPUT_E_HISTORY,
            0x80400006 => INPUT::INPUT_E_DEVICE_INFO,
            0x80400007 => INPUT::INPUT_E_TRANSFORM,
            0x80400008 => INPUT::INPUT_E_DEVICE_PROPERTY,
        }
    }
}
