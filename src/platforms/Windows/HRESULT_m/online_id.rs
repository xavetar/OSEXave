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
pub enum ONLINE_ID {
    ONL_E_INVALID_AUTHENTICATION_TARGET,
    ONL_E_ACCESS_DENIED_BY_TOU,
    ONL_E_INVALID_APPLICATION,
    ONL_E_PASSWORD_UPDATE_REQUIRED,
    ONL_E_ACCOUNT_UPDATE_REQUIRED,
    ONL_E_FORCESIGNIN,
    ONL_E_ACCOUNT_LOCKED,
    ONL_E_PARENTAL_CONSENT_REQUIRED,
    ONL_E_EMAIL_VERIFICATION_REQUIRED,
    ONL_E_ACCOUNT_SUSPENDED_COMPROIMISE,
    ONL_E_ACCOUNT_SUSPENDED_ABUSE,
    ONL_E_ACTION_REQUIRED,
    ONL_CONNECTION_COUNT_LIMIT,
    ONL_E_CONNECTED_ACCOUNT_CAN_NOT_SIGNOUT,
    ONL_E_USER_AUTHENTICATION_REQUIRED,
    ONL_E_REQUEST_THROTTLED,
}

impl ONLINE_ID {
    pub fn code(&self) -> u32 {
        return match self {
            ONLINE_ID::ONL_E_INVALID_AUTHENTICATION_TARGET => 0x80860001 as u32,
            ONLINE_ID::ONL_E_ACCESS_DENIED_BY_TOU => 0x80860002 as u32,
            ONLINE_ID::ONL_E_INVALID_APPLICATION => 0x80860003 as u32,
            ONLINE_ID::ONL_E_PASSWORD_UPDATE_REQUIRED => 0x80860004 as u32,
            ONLINE_ID::ONL_E_ACCOUNT_UPDATE_REQUIRED => 0x80860005 as u32,
            ONLINE_ID::ONL_E_FORCESIGNIN => 0x80860006 as u32,
            ONLINE_ID::ONL_E_ACCOUNT_LOCKED => 0x80860007 as u32,
            ONLINE_ID::ONL_E_PARENTAL_CONSENT_REQUIRED => 0x80860008 as u32,
            ONLINE_ID::ONL_E_EMAIL_VERIFICATION_REQUIRED => 0x80860009 as u32,
            ONLINE_ID::ONL_E_ACCOUNT_SUSPENDED_COMPROIMISE => 0x8086000A as u32,
            ONLINE_ID::ONL_E_ACCOUNT_SUSPENDED_ABUSE => 0x8086000B as u32,
            ONLINE_ID::ONL_E_ACTION_REQUIRED => 0x8086000C as u32,
            ONLINE_ID::ONL_CONNECTION_COUNT_LIMIT => 0x8086000D as u32,
            ONLINE_ID::ONL_E_CONNECTED_ACCOUNT_CAN_NOT_SIGNOUT => 0x8086000E as u32,
            ONLINE_ID::ONL_E_USER_AUTHENTICATION_REQUIRED => 0x8086000F as u32,
            ONLINE_ID::ONL_E_REQUEST_THROTTLED => 0x80860010 as u32,
        }
    }

