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

pub enum P2P_INT {
    DRT_E_TIMEOUT = 0x80621001,
    DRT_E_INVALID_KEY_SIZE = 0x80621002,
    DRT_E_INVALID_CERT_CHAIN = 0x80621004,
    DRT_E_INVALID_MESSAGE = 0x80621005,
    DRT_E_NO_MORE = 0x80621006,
    DRT_E_INVALID_MAX_ADDRESSES = 0x80621007,
    DRT_E_SEARCH_IN_PROGRESS = 0x80621008,
    DRT_E_INVALID_KEY = 0x80621009,
    DRT_S_RETRY = 0x80621010,
    DRT_E_INVALID_MAX_ENDPOINTS = 0x80621011,
    DRT_E_INVALID_SEARCH_RANGE = 0x80621012,
    DRT_E_INVALID_PORT = 0x80622000,
    DRT_E_INVALID_TRANSPORT_PROVIDER = 0x80622001,
    DRT_E_INVALID_SECURITY_PROVIDER = 0x80622002,
    DRT_E_STILL_IN_USE = 0x80622003,
    DRT_E_INVALID_BOOTSTRAP_PROVIDER = 0x80622004,
    DRT_E_INVALID_ADDRESS = 0x80622005,
    DRT_E_INVALID_SCOPE = 0x80622006,
    DRT_E_TRANSPORT_SHUTTING_DOWN = 0x80622007,
    DRT_E_NO_ADDRESSES_AVAILABLE = 0x80622008,
    DRT_E_DUPLICATE_KEY = 0x80622009,
    DRT_E_TRANSPORTPROVIDER_IN_USE = 0x8062200A,
    DRT_E_TRANSPORTPROVIDER_NOT_ATTACHED = 0x8062200B,
    DRT_E_SECURITYPROVIDER_IN_USE = 0x8062200C,
    DRT_E_SECURITYPROVIDER_NOT_ATTACHED = 0x8062200D,
    DRT_E_BOOTSTRAPPROVIDER_IN_USE = 0x8062200E,
    DRT_E_BOOTSTRAPPROVIDER_NOT_ATTACHED = 0x8062200F,
    DRT_E_TRANSPORT_ALREADY_BOUND = 0x80622101,
    DRT_E_TRANSPORT_NOT_BOUND = 0x80622102,
    DRT_E_TRANSPORT_UNEXPECTED = 0x80622103,
    DRT_E_TRANSPORT_INVALID_ARGUMENT = 0x80622104,
    DRT_E_TRANSPORT_NO_DEST_ADDRESSES = 0x80622105,
    DRT_E_TRANSPORT_EXECUTING_CALLBACK = 0x80622106,
    DRT_E_TRANSPORT_ALREADY_EXISTS_FOR_SCOPE = 0x80622107,
    DRT_E_INVALID_SETTINGS = 0x80622108,
    DRT_E_INVALID_SEARCH_INFO = 0x80622109,
    DRT_E_FAULTED = 0x8062210A,
    DRT_E_TRANSPORT_STILL_BOUND = 0x8062210B,
    DRT_E_INSUFFICIENT_BUFFER = 0x8062210C,
    DRT_E_INVALID_INSTANCE_PREFIX = 0x8062210D,
    DRT_E_INVALID_SECURITY_MODE = 0x8062210E,
    DRT_E_CAPABILITY_MISMATCH = 0x8062210F,
}

