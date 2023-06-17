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
pub enum LEAP {
    LEAP_E_INVALID_CALL,
    LEAP_E_INVALID_ARGUMENT,
    LEAP_E_INVALID_FLAGS,
    LEAP_E_INVALID_POINTER,
    LEAP_E_INVALID_INDEX,
    LEAP_E_INVALID_LEAP_BUFFER,
    LEAP_E_INVALID_FORMAT,
    LEAP_E_INVALID_FILTER,
    LEAP_E_INVALID_QUANTUM,
    LEAP_E_INVALID_RENDERER,
    LEAP_E_INVALID_PACKET,
    LEAP_E_INSUFFICIENT_SPACE,
    LEAP_E_NOT_INITIALIZED,
    LEAP_E_ALREADY_INITIALIZED,
    LEAP_E_NEED_OUTPUT_FILTER,
    LEAP_E_GRAPH_RUNNING,
    LEAP_E_GRAPH_STOPPED,
    LEAP_E_OBJECT_DESTROYED,
    LEAP_E_RENDERER_BUSY,
    LEAP_E_VOICE_QUEUE_FULL,
    LEAP_E_NO_RENDERERS,
    LEAP_E_RENDERER_INVALIDATED,
    LEAP_E_INTERNAL_ERROR,
}

impl LEAP {
    pub fn code(&self) -> u32 {
        return match self {
            LEAP::LEAP_E_INVALID_CALL => 0x88880001 as u32,
            LEAP::LEAP_E_INVALID_ARGUMENT => 0x88880002 as u32,
            LEAP::LEAP_E_INVALID_FLAGS => 0x88880003 as u32,
            LEAP::LEAP_E_INVALID_POINTER => 0x88880004 as u32,
            LEAP::LEAP_E_INVALID_INDEX => 0x88880005 as u32,
            LEAP::LEAP_E_INVALID_LEAP_BUFFER => 0x88880006 as u32,
            LEAP::LEAP_E_INVALID_FORMAT => 0x88880007 as u32,
            LEAP::LEAP_E_INVALID_FILTER => 0x88880008 as u32,
            LEAP::LEAP_E_INVALID_QUANTUM => 0x88880009 as u32,
            LEAP::LEAP_E_INVALID_RENDERER => 0x8888000A as u32,
            LEAP::LEAP_E_INVALID_PACKET => 0x8888000B as u32,
            LEAP::LEAP_E_INSUFFICIENT_SPACE => 0x8888000E as u32,
            LEAP::LEAP_E_NOT_INITIALIZED => 0x8888000F as u32,
            LEAP::LEAP_E_ALREADY_INITIALIZED => 0x88880010 as u32,
            LEAP::LEAP_E_NEED_OUTPUT_FILTER => 0x88880011 as u32,
            LEAP::LEAP_E_GRAPH_RUNNING => 0x88880012 as u32,
            LEAP::LEAP_E_GRAPH_STOPPED => 0x88880013 as u32,
            LEAP::LEAP_E_OBJECT_DESTROYED => 0x88880014 as u32,
            LEAP::LEAP_E_RENDERER_BUSY => 0x88880015 as u32,
            LEAP::LEAP_E_VOICE_QUEUE_FULL => 0x88880016 as u32,
            LEAP::LEAP_E_NO_RENDERERS => 0x88880017 as u32,
            LEAP::LEAP_E_RENDERER_INVALIDATED => 0x88880018 as u32,
            LEAP::LEAP_E_INTERNAL_ERROR => 0x88880EEE as u32,
        }
    }