    pub fn error(&self) -> RawError {
        return match self {
            ONLINE_ID::ONL_E_INVALID_AUTHENTICATION_TARGET => RawError::Kind(ONLINE_ID::ONL_E_INVALID_AUTHENTICATION_TARGET),
            ONLINE_ID::ONL_E_ACCESS_DENIED_BY_TOU => RawError::Kind(ONLINE_ID::ONL_E_ACCESS_DENIED_BY_TOU),
            ONLINE_ID::ONL_E_INVALID_APPLICATION => RawError::Kind(ONLINE_ID::ONL_E_INVALID_APPLICATION),
            ONLINE_ID::ONL_E_PASSWORD_UPDATE_REQUIRED => RawError::Kind(ONLINE_ID::ONL_E_PASSWORD_UPDATE_REQUIRED),
            ONLINE_ID::ONL_E_ACCOUNT_UPDATE_REQUIRED => RawError::Kind(ONLINE_ID::ONL_E_ACCOUNT_UPDATE_REQUIRED),
            ONLINE_ID::ONL_E_FORCESIGNIN => RawError::Kind(ONLINE_ID::ONL_E_FORCESIGNIN),
            ONLINE_ID::ONL_E_ACCOUNT_LOCKED => RawError::Kind(ONLINE_ID::ONL_E_ACCOUNT_LOCKED),
            ONLINE_ID::ONL_E_PARENTAL_CONSENT_REQUIRED => RawError::Kind(ONLINE_ID::ONL_E_PARENTAL_CONSENT_REQUIRED),
            ONLINE_ID::ONL_E_EMAIL_VERIFICATION_REQUIRED => RawError::Kind(ONLINE_ID::ONL_E_EMAIL_VERIFICATION_REQUIRED),
            ONLINE_ID::ONL_E_ACCOUNT_SUSPENDED_COMPROIMISE => RawError::Kind(ONLINE_ID::ONL_E_ACCOUNT_SUSPENDED_COMPROIMISE),
            ONLINE_ID::ONL_E_ACCOUNT_SUSPENDED_ABUSE => RawError::Kind(ONLINE_ID::ONL_E_ACCOUNT_SUSPENDED_ABUSE),
            ONLINE_ID::ONL_E_ACTION_REQUIRED => RawError::Kind(ONLINE_ID::ONL_E_ACTION_REQUIRED),
            ONLINE_ID::ONL_CONNECTION_COUNT_LIMIT => RawError::Kind(ONLINE_ID::ONL_CONNECTION_COUNT_LIMIT),
            ONLINE_ID::ONL_E_CONNECTED_ACCOUNT_CAN_NOT_SIGNOUT => RawError::Kind(ONLINE_ID::ONL_E_CONNECTED_ACCOUNT_CAN_NOT_SIGNOUT),
            ONLINE_ID::ONL_E_USER_AUTHENTICATION_REQUIRED => RawError::Kind(ONLINE_ID::ONL_E_USER_AUTHENTICATION_REQUIRED),
            ONLINE_ID::ONL_E_REQUEST_THROTTLED => RawError::Kind(ONLINE_ID::ONL_E_REQUEST_THROTTLED),
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            ONLINE_ID::ONL_E_INVALID_AUTHENTICATION_TARGET => "Authentication target is invalid or not configured correctly.",
            ONLINE_ID::ONL_E_ACCESS_DENIED_BY_TOU => "Your application cannot get the Online Id properties due to the Terms of Use accepted by the user.",
            ONLINE_ID::ONL_E_INVALID_APPLICATION => "The application requesting authentication tokens is either disabled or incorrectly configured.",
            ONLINE_ID::ONL_E_PASSWORD_UPDATE_REQUIRED => "Online Id password must be updated before signin.",
            ONLINE_ID::ONL_E_ACCOUNT_UPDATE_REQUIRED => "Online Id account properties must be updated before signin.",
            ONLINE_ID::ONL_E_FORCESIGNIN => "To help protect your Online Id account you must signin again.",
            ONLINE_ID::ONL_E_ACCOUNT_LOCKED => "Online Id account was locked because there have been too many attempts to sign in.",
            ONLINE_ID::ONL_E_PARENTAL_CONSENT_REQUIRED => "Online Id account requires parental consent before proceeding.",
            ONLINE_ID::ONL_E_EMAIL_VERIFICATION_REQUIRED => "Online Id signin name is not yet verified. Email verification is required before signin.",
            ONLINE_ID::ONL_E_ACCOUNT_SUSPENDED_COMPROIMISE => "We have noticed some unusual activity in your Online Id account. Your action is needed to make sure no one else is using your account.",
            ONLINE_ID::ONL_E_ACCOUNT_SUSPENDED_ABUSE => "We detected some suspicious activity with your Online Id account. To help protect you, we've temporarily blocked your account.",
            ONLINE_ID::ONL_E_ACTION_REQUIRED => "User interaction is required for authentication.",
            ONLINE_ID::ONL_CONNECTION_COUNT_LIMIT => "User has reached the maximum device associations per user limit.",
            ONLINE_ID::ONL_E_CONNECTED_ACCOUNT_CAN_NOT_SIGNOUT => "Cannot sign out from the application since the user account is connected.",
            ONLINE_ID::ONL_E_USER_AUTHENTICATION_REQUIRED => "User authentication is required for this operation.",
            ONLINE_ID::ONL_E_REQUEST_THROTTLED => "We want to make sure this is you. User interaction is required for authentication.",
        }
    }

