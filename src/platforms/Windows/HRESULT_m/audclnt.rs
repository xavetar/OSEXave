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

pub enum AUDCLNT {
    AUDCLNT_E_NOT_INITIALIZED = 0x88890001,
    AUDCLNT_E_ALREADY_INITIALIZED = 0x88890002,
    AUDCLNT_E_WRONG_ENDPOINT_TYPE = 0x88890003,
    AUDCLNT_E_DEVICE_INVALIDATED = 0x88890004,
    AUDCLNT_E_NOT_STOPPED = 0x88890005,
    AUDCLNT_E_BUFFER_TOO_LARGE = 0x88890006,
    AUDCLNT_E_OUT_OF_ORDER = 0x88890007,
    AUDCLNT_E_UNSUPPORTED_FORMAT = 0x88890008,
    AUDCLNT_E_INVALID_SIZE = 0x88890009,
    AUDCLNT_E_DEVICE_IN_USE = 0x8889000A,
    AUDCLNT_E_BUFFER_OPERATION_PENDING = 0x8889000B,
    AUDCLNT_E_THREAD_NOT_REGISTERED = 0x8889000C,
    AUDCLNT_E_NO_SINGLE_PROCESS = 0x8889000D,
    AUDCLNT_E_EXCLUSIVE_MODE_NOT_ALLOWED = 0x8889000E,
    AUDCLNT_E_ENDPOINT_CREATE_FAILED = 0x8889000F,
    AUDCLNT_E_SERVICE_NOT_RUNNING = 0x88890010,
    AUDCLNT_E_EVENTHANDLE_NOT_EXPECTED = 0x88890011,
    AUDCLNT_E_EXCLUSIVE_MODE_ONLY = 0x88890012,
    AUDCLNT_E_BUFDURATION_PERIOD_NOT_EQUAL = 0x88890013,
    AUDCLNT_E_EVENTHANDLE_NOT_SET = 0x88890014,
    AUDCLNT_E_INCORRECT_BUFFER_SIZE = 0x88890015,
    AUDCLNT_E_BUFFER_SIZE_ERROR = 0x88890016,
    AUDCLNT_E_CPUUSAGE_EXCEEDED = 0x88890017,
    AUDCLNT_E_BUFFER_ERROR = 0x88890018,
    AUDCLNT_E_BUFFER_SIZE_NOT_ALIGNED = 0x88890019,
}

