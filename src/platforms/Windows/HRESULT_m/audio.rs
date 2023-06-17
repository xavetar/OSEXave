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
pub enum AUDIO {
    E_AUDIO_ENGINE_NODE_NOT_FOUND,
    E_HDAUDIO_EMPTY_CONNECTION_LIST,
    E_HDAUDIO_CONNECTION_LIST_NOT_SUPPORTED,
    E_HDAUDIO_NO_LOGICAL_DEVICES_CREATED,
    E_HDAUDIO_NULL_LINKED_LIST_ENTRY,
}

impl AUDIO {
    pub fn code(&self) -> u32 {
        return match self {
            AUDIO::E_AUDIO_ENGINE_NODE_NOT_FOUND => 0x80660001 as u32,
            AUDIO::E_HDAUDIO_EMPTY_CONNECTION_LIST => 0x80660002 as u32,
            AUDIO::E_HDAUDIO_CONNECTION_LIST_NOT_SUPPORTED => 0x80660003 as u32,
            AUDIO::E_HDAUDIO_NO_LOGICAL_DEVICES_CREATED => 0x80660004 as u32,
            AUDIO::E_HDAUDIO_NULL_LINKED_LIST_ENTRY => 0x80660005 as u32,
        }
    }

    pub fn error(&self) -> RawError {
        return match self {
            AUDIO::E_AUDIO_ENGINE_NODE_NOT_FOUND => RawError::Kind(AUDIO::E_AUDIO_ENGINE_NODE_NOT_FOUND),
            AUDIO::E_HDAUDIO_EMPTY_CONNECTION_LIST => RawError::Kind(AUDIO::E_HDAUDIO_EMPTY_CONNECTION_LIST),
            AUDIO::E_HDAUDIO_CONNECTION_LIST_NOT_SUPPORTED => RawError::Kind(AUDIO::E_HDAUDIO_CONNECTION_LIST_NOT_SUPPORTED),
            AUDIO::E_HDAUDIO_NO_LOGICAL_DEVICES_CREATED => RawError::Kind(AUDIO::E_HDAUDIO_NO_LOGICAL_DEVICES_CREATED),
            AUDIO::E_HDAUDIO_NULL_LINKED_LIST_ENTRY => RawError::Kind(AUDIO::E_HDAUDIO_NULL_LINKED_LIST_ENTRY),
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            AUDIO::E_AUDIO_ENGINE_NODE_NOT_FOUND => "PortCls could not find an audio engine node exposed by a miniport driver claiming support for IMiniportAudioEngineNode.",
            AUDIO::E_HDAUDIO_EMPTY_CONNECTION_LIST => "HD Audio widget encountered an unexpected empty connection list.",
            AUDIO::E_HDAUDIO_CONNECTION_LIST_NOT_SUPPORTED => "HD Audio widget does not support the connection list parameter.",
            AUDIO::E_HDAUDIO_NO_LOGICAL_DEVICES_CREATED => "No HD Audio subdevices were successfully created.",
            AUDIO::E_HDAUDIO_NULL_LINKED_LIST_ENTRY => "An unexpected NULL pointer was encountered in a linked list.",
        }
    }

    pub fn from_name(name: &str) -> AUDIO {
        return match name {
            "E_AUDIO_ENGINE_NODE_NOT_FOUND" => AUDIO::E_AUDIO_ENGINE_NODE_NOT_FOUND,
            "E_HDAUDIO_EMPTY_CONNECTION_LIST" => AUDIO::E_HDAUDIO_EMPTY_CONNECTION_LIST,
            "E_HDAUDIO_CONNECTION_LIST_NOT_SUPPORTED" => AUDIO::E_HDAUDIO_CONNECTION_LIST_NOT_SUPPORTED,
            "E_HDAUDIO_NO_LOGICAL_DEVICES_CREATED" => AUDIO::E_HDAUDIO_NO_LOGICAL_DEVICES_CREATED,
            "E_HDAUDIO_NULL_LINKED_LIST_ENTRY" => AUDIO::E_HDAUDIO_NULL_LINKED_LIST_ENTRY,
        }
    }
    pub fn from_code(code: u32) -> AUDIO {
        return match code {
            0x80660001 => AUDIO::E_AUDIO_ENGINE_NODE_NOT_FOUND,
            0x80660002 => AUDIO::E_HDAUDIO_EMPTY_CONNECTION_LIST,
            0x80660003 => AUDIO::E_HDAUDIO_CONNECTION_LIST_NOT_SUPPORTED,
            0x80660004 => AUDIO::E_HDAUDIO_NO_LOGICAL_DEVICES_CREATED,
            0x80660005 => AUDIO::E_HDAUDIO_NULL_LINKED_LIST_ENTRY,
        }
    }
}