    pub fn from_name(name: &str) -> ONLINE_ID {
        return match name {
            "ONL_E_INVALID_AUTHENTICATION_TARGET" => ONLINE_ID::ONL_E_INVALID_AUTHENTICATION_TARGET,
            "ONL_E_ACCESS_DENIED_BY_TOU" => ONLINE_ID::ONL_E_ACCESS_DENIED_BY_TOU,
            "ONL_E_INVALID_APPLICATION" => ONLINE_ID::ONL_E_INVALID_APPLICATION,
            "ONL_E_PASSWORD_UPDATE_REQUIRED" => ONLINE_ID::ONL_E_PASSWORD_UPDATE_REQUIRED,
            "ONL_E_ACCOUNT_UPDATE_REQUIRED" => ONLINE_ID::ONL_E_ACCOUNT_UPDATE_REQUIRED,
            "ONL_E_FORCESIGNIN" => ONLINE_ID::ONL_E_FORCESIGNIN,
            "ONL_E_ACCOUNT_LOCKED" => ONLINE_ID::ONL_E_ACCOUNT_LOCKED,
            "ONL_E_PARENTAL_CONSENT_REQUIRED" => ONLINE_ID::ONL_E_PARENTAL_CONSENT_REQUIRED,
            "ONL_E_EMAIL_VERIFICATION_REQUIRED" => ONLINE_ID::ONL_E_EMAIL_VERIFICATION_REQUIRED,
            "ONL_E_ACCOUNT_SUSPENDED_COMPROIMISE" => ONLINE_ID::ONL_E_ACCOUNT_SUSPENDED_COMPROIMISE,
            "ONL_E_ACCOUNT_SUSPENDED_ABUSE" => ONLINE_ID::ONL_E_ACCOUNT_SUSPENDED_ABUSE,
            "ONL_E_ACTION_REQUIRED" => ONLINE_ID::ONL_E_ACTION_REQUIRED,
            "ONL_CONNECTION_COUNT_LIMIT" => ONLINE_ID::ONL_CONNECTION_COUNT_LIMIT,
            "ONL_E_CONNECTED_ACCOUNT_CAN_NOT_SIGNOUT" => ONLINE_ID::ONL_E_CONNECTED_ACCOUNT_CAN_NOT_SIGNOUT,
            "ONL_E_USER_AUTHENTICATION_REQUIRED" => ONLINE_ID::ONL_E_USER_AUTHENTICATION_REQUIRED,
            "ONL_E_REQUEST_THROTTLED" => ONLINE_ID::ONL_E_REQUEST_THROTTLED,
        }
    }
    pub fn from_code(code: u32) -> ONLINE_ID {
        return match code {
            0x80860001 => ONLINE_ID::ONL_E_INVALID_AUTHENTICATION_TARGET,
            0x80860002 => ONLINE_ID::ONL_E_ACCESS_DENIED_BY_TOU,
            0x80860003 => ONLINE_ID::ONL_E_INVALID_APPLICATION,
            0x80860004 => ONLINE_ID::ONL_E_PASSWORD_UPDATE_REQUIRED,
            0x80860005 => ONLINE_ID::ONL_E_ACCOUNT_UPDATE_REQUIRED,
            0x80860006 => ONLINE_ID::ONL_E_FORCESIGNIN,
            0x80860007 => ONLINE_ID::ONL_E_ACCOUNT_LOCKED,
            0x80860008 => ONLINE_ID::ONL_E_PARENTAL_CONSENT_REQUIRED,
            0x80860009 => ONLINE_ID::ONL_E_EMAIL_VERIFICATION_REQUIRED,
            0x8086000A => ONLINE_ID::ONL_E_ACCOUNT_SUSPENDED_COMPROIMISE,
            0x8086000B => ONLINE_ID::ONL_E_ACCOUNT_SUSPENDED_ABUSE,
            0x8086000C => ONLINE_ID::ONL_E_ACTION_REQUIRED,
            0x8086000D => ONLINE_ID::ONL_CONNECTION_COUNT_LIMIT,
            0x8086000E => ONLINE_ID::ONL_E_CONNECTED_ACCOUNT_CAN_NOT_SIGNOUT,
            0x8086000F => ONLINE_ID::ONL_E_USER_AUTHENTICATION_REQUIRED,
            0x80860010 => ONLINE_ID::ONL_E_REQUEST_THROTTLED,
        }
    }
}
