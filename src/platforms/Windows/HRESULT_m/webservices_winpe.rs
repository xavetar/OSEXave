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

pub enum WEBSERVICES_WINPE {
    WS_S_ASYNC = 0x003D0000,
    WS_S_END = 0x003D0001,
    WS_E_INVALID_FORMAT = 0x803D0000,
    WS_E_OBJECT_FAULTED = 0x803D0001,
    WS_E_NUMERIC_OVERFLOW = 0x803D0002,
    WS_E_INVALID_OPERATION = 0x803D0003,
    WS_E_OPERATION_ABORTED = 0x803D0004,
    WS_E_ENDPOINT_ACCESS_DENIED = 0x803D0005,
    WS_E_OPERATION_TIMED_OUT = 0x803D0006,
    WS_E_OPERATION_ABANDONED = 0x803D0007,
    WS_E_QUOTA_EXCEEDED = 0x803D0008,
    WS_E_NO_TRANSLATION_AVAILABLE = 0x803D0009,
    WS_E_SECURITY_VERIFICATION_FAILURE = 0x803D000A,
    WS_E_ADDRESS_IN_USE = 0x803D000B,
    WS_E_ADDRESS_NOT_AVAILABLE = 0x803D000C,
    WS_E_ENDPOINT_NOT_FOUND = 0x803D000D,
    WS_E_ENDPOINT_NOT_AVAILABLE = 0x803D000E,
    WS_E_ENDPOINT_FAILURE = 0x803D000F,
    WS_E_ENDPOINT_UNREACHABLE = 0x803D0010,
    WS_E_ENDPOINT_ACTION_NOT_SUPPORTED = 0x803D0011,
    WS_E_ENDPOINT_TOO_BUSY = 0x803D0012,
    WS_E_ENDPOINT_FAULT_RECEIVED = 0x803D0013,
    WS_E_ENDPOINT_DISCONNECTED = 0x803D0014,
    WS_E_PROXY_FAILURE = 0x803D0015,
    WS_E_PROXY_ACCESS_DENIED = 0x803D0016,
    WS_E_NOT_SUPPORTED = 0x803D0017,
    WS_E_PROXY_REQUIRES_BASIC_AUTH = 0x803D0018,
    WS_E_PROXY_REQUIRES_DIGEST_AUTH = 0x803D0019,
    WS_E_PROXY_REQUIRES_NTLM_AUTH = 0x803D001A,
    WS_E_PROXY_REQUIRES_NEGOTIATE_AUTH = 0x803D001B,
    WS_E_SERVER_REQUIRES_BASIC_AUTH = 0x803D001C,
    WS_E_SERVER_REQUIRES_DIGEST_AUTH = 0x803D001D,
    WS_E_SERVER_REQUIRES_NTLM_AUTH = 0x803D001E,
    WS_E_SERVER_REQUIRES_NEGOTIATE_AUTH = 0x803D001F,
    WS_E_INVALID_ENDPOINT_URL = 0x803D0020,
    WS_E_OTHER = 0x803D0021,
    WS_E_SECURITY_TOKEN_EXPIRED = 0x803D0022,
    WS_E_SECURITY_SYSTEM_FAILURE = 0x803D0023,
}

