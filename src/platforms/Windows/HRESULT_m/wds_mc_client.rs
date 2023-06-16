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

pub enum WDS_MC_CLIENT {
    WDSTPC_E_CALLBACKS_NOT_REG = 0xC1220300,
    WDSTPC_E_ALREADY_COMPLETED = 0xC1220301,
    WDSTPC_E_ALREADY_IN_PROGRESS = 0xC1220302,
    WDSTPC_E_UNKNOWN_ERROR = 0xC1220303,
    WDSTPC_E_NOT_INITIALIZED = 0xC1220304,
    WDSTPC_E_KICKED_POLICY_NOT_MET = 0xC1220305,
    WDSTPC_E_KICKED_FALLBACK = 0xC1220306,
    WDSTPC_E_KICKED_FAIL = 0xC1220307,
    WDSTPC_E_KICKED_UNKNOWN = 0xC1220308,
    WDSTPC_E_MULTISTREAM_NOT_ENABLED = 0xC1220309,
    WDSTPC_E_ALREADY_IN_LOWEST_SESSION = 0xC122030A,
    WDSTPC_E_CLIENT_DEMOTE_NOT_SUPPORTED = 0xC122030B,
    WDSTPC_E_NO_IP4_INTERFACE = 0xC122030C,
    WDSTPTC_E_WIM_APPLY_REQUIRES_REFERENCE_IMAGE = 0xC122030D,
}

impl WDS_MC_CLIENT {
    pub fn description(&self) -> &'static str {
        match self {
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
}