    pub fn error(&self) -> RawError {
        return match self {
            LEAP::LEAP_E_INVALID_CALL => RawError::Kind(LEAP::LEAP_E_INVALID_CALL),
            LEAP::LEAP_E_INVALID_ARGUMENT => RawError::Kind(LEAP::LEAP_E_INVALID_ARGUMENT),
            LEAP::LEAP_E_INVALID_FLAGS => RawError::Kind(LEAP::LEAP_E_INVALID_FLAGS),
            LEAP::LEAP_E_INVALID_POINTER => RawError::Kind(LEAP::LEAP_E_INVALID_POINTER),
            LEAP::LEAP_E_INVALID_INDEX => RawError::Kind(LEAP::LEAP_E_INVALID_INDEX),
            LEAP::LEAP_E_INVALID_LEAP_BUFFER => RawError::Kind(LEAP::LEAP_E_INVALID_LEAP_BUFFER),
            LEAP::LEAP_E_INVALID_FORMAT => RawError::Kind(LEAP::LEAP_E_INVALID_FORMAT),
            LEAP::LEAP_E_INVALID_FILTER => RawError::Kind(LEAP::LEAP_E_INVALID_FILTER),
            LEAP::LEAP_E_INVALID_QUANTUM => RawError::Kind(LEAP::LEAP_E_INVALID_QUANTUM),
            LEAP::LEAP_E_INVALID_RENDERER => RawError::Kind(LEAP::LEAP_E_INVALID_RENDERER),
            LEAP::LEAP_E_INVALID_PACKET => RawError::Kind(LEAP::LEAP_E_INVALID_PACKET),
            LEAP::LEAP_E_INSUFFICIENT_SPACE => RawError::Kind(LEAP::LEAP_E_INSUFFICIENT_SPACE),
            LEAP::LEAP_E_NOT_INITIALIZED => RawError::Kind(LEAP::LEAP_E_NOT_INITIALIZED),
            LEAP::LEAP_E_ALREADY_INITIALIZED => RawError::Kind(LEAP::LEAP_E_ALREADY_INITIALIZED),
            LEAP::LEAP_E_NEED_OUTPUT_FILTER => RawError::Kind(LEAP::LEAP_E_NEED_OUTPUT_FILTER),
            LEAP::LEAP_E_GRAPH_RUNNING => RawError::Kind(LEAP::LEAP_E_GRAPH_RUNNING),
            LEAP::LEAP_E_GRAPH_STOPPED => RawError::Kind(LEAP::LEAP_E_GRAPH_STOPPED),
            LEAP::LEAP_E_OBJECT_DESTROYED => RawError::Kind(LEAP::LEAP_E_OBJECT_DESTROYED),
            LEAP::LEAP_E_RENDERER_BUSY => RawError::Kind(LEAP::LEAP_E_RENDERER_BUSY),
            LEAP::LEAP_E_VOICE_QUEUE_FULL => RawError::Kind(LEAP::LEAP_E_VOICE_QUEUE_FULL),
            LEAP::LEAP_E_NO_RENDERERS => RawError::Kind(LEAP::LEAP_E_NO_RENDERERS),
            LEAP::LEAP_E_RENDERER_INVALIDATED => RawError::Kind(LEAP::LEAP_E_RENDERER_INVALIDATED),
            LEAP::LEAP_E_INTERNAL_ERROR => RawError::Kind(LEAP::LEAP_E_INTERNAL_ERROR),
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            LEAP::LEAP_E_INVALID_CALL => "A method was called in an invalid way.",
            LEAP::LEAP_E_INVALID_ARGUMENT => "An argument was invalid.",
            LEAP::LEAP_E_INVALID_FLAGS => "An invalid flag was specified.",
            LEAP::LEAP_E_INVALID_POINTER => "A required pointer argument was NULL.",
            LEAP::LEAP_E_INVALID_INDEX => "An index value was out of bounds.",
            LEAP::LEAP_E_INVALID_LEAP_BUFFER => "No LeapBuffer was found with the given identifier.",
            LEAP::LEAP_E_INVALID_FORMAT => "An invalid or unsupported audio format was used.",
            LEAP::LEAP_E_INVALID_FILTER => "An invalid ILeapFilter pointer was used.",
            LEAP::LEAP_E_INVALID_QUANTUM => "The specified processing quantum was invalid or out of bounds.",
            LEAP::LEAP_E_INVALID_RENDERER => "An invalid ILeapRendererConnection value was used.",
            LEAP::LEAP_E_INVALID_PACKET => "An invalid audio packet was submitted to a LEAP voice.",
            LEAP::LEAP_E_INSUFFICIENT_SPACE => "The specified array was too small to return the data requested.",
            LEAP::LEAP_E_NOT_INITIALIZED => "ILeap::Initialize must be called successfully before any other API call.",
            LEAP::LEAP_E_ALREADY_INITIALIZED => "ILeap::Initialize cannot be called twice (if first call succeeded).",
            LEAP::LEAP_E_NEED_OUTPUT_FILTER => "ILeap::StartGraph cannot be called with no output filter in the graph.",
            LEAP::LEAP_E_GRAPH_RUNNING => "Function cannot be called while the LEAP graph is running.",
            LEAP::LEAP_E_GRAPH_STOPPED => "Function cannot be called while the LEAP graph is stopped.",
            LEAP::LEAP_E_OBJECT_DESTROYED => "The object this interface referred to has been destroyed.",
            LEAP::LEAP_E_RENDERER_BUSY => "The renderer is already being used by an output object.",
            LEAP::LEAP_E_VOICE_QUEUE_FULL => "The voice object's packet queue has reached maximum capacity.",
            LEAP::LEAP_E_NO_RENDERERS => "No audio rendering devices are enabled on the system.",
            LEAP::LEAP_E_RENDERER_INVALIDATED => "A rendering device was removed or stopped responding.",
            LEAP::LEAP_E_INTERNAL_ERROR => "An internal error such as memory corruption was detected.",
        }
    }

