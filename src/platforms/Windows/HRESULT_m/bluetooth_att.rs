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

pub enum BLUETOOTH_ATT {
    E_BLUETOOTH_ATT_INVALID_HANDLE = 0x80650001,
    E_BLUETOOTH_ATT_READ_NOT_PERMITTED = 0x80650002,
    E_BLUETOOTH_ATT_WRITE_NOT_PERMITTED = 0x80650003,
    E_BLUETOOTH_ATT_INVALID_PDU = 0x80650004,
    E_BLUETOOTH_ATT_INSUFFICIENT_AUTHENTICATION = 0x80650005,
    E_BLUETOOTH_ATT_REQUEST_NOT_SUPPORTED = 0x80650006,
    E_BLUETOOTH_ATT_INVALID_OFFSET = 0x80650007,
    E_BLUETOOTH_ATT_INSUFFICIENT_AUTHORIZATION = 0x80650008,
    E_BLUETOOTH_ATT_PREPARE_QUEUE_FULL = 0x80650009,
    E_BLUETOOTH_ATT_ATTRIBUTE_NOT_FOUND = 0x8065000A,
    E_BLUETOOTH_ATT_ATTRIBUTE_NOT_LONG = 0x8065000B,
    E_BLUETOOTH_ATT_INSUFFICIENT_ENCRYPTION_KEY_SIZE = 0x8065000C,
    E_BLUETOOTH_ATT_INVALID_ATTRIBUTE_VALUE_LENGTH = 0x8065000D,
    E_BLUETOOTH_ATT_UNLIKELY = 0x8065000E,
    E_BLUETOOTH_ATT_INSUFFICIENT_ENCRYPTION = 0x8065000F,
    E_BLUETOOTH_ATT_UNSUPPORTED_GROUP_TYPE = 0x80650010,
    E_BLUETOOTH_ATT_INSUFFICIENT_RESOURCES = 0x80650011,
    E_BLUETOOTH_ATT_UNKNOWN_ERROR = 0x80651000,
}

impl BLUETOOTH_ATT {
    pub fn description(&self) -> &'static str {
        match self {
            BLUETOOTH_ATT::E_BLUETOOTH_ATT_INVALID_HANDLE => "The attribute handle given was not valid on this server.",
            BLUETOOTH_ATT::E_BLUETOOTH_ATT_READ_NOT_PERMITTED => "The attribute cannot be read.",
            BLUETOOTH_ATT::E_BLUETOOTH_ATT_WRITE_NOT_PERMITTED => "The attribute cannot be written.",
            BLUETOOTH_ATT::E_BLUETOOTH_ATT_INVALID_PDU => "The attribute PDU was invalid.",
            BLUETOOTH_ATT::E_BLUETOOTH_ATT_INSUFFICIENT_AUTHENTICATION => "The attribute requires authentication before it can be read or written.",
            BLUETOOTH_ATT::E_BLUETOOTH_ATT_REQUEST_NOT_SUPPORTED => "Attribute server does not support the request received from the client.",
            BLUETOOTH_ATT::E_BLUETOOTH_ATT_INVALID_OFFSET => "Offset specified was past the end of the attribute.",
            BLUETOOTH_ATT::E_BLUETOOTH_ATT_INSUFFICIENT_AUTHORIZATION => "The attribute requires authorization before it can be read or written.",
            BLUETOOTH_ATT::E_BLUETOOTH_ATT_PREPARE_QUEUE_FULL => "Too many prepare writes have been queued.",
            BLUETOOTH_ATT::E_BLUETOOTH_ATT_ATTRIBUTE_NOT_FOUND => "No attribute found within the given attribute handle range.",
            BLUETOOTH_ATT::E_BLUETOOTH_ATT_ATTRIBUTE_NOT_LONG => "The attribute cannot be read or written using the Read Blob Request.",
            BLUETOOTH_ATT::E_BLUETOOTH_ATT_INSUFFICIENT_ENCRYPTION_KEY_SIZE => "The Encryption Key Size used for encrypting this link is insufficient.",
            BLUETOOTH_ATT::E_BLUETOOTH_ATT_INVALID_ATTRIBUTE_VALUE_LENGTH => "The attribute value length is invalid for the operation.",
            BLUETOOTH_ATT::E_BLUETOOTH_ATT_UNLIKELY => "The attribute request that was requested has encountered an error that was unlikely, and therefore could not be completed as requested.",
            BLUETOOTH_ATT::E_BLUETOOTH_ATT_INSUFFICIENT_ENCRYPTION => "The attribute requires encryption before it can be read or written.",
            BLUETOOTH_ATT::E_BLUETOOTH_ATT_UNSUPPORTED_GROUP_TYPE => "The attribute type is not a supported grouping attribute as defined by a higher layer specification.",
            BLUETOOTH_ATT::E_BLUETOOTH_ATT_INSUFFICIENT_RESOURCES => "Insufficient Resources to complete the request.",
            BLUETOOTH_ATT::E_BLUETOOTH_ATT_UNKNOWN_ERROR => "An error that lies in the reserved range has been received.",
        }
    }
}
