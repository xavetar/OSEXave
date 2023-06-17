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
pub enum WDS_MC_CLIENT {
    WDSTPC_E_CALLBACKS_NOT_REG,
    WDSTPC_E_ALREADY_COMPLETED,
    WDSTPC_E_ALREADY_IN_PROGRESS,
    WDSTPC_E_UNKNOWN_ERROR,
    WDSTPC_E_NOT_INITIALIZED,
    WDSTPC_E_KICKED_POLICY_NOT_MET,
    WDSTPC_E_KICKED_FALLBACK,
    WDSTPC_E_KICKED_FAIL,
    WDSTPC_E_KICKED_UNKNOWN,
    WDSTPC_E_MULTISTREAM_NOT_ENABLED,
    WDSTPC_E_ALREADY_IN_LOWEST_SESSION,
    WDSTPC_E_CLIENT_DEMOTE_NOT_SUPPORTED,
    WDSTPC_E_NO_IP4_INTERFACE,
    WDSTPTC_E_WIM_APPLY_REQUIRES_REFERENCE_IMAGE,
}

impl WDS_MC_CLIENT {
    pub fn code(&self) -> u32 {
        return match self {
            WDS_MC_CLIENT::WDSTPC_E_CALLBACKS_NOT_REG => 0xC1220300 as u32,
            WDS_MC_CLIENT::WDSTPC_E_ALREADY_COMPLETED => 0xC1220301 as u32,
            WDS_MC_CLIENT::WDSTPC_E_ALREADY_IN_PROGRESS => 0xC1220302 as u32,
            WDS_MC_CLIENT::WDSTPC_E_UNKNOWN_ERROR => 0xC1220303 as u32,
            WDS_MC_CLIENT::WDSTPC_E_NOT_INITIALIZED => 0xC1220304 as u32,
            WDS_MC_CLIENT::WDSTPC_E_KICKED_POLICY_NOT_MET => 0xC1220305 as u32,
            WDS_MC_CLIENT::WDSTPC_E_KICKED_FALLBACK => 0xC1220306 as u32,
            WDS_MC_CLIENT::WDSTPC_E_KICKED_FAIL => 0xC1220307 as u32,
            WDS_MC_CLIENT::WDSTPC_E_KICKED_UNKNOWN => 0xC1220308 as u32,
            WDS_MC_CLIENT::WDSTPC_E_MULTISTREAM_NOT_ENABLED => 0xC1220309 as u32,
            WDS_MC_CLIENT::WDSTPC_E_ALREADY_IN_LOWEST_SESSION => 0xC122030A as u32,
            WDS_MC_CLIENT::WDSTPC_E_CLIENT_DEMOTE_NOT_SUPPORTED => 0xC122030B as u32,
            WDS_MC_CLIENT::WDSTPC_E_NO_IP4_INTERFACE => 0xC122030C as u32,
            WDS_MC_CLIENT::WDSTPTC_E_WIM_APPLY_REQUIRES_REFERENCE_IMAGE => 0xC122030D as u32,
        }
    }

