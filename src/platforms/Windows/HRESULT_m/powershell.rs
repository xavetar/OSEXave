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

pub enum POWERSHELL {
    E_MBN_CONTEXT_NOT_ACTIVATED = 0x80548201,
    E_MBN_BAD_SIM = 0x80548202,
    E_MBN_DATA_CLASS_NOT_AVAILABLE = 0x80548203,
    E_MBN_INVALID_ACCESS_STRING = 0x80548204,
    E_MBN_MAX_ACTIVATED_CONTEXTS = 0x80548205,
    E_MBN_PACKET_SVC_DETACHED = 0x80548206,
    E_MBN_PROVIDER_NOT_VISIBLE = 0x80548207,
    E_MBN_RADIO_POWER_OFF = 0x80548208,
    E_MBN_SERVICE_NOT_ACTIVATED = 0x80548209,
    E_MBN_SIM_NOT_INSERTED = 0x8054820A,
    E_MBN_VOICE_CALL_IN_PROGRESS = 0x8054820B,
    E_MBN_INVALID_CACHE = 0x8054820C,
    E_MBN_NOT_REGISTERED = 0x8054820D,
    E_MBN_PROVIDERS_NOT_FOUND = 0x8054820E,
    E_MBN_PIN_NOT_SUPPORTED = 0x8054820F,
    E_MBN_PIN_REQUIRED = 0x80548210,
    E_MBN_PIN_DISABLED = 0x80548211,
    E_MBN_FAILURE = 0x80548212,
    E_MBN_INVALID_PROFILE = 0x80548218,
    E_MBN_DEFAULT_PROFILE_EXIST = 0x80548219,
    E_MBN_SMS_ENCODING_NOT_SUPPORTED = 0x80548220,
    E_MBN_SMS_FILTER_NOT_SUPPORTED = 0x80548221,
    E_MBN_SMS_INVALID_MEMORY_INDEX = 0x80548222,
    E_MBN_SMS_LANG_NOT_SUPPORTED = 0x80548223,
    E_MBN_SMS_MEMORY_FAILURE = 0x80548224,
    E_MBN_SMS_NETWORK_TIMEOUT = 0x80548225,
    E_MBN_SMS_UNKNOWN_SMSC_ADDRESS = 0x80548226,
    E_MBN_SMS_FORMAT_NOT_SUPPORTED = 0x80548227,
    E_MBN_SMS_OPERATION_NOT_ALLOWED = 0x80548228,
    E_MBN_SMS_MEMORY_FULL = 0x80548229,
}

impl POWERSHELL {
    pub fn description(&self) -> &'static str {
        match self {
            POWERSHELL::E_MBN_CONTEXT_NOT_ACTIVATED => "Context is not activated.",
            POWERSHELL::E_MBN_BAD_SIM => "Bad SIM is inserted.",
            POWERSHELL::E_MBN_DATA_CLASS_NOT_AVAILABLE => "Requested data class is not avaialable.",
            POWERSHELL::E_MBN_INVALID_ACCESS_STRING => "Access point name (APN) or Access string is incorrect.",
            POWERSHELL::E_MBN_MAX_ACTIVATED_CONTEXTS => "Max activated contexts have reached.",
            POWERSHELL::E_MBN_PACKET_SVC_DETACHED => "Device is in packet detach state.",
            POWERSHELL::E_MBN_PROVIDER_NOT_VISIBLE => "Provider is not visible.",
            POWERSHELL::E_MBN_RADIO_POWER_OFF => "Radio is powered off.",
            POWERSHELL::E_MBN_SERVICE_NOT_ACTIVATED => "MBN subscription is not activated.",
            POWERSHELL::E_MBN_SIM_NOT_INSERTED => "SIM is not inserted.",
            POWERSHELL::E_MBN_VOICE_CALL_IN_PROGRESS => "Voice call in progress.",
            POWERSHELL::E_MBN_INVALID_CACHE => "Visible provider cache is invalid.",
            POWERSHELL::E_MBN_NOT_REGISTERED => "Device is not registered.",
            POWERSHELL::E_MBN_PROVIDERS_NOT_FOUND => "Providers not found.",
            POWERSHELL::E_MBN_PIN_NOT_SUPPORTED => "Pin is not supported.",
            POWERSHELL::E_MBN_PIN_REQUIRED => "Pin is required.",
            POWERSHELL::E_MBN_PIN_DISABLED => "PIN is disabled.",
            POWERSHELL::E_MBN_FAILURE => "Generic Failure.",
            POWERSHELL::E_MBN_INVALID_PROFILE => "Profile is invalid.",
            POWERSHELL::E_MBN_DEFAULT_PROFILE_EXIST => "Default profile exist.",
            POWERSHELL::E_MBN_SMS_ENCODING_NOT_SUPPORTED => "SMS encoding is not supported.",
            POWERSHELL::E_MBN_SMS_FILTER_NOT_SUPPORTED => "SMS filter is not supported.",
            POWERSHELL::E_MBN_SMS_INVALID_MEMORY_INDEX => "Invalid SMS memory index is used.",
            POWERSHELL::E_MBN_SMS_LANG_NOT_SUPPORTED => "SMS language is not supported.",
            POWERSHELL::E_MBN_SMS_MEMORY_FAILURE => "SMS memory failure occurred.",
            POWERSHELL::E_MBN_SMS_NETWORK_TIMEOUT => "SMS network timeout happened.",
            POWERSHELL::E_MBN_SMS_UNKNOWN_SMSC_ADDRESS => "Unknown SMSC address is used.",
            POWERSHELL::E_MBN_SMS_FORMAT_NOT_SUPPORTED => "SMS format is not supported.",
            POWERSHELL::E_MBN_SMS_OPERATION_NOT_ALLOWED => "SMS operation is not allowed.",
            POWERSHELL::E_MBN_SMS_MEMORY_FULL => "Device SMS memory is full.",
        }
    }
}
