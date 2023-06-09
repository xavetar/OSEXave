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
pub enum WEBSERVICES_WINPE {
    WS_S_ASYNC,
    WS_S_END,
    WS_E_INVALID_FORMAT,
    WS_E_OBJECT_FAULTED,
    WS_E_NUMERIC_OVERFLOW,
    WS_E_INVALID_OPERATION,
    WS_E_OPERATION_ABORTED,
    WS_E_ENDPOINT_ACCESS_DENIED,
    WS_E_OPERATION_TIMED_OUT,
    WS_E_OPERATION_ABANDONED,
    WS_E_QUOTA_EXCEEDED,
    WS_E_NO_TRANSLATION_AVAILABLE,
    WS_E_SECURITY_VERIFICATION_FAILURE,
    WS_E_ADDRESS_IN_USE,
    WS_E_ADDRESS_NOT_AVAILABLE,
    WS_E_ENDPOINT_NOT_FOUND,
    WS_E_ENDPOINT_NOT_AVAILABLE,
    WS_E_ENDPOINT_FAILURE,
    WS_E_ENDPOINT_UNREACHABLE,
    WS_E_ENDPOINT_ACTION_NOT_SUPPORTED,
    WS_E_ENDPOINT_TOO_BUSY,
    WS_E_ENDPOINT_FAULT_RECEIVED,
    WS_E_ENDPOINT_DISCONNECTED,
    WS_E_PROXY_FAILURE,
    WS_E_PROXY_ACCESS_DENIED,
    WS_E_NOT_SUPPORTED,
    WS_E_PROXY_REQUIRES_BASIC_AUTH,
    WS_E_PROXY_REQUIRES_DIGEST_AUTH,
    WS_E_PROXY_REQUIRES_NTLM_AUTH,
    WS_E_PROXY_REQUIRES_NEGOTIATE_AUTH,
    WS_E_SERVER_REQUIRES_BASIC_AUTH,
    WS_E_SERVER_REQUIRES_DIGEST_AUTH,
    WS_E_SERVER_REQUIRES_NTLM_AUTH,
    WS_E_SERVER_REQUIRES_NEGOTIATE_AUTH,
    WS_E_INVALID_ENDPOINT_URL,
    WS_E_OTHER,
    WS_E_SECURITY_TOKEN_EXPIRED,
    WS_E_SECURITY_SYSTEM_FAILURE,
}

