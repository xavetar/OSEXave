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

pub enum AUDIO {
    E_AUDIO_ENGINE_NODE_NOT_FOUND = 0x80660001,
    E_HDAUDIO_EMPTY_CONNECTION_LIST = 0x80660002,
    E_HDAUDIO_CONNECTION_LIST_NOT_SUPPORTED = 0x80660003,
    E_HDAUDIO_NO_LOGICAL_DEVICES_CREATED = 0x80660004,
    E_HDAUDIO_NULL_LINKED_LIST_ENTRY = 0x80660005,
}

impl AUDIO {
    pub fn description(&self) -> &'static str {
        match self {
            AUDIO::E_AUDIO_ENGINE_NODE_NOT_FOUND => "PortCls could not find an audio engine node exposed by a miniport driver claiming support for IMiniportAudioEngineNode.",
            AUDIO::E_HDAUDIO_EMPTY_CONNECTION_LIST => "HD Audio widget encountered an unexpected empty connection list.",
            AUDIO::E_HDAUDIO_CONNECTION_LIST_NOT_SUPPORTED => "HD Audio widget does not support the connection list parameter.",
            AUDIO::E_HDAUDIO_NO_LOGICAL_DEVICES_CREATED => "No HD Audio subdevices were successfully created.",
            AUDIO::E_HDAUDIO_NULL_LINKED_LIST_ENTRY => "An unexpected NULL pointer was encountered in a linked list.",
        }
    }
}