impl WEBSERVICES_WINPE {
    pub fn description(&self) -> &'static str {
        match self {
            WEBSERVICES_WINPE::WS_S_ASYNC => "The function call is completing asynchronously.",
            WEBSERVICES_WINPE::WS_S_END => "There are no more messages available on the channel.",
            WEBSERVICES_WINPE::WS_E_INVALID_FORMAT => "The input data was not in the expected format or did not have the expected value.",
            WEBSERVICES_WINPE::WS_E_OBJECT_FAULTED => "The operation could not be completed because the object is in a faulted state due to a previous error.",
            WEBSERVICES_WINPE::WS_E_NUMERIC_OVERFLOW => "The operation could not be completed because it would lead to numeric overflow.",
            WEBSERVICES_WINPE::WS_E_INVALID_OPERATION => "The operation is not allowed due to the current state of the object.",
            WEBSERVICES_WINPE::WS_E_OPERATION_ABORTED => "The operation was aborted.",
            WEBSERVICES_WINPE::WS_E_ENDPOINT_ACCESS_DENIED => "Access was denied by the remote endpoint.",
            WEBSERVICES_WINPE::WS_E_OPERATION_TIMED_OUT => "The operation did not complete within the time allotted.",
            WEBSERVICES_WINPE::WS_E_OPERATION_ABANDONED => "The operation was abandoned.",
            WEBSERVICES_WINPE::WS_E_QUOTA_EXCEEDED => "A quota was exceeded.",
            WEBSERVICES_WINPE::WS_E_NO_TRANSLATION_AVAILABLE => "The information was not available in the specified language.",
            WEBSERVICES_WINPE::WS_E_SECURITY_VERIFICATION_FAILURE => "Security verification was not successful for the received data.",
            WEBSERVICES_WINPE::WS_E_ADDRESS_IN_USE => "The address is already being used.",
            WEBSERVICES_WINPE::WS_E_ADDRESS_NOT_AVAILABLE => "The address is not valid for this context.",
            WEBSERVICES_WINPE::WS_E_ENDPOINT_NOT_FOUND => "The remote endpoint does not exist or could not be located.",
            WEBSERVICES_WINPE::WS_E_ENDPOINT_NOT_AVAILABLE => "The remote endpoint is not currently in service at this location.",
            WEBSERVICES_WINPE::WS_E_ENDPOINT_FAILURE => "The remote endpoint could not process the request.",
            WEBSERVICES_WINPE::WS_E_ENDPOINT_UNREACHABLE => "The remote endpoint was not reachable.",
            WEBSERVICES_WINPE::WS_E_ENDPOINT_ACTION_NOT_SUPPORTED => "The operation was not supported by the remote endpoint.",
            WEBSERVICES_WINPE::WS_E_ENDPOINT_TOO_BUSY => "The remote endpoint is unable to process the request due to being overloaded.",
            WEBSERVICES_WINPE::WS_E_ENDPOINT_FAULT_RECEIVED => "A message containing a fault was received from the remote endpoint.",
            WEBSERVICES_WINPE::WS_E_ENDPOINT_DISCONNECTED => "The connection with the remote endpoint was terminated.",
            WEBSERVICES_WINPE::WS_E_PROXY_FAILURE => "The HTTP proxy server could not process the request.",
            WEBSERVICES_WINPE::WS_E_PROXY_ACCESS_DENIED => "Access was denied by the HTTP proxy server.",
            WEBSERVICES_WINPE::WS_E_NOT_SUPPORTED => "The requested feature is not available on this platform.",
            WEBSERVICES_WINPE::WS_E_PROXY_REQUIRES_BASIC_AUTH => "The HTTP proxy server requires HTTP authentication scheme 'basic'.",
            WEBSERVICES_WINPE::WS_E_PROXY_REQUIRES_DIGEST_AUTH => "The HTTP proxy server requires HTTP authentication scheme 'digest'.",
            WEBSERVICES_WINPE::WS_E_PROXY_REQUIRES_NTLM_AUTH => "The HTTP proxy server requires HTTP authentication scheme 'NTLM'.",
            WEBSERVICES_WINPE::WS_E_PROXY_REQUIRES_NEGOTIATE_AUTH => "The HTTP proxy server requires HTTP authentication scheme 'negotiate'.",
            WEBSERVICES_WINPE::WS_E_SERVER_REQUIRES_BASIC_AUTH => "The remote endpoint requires HTTP authentication scheme 'basic'.",
            WEBSERVICES_WINPE::WS_E_SERVER_REQUIRES_DIGEST_AUTH => "The remote endpoint requires HTTP authentication scheme 'digest'.",
            WEBSERVICES_WINPE::WS_E_SERVER_REQUIRES_NTLM_AUTH => "The remote endpoint requires HTTP authentication scheme 'NTLM'.",
            WEBSERVICES_WINPE::WS_E_SERVER_REQUIRES_NEGOTIATE_AUTH => "The remote endpoint requires HTTP authentication scheme 'negotiate'.",
            WEBSERVICES_WINPE::WS_E_INVALID_ENDPOINT_URL => "The endpoint address URL is invalid.",
            WEBSERVICES_WINPE::WS_E_OTHER => "Unrecognized error occurred in the Windows Web Services framework.",
            WEBSERVICES_WINPE::WS_E_SECURITY_TOKEN_EXPIRED => "A security token was rejected by the server because it has expired.",
            WEBSERVICES_WINPE::WS_E_SECURITY_SYSTEM_FAILURE => "A security operation failed in the Windows Web Services framework.",
        }
    }
}