impl WEBSERVICES_WINPE {
    pub fn code(&self) -> u32 {
        return match self {
            WEBSERVICES_WINPE::WS_S_ASYNC => 0x003D0000 as u32,
            WEBSERVICES_WINPE::WS_S_END => 0x003D0001 as u32,
            WEBSERVICES_WINPE::WS_E_INVALID_FORMAT => 0x803D0000 as u32,
            WEBSERVICES_WINPE::WS_E_OBJECT_FAULTED => 0x803D0001 as u32,
            WEBSERVICES_WINPE::WS_E_NUMERIC_OVERFLOW => 0x803D0002 as u32,
            WEBSERVICES_WINPE::WS_E_INVALID_OPERATION => 0x803D0003 as u32,
            WEBSERVICES_WINPE::WS_E_OPERATION_ABORTED => 0x803D0004 as u32,
            WEBSERVICES_WINPE::WS_E_ENDPOINT_ACCESS_DENIED => 0x803D0005 as u32,
            WEBSERVICES_WINPE::WS_E_OPERATION_TIMED_OUT => 0x803D0006 as u32,
            WEBSERVICES_WINPE::WS_E_OPERATION_ABANDONED => 0x803D0007 as u32,
            WEBSERVICES_WINPE::WS_E_QUOTA_EXCEEDED => 0x803D0008 as u32,
            WEBSERVICES_WINPE::WS_E_NO_TRANSLATION_AVAILABLE => 0x803D0009 as u32,
            WEBSERVICES_WINPE::WS_E_SECURITY_VERIFICATION_FAILURE => 0x803D000A as u32,
            WEBSERVICES_WINPE::WS_E_ADDRESS_IN_USE => 0x803D000B as u32,
            WEBSERVICES_WINPE::WS_E_ADDRESS_NOT_AVAILABLE => 0x803D000C as u32,
            WEBSERVICES_WINPE::WS_E_ENDPOINT_NOT_FOUND => 0x803D000D as u32,
            WEBSERVICES_WINPE::WS_E_ENDPOINT_NOT_AVAILABLE => 0x803D000E as u32,
            WEBSERVICES_WINPE::WS_E_ENDPOINT_FAILURE => 0x803D000F as u32,
            WEBSERVICES_WINPE::WS_E_ENDPOINT_UNREACHABLE => 0x803D0010 as u32,
            WEBSERVICES_WINPE::WS_E_ENDPOINT_ACTION_NOT_SUPPORTED => 0x803D0011 as u32,
            WEBSERVICES_WINPE::WS_E_ENDPOINT_TOO_BUSY => 0x803D0012 as u32,
            WEBSERVICES_WINPE::WS_E_ENDPOINT_FAULT_RECEIVED => 0x803D0013 as u32,
            WEBSERVICES_WINPE::WS_E_ENDPOINT_DISCONNECTED => 0x803D0014 as u32,
            WEBSERVICES_WINPE::WS_E_PROXY_FAILURE => 0x803D0015 as u32,
            WEBSERVICES_WINPE::WS_E_PROXY_ACCESS_DENIED => 0x803D0016 as u32,
            WEBSERVICES_WINPE::WS_E_NOT_SUPPORTED => 0x803D0017 as u32,
            WEBSERVICES_WINPE::WS_E_PROXY_REQUIRES_BASIC_AUTH => 0x803D0018 as u32,
            WEBSERVICES_WINPE::WS_E_PROXY_REQUIRES_DIGEST_AUTH => 0x803D0019 as u32,
            WEBSERVICES_WINPE::WS_E_PROXY_REQUIRES_NTLM_AUTH => 0x803D001A as u32,
            WEBSERVICES_WINPE::WS_E_PROXY_REQUIRES_NEGOTIATE_AUTH => 0x803D001B as u32,
            WEBSERVICES_WINPE::WS_E_SERVER_REQUIRES_BASIC_AUTH => 0x803D001C as u32,
            WEBSERVICES_WINPE::WS_E_SERVER_REQUIRES_DIGEST_AUTH => 0x803D001D as u32,
            WEBSERVICES_WINPE::WS_E_SERVER_REQUIRES_NTLM_AUTH => 0x803D001E as u32,
            WEBSERVICES_WINPE::WS_E_SERVER_REQUIRES_NEGOTIATE_AUTH => 0x803D001F as u32,
            WEBSERVICES_WINPE::WS_E_INVALID_ENDPOINT_URL => 0x803D0020 as u32,
            WEBSERVICES_WINPE::WS_E_OTHER => 0x803D0021 as u32,
            WEBSERVICES_WINPE::WS_E_SECURITY_TOKEN_EXPIRED => 0x803D0022 as u32,
            WEBSERVICES_WINPE::WS_E_SECURITY_SYSTEM_FAILURE => 0x803D0023 as u32,
        }
    }

