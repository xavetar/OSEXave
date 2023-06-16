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

pub enum INPUT_RESUME_KEY_FILTER {
    INPUT_E_OUT_OF_ORDER = 0x80400000,
    INPUT_E_REENTRANCY = 0x80400001,
    INPUT_E_MULTIMODAL = 0x80400002,
    INPUT_E_PACKET = 0x80400003,
    INPUT_E_FRAME = 0x80400004,
    INPUT_E_HISTORY = 0x80400005,
    INPUT_E_DEVICE_INFO = 0x80400006,
    INPUT_E_TRANSFORM = 0x80400007,
    INPUT_E_DEVICE_PROPERTY = 0x80400008,
}

impl INPUT_RESUME_KEY_FILTER {
    pub fn description(&self) -> &'static str {
        match self {
            INPUT_RESUME_KEY_FILTER::INPUT_E_OUT_OF_ORDER => "Input data cannot be processed in the non-chronological order.",
            INPUT_RESUME_KEY_FILTER::INPUT_E_REENTRANCY => "Requested operation cannot be performed inside the callback or event handler.",
            INPUT_RESUME_KEY_FILTER::INPUT_E_MULTIMODAL => "Input cannot be processed because there is ongoing interaction with another pointer type.",
            INPUT_RESUME_KEY_FILTER::INPUT_E_PACKET => "One or more fields in the input packet are invalid.",
            INPUT_RESUME_KEY_FILTER::INPUT_E_FRAME => "Packets in the frame are inconsistent. Either pointer ids are not unique or there is a discrepancy in timestamps, frame ids, pointer types or source devices.",
            INPUT_RESUME_KEY_FILTER::INPUT_E_HISTORY => "The history of frames is inconsistent. Pointer ids, types, source devices don't match, or frame ids are not unique, or timestamps are out of order.",
            INPUT_RESUME_KEY_FILTER::INPUT_E_DEVICE_INFO => "Failed to retrieve information about the input device.",
            INPUT_RESUME_KEY_FILTER::INPUT_E_TRANSFORM => "Coordinate system transformation failed to transform the data.",
            INPUT_RESUME_KEY_FILTER::INPUT_E_DEVICE_PROPERTY => "The property is not supported or not reported correctly by the input device.",
        }
    }
}