    pub fn from_name(name: &str) -> LEAP {
        return match name {
            "LEAP_E_INVALID_CALL" => LEAP::LEAP_E_INVALID_CALL,
            "LEAP_E_INVALID_ARGUMENT" => LEAP::LEAP_E_INVALID_ARGUMENT,
            "LEAP_E_INVALID_FLAGS" => LEAP::LEAP_E_INVALID_FLAGS,
            "LEAP_E_INVALID_POINTER" => LEAP::LEAP_E_INVALID_POINTER,
            "LEAP_E_INVALID_INDEX" => LEAP::LEAP_E_INVALID_INDEX,
            "LEAP_E_INVALID_LEAP_BUFFER" => LEAP::LEAP_E_INVALID_LEAP_BUFFER,
            "LEAP_E_INVALID_FORMAT" => LEAP::LEAP_E_INVALID_FORMAT,
            "LEAP_E_INVALID_FILTER" => LEAP::LEAP_E_INVALID_FILTER,
            "LEAP_E_INVALID_QUANTUM" => LEAP::LEAP_E_INVALID_QUANTUM,
            "LEAP_E_INVALID_RENDERER" => LEAP::LEAP_E_INVALID_RENDERER,
            "LEAP_E_INVALID_PACKET" => LEAP::LEAP_E_INVALID_PACKET,
            "LEAP_E_INSUFFICIENT_SPACE" => LEAP::LEAP_E_INSUFFICIENT_SPACE,
            "LEAP_E_NOT_INITIALIZED" => LEAP::LEAP_E_NOT_INITIALIZED,
            "LEAP_E_ALREADY_INITIALIZED" => LEAP::LEAP_E_ALREADY_INITIALIZED,
            "LEAP_E_NEED_OUTPUT_FILTER" => LEAP::LEAP_E_NEED_OUTPUT_FILTER,
            "LEAP_E_GRAPH_RUNNING" => LEAP::LEAP_E_GRAPH_RUNNING,
            "LEAP_E_GRAPH_STOPPED" => LEAP::LEAP_E_GRAPH_STOPPED,
            "LEAP_E_OBJECT_DESTROYED" => LEAP::LEAP_E_OBJECT_DESTROYED,
            "LEAP_E_RENDERER_BUSY" => LEAP::LEAP_E_RENDERER_BUSY,
            "LEAP_E_VOICE_QUEUE_FULL" => LEAP::LEAP_E_VOICE_QUEUE_FULL,
            "LEAP_E_NO_RENDERERS" => LEAP::LEAP_E_NO_RENDERERS,
            "LEAP_E_RENDERER_INVALIDATED" => LEAP::LEAP_E_RENDERER_INVALIDATED,
            "LEAP_E_INTERNAL_ERROR" => LEAP::LEAP_E_INTERNAL_ERROR,
        }
    }
    pub fn from_code(code: u32) -> LEAP {
        return match code {
            0x88880001 => LEAP::LEAP_E_INVALID_CALL,
            0x88880002 => LEAP::LEAP_E_INVALID_ARGUMENT,
            0x88880003 => LEAP::LEAP_E_INVALID_FLAGS,
            0x88880004 => LEAP::LEAP_E_INVALID_POINTER,
            0x88880005 => LEAP::LEAP_E_INVALID_INDEX,
            0x88880006 => LEAP::LEAP_E_INVALID_LEAP_BUFFER,
            0x88880007 => LEAP::LEAP_E_INVALID_FORMAT,
            0x88880008 => LEAP::LEAP_E_INVALID_FILTER,
            0x88880009 => LEAP::LEAP_E_INVALID_QUANTUM,
            0x8888000A => LEAP::LEAP_E_INVALID_RENDERER,
            0x8888000B => LEAP::LEAP_E_INVALID_PACKET,
            0x8888000E => LEAP::LEAP_E_INSUFFICIENT_SPACE,
            0x8888000F => LEAP::LEAP_E_NOT_INITIALIZED,
            0x88880010 => LEAP::LEAP_E_ALREADY_INITIALIZED,
            0x88880011 => LEAP::LEAP_E_NEED_OUTPUT_FILTER,
            0x88880012 => LEAP::LEAP_E_GRAPH_RUNNING,
            0x88880013 => LEAP::LEAP_E_GRAPH_STOPPED,
            0x88880014 => LEAP::LEAP_E_OBJECT_DESTROYED,
            0x88880015 => LEAP::LEAP_E_RENDERER_BUSY,
            0x88880016 => LEAP::LEAP_E_VOICE_QUEUE_FULL,
            0x88880017 => LEAP::LEAP_E_NO_RENDERERS,
            0x88880018 => LEAP::LEAP_E_RENDERER_INVALIDATED,
            0x88880EEE => LEAP::LEAP_E_INTERNAL_ERROR,
        }
    }
}