    pub fn error(&self) -> RawError {
        return match self {
            WEBSERVICES_WINPE::WS_S_ASYNC => RawError::Kind(WEBSERVICES_WINPE::WS_S_ASYNC),
            WEBSERVICES_WINPE::WS_S_END => RawError::Kind(WEBSERVICES_WINPE::WS_S_END),
            WEBSERVICES_WINPE::WS_E_INVALID_FORMAT => RawError::Kind(WEBSERVICES_WINPE::WS_E_INVALID_FORMAT),
            WEBSERVICES_WINPE::WS_E_OBJECT_FAULTED => RawError::Kind(WEBSERVICES_WINPE::WS_E_OBJECT_FAULTED),
            WEBSERVICES_WINPE::WS_E_NUMERIC_OVERFLOW => RawError::Kind(WEBSERVICES_WINPE::WS_E_NUMERIC_OVERFLOW),
            WEBSERVICES_WINPE::WS_E_INVALID_OPERATION => RawError::Kind(WEBSERVICES_WINPE::WS_E_INVALID_OPERATION),
            WEBSERVICES_WINPE::WS_E_OPERATION_ABORTED => RawError::Kind(WEBSERVICES_WINPE::WS_E_OPERATION_ABORTED),
            WEBSERVICES_WINPE::WS_E_ENDPOINT_ACCESS_DENIED => RawError::Kind(WEBSERVICES_WINPE::WS_E_ENDPOINT_ACCESS_DENIED),
            WEBSERVICES_WINPE::WS_E_OPERATION_TIMED_OUT => RawError::Kind(WEBSERVICES_WINPE::WS_E_OPERATION_TIMED_OUT),
            WEBSERVICES_WINPE::WS_E_OPERATION_ABANDONED => RawError::Kind(WEBSERVICES_WINPE::WS_E_OPERATION_ABANDONED),
            WEBSERVICES_WINPE::WS_E_QUOTA_EXCEEDED => RawError::Kind(WEBSERVICES_WINPE::WS_E_QUOTA_EXCEEDED),
            WEBSERVICES_WINPE::WS_E_NO_TRANSLATION_AVAILABLE => RawError::Kind(WEBSERVICES_WINPE::WS_E_NO_TRANSLATION_AVAILABLE),
            WEBSERVICES_WINPE::WS_E_SECURITY_VERIFICATION_FAILURE => RawError::Kind(WEBSERVICES_WINPE::WS_E_SECURITY_VERIFICATION_FAILURE),
            WEBSERVICES_WINPE::WS_E_ADDRESS_IN_USE => RawError::Kind(WEBSERVICES_WINPE::WS_E_ADDRESS_IN_USE),
            WEBSERVICES_WINPE::WS_E_ADDRESS_NOT_AVAILABLE => RawError::Kind(WEBSERVICES_WINPE::WS_E_ADDRESS_NOT_AVAILABLE),
            WEBSERVICES_WINPE::WS_E_ENDPOINT_NOT_FOUND => RawError::Kind(WEBSERVICES_WINPE::WS_E_ENDPOINT_NOT_FOUND),
            WEBSERVICES_WINPE::WS_E_ENDPOINT_NOT_AVAILABLE => RawError::Kind(WEBSERVICES_WINPE::WS_E_ENDPOINT_NOT_AVAILABLE),
            WEBSERVICES_WINPE::WS_E_ENDPOINT_FAILURE => RawError::Kind(WEBSERVICES_WINPE::WS_E_ENDPOINT_FAILURE),
            WEBSERVICES_WINPE::WS_E_ENDPOINT_UNREACHABLE => RawError::Kind(WEBSERVICES_WINPE::WS_E_ENDPOINT_UNREACHABLE),
            WEBSERVICES_WINPE::WS_E_ENDPOINT_ACTION_NOT_SUPPORTED => RawError::Kind(WEBSERVICES_WINPE::WS_E_ENDPOINT_ACTION_NOT_SUPPORTED),
            WEBSERVICES_WINPE::WS_E_ENDPOINT_TOO_BUSY => RawError::Kind(WEBSERVICES_WINPE::WS_E_ENDPOINT_TOO_BUSY),
            WEBSERVICES_WINPE::WS_E_ENDPOINT_FAULT_RECEIVED => RawError::Kind(WEBSERVICES_WINPE::WS_E_ENDPOINT_FAULT_RECEIVED),
            WEBSERVICES_WINPE::WS_E_ENDPOINT_DISCONNECTED => RawError::Kind(WEBSERVICES_WINPE::WS_E_ENDPOINT_DISCONNECTED),
            WEBSERVICES_WINPE::WS_E_PROXY_FAILURE => RawError::Kind(WEBSERVICES_WINPE::WS_E_PROXY_FAILURE),
            WEBSERVICES_WINPE::WS_E_PROXY_ACCESS_DENIED => RawError::Kind(WEBSERVICES_WINPE::WS_E_PROXY_ACCESS_DENIED),
            WEBSERVICES_WINPE::WS_E_NOT_SUPPORTED => RawError::Kind(WEBSERVICES_WINPE::WS_E_NOT_SUPPORTED),
            WEBSERVICES_WINPE::WS_E_PROXY_REQUIRES_BASIC_AUTH => RawError::Kind(WEBSERVICES_WINPE::WS_E_PROXY_REQUIRES_BASIC_AUTH),
            WEBSERVICES_WINPE::WS_E_PROXY_REQUIRES_DIGEST_AUTH => RawError::Kind(WEBSERVICES_WINPE::WS_E_PROXY_REQUIRES_DIGEST_AUTH),
            WEBSERVICES_WINPE::WS_E_PROXY_REQUIRES_NTLM_AUTH => RawError::Kind(WEBSERVICES_WINPE::WS_E_PROXY_REQUIRES_NTLM_AUTH),
            WEBSERVICES_WINPE::WS_E_PROXY_REQUIRES_NEGOTIATE_AUTH => RawError::Kind(WEBSERVICES_WINPE::WS_E_PROXY_REQUIRES_NEGOTIATE_AUTH),
            WEBSERVICES_WINPE::WS_E_SERVER_REQUIRES_BASIC_AUTH => RawError::Kind(WEBSERVICES_WINPE::WS_E_SERVER_REQUIRES_BASIC_AUTH),
            WEBSERVICES_WINPE::WS_E_SERVER_REQUIRES_DIGEST_AUTH => RawError::Kind(WEBSERVICES_WINPE::WS_E_SERVER_REQUIRES_DIGEST_AUTH),
            WEBSERVICES_WINPE::WS_E_SERVER_REQUIRES_NTLM_AUTH => RawError::Kind(WEBSERVICES_WINPE::WS_E_SERVER_REQUIRES_NTLM_AUTH),
            WEBSERVICES_WINPE::WS_E_SERVER_REQUIRES_NEGOTIATE_AUTH => RawError::Kind(WEBSERVICES_WINPE::WS_E_SERVER_REQUIRES_NEGOTIATE_AUTH),
            WEBSERVICES_WINPE::WS_E_INVALID_ENDPOINT_URL => RawError::Kind(WEBSERVICES_WINPE::WS_E_INVALID_ENDPOINT_URL),
            WEBSERVICES_WINPE::WS_E_OTHER => RawError::Kind(WEBSERVICES_WINPE::WS_E_OTHER),
            WEBSERVICES_WINPE::WS_E_SECURITY_TOKEN_EXPIRED => RawError::Kind(WEBSERVICES_WINPE::WS_E_SECURITY_TOKEN_EXPIRED),
            WEBSERVICES_WINPE::WS_E_SECURITY_SYSTEM_FAILURE => RawError::Kind(WEBSERVICES_WINPE::WS_E_SECURITY_SYSTEM_FAILURE),
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
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

    pub fn from_name(name: &str) -> WEBSERVICES_WINPE {
        return match name {
            "WS_S_ASYNC" => WEBSERVICES_WINPE::WS_S_ASYNC,
            "WS_S_END" => WEBSERVICES_WINPE::WS_S_END,
            "WS_E_INVALID_FORMAT" => WEBSERVICES_WINPE::WS_E_INVALID_FORMAT,
            "WS_E_OBJECT_FAULTED" => WEBSERVICES_WINPE::WS_E_OBJECT_FAULTED,
            "WS_E_NUMERIC_OVERFLOW" => WEBSERVICES_WINPE::WS_E_NUMERIC_OVERFLOW,
            "WS_E_INVALID_OPERATION" => WEBSERVICES_WINPE::WS_E_INVALID_OPERATION,
            "WS_E_OPERATION_ABORTED" => WEBSERVICES_WINPE::WS_E_OPERATION_ABORTED,
            "WS_E_ENDPOINT_ACCESS_DENIED" => WEBSERVICES_WINPE::WS_E_ENDPOINT_ACCESS_DENIED,
            "WS_E_OPERATION_TIMED_OUT" => WEBSERVICES_WINPE::WS_E_OPERATION_TIMED_OUT,
            "WS_E_OPERATION_ABANDONED" => WEBSERVICES_WINPE::WS_E_OPERATION_ABANDONED,
            "WS_E_QUOTA_EXCEEDED" => WEBSERVICES_WINPE::WS_E_QUOTA_EXCEEDED,
            "WS_E_NO_TRANSLATION_AVAILABLE" => WEBSERVICES_WINPE::WS_E_NO_TRANSLATION_AVAILABLE,
            "WS_E_SECURITY_VERIFICATION_FAILURE" => WEBSERVICES_WINPE::WS_E_SECURITY_VERIFICATION_FAILURE,
            "WS_E_ADDRESS_IN_USE" => WEBSERVICES_WINPE::WS_E_ADDRESS_IN_USE,
            "WS_E_ADDRESS_NOT_AVAILABLE" => WEBSERVICES_WINPE::WS_E_ADDRESS_NOT_AVAILABLE,
            "WS_E_ENDPOINT_NOT_FOUND" => WEBSERVICES_WINPE::WS_E_ENDPOINT_NOT_FOUND,
            "WS_E_ENDPOINT_NOT_AVAILABLE" => WEBSERVICES_WINPE::WS_E_ENDPOINT_NOT_AVAILABLE,
            "WS_E_ENDPOINT_FAILURE" => WEBSERVICES_WINPE::WS_E_ENDPOINT_FAILURE,
            "WS_E_ENDPOINT_UNREACHABLE" => WEBSERVICES_WINPE::WS_E_ENDPOINT_UNREACHABLE,
            "WS_E_ENDPOINT_ACTION_NOT_SUPPORTED" => WEBSERVICES_WINPE::WS_E_ENDPOINT_ACTION_NOT_SUPPORTED,
            "WS_E_ENDPOINT_TOO_BUSY" => WEBSERVICES_WINPE::WS_E_ENDPOINT_TOO_BUSY,
            "WS_E_ENDPOINT_FAULT_RECEIVED" => WEBSERVICES_WINPE::WS_E_ENDPOINT_FAULT_RECEIVED,
            "WS_E_ENDPOINT_DISCONNECTED" => WEBSERVICES_WINPE::WS_E_ENDPOINT_DISCONNECTED,
            "WS_E_PROXY_FAILURE" => WEBSERVICES_WINPE::WS_E_PROXY_FAILURE,
            "WS_E_PROXY_ACCESS_DENIED" => WEBSERVICES_WINPE::WS_E_PROXY_ACCESS_DENIED,
            "WS_E_NOT_SUPPORTED" => WEBSERVICES_WINPE::WS_E_NOT_SUPPORTED,
            "WS_E_PROXY_REQUIRES_BASIC_AUTH" => WEBSERVICES_WINPE::WS_E_PROXY_REQUIRES_BASIC_AUTH,
            "WS_E_PROXY_REQUIRES_DIGEST_AUTH" => WEBSERVICES_WINPE::WS_E_PROXY_REQUIRES_DIGEST_AUTH,
            "WS_E_PROXY_REQUIRES_NTLM_AUTH" => WEBSERVICES_WINPE::WS_E_PROXY_REQUIRES_NTLM_AUTH,
            "WS_E_PROXY_REQUIRES_NEGOTIATE_AUTH" => WEBSERVICES_WINPE::WS_E_PROXY_REQUIRES_NEGOTIATE_AUTH,
            "WS_E_SERVER_REQUIRES_BASIC_AUTH" => WEBSERVICES_WINPE::WS_E_SERVER_REQUIRES_BASIC_AUTH,
            "WS_E_SERVER_REQUIRES_DIGEST_AUTH" => WEBSERVICES_WINPE::WS_E_SERVER_REQUIRES_DIGEST_AUTH,
            "WS_E_SERVER_REQUIRES_NTLM_AUTH" => WEBSERVICES_WINPE::WS_E_SERVER_REQUIRES_NTLM_AUTH,
            "WS_E_SERVER_REQUIRES_NEGOTIATE_AUTH" => WEBSERVICES_WINPE::WS_E_SERVER_REQUIRES_NEGOTIATE_AUTH,
            "WS_E_INVALID_ENDPOINT_URL" => WEBSERVICES_WINPE::WS_E_INVALID_ENDPOINT_URL,
            "WS_E_OTHER" => WEBSERVICES_WINPE::WS_E_OTHER,
            "WS_E_SECURITY_TOKEN_EXPIRED" => WEBSERVICES_WINPE::WS_E_SECURITY_TOKEN_EXPIRED,
            "WS_E_SECURITY_SYSTEM_FAILURE" => WEBSERVICES_WINPE::WS_E_SECURITY_SYSTEM_FAILURE,
        }
    }
    pub fn from_code(code: u32) -> WEBSERVICES_WINPE {
        return match code {
            0x003D0000 => WEBSERVICES_WINPE::WS_S_ASYNC,
            0x003D0001 => WEBSERVICES_WINPE::WS_S_END,
            0x803D0000 => WEBSERVICES_WINPE::WS_E_INVALID_FORMAT,
            0x803D0001 => WEBSERVICES_WINPE::WS_E_OBJECT_FAULTED,
            0x803D0002 => WEBSERVICES_WINPE::WS_E_NUMERIC_OVERFLOW,
            0x803D0003 => WEBSERVICES_WINPE::WS_E_INVALID_OPERATION,
            0x803D0004 => WEBSERVICES_WINPE::WS_E_OPERATION_ABORTED,
            0x803D0005 => WEBSERVICES_WINPE::WS_E_ENDPOINT_ACCESS_DENIED,
            0x803D0006 => WEBSERVICES_WINPE::WS_E_OPERATION_TIMED_OUT,
            0x803D0007 => WEBSERVICES_WINPE::WS_E_OPERATION_ABANDONED,
            0x803D0008 => WEBSERVICES_WINPE::WS_E_QUOTA_EXCEEDED,
            0x803D0009 => WEBSERVICES_WINPE::WS_E_NO_TRANSLATION_AVAILABLE,
            0x803D000A => WEBSERVICES_WINPE::WS_E_SECURITY_VERIFICATION_FAILURE,
            0x803D000B => WEBSERVICES_WINPE::WS_E_ADDRESS_IN_USE,
            0x803D000C => WEBSERVICES_WINPE::WS_E_ADDRESS_NOT_AVAILABLE,
            0x803D000D => WEBSERVICES_WINPE::WS_E_ENDPOINT_NOT_FOUND,
            0x803D000E => WEBSERVICES_WINPE::WS_E_ENDPOINT_NOT_AVAILABLE,
            0x803D000F => WEBSERVICES_WINPE::WS_E_ENDPOINT_FAILURE,
            0x803D0010 => WEBSERVICES_WINPE::WS_E_ENDPOINT_UNREACHABLE,
            0x803D0011 => WEBSERVICES_WINPE::WS_E_ENDPOINT_ACTION_NOT_SUPPORTED,
            0x803D0012 => WEBSERVICES_WINPE::WS_E_ENDPOINT_TOO_BUSY,
            0x803D0013 => WEBSERVICES_WINPE::WS_E_ENDPOINT_FAULT_RECEIVED,
            0x803D0014 => WEBSERVICES_WINPE::WS_E_ENDPOINT_DISCONNECTED,
            0x803D0015 => WEBSERVICES_WINPE::WS_E_PROXY_FAILURE,
            0x803D0016 => WEBSERVICES_WINPE::WS_E_PROXY_ACCESS_DENIED,
            0x803D0017 => WEBSERVICES_WINPE::WS_E_NOT_SUPPORTED,
            0x803D0018 => WEBSERVICES_WINPE::WS_E_PROXY_REQUIRES_BASIC_AUTH,
            0x803D0019 => WEBSERVICES_WINPE::WS_E_PROXY_REQUIRES_DIGEST_AUTH,
            0x803D001A => WEBSERVICES_WINPE::WS_E_PROXY_REQUIRES_NTLM_AUTH,
            0x803D001B => WEBSERVICES_WINPE::WS_E_PROXY_REQUIRES_NEGOTIATE_AUTH,
            0x803D001C => WEBSERVICES_WINPE::WS_E_SERVER_REQUIRES_BASIC_AUTH,
            0x803D001D => WEBSERVICES_WINPE::WS_E_SERVER_REQUIRES_DIGEST_AUTH,
            0x803D001E => WEBSERVICES_WINPE::WS_E_SERVER_REQUIRES_NTLM_AUTH,
            0x803D001F => WEBSERVICES_WINPE::WS_E_SERVER_REQUIRES_NEGOTIATE_AUTH,
            0x803D0020 => WEBSERVICES_WINPE::WS_E_INVALID_ENDPOINT_URL,
            0x803D0021 => WEBSERVICES_WINPE::WS_E_OTHER,
            0x803D0022 => WEBSERVICES_WINPE::WS_E_SECURITY_TOKEN_EXPIRED,
            0x803D0023 => WEBSERVICES_WINPE::WS_E_SECURITY_SYSTEM_FAILURE,
        }
    }
}