    pub fn error(&self) -> RawError {
        return match self {
            WDS_MC_CLIENT::WDSTPC_E_CALLBACKS_NOT_REG => RawError::Kind(WDS_MC_CLIENT::WDSTPC_E_CALLBACKS_NOT_REG),
            WDS_MC_CLIENT::WDSTPC_E_ALREADY_COMPLETED => RawError::Kind(WDS_MC_CLIENT::WDSTPC_E_ALREADY_COMPLETED),
            WDS_MC_CLIENT::WDSTPC_E_ALREADY_IN_PROGRESS => RawError::Kind(WDS_MC_CLIENT::WDSTPC_E_ALREADY_IN_PROGRESS),
            WDS_MC_CLIENT::WDSTPC_E_UNKNOWN_ERROR => RawError::Kind(WDS_MC_CLIENT::WDSTPC_E_UNKNOWN_ERROR),
            WDS_MC_CLIENT::WDSTPC_E_NOT_INITIALIZED => RawError::Kind(WDS_MC_CLIENT::WDSTPC_E_NOT_INITIALIZED),
            WDS_MC_CLIENT::WDSTPC_E_KICKED_POLICY_NOT_MET => RawError::Kind(WDS_MC_CLIENT::WDSTPC_E_KICKED_POLICY_NOT_MET),
            WDS_MC_CLIENT::WDSTPC_E_KICKED_FALLBACK => RawError::Kind(WDS_MC_CLIENT::WDSTPC_E_KICKED_FALLBACK),
            WDS_MC_CLIENT::WDSTPC_E_KICKED_FAIL => RawError::Kind(WDS_MC_CLIENT::WDSTPC_E_KICKED_FAIL),
            WDS_MC_CLIENT::WDSTPC_E_KICKED_UNKNOWN => RawError::Kind(WDS_MC_CLIENT::WDSTPC_E_KICKED_UNKNOWN),
            WDS_MC_CLIENT::WDSTPC_E_MULTISTREAM_NOT_ENABLED => RawError::Kind(WDS_MC_CLIENT::WDSTPC_E_MULTISTREAM_NOT_ENABLED),
            WDS_MC_CLIENT::WDSTPC_E_ALREADY_IN_LOWEST_SESSION => RawError::Kind(WDS_MC_CLIENT::WDSTPC_E_ALREADY_IN_LOWEST_SESSION),
            WDS_MC_CLIENT::WDSTPC_E_CLIENT_DEMOTE_NOT_SUPPORTED => RawError::Kind(WDS_MC_CLIENT::WDSTPC_E_CLIENT_DEMOTE_NOT_SUPPORTED),
            WDS_MC_CLIENT::WDSTPC_E_NO_IP4_INTERFACE => RawError::Kind(WDS_MC_CLIENT::WDSTPC_E_NO_IP4_INTERFACE),
            WDS_MC_CLIENT::WDSTPTC_E_WIM_APPLY_REQUIRES_REFERENCE_IMAGE => RawError::Kind(WDS_MC_CLIENT::WDSTPTC_E_WIM_APPLY_REQUIRES_REFERENCE_IMAGE),
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            WDS_MC_CLIENT::WDSTPC_E_CALLBACKS_NOT_REG => "The required callbacks were not registered.",
            WDS_MC_CLIENT::WDSTPC_E_ALREADY_COMPLETED => "The session has already completed the download.",
            WDS_MC_CLIENT::WDSTPC_E_ALREADY_IN_PROGRESS => "The download is already in progress.",
            WDS_MC_CLIENT::WDSTPC_E_UNKNOWN_ERROR => "An unknown error occurred.",
            WDS_MC_CLIENT::WDSTPC_E_NOT_INITIALIZED => "WDS Multicast Client not initialized.",
            WDS_MC_CLIENT::WDSTPC_E_KICKED_POLICY_NOT_MET => "The client did not meet the policy requirements set by the administrator and was kicked from the session.",
            WDS_MC_CLIENT::WDSTPC_E_KICKED_FALLBACK => "The client was kicked by the administrator. The client should fallback to some other mechanism to get the contents.",
            WDS_MC_CLIENT::WDSTPC_E_KICKED_FAIL => "The client was kicked by the administrator. The client should fail the operation completely.",
            WDS_MC_CLIENT::WDSTPC_E_KICKED_UNKNOWN => "The client was kicked by the administrator. An unknown reason was specified for kicking from session.",
            WDS_MC_CLIENT::WDSTPC_E_MULTISTREAM_NOT_ENABLED => "Multistream support is not enabled.",
            WDS_MC_CLIENT::WDSTPC_E_ALREADY_IN_LOWEST_SESSION => "The specified client is already in the lowest multistream session.",
            WDS_MC_CLIENT::WDSTPC_E_CLIENT_DEMOTE_NOT_SUPPORTED => "The specified client does not support demotion.",
            WDS_MC_CLIENT::WDSTPC_E_NO_IP4_INTERFACE => "No IPv4 interface available on server.",
            WDS_MC_CLIENT::WDSTPTC_E_WIM_APPLY_REQUIRES_REFERENCE_IMAGE => "The specified WIM file requires a reference WIM such as res.rwm in order to be applied.",
        }
    }