impl P2P_INT {
    pub fn description(&self) -> &'static str {
        match self {
            P2P_INT::DRT_E_TIMEOUT => "The DRT operation has timed out.",
            P2P_INT::DRT_E_INVALID_KEY_SIZE => "The specified key does not meet, or exceeds, the allowed DRT key size.",
            P2P_INT::DRT_E_INVALID_CERT_CHAIN => "There is no certificate store attached or there is an error in the certificate chain.",
            P2P_INT::DRT_E_INVALID_MESSAGE => "The message exceeds the allowed character limit or is in an incorrect format.",
            P2P_INT::DRT_E_NO_MORE => "There is no more event data available.",
            P2P_INT::DRT_E_INVALID_MAX_ADDRESSES => "No addresses are available or the number of addresses exceeds 20.",
            P2P_INT::DRT_E_SEARCH_IN_PROGRESS => "A search operation is already in progress.",
            P2P_INT::DRT_E_INVALID_KEY => "Supplied key does not match generated key.",
            P2P_INT::DRT_S_RETRY => "The operation could not complete, but may be successful in a second attempt.",
            P2P_INT::DRT_E_INVALID_MAX_ENDPOINTS => "The number of endpoints exceeds maximum allowed endpoints.",
            P2P_INT::DRT_E_INVALID_SEARCH_RANGE => "Min and max key values are equal, but target is different.",
            P2P_INT::DRT_E_INVALID_PORT => "The specified port value is NULL.",
            P2P_INT::DRT_E_INVALID_TRANSPORT_PROVIDER => "The specified transport provider is invalid.",
            P2P_INT::DRT_E_INVALID_SECURITY_PROVIDER => "The specified security provider is invalid.",
            P2P_INT::DRT_E_STILL_IN_USE => "The DRT infrastructure is currently busy and cannot complete the operation.",
            P2P_INT::DRT_E_INVALID_BOOTSTRAP_PROVIDER => "The specified bootstrap provider is invalid.",
            P2P_INT::DRT_E_INVALID_ADDRESS => "The supplied address is not in the accepted and complete format, or is NULL.",
            P2P_INT::DRT_E_INVALID_SCOPE => "The specified scope is not one of the values defined in DRT_SCOPE.",
            P2P_INT::DRT_E_TRANSPORT_SHUTTING_DOWN => "The specified transport is in the process of shutting down.",
            P2P_INT::DRT_E_NO_ADDRESSES_AVAILABLE => "There are currently no remote nodes present in the DRT.",
            P2P_INT::DRT_E_DUPLICATE_KEY => "This key already exists within the DRT infrastructure.",
            P2P_INT::DRT_E_TRANSPORTPROVIDER_IN_USE => "The specified transport provider is already in use.",
            P2P_INT::DRT_E_TRANSPORTPROVIDER_NOT_ATTACHED => "The transport provider is not attached.",
            P2P_INT::DRT_E_SECURITYPROVIDER_IN_USE => "The specified security provider is already in use.",
            P2P_INT::DRT_E_SECURITYPROVIDER_NOT_ATTACHED => "The security provider is not attached.",
            P2P_INT::DRT_E_BOOTSTRAPPROVIDER_IN_USE => "The specified security provider is already in use.",
            P2P_INT::DRT_E_BOOTSTRAPPROVIDER_NOT_ATTACHED => "The bootstrap provider is not attached.",
            P2P_INT::DRT_E_TRANSPORT_ALREADY_BOUND => "The transport is already bound.",
            P2P_INT::DRT_E_TRANSPORT_NOT_BOUND => "The transport is not bound.",
            P2P_INT::DRT_E_TRANSPORT_UNEXPECTED => "The transport has encountered an unexpected error.",
            P2P_INT::DRT_E_TRANSPORT_INVALID_ARGUMENT => "An invalid argument was passed to the transport.",
            P2P_INT::DRT_E_TRANSPORT_NO_DEST_ADDRESSES => "Destination addresses have not been supplied to the transport.",
            P2P_INT::DRT_E_TRANSPORT_EXECUTING_CALLBACK => "The transport is currently executing a callback operation.",
            P2P_INT::DRT_E_TRANSPORT_ALREADY_EXISTS_FOR_SCOPE => "A transport already exists for this DRT scope.",
            P2P_INT::DRT_E_INVALID_SETTINGS => "The data contained inDRT SETTINGS structure is invalid or the relevant parameter value is NULL.",
            P2P_INT::DRT_E_INVALID_SEARCH_INFO => "The data contained in DRT_SEARCH_INFO structure is invalid or the relevant parameter value is NULL.",
            P2P_INT::DRT_E_FAULTED => "The DRT infrastructure has faulted. The DrtClose function must be called, after which an attempt to re-open the DRT can be made.",
            P2P_INT::DRT_E_TRANSPORT_STILL_BOUND => "The transport is currently bound.",
            P2P_INT::DRT_E_INSUFFICIENT_BUFFER => "The size of the buffer is insufficient for the operation.",
            P2P_INT::DRT_E_INVALID_INSTANCE_PREFIX => "The DRT instance prefix is invalid.",
            P2P_INT::DRT_E_INVALID_SECURITY_MODE => "The specified security mode is not one of the values defined in DRT_SECURITY_MODE.",
            P2P_INT::DRT_E_CAPABILITY_MISMATCH => "The requested security algorithms are not available.",
        }
    }
}
