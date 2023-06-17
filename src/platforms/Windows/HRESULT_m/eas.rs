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
pub enum EAS {
    EAS_E_POLICY_NOT_MANAGED_BY_OS,
    EAS_E_POLICY_COMPLIANT_WITH_ACTIONS,
    EAS_E_REQUESTED_POLICY_NOT_ENFORCEABLE,
    EAS_E_CURRENT_USER_HAS_BLANK_PASSWORD,
    EAS_E_REQUESTED_POLICY_PASSWORD_EXPIRATION_INCOMPATIBLE,
    EAS_E_USER_CANNOT_CHANGE_PASSWORD,
    EAS_E_ADMINS_HAVE_BLANK_PASSWORD,
    EAS_E_ADMINS_CANNOT_CHANGE_PASSWORD,
    EAS_E_LOCAL_CONTROLLED_USERS_CANNOT_CHANGE_PASSWORD,
    EAS_E_PASSWORD_POLICY_NOT_ENFORCEABLE_FOR_CONNECTED_ADMINS,
    EAS_E_CONNECTED_ADMINS_NEED_TO_CHANGE_PASSWORD,
    EAS_E_PASSWORD_POLICY_NOT_ENFORCEABLE_FOR_CURRENT_CONNECTED_USER,
    EAS_E_CURRENT_CONNECTED_USER_NEED_TO_CHANGE_PASSWORD,
}

impl EAS {
    pub fn code(&self) -> u32 {
        return match self {
            EAS::EAS_E_POLICY_NOT_MANAGED_BY_OS => 0x80550001 as u32,
            EAS::EAS_E_POLICY_COMPLIANT_WITH_ACTIONS => 0x80550002 as u32,
            EAS::EAS_E_REQUESTED_POLICY_NOT_ENFORCEABLE => 0x80550003 as u32,
            EAS::EAS_E_CURRENT_USER_HAS_BLANK_PASSWORD => 0x80550004 as u32,
            EAS::EAS_E_REQUESTED_POLICY_PASSWORD_EXPIRATION_INCOMPATIBLE => 0x80550005 as u32,
            EAS::EAS_E_USER_CANNOT_CHANGE_PASSWORD => 0x80550006 as u32,
            EAS::EAS_E_ADMINS_HAVE_BLANK_PASSWORD => 0x80550007 as u32,
            EAS::EAS_E_ADMINS_CANNOT_CHANGE_PASSWORD => 0x80550008 as u32,
            EAS::EAS_E_LOCAL_CONTROLLED_USERS_CANNOT_CHANGE_PASSWORD => 0x80550009 as u32,
            EAS::EAS_E_PASSWORD_POLICY_NOT_ENFORCEABLE_FOR_CONNECTED_ADMINS => 0x8055000A as u32,
            EAS::EAS_E_CONNECTED_ADMINS_NEED_TO_CHANGE_PASSWORD => 0x8055000B as u32,
            EAS::EAS_E_PASSWORD_POLICY_NOT_ENFORCEABLE_FOR_CURRENT_CONNECTED_USER => 0x8055000C as u32,
            EAS::EAS_E_CURRENT_CONNECTED_USER_NEED_TO_CHANGE_PASSWORD => 0x8055000D as u32,
        }
    }