impl AUDCLNT {
    pub fn description(&self) -> &'static str {
        match self {
            AUDCLNT::AUDCLNT_E_NOT_INITIALIZED => "The IAudioClient object is not initialized.",
            AUDCLNT::AUDCLNT_E_ALREADY_INITIALIZED => "The IAudioClient object is already initialized.",
            AUDCLNT::AUDCLNT_E_WRONG_ENDPOINT_TYPE => "The AUDCLNT_STREAMFLAGS_LOOPBACK flag is set but the endpoint device is a capture device, not a rendering device.",
            AUDCLNT::AUDCLNT_E_DEVICE_INVALIDATED => "The audio endpoint device has been unplugged, or the audio hardware or associated hardware resources have been reconfigured, disabled, removed, or otherwise made unavailable for use.",
            AUDCLNT::AUDCLNT_E_NOT_STOPPED => "The audio stream was not stopped at the time of the Start call.",
            AUDCLNT::AUDCLNT_E_BUFFER_TOO_LARGE => "The NumFramesRequested value exceeds the available buffer space (buffer size minus padding size).",
            AUDCLNT::AUDCLNT_E_OUT_OF_ORDER => "A previous IAudioRenderClient::GetBuffer call is still in effect.",
            AUDCLNT::AUDCLNT_E_UNSUPPORTED_FORMAT => "The audio engine (shared mode) or audio endpoint device (exclusive mode) does not support the specified format.",
            AUDCLNT::AUDCLNT_E_INVALID_SIZE => "The NumFramesWritten value exceeds the NumFramesRequested value specified in the previous IAudioRenderClient::GetBuffer call",
            AUDCLNT::AUDCLNT_E_DEVICE_IN_USE => "The endpoint device is already in use. Either the device is being used in exclusive mode, or the device is being used in shared mode and the caller asked to use the device in exclusive mode.",
            AUDCLNT::AUDCLNT_E_BUFFER_OPERATION_PENDING => "Buffer cannot be accessed because a stream reset is in progress.",
            AUDCLNT::AUDCLNT_E_THREAD_NOT_REGISTERED => "The thread is not registered.",
            AUDCLNT::AUDCLNT_E_NO_SINGLE_PROCESS => "Indicates that the session spans more than one process.",
            AUDCLNT::AUDCLNT_E_EXCLUSIVE_MODE_NOT_ALLOWED => "The caller is requesting exclusive-mode use of the endpoint device, but the user has disabled exclusive-mode use of the device.",
            AUDCLNT::AUDCLNT_E_ENDPOINT_CREATE_FAILED => "The method failed to create the audio endpoint for the render or the capture device. This can occur if the audio endpoint device has been unplugged, or the audio hardware or associated hardware resources have been reconfigured, disabled, removed, or otherwise made unavailable for use.",
            AUDCLNT::AUDCLNT_E_SERVICE_NOT_RUNNING => "The Windows audio service is not running.",
            AUDCLNT::AUDCLNT_E_EVENTHANDLE_NOT_EXPECTED => "The audio stream was not initialized for event-driven buffering.",
            AUDCLNT::AUDCLNT_E_EXCLUSIVE_MODE_ONLY => "Exclusive mode only.",
            AUDCLNT::AUDCLNT_E_BUFDURATION_PERIOD_NOT_EQUAL => "The AUDCLNT_STREAMFLAGS_EVENTCALLBACK flag is set but parameters hnsBufferDuration and hnsPeriodicity are not equal.",
            AUDCLNT::AUDCLNT_E_EVENTHANDLE_NOT_SET => "The audio stream is configured to use event-driven buffering, but the caller has not called IAudioClient::SetEventHandle to set the event handle on the stream.",
            AUDCLNT::AUDCLNT_E_INCORRECT_BUFFER_SIZE => "Indicates that the buffer has an incorrect size.",
            AUDCLNT::AUDCLNT_E_BUFFER_SIZE_ERROR => "Indicates that the buffer duration value requested by an exclusive-mode client is out of range. The requested duration value for pull mode must not be greater than 500 milliseconds; for push mode the duration value must not be greater than 2 seconds.",
            AUDCLNT::AUDCLNT_E_CPUUSAGE_EXCEEDED => "The audio endpoint device has been unplugged, or the audio hardware or associated hardware rIndicates that the process-pass duration exceeded the maximum CPU usage. The audio engine keeps track of CPU usage by maintaining the number of times the process-pass duration exceeds the maximum CPU usage. The maximum CPU usage is calculated as a percent of the engine's periodicity. The percentage value is the system's CPU throttle value (within the range of 10% and 90%). If this value is not found, then the default value of 40% is used to calculate the maximum CPU usage.esources have been reconfigured, disabled, removed, or otherwise made unavailable for use.",
            AUDCLNT::AUDCLNT_E_BUFFER_ERROR => "GetBuffer failed to retrieve a data buffer and *ppData points to NULL. For more information, see Remarks.",
            AUDCLNT::AUDCLNT_E_BUFFER_SIZE_NOT_ALIGNED => "The requested buffer size is not aligned. This code can be returned for a render or a capture device if the caller specified AUDCLNT_SHAREMODE_EXCLUSIVE and the AUDCLNT_STREAMFLAGS_EVENTCALLBACK flags. The caller must call Initialize again with the aligned buffer size. For more information, see Remarks.",
        }
    }
}
