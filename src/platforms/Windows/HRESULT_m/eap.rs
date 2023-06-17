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
pub enum EAP {
    EAP_I_USER_ACCOUNT_OTHER_ERROR,
    EAP_E_CERT_STORE_INACCESSIBLE,
    EAP_E_EAPHOST_METHOD_NOT_INSTALLED,
    EAP_E_EAPHOST_EAPQEC_INACCESSIBLE,
    EAP_E_EAPHOST_IDENTITY_UNKNOWN,
    EAP_E_AUTHENTICATION_FAILED,
    EAP_I_EAPHOST_EAP_NEGOTIATION_FAILED,
    EAP_E_EAPHOST_METHOD_INVALID_PACKET,
    EAP_E_EAPHOST_REMOTE_INVALID_PACKET,
    EAP_E_EAPHOST_XML_MALFORMED,
    EAP_E_METHOD_CONFIG_DOES_NOT_SUPPORT_SSO,
    EAP_E_EAPHOST_METHOD_OPERATION_NOT_SUPPORTED,
    EAP_E_USER_CERT_NOT_FOUND,
    EAP_E_USER_CERT_INVALID,
    EAP_E_USER_CERT_EXPIRED,
    EAP_E_USER_CERT_REVOKED,
    EAP_E_USER_CERT_OTHER_ERROR,
    EAP_E_USER_CERT_REJECTED,
    EAP_E_USER_CREDENTIALS_REJECTED,
    EAP_E_USER_NAME_PASSWORD_REJECTED,
    EAP_E_NO_SMART_CARD_READER,
    EAP_E_SERVER_CERT_INVALID,
    EAP_E_SERVER_CERT_EXPIRED,
    EAP_E_SERVER_CERT_REVOKED,
    EAP_E_SERVER_CERT_OTHER_ERROR,
    EAP_E_USER_ROOT_CERT_NOT_FOUND,
    EAP_E_USER_ROOT_CERT_INVALID,
    EAP_E_USER_ROOT_CERT_EXPIRED,
    EAP_E_SERVER_ROOT_CERT_NOT_FOUND,
}

impl EAP {
    pub fn code(&self) -> u32 {
        return match self {
            EAP::EAP_I_USER_ACCOUNT_OTHER_ERROR => 0x40420110 as u32,
            EAP::EAP_E_CERT_STORE_INACCESSIBLE => 0x80420010 as u32,
            EAP::EAP_E_EAPHOST_METHOD_NOT_INSTALLED => 0x80420011 as u32,
            EAP::EAP_E_EAPHOST_EAPQEC_INACCESSIBLE => 0x80420013 as u32,
            EAP::EAP_E_EAPHOST_IDENTITY_UNKNOWN => 0x80420014 as u32,
            EAP::EAP_E_AUTHENTICATION_FAILED => 0x80420015 as u32,
            EAP::EAP_I_EAPHOST_EAP_NEGOTIATION_FAILED => 0x80420016 as u32,
            EAP::EAP_E_EAPHOST_METHOD_INVALID_PACKET => 0x80420017 as u32,
            EAP::EAP_E_EAPHOST_REMOTE_INVALID_PACKET => 0x80420018 as u32,
            EAP::EAP_E_EAPHOST_XML_MALFORMED => 0x80420019 as u32,
            EAP::EAP_E_METHOD_CONFIG_DOES_NOT_SUPPORT_SSO => 0x8042001A as u32,
            EAP::EAP_E_EAPHOST_METHOD_OPERATION_NOT_SUPPORTED => 0x80420020 as u32,
            EAP::EAP_E_USER_CERT_NOT_FOUND => 0x80420100 as u32,
            EAP::EAP_E_USER_CERT_INVALID => 0x80420101 as u32,
            EAP::EAP_E_USER_CERT_EXPIRED => 0x80420102 as u32,
            EAP::EAP_E_USER_CERT_REVOKED => 0x80420103 as u32,
            EAP::EAP_E_USER_CERT_OTHER_ERROR => 0x80420104 as u32,
            EAP::EAP_E_USER_CERT_REJECTED => 0x80420105 as u32,
            EAP::EAP_E_USER_CREDENTIALS_REJECTED => 0x80420111 as u32,
            EAP::EAP_E_USER_NAME_PASSWORD_REJECTED => 0x80420112 as u32,
            EAP::EAP_E_NO_SMART_CARD_READER => 0x80420113 as u32,
            EAP::EAP_E_SERVER_CERT_INVALID => 0x80420201 as u32,
            EAP::EAP_E_SERVER_CERT_EXPIRED => 0x80420202 as u32,
            EAP::EAP_E_SERVER_CERT_REVOKED => 0x80420203 as u32,
            EAP::EAP_E_SERVER_CERT_OTHER_ERROR => 0x80420204 as u32,
            EAP::EAP_E_USER_ROOT_CERT_NOT_FOUND => 0x80420300 as u32,
            EAP::EAP_E_USER_ROOT_CERT_INVALID => 0x80420301 as u32,
            EAP::EAP_E_USER_ROOT_CERT_EXPIRED => 0x80420302 as u32,
            EAP::EAP_E_SERVER_ROOT_CERT_NOT_FOUND => 0x80420400 as u32,
        }
    }