    pub fn from_name(name: &str) -> WDS_MC_CLIENT {
        return match name {
            "WDSTPC_E_CALLBACKS_NOT_REG" => WDS_MC_CLIENT::WDSTPC_E_CALLBACKS_NOT_REG,
            "WDSTPC_E_ALREADY_COMPLETED" => WDS_MC_CLIENT::WDSTPC_E_ALREADY_COMPLETED,
            "WDSTPC_E_ALREADY_IN_PROGRESS" => WDS_MC_CLIENT::WDSTPC_E_ALREADY_IN_PROGRESS,
            "WDSTPC_E_UNKNOWN_ERROR" => WDS_MC_CLIENT::WDSTPC_E_UNKNOWN_ERROR,
            "WDSTPC_E_NOT_INITIALIZED" => WDS_MC_CLIENT::WDSTPC_E_NOT_INITIALIZED,
            "WDSTPC_E_KICKED_POLICY_NOT_MET" => WDS_MC_CLIENT::WDSTPC_E_KICKED_POLICY_NOT_MET,
            "WDSTPC_E_KICKED_FALLBACK" => WDS_MC_CLIENT::WDSTPC_E_KICKED_FALLBACK,
            "WDSTPC_E_KICKED_FAIL" => WDS_MC_CLIENT::WDSTPC_E_KICKED_FAIL,
            "WDSTPC_E_KICKED_UNKNOWN" => WDS_MC_CLIENT::WDSTPC_E_KICKED_UNKNOWN,
            "WDSTPC_E_MULTISTREAM_NOT_ENABLED" => WDS_MC_CLIENT::WDSTPC_E_MULTISTREAM_NOT_ENABLED,
            "WDSTPC_E_ALREADY_IN_LOWEST_SESSION" => WDS_MC_CLIENT::WDSTPC_E_ALREADY_IN_LOWEST_SESSION,
            "WDSTPC_E_CLIENT_DEMOTE_NOT_SUPPORTED" => WDS_MC_CLIENT::WDSTPC_E_CLIENT_DEMOTE_NOT_SUPPORTED,
            "WDSTPC_E_NO_IP4_INTERFACE" => WDS_MC_CLIENT::WDSTPC_E_NO_IP4_INTERFACE,
            "WDSTPTC_E_WIM_APPLY_REQUIRES_REFERENCE_IMAGE" => WDS_MC_CLIENT::WDSTPTC_E_WIM_APPLY_REQUIRES_REFERENCE_IMAGE,
        }
    }
    pub fn from_code(code: u32) -> WDS_MC_CLIENT {
        return match code {
            0xC1220300 => WDS_MC_CLIENT::WDSTPC_E_CALLBACKS_NOT_REG,
            0xC1220301 => WDS_MC_CLIENT::WDSTPC_E_ALREADY_COMPLETED,
            0xC1220302 => WDS_MC_CLIENT::WDSTPC_E_ALREADY_IN_PROGRESS,
            0xC1220303 => WDS_MC_CLIENT::WDSTPC_E_UNKNOWN_ERROR,
            0xC1220304 => WDS_MC_CLIENT::WDSTPC_E_NOT_INITIALIZED,
            0xC1220305 => WDS_MC_CLIENT::WDSTPC_E_KICKED_POLICY_NOT_MET,
            0xC1220306 => WDS_MC_CLIENT::WDSTPC_E_KICKED_FALLBACK,
            0xC1220307 => WDS_MC_CLIENT::WDSTPC_E_KICKED_FAIL,
            0xC1220308 => WDS_MC_CLIENT::WDSTPC_E_KICKED_UNKNOWN,
            0xC1220309 => WDS_MC_CLIENT::WDSTPC_E_MULTISTREAM_NOT_ENABLED,
            0xC122030A => WDS_MC_CLIENT::WDSTPC_E_ALREADY_IN_LOWEST_SESSION,
            0xC122030B => WDS_MC_CLIENT::WDSTPC_E_CLIENT_DEMOTE_NOT_SUPPORTED,
            0xC122030C => WDS_MC_CLIENT::WDSTPC_E_NO_IP4_INTERFACE,
            0xC122030D => WDS_MC_CLIENT::WDSTPTC_E_WIM_APPLY_REQUIRES_REFERENCE_IMAGE,
        }
    }
}
