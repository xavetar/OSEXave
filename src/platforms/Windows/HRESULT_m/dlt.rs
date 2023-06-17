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
pub enum DLT {
    TRK_S_OUT_OF_SYNC,
    TRK_VOLUME_NOT_FOUND,
    TRK_VOLUME_NOT_OWNED,
    TRK_S_NOTIFICATION_QUOTA_EXCEEDED,
    TRK_E_NOT_FOUND,
    TRK_E_VOLUME_QUOTA_EXCEEDED,
    TRK_SERVER_TOO_BUSY,
}

impl DLT {
    pub fn code(&self) -> u32 {
        return match self {
            DLT::TRK_S_OUT_OF_SYNC => 0x0DEAD100 as u32,
            DLT::TRK_VOLUME_NOT_FOUND => 0x0DEAD102 as u32,
            DLT::TRK_VOLUME_NOT_OWNED => 0x0DEAD103 as u32,
            DLT::TRK_S_NOTIFICATION_QUOTA_EXCEEDED => 0x0DEAD107 as u32,
            DLT::TRK_E_NOT_FOUND => 0x8DEAD01B as u32,
            DLT::TRK_E_VOLUME_QUOTA_EXCEEDED => 0x8DEAD01C as u32,
            DLT::TRK_SERVER_TOO_BUSY => 0x8DEAD01E as u32,
        }
    }

    pub fn error(&self) -> RawError {
        return match self {
            DLT::TRK_S_OUT_OF_SYNC => RawError::Kind(DLT::TRK_S_OUT_OF_SYNC),
            DLT::TRK_VOLUME_NOT_FOUND => RawError::Kind(DLT::TRK_VOLUME_NOT_FOUND),
            DLT::TRK_VOLUME_NOT_OWNED => RawError::Kind(DLT::TRK_VOLUME_NOT_OWNED),
            DLT::TRK_S_NOTIFICATION_QUOTA_EXCEEDED => RawError::Kind(DLT::TRK_S_NOTIFICATION_QUOTA_EXCEEDED),
            DLT::TRK_E_NOT_FOUND => RawError::Kind(DLT::TRK_E_NOT_FOUND),
            DLT::TRK_E_VOLUME_QUOTA_EXCEEDED => RawError::Kind(DLT::TRK_E_VOLUME_QUOTA_EXCEEDED),
            DLT::TRK_SERVER_TOO_BUSY => RawError::Kind(DLT::TRK_SERVER_TOO_BUSY),
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            DLT::TRK_S_OUT_OF_SYNC => "The VolumeSequenceNumber of a MOVE_NOTIFICATION request is incorrect.",
            DLT::TRK_VOLUME_NOT_FOUND => "The VolumeID in a request was not found in the server's ServerVolumeTable.",
            DLT::TRK_VOLUME_NOT_OWNED => "A notification was sent to the LnkSvrMessage method, but the RequestMachine for the request was not the VolumeOwner for a VolumeID in the request.",
            DLT::TRK_S_NOTIFICATION_QUOTA_EXCEEDED => "The server received a MOVE_NOTIFICATION request, but the FileTable size limit has already been reached.",
            DLT::TRK_E_NOT_FOUND => "A requested object was not found.",
            DLT::TRK_E_VOLUME_QUOTA_EXCEEDED => "The server received a CREATE_VOLUME subrequest of a SYNC_VOLUMES request, but the ServerVolumeTable size limit for the RequestMachine has already been reached.",
            DLT::TRK_SERVER_TOO_BUSY => "The server is busy, and the client should retry the request at a later time.",
        }
    }

    pub fn from_name(name: &str) -> DLT {
        return match name {
            "TRK_S_OUT_OF_SYNC" => DLT::TRK_S_OUT_OF_SYNC,
            "TRK_VOLUME_NOT_FOUND" => DLT::TRK_VOLUME_NOT_FOUND,
            "TRK_VOLUME_NOT_OWNED" => DLT::TRK_VOLUME_NOT_OWNED,
            "TRK_S_NOTIFICATION_QUOTA_EXCEEDED" => DLT::TRK_S_NOTIFICATION_QUOTA_EXCEEDED,
            "TRK_E_NOT_FOUND" => DLT::TRK_E_NOT_FOUND,
            "TRK_E_VOLUME_QUOTA_EXCEEDED" => DLT::TRK_E_VOLUME_QUOTA_EXCEEDED,
            "TRK_SERVER_TOO_BUSY" => DLT::TRK_SERVER_TOO_BUSY,
        }
    }
    pub fn from_code(code: u32) -> DLT {
        return match code {
            0x0DEAD100 => DLT::TRK_S_OUT_OF_SYNC,
            0x0DEAD102 => DLT::TRK_VOLUME_NOT_FOUND,
            0x0DEAD103 => DLT::TRK_VOLUME_NOT_OWNED,
            0x0DEAD107 => DLT::TRK_S_NOTIFICATION_QUOTA_EXCEEDED,
            0x8DEAD01B => DLT::TRK_E_NOT_FOUND,
            0x8DEAD01C => DLT::TRK_E_VOLUME_QUOTA_EXCEEDED,
            0x8DEAD01E => DLT::TRK_SERVER_TOO_BUSY,
        }
    }
}