    pub fn error(&self) -> RawError {
        return match self {
            EAS::EAS_E_POLICY_NOT_MANAGED_BY_OS => RawError::Kind(EAS::EAS_E_POLICY_NOT_MANAGED_BY_OS),
            EAS::EAS_E_POLICY_COMPLIANT_WITH_ACTIONS => RawError::Kind(EAS::EAS_E_POLICY_COMPLIANT_WITH_ACTIONS),
            EAS::EAS_E_REQUESTED_POLICY_NOT_ENFORCEABLE => RawError::Kind(EAS::EAS_E_REQUESTED_POLICY_NOT_ENFORCEABLE),
            EAS::EAS_E_CURRENT_USER_HAS_BLANK_PASSWORD => RawError::Kind(EAS::EAS_E_CURRENT_USER_HAS_BLANK_PASSWORD),
            EAS::EAS_E_REQUESTED_POLICY_PASSWORD_EXPIRATION_INCOMPATIBLE => RawError::Kind(EAS::EAS_E_REQUESTED_POLICY_PASSWORD_EXPIRATION_INCOMPATIBLE),
            EAS::EAS_E_USER_CANNOT_CHANGE_PASSWORD => RawError::Kind(EAS::EAS_E_USER_CANNOT_CHANGE_PASSWORD),
            EAS::EAS_E_ADMINS_HAVE_BLANK_PASSWORD => RawError::Kind(EAS::EAS_E_ADMINS_HAVE_BLANK_PASSWORD),
            EAS::EAS_E_ADMINS_CANNOT_CHANGE_PASSWORD => RawError::Kind(EAS::EAS_E_ADMINS_CANNOT_CHANGE_PASSWORD),
            EAS::EAS_E_LOCAL_CONTROLLED_USERS_CANNOT_CHANGE_PASSWORD => RawError::Kind(EAS::EAS_E_LOCAL_CONTROLLED_USERS_CANNOT_CHANGE_PASSWORD),
            EAS::EAS_E_PASSWORD_POLICY_NOT_ENFORCEABLE_FOR_CONNECTED_ADMINS => RawError::Kind(EAS::EAS_E_PASSWORD_POLICY_NOT_ENFORCEABLE_FOR_CONNECTED_ADMINS),
            EAS::EAS_E_CONNECTED_ADMINS_NEED_TO_CHANGE_PASSWORD => RawError::Kind(EAS::EAS_E_CONNECTED_ADMINS_NEED_TO_CHANGE_PASSWORD),
            EAS::EAS_E_PASSWORD_POLICY_NOT_ENFORCEABLE_FOR_CURRENT_CONNECTED_USER => RawError::Kind(EAS::EAS_E_PASSWORD_POLICY_NOT_ENFORCEABLE_FOR_CURRENT_CONNECTED_USER),
            EAS::EAS_E_CURRENT_CONNECTED_USER_NEED_TO_CHANGE_PASSWORD => RawError::Kind(EAS::EAS_E_CURRENT_CONNECTED_USER_NEED_TO_CHANGE_PASSWORD),
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            EAS::EAS_E_POLICY_NOT_MANAGED_BY_OS => "Windows cannot evaluate this EAS policy since this is not managed by the operating system.",
            EAS::EAS_E_POLICY_COMPLIANT_WITH_ACTIONS => "The system can be made compliant to this EAS policy if certain actions are performed by the user.",
            EAS::EAS_E_REQUESTED_POLICY_NOT_ENFORCEABLE => "The EAS policy being evaluated cannot be enforced by the system.",
            EAS::EAS_E_CURRENT_USER_HAS_BLANK_PASSWORD => "EAS password policies for the user cannot be evaluated as the user has a blank password.",
            EAS::EAS_E_REQUESTED_POLICY_PASSWORD_EXPIRATION_INCOMPATIBLE => "EAS password expiration policy cannot be satisfied as the password expiration interval is less than the minimum password interval of the system.",
            EAS::EAS_E_USER_CANNOT_CHANGE_PASSWORD => "The user is not allowed to change her password.",
            EAS::EAS_E_ADMINS_HAVE_BLANK_PASSWORD => "EAS password policies cannot be evaluated as one or more admins have blank passwords.",
            EAS::EAS_E_ADMINS_CANNOT_CHANGE_PASSWORD => "One or more admins are not allowed to change their password.",
            EAS::EAS_E_LOCAL_CONTROLLED_USERS_CANNOT_CHANGE_PASSWORD => "There are other standard users present who are not allowed to change their password.",
            EAS::EAS_E_PASSWORD_POLICY_NOT_ENFORCEABLE_FOR_CONNECTED_ADMINS => "The EAS password policy cannot be enforced by the connected account provider of at least one administrator.",
            EAS::EAS_E_CONNECTED_ADMINS_NEED_TO_CHANGE_PASSWORD => "There is at least one administrator whose connected account password needs to be changed for EAS password policy compliance.",
            EAS::EAS_E_PASSWORD_POLICY_NOT_ENFORCEABLE_FOR_CURRENT_CONNECTED_USER => "The EAS password policy cannot be enforced by the connected account provider of the current user.",
            EAS::EAS_E_CURRENT_CONNECTED_USER_NEED_TO_CHANGE_PASSWORD => "The connected account password of the current user needs to be changed for EAS password policy compliance.",
        }
    }