    pub fn error(&self) -> RawError {
        return match self {
            EAP::EAP_I_USER_ACCOUNT_OTHER_ERROR => RawError::Kind(EAP::EAP_I_USER_ACCOUNT_OTHER_ERROR),
            EAP::EAP_E_CERT_STORE_INACCESSIBLE => RawError::Kind(EAP::EAP_E_CERT_STORE_INACCESSIBLE),
            EAP::EAP_E_EAPHOST_METHOD_NOT_INSTALLED => RawError::Kind(EAP::EAP_E_EAPHOST_METHOD_NOT_INSTALLED),
            EAP::EAP_E_EAPHOST_EAPQEC_INACCESSIBLE => RawError::Kind(EAP::EAP_E_EAPHOST_EAPQEC_INACCESSIBLE),
            EAP::EAP_E_EAPHOST_IDENTITY_UNKNOWN => RawError::Kind(EAP::EAP_E_EAPHOST_IDENTITY_UNKNOWN),
            EAP::EAP_E_AUTHENTICATION_FAILED => RawError::Kind(EAP::EAP_E_AUTHENTICATION_FAILED),
            EAP::EAP_I_EAPHOST_EAP_NEGOTIATION_FAILED => RawError::Kind(EAP::EAP_I_EAPHOST_EAP_NEGOTIATION_FAILED),
            EAP::EAP_E_EAPHOST_METHOD_INVALID_PACKET => RawError::Kind(EAP::EAP_E_EAPHOST_METHOD_INVALID_PACKET),
            EAP::EAP_E_EAPHOST_REMOTE_INVALID_PACKET => RawError::Kind(EAP::EAP_E_EAPHOST_REMOTE_INVALID_PACKET),
            EAP::EAP_E_EAPHOST_XML_MALFORMED => RawError::Kind(EAP::EAP_E_EAPHOST_XML_MALFORMED),
            EAP::EAP_E_METHOD_CONFIG_DOES_NOT_SUPPORT_SSO => RawError::Kind(EAP::EAP_E_METHOD_CONFIG_DOES_NOT_SUPPORT_SSO),
            EAP::EAP_E_EAPHOST_METHOD_OPERATION_NOT_SUPPORTED => RawError::Kind(EAP::EAP_E_EAPHOST_METHOD_OPERATION_NOT_SUPPORTED),
            EAP::EAP_E_USER_CERT_NOT_FOUND => RawError::Kind(EAP::EAP_E_USER_CERT_NOT_FOUND),
            EAP::EAP_E_USER_CERT_INVALID => RawError::Kind(EAP::EAP_E_USER_CERT_INVALID),
            EAP::EAP_E_USER_CERT_EXPIRED => RawError::Kind(EAP::EAP_E_USER_CERT_EXPIRED),
            EAP::EAP_E_USER_CERT_REVOKED => RawError::Kind(EAP::EAP_E_USER_CERT_REVOKED),
            EAP::EAP_E_USER_CERT_OTHER_ERROR => RawError::Kind(EAP::EAP_E_USER_CERT_OTHER_ERROR),
            EAP::EAP_E_USER_CERT_REJECTED => RawError::Kind(EAP::EAP_E_USER_CERT_REJECTED),
            EAP::EAP_E_USER_CREDENTIALS_REJECTED => RawError::Kind(EAP::EAP_E_USER_CREDENTIALS_REJECTED),
            EAP::EAP_E_USER_NAME_PASSWORD_REJECTED => RawError::Kind(EAP::EAP_E_USER_NAME_PASSWORD_REJECTED),
            EAP::EAP_E_NO_SMART_CARD_READER => RawError::Kind(EAP::EAP_E_NO_SMART_CARD_READER),
            EAP::EAP_E_SERVER_CERT_INVALID => RawError::Kind(EAP::EAP_E_SERVER_CERT_INVALID),
            EAP::EAP_E_SERVER_CERT_EXPIRED => RawError::Kind(EAP::EAP_E_SERVER_CERT_EXPIRED),
            EAP::EAP_E_SERVER_CERT_REVOKED => RawError::Kind(EAP::EAP_E_SERVER_CERT_REVOKED),
            EAP::EAP_E_SERVER_CERT_OTHER_ERROR => RawError::Kind(EAP::EAP_E_SERVER_CERT_OTHER_ERROR),
            EAP::EAP_E_USER_ROOT_CERT_NOT_FOUND => RawError::Kind(EAP::EAP_E_USER_ROOT_CERT_NOT_FOUND),
            EAP::EAP_E_USER_ROOT_CERT_INVALID => RawError::Kind(EAP::EAP_E_USER_ROOT_CERT_INVALID),
            EAP::EAP_E_USER_ROOT_CERT_EXPIRED => RawError::Kind(EAP::EAP_E_USER_ROOT_CERT_EXPIRED),
            EAP::EAP_E_SERVER_ROOT_CERT_NOT_FOUND => RawError::Kind(EAP::EAP_E_SERVER_ROOT_CERT_NOT_FOUND),
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            EAP::EAP_I_USER_ACCOUNT_OTHER_ERROR => "The EAPHost received EAP failure after the identity exchange. There is likely a problem with the authenticating user's account.",
            EAP::EAP_E_CERT_STORE_INACCESSIBLE => "The certificate store can't be accessed on either the authenticator or the peer.",
            EAP::EAP_E_EAPHOST_METHOD_NOT_INSTALLED => "The requested EAP method is not installed.",
            EAP::EAP_E_EAPHOST_EAPQEC_INACCESSIBLE => "The EAPHost is not able to communicate with the EAP quarantine enforcement client (QEC) on a client with Network Access Protection (NAP) enabled.",
            EAP::EAP_E_EAPHOST_IDENTITY_UNKNOWN => "The EAPHost returns this error if the authenticator fails the authentication after the peer sent its identity.",
            EAP::EAP_E_AUTHENTICATION_FAILED => "The EAPHost returns this error on authentication failure.",
            EAP::EAP_I_EAPHOST_EAP_NEGOTIATION_FAILED => "The EAPHost returns this error when the client and the server aren't configured with compatible EAP types.",
            EAP::EAP_E_EAPHOST_METHOD_INVALID_PACKET => "The EAPMethod received an EAP packet that cannot be processed.",
            EAP::EAP_E_EAPHOST_REMOTE_INVALID_PACKET => "The EAPHost received a packet that cannot be processed.",
            EAP::EAP_E_EAPHOST_XML_MALFORMED => "The EAPHost configuration schema validation failed.",
            EAP::EAP_E_METHOD_CONFIG_DOES_NOT_SUPPORT_SSO => "The EAP method does not support single signon for the provided configuration.",
            EAP::EAP_E_EAPHOST_METHOD_OPERATION_NOT_SUPPORTED => "The EAPHost returns this error when a configured EAP method does not support a requested operation (procedure call).",
            EAP::EAP_E_USER_CERT_NOT_FOUND => "The EAPHost could not find the user certificate for authentication.",
            EAP::EAP_E_USER_CERT_INVALID => "The user certificate being used for authentication does not have a proper extended key usage (EKU) set.",
            EAP::EAP_E_USER_CERT_EXPIRED => "The EAPhost found a user certificate which has expired.",
            EAP::EAP_E_USER_CERT_REVOKED => "The user certificate being used for authentication has been revoked.",
            EAP::EAP_E_USER_CERT_OTHER_ERROR => "An unknown error occurred with the user certificate being used for authentication.",
            EAP::EAP_E_USER_CERT_REJECTED => "The authenticator rejected the user certificate being used for authentication.",
            EAP::EAP_E_USER_CREDENTIALS_REJECTED => "The authenticator rejected the user credentials for authentication.",
            EAP::EAP_E_USER_NAME_PASSWORD_REJECTED => "The authenticator rejected the user credentials for authentication.",
            EAP::EAP_E_NO_SMART_CARD_READER => "No smart card reader was present.",
            EAP::EAP_E_SERVER_CERT_INVALID => "The server certificate being user for authentication does not have a proper EKU set .",
            EAP::EAP_E_SERVER_CERT_EXPIRED => "The EAPhost found a server certificate which has expired.",
            EAP::EAP_E_SERVER_CERT_REVOKED => "The server certificate being used for authentication has been revoked.",
            EAP::EAP_E_SERVER_CERT_OTHER_ERROR => "An unknown error occurred with the server certificate being used for authentication.",
            EAP::EAP_E_USER_ROOT_CERT_NOT_FOUND => "The EAPHost could not find a certificate in trusted root certificate store for user certificate validation.",
            EAP::EAP_E_USER_ROOT_CERT_INVALID => "The authentication failed because the root certificate used for this network is not valid.",
            EAP::EAP_E_USER_ROOT_CERT_EXPIRED => "The trusted root certificate needed for user certificate validation has expired.",
            EAP::EAP_E_SERVER_ROOT_CERT_NOT_FOUND => "The EAPHost could not find a root certificate in the trusted root certificate store for server certificate velidation.",
        }
    }

