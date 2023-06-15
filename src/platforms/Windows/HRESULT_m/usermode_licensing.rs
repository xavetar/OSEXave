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

pub enum USERMODE_LICENSING {
    ERROR_NO_APPLICABLE_APP_LICENSES_FOUND = 0xC0EA0001,
    ERROR_CLIP_LICENSE_NOT_FOUND = 0xC0EA0002,
    ERROR_CLIP_DEVICE_LICENSE_MISSING = 0xC0EA0003,
    ERROR_CLIP_LICENSE_INVALID_SIGNATURE = 0xC0EA0004,
    ERROR_CLIP_KEYHOLDER_LICENSE_MISSING_OR_INVALID = 0xC0EA0005,
    ERROR_CLIP_LICENSE_EXPIRED = 0xC0EA0006,
    ERROR_CLIP_LICENSE_SIGNED_BY_UNKNOWN_SOURCE = 0xC0EA0007,
    ERROR_CLIP_LICENSE_NOT_SIGNED = 0xC0EA0008,
    ERROR_CLIP_LICENSE_HARDWARE_ID_OUT_OF_TOLERANCE = 0xC0EA0009,
    ERROR_CLIP_LICENSE_DEVICE_ID_MISMATCH = 0xC0EA000A,
}

impl USERMODE_LICENSING {
    pub fn description(&self) -> &'static str {
        match self {
            USERMODE_LICENSING::ERROR_NO_APPLICABLE_APP_LICENSES_FOUND => "No applicable app licenses found.",
            USERMODE_LICENSING::ERROR_CLIP_LICENSE_NOT_FOUND => "CLiP license not found.",
            USERMODE_LICENSING::ERROR_CLIP_DEVICE_LICENSE_MISSING => "CLiP device license not found.",
            USERMODE_LICENSING::ERROR_CLIP_LICENSE_INVALID_SIGNATURE => "CLiP license has an invalid signature.",
            USERMODE_LICENSING::ERROR_CLIP_KEYHOLDER_LICENSE_MISSING_OR_INVALID => "CLiP keyholder license is invalid or missing.",
            USERMODE_LICENSING::ERROR_CLIP_LICENSE_EXPIRED => "CLiP license has expired.",
            USERMODE_LICENSING::ERROR_CLIP_LICENSE_SIGNED_BY_UNKNOWN_SOURCE => "CLiP license is signed by an unknown source.",
            USERMODE_LICENSING::ERROR_CLIP_LICENSE_NOT_SIGNED => "CLiP license is not signed.",
            USERMODE_LICENSING::ERROR_CLIP_LICENSE_HARDWARE_ID_OUT_OF_TOLERANCE => "CLiP license hardware ID is out of tolerance.",
            USERMODE_LICENSING::ERROR_CLIP_LICENSE_DEVICE_ID_MISMATCH => "CLiP license device ID does not match the device ID in the bound device license.",
        }
    }
}
