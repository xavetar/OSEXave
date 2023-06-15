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

pub enum SVHDX {
    ERROR_SSVHDX_ERROR_STORED = 0xC05C0000,
    ERROR_SSVHDX_ERROR_NOT_AVAILABLE = 0xC05CFF00,
    ERROR_SSVHDX_UNIT_ATTENTION_AVAILABLE = 0xC05CFF01,
    ERROR_SSVHDX_UNIT_ATTENTION_CAPACITY_DATA_CHANGED = 0xC05CFF02,
    ERROR_SSVHDX_UNIT_ATTENTION_RESERVATIONS_PREEMPTED = 0xC05CFF03,
    ERROR_SSVHDX_UNIT_ATTENTION_RESERVATIONS_RELEASED = 0xC05CFF04,
    ERROR_SSVHDX_UNIT_ATTENTION_REGISTRATIONS_PREEMPTED = 0xC05CFF05,
    ERROR_SSVHDX_UNIT_ATTENTION_OPERATING_DEFINITION_CHANGED = 0xC05CFF06,
    ERROR_SSVHDX_RESERVATION_CONFLICT = 0xC05CFF07,
    ERROR_SSVHDX_WRONG_FILE_TYPE = 0xC05CFF08,
    ERROR_SSVHDX_VERSION_MISMATCH = 0xC05CFF09,
    ERROR_VHD_SHARED = 0xC05CFF0A,
    ERROR_SSVHDX_NO_INITIATOR = 0xC05CFF0B,
    ERROR_VHDSET_BACKING_STORAGE_NOT_FOUND = 0xC05CFF0C,
}

impl SVHDX {
    pub fn description(&self) -> &'static str {
        match self {
            SVHDX::ERROR_SSVHDX_ERROR_STORED => "The proper error code with sense data was stored on server side.",
            SVHDX::ERROR_SSVHDX_ERROR_NOT_AVAILABLE => "The requested error data is not available on the server.",
            SVHDX::ERROR_SSVHDX_UNIT_ATTENTION_AVAILABLE => "Unit Attention data is available for the initiator to query.",
            SVHDX::ERROR_SSVHDX_UNIT_ATTENTION_CAPACITY_DATA_CHANGED => "The data capacity of the device has changed, resulting in a Unit Attention condition.",
            SVHDX::ERROR_SSVHDX_UNIT_ATTENTION_RESERVATIONS_PREEMPTED => "A previous operation resulted in this initiator's reservations being preempted, resulting in a Unit Attention condition.",
            SVHDX::ERROR_SSVHDX_UNIT_ATTENTION_RESERVATIONS_RELEASED => "A previous operation resulted in this initiator's reservations being released, resulting in a Unit Attention condition.",
            SVHDX::ERROR_SSVHDX_UNIT_ATTENTION_REGISTRATIONS_PREEMPTED => "A previous operation resulted in this initiator's registrations being preempted, resulting in a Unit Attention condition.",
            SVHDX::ERROR_SSVHDX_UNIT_ATTENTION_OPERATING_DEFINITION_CHANGED => "The data storage format of the device has changed, resulting in a Unit Attention condition.",
            SVHDX::ERROR_SSVHDX_RESERVATION_CONFLICT => "The current initiator is not allowed to perform the SCSI command because of a reservation conflict.",
            SVHDX::ERROR_SSVHDX_WRONG_FILE_TYPE => "Multiple virtual machines sharing a virtual hard disk is supported only on Fixed or Dynamic SVHDX format virtual hard disks.",
            SVHDX::ERROR_SSVHDX_VERSION_MISMATCH => "The server version does not match the requested version.",
            SVHDX::ERROR_VHD_SHARED => "The requested operation cannot be performed on the virtual disk as it is currently used in shared mode.",
            SVHDX::ERROR_SSVHDX_NO_INITIATOR => "Invalid Shared SVHDX open due to lack of initiator ID. Check for related Continuous Availability failures.",
            SVHDX::ERROR_VHDSET_BACKING_STORAGE_NOT_FOUND => "The requested operation failed due to a missing backing storage file.",
        }
    }
}