    pub fn from_name(name: &str) -> EAP {
        return match name {
            "EAP_I_USER_ACCOUNT_OTHER_ERROR" => EAP::EAP_I_USER_ACCOUNT_OTHER_ERROR,
            "EAP_E_CERT_STORE_INACCESSIBLE" => EAP::EAP_E_CERT_STORE_INACCESSIBLE,
            "EAP_E_EAPHOST_METHOD_NOT_INSTALLED" => EAP::EAP_E_EAPHOST_METHOD_NOT_INSTALLED,
            "EAP_E_EAPHOST_EAPQEC_INACCESSIBLE" => EAP::EAP_E_EAPHOST_EAPQEC_INACCESSIBLE,
            "EAP_E_EAPHOST_IDENTITY_UNKNOWN" => EAP::EAP_E_EAPHOST_IDENTITY_UNKNOWN,
            "EAP_E_AUTHENTICATION_FAILED" => EAP::EAP_E_AUTHENTICATION_FAILED,
            "EAP_I_EAPHOST_EAP_NEGOTIATION_FAILED" => EAP::EAP_I_EAPHOST_EAP_NEGOTIATION_FAILED,
            "EAP_E_EAPHOST_METHOD_INVALID_PACKET" => EAP::EAP_E_EAPHOST_METHOD_INVALID_PACKET,
            "EAP_E_EAPHOST_REMOTE_INVALID_PACKET" => EAP::EAP_E_EAPHOST_REMOTE_INVALID_PACKET,
            "EAP_E_EAPHOST_XML_MALFORMED" => EAP::EAP_E_EAPHOST_XML_MALFORMED,
            "EAP_E_METHOD_CONFIG_DOES_NOT_SUPPORT_SSO" => EAP::EAP_E_METHOD_CONFIG_DOES_NOT_SUPPORT_SSO,
            "EAP_E_EAPHOST_METHOD_OPERATION_NOT_SUPPORTED" => EAP::EAP_E_EAPHOST_METHOD_OPERATION_NOT_SUPPORTED,
            "EAP_E_USER_CERT_NOT_FOUND" => EAP::EAP_E_USER_CERT_NOT_FOUND,
            "EAP_E_USER_CERT_INVALID" => EAP::EAP_E_USER_CERT_INVALID,
            "EAP_E_USER_CERT_EXPIRED" => EAP::EAP_E_USER_CERT_EXPIRED,
            "EAP_E_USER_CERT_REVOKED" => EAP::EAP_E_USER_CERT_REVOKED,
            "EAP_E_USER_CERT_OTHER_ERROR" => EAP::EAP_E_USER_CERT_OTHER_ERROR,
            "EAP_E_USER_CERT_REJECTED" => EAP::EAP_E_USER_CERT_REJECTED,
            "EAP_E_USER_CREDENTIALS_REJECTED" => EAP::EAP_E_USER_CREDENTIALS_REJECTED,
            "EAP_E_USER_NAME_PASSWORD_REJECTED" => EAP::EAP_E_USER_NAME_PASSWORD_REJECTED,
            "EAP_E_NO_SMART_CARD_READER" => EAP::EAP_E_NO_SMART_CARD_READER,
            "EAP_E_SERVER_CERT_INVALID" => EAP::EAP_E_SERVER_CERT_INVALID,
            "EAP_E_SERVER_CERT_EXPIRED" => EAP::EAP_E_SERVER_CERT_EXPIRED,
            "EAP_E_SERVER_CERT_REVOKED" => EAP::EAP_E_SERVER_CERT_REVOKED,
            "EAP_E_SERVER_CERT_OTHER_ERROR" => EAP::EAP_E_SERVER_CERT_OTHER_ERROR,
            "EAP_E_USER_ROOT_CERT_NOT_FOUND" => EAP::EAP_E_USER_ROOT_CERT_NOT_FOUND,
            "EAP_E_USER_ROOT_CERT_INVALID" => EAP::EAP_E_USER_ROOT_CERT_INVALID,
            "EAP_E_USER_ROOT_CERT_EXPIRED" => EAP::EAP_E_USER_ROOT_CERT_EXPIRED,
            "EAP_E_SERVER_ROOT_CERT_NOT_FOUND" => EAP::EAP_E_SERVER_ROOT_CERT_NOT_FOUND,
        }
    }
    pub fn from_code(code: u32) -> EAP {
        return match code {
            0x40420110 => EAP::EAP_I_USER_ACCOUNT_OTHER_ERROR,
            0x80420010 => EAP::EAP_E_CERT_STORE_INACCESSIBLE,
            0x80420011 => EAP::EAP_E_EAPHOST_METHOD_NOT_INSTALLED,
            0x80420013 => EAP::EAP_E_EAPHOST_EAPQEC_INACCESSIBLE,
            0x80420014 => EAP::EAP_E_EAPHOST_IDENTITY_UNKNOWN,
            0x80420015 => EAP::EAP_E_AUTHENTICATION_FAILED,
            0x80420016 => EAP::EAP_I_EAPHOST_EAP_NEGOTIATION_FAILED,
            0x80420017 => EAP::EAP_E_EAPHOST_METHOD_INVALID_PACKET,
            0x80420018 => EAP::EAP_E_EAPHOST_REMOTE_INVALID_PACKET,
            0x80420019 => EAP::EAP_E_EAPHOST_XML_MALFORMED,
            0x8042001A => EAP::EAP_E_METHOD_CONFIG_DOES_NOT_SUPPORT_SSO,
            0x80420020 => EAP::EAP_E_EAPHOST_METHOD_OPERATION_NOT_SUPPORTED,
            0x80420100 => EAP::EAP_E_USER_CERT_NOT_FOUND,
            0x80420101 => EAP::EAP_E_USER_CERT_INVALID,
            0x80420102 => EAP::EAP_E_USER_CERT_EXPIRED,
            0x80420103 => EAP::EAP_E_USER_CERT_REVOKED,
            0x80420104 => EAP::EAP_E_USER_CERT_OTHER_ERROR,
            0x80420105 => EAP::EAP_E_USER_CERT_REJECTED,
            0x80420111 => EAP::EAP_E_USER_CREDENTIALS_REJECTED,
            0x80420112 => EAP::EAP_E_USER_NAME_PASSWORD_REJECTED,
            0x80420113 => EAP::EAP_E_NO_SMART_CARD_READER,
            0x80420201 => EAP::EAP_E_SERVER_CERT_INVALID,
            0x80420202 => EAP::EAP_E_SERVER_CERT_EXPIRED,
            0x80420203 => EAP::EAP_E_SERVER_CERT_REVOKED,
            0x80420204 => EAP::EAP_E_SERVER_CERT_OTHER_ERROR,
            0x80420300 => EAP::EAP_E_USER_ROOT_CERT_NOT_FOUND,
            0x80420301 => EAP::EAP_E_USER_ROOT_CERT_INVALID,
            0x80420302 => EAP::EAP_E_USER_ROOT_CERT_EXPIRED,
            0x80420400 => EAP::EAP_E_SERVER_ROOT_CERT_NOT_FOUND,
        }
    }
}
