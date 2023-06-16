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

pub enum LEAP {
    LEAP_E_INVALID_CALL = 0x88880001,
    LEAP_E_INVALID_ARGUMENT = 0x88880002,
    LEAP_E_INVALID_FLAGS = 0x88880003,
    LEAP_E_INVALID_POINTER = 0x88880004,
    LEAP_E_INVALID_INDEX = 0x88880005,
    LEAP_E_INVALID_LEAP_BUFFER = 0x88880006,
    LEAP_E_INVALID_FORMAT = 0x88880007,
    LEAP_E_INVALID_FILTER = 0x88880008,
    LEAP_E_INVALID_QUANTUM = 0x88880009,
    LEAP_E_INVALID_RENDERER = 0x8888000A,
    LEAP_E_INVALID_PACKET = 0x8888000B,
    LEAP_E_INSUFFICIENT_SPACE = 0x8888000E,
    LEAP_E_NOT_INITIALIZED = 0x8888000F,
    LEAP_E_ALREADY_INITIALIZED = 0x88880010,
    LEAP_E_NEED_OUTPUT_FILTER = 0x88880011,
    LEAP_E_GRAPH_RUNNING = 0x88880012,
    LEAP_E_GRAPH_STOPPED = 0x88880013,
    LEAP_E_OBJECT_DESTROYED = 0x88880014,
    LEAP_E_RENDERER_BUSY = 0x88880015,
    LEAP_E_VOICE_QUEUE_FULL = 0x88880016,
    LEAP_E_NO_RENDERERS = 0x88880017,
    LEAP_E_RENDERER_INVALIDATED = 0x88880018,
    LEAP_E_INTERNAL_ERROR = 0x88880EEE,
}

impl LEAP {
    pub fn description(&self) -> &'static str {
        match self {
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
}