    pub fn from_name(name: &str) -> EAS {
        return match name {
            "EAS_E_POLICY_NOT_MANAGED_BY_OS" => EAS::EAS_E_POLICY_NOT_MANAGED_BY_OS,
            "EAS_E_POLICY_COMPLIANT_WITH_ACTIONS" => EAS::EAS_E_POLICY_COMPLIANT_WITH_ACTIONS,
            "EAS_E_REQUESTED_POLICY_NOT_ENFORCEABLE" => EAS::EAS_E_REQUESTED_POLICY_NOT_ENFORCEABLE,
            "EAS_E_CURRENT_USER_HAS_BLANK_PASSWORD" => EAS::EAS_E_CURRENT_USER_HAS_BLANK_PASSWORD,
            "EAS_E_REQUESTED_POLICY_PASSWORD_EXPIRATION_INCOMPATIBLE" => EAS::EAS_E_REQUESTED_POLICY_PASSWORD_EXPIRATION_INCOMPATIBLE,
            "EAS_E_USER_CANNOT_CHANGE_PASSWORD" => EAS::EAS_E_USER_CANNOT_CHANGE_PASSWORD,
            "EAS_E_ADMINS_HAVE_BLANK_PASSWORD" => EAS::EAS_E_ADMINS_HAVE_BLANK_PASSWORD,
            "EAS_E_ADMINS_CANNOT_CHANGE_PASSWORD" => EAS::EAS_E_ADMINS_CANNOT_CHANGE_PASSWORD,
            "EAS_E_LOCAL_CONTROLLED_USERS_CANNOT_CHANGE_PASSWORD" => EAS::EAS_E_LOCAL_CONTROLLED_USERS_CANNOT_CHANGE_PASSWORD,
            "EAS_E_PASSWORD_POLICY_NOT_ENFORCEABLE_FOR_CONNECTED_ADMINS" => EAS::EAS_E_PASSWORD_POLICY_NOT_ENFORCEABLE_FOR_CONNECTED_ADMINS,
            "EAS_E_CONNECTED_ADMINS_NEED_TO_CHANGE_PASSWORD" => EAS::EAS_E_CONNECTED_ADMINS_NEED_TO_CHANGE_PASSWORD,
            "EAS_E_PASSWORD_POLICY_NOT_ENFORCEABLE_FOR_CURRENT_CONNECTED_USER" => EAS::EAS_E_PASSWORD_POLICY_NOT_ENFORCEABLE_FOR_CURRENT_CONNECTED_USER,
            "EAS_E_CURRENT_CONNECTED_USER_NEED_TO_CHANGE_PASSWORD" => EAS::EAS_E_CURRENT_CONNECTED_USER_NEED_TO_CHANGE_PASSWORD,
        }
    }
    pub fn from_code(code: u32) -> EAS {
        return match code {
            0x80550001 => EAS::EAS_E_POLICY_NOT_MANAGED_BY_OS,
            0x80550002 => EAS::EAS_E_POLICY_COMPLIANT_WITH_ACTIONS,
            0x80550003 => EAS::EAS_E_REQUESTED_POLICY_NOT_ENFORCEABLE,
            0x80550004 => EAS::EAS_E_CURRENT_USER_HAS_BLANK_PASSWORD,
            0x80550005 => EAS::EAS_E_REQUESTED_POLICY_PASSWORD_EXPIRATION_INCOMPATIBLE,
            0x80550006 => EAS::EAS_E_USER_CANNOT_CHANGE_PASSWORD,
            0x80550007 => EAS::EAS_E_ADMINS_HAVE_BLANK_PASSWORD,
            0x80550008 => EAS::EAS_E_ADMINS_CANNOT_CHANGE_PASSWORD,
            0x80550009 => EAS::EAS_E_LOCAL_CONTROLLED_USERS_CANNOT_CHANGE_PASSWORD,
            0x8055000A => EAS::EAS_E_PASSWORD_POLICY_NOT_ENFORCEABLE_FOR_CONNECTED_ADMINS,
            0x8055000B => EAS::EAS_E_CONNECTED_ADMINS_NEED_TO_CHANGE_PASSWORD,
            0x8055000C => EAS::EAS_E_PASSWORD_POLICY_NOT_ENFORCEABLE_FOR_CURRENT_CONNECTED_USER,
            0x8055000D => EAS::EAS_E_CURRENT_CONNECTED_USER_NEED_TO_CHANGE_PASSWORD,
        }
    }
}
