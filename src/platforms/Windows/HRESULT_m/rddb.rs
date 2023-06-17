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
pub enum RDDB {
    E_RDDB_LOGIN_FAILED,
    E_RDDB_DATA_INTEGRITY_VIOLATION,
    E_RDDB_FATAL_ERR,
    E_RDDB_DB_OFFLINE,
    E_RDDB_INVALID_ARG,
    E_RDDB_SYNCH_CONFLICT,
    E_RDDB_VM_ALREADY_ASSIGNED,
    E_RDDB_USER_ALREADY_ASSIGNED,
    E_RDDB_USER_CONN_PENDING,
    E_RDDB_TOO_MANY_RECORDS,
    E_RDDB_NO_RESOURCE_AVAILABLE,
    E_RDDB_TARGET_ENDPOINT_DOWN,
    E_RDDB_UNKNOWN_ERR,
    E_RDDB_ALREADY_EXISTS,
    E_RDDB_KEYVALUE_MISMATCH,
}

impl RDDB {
    pub fn code(&self) -> u32 {
        return match self {
            RDDB::E_RDDB_LOGIN_FAILED => 0x88250001 as u32,
            RDDB::E_RDDB_DATA_INTEGRITY_VIOLATION => 0x88250002 as u32,
            RDDB::E_RDDB_FATAL_ERR => 0x88250003 as u32,
            RDDB::E_RDDB_DB_OFFLINE => 0x88250004 as u32,
            RDDB::E_RDDB_INVALID_ARG => 0x88250005 as u32,
            RDDB::E_RDDB_SYNCH_CONFLICT => 0x88250006 as u32,
            RDDB::E_RDDB_VM_ALREADY_ASSIGNED => 0x88250007 as u32,
            RDDB::E_RDDB_USER_ALREADY_ASSIGNED => 0x88250008 as u32,
            RDDB::E_RDDB_USER_CONN_PENDING => 0x88250009 as u32,
            RDDB::E_RDDB_TOO_MANY_RECORDS => 0x8825000A as u32,
            RDDB::E_RDDB_NO_RESOURCE_AVAILABLE => 0x8825000B as u32,
            RDDB::E_RDDB_TARGET_ENDPOINT_DOWN => 0x8825000C as u32,
            RDDB::E_RDDB_UNKNOWN_ERR => 0x8825000D as u32,
            RDDB::E_RDDB_ALREADY_EXISTS => 0x8825000E as u32,
            RDDB::E_RDDB_KEYVALUE_MISMATCH => 0x8825000F as u32,
        }
    }

    pub fn error(&self) -> RawError {
        return match self {
            RDDB::E_RDDB_LOGIN_FAILED => RawError::Kind(RDDB::E_RDDB_LOGIN_FAILED),
            RDDB::E_RDDB_DATA_INTEGRITY_VIOLATION => RawError::Kind(RDDB::E_RDDB_DATA_INTEGRITY_VIOLATION),
            RDDB::E_RDDB_FATAL_ERR => RawError::Kind(RDDB::E_RDDB_FATAL_ERR),
            RDDB::E_RDDB_DB_OFFLINE => RawError::Kind(RDDB::E_RDDB_DB_OFFLINE),
            RDDB::E_RDDB_INVALID_ARG => RawError::Kind(RDDB::E_RDDB_INVALID_ARG),
            RDDB::E_RDDB_SYNCH_CONFLICT => RawError::Kind(RDDB::E_RDDB_SYNCH_CONFLICT),
            RDDB::E_RDDB_VM_ALREADY_ASSIGNED => RawError::Kind(RDDB::E_RDDB_VM_ALREADY_ASSIGNED),
            RDDB::E_RDDB_USER_ALREADY_ASSIGNED => RawError::Kind(RDDB::E_RDDB_USER_ALREADY_ASSIGNED),
            RDDB::E_RDDB_USER_CONN_PENDING => RawError::Kind(RDDB::E_RDDB_USER_CONN_PENDING),
            RDDB::E_RDDB_TOO_MANY_RECORDS => RawError::Kind(RDDB::E_RDDB_TOO_MANY_RECORDS),
            RDDB::E_RDDB_NO_RESOURCE_AVAILABLE => RawError::Kind(RDDB::E_RDDB_NO_RESOURCE_AVAILABLE),
            RDDB::E_RDDB_TARGET_ENDPOINT_DOWN => RawError::Kind(RDDB::E_RDDB_TARGET_ENDPOINT_DOWN),
            RDDB::E_RDDB_UNKNOWN_ERR => RawError::Kind(RDDB::E_RDDB_UNKNOWN_ERR),
            RDDB::E_RDDB_ALREADY_EXISTS => RawError::Kind(RDDB::E_RDDB_ALREADY_EXISTS),
            RDDB::E_RDDB_KEYVALUE_MISMATCH => RawError::Kind(RDDB::E_RDDB_KEYVALUE_MISMATCH),
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            RDDB::E_RDDB_LOGIN_FAILED => "User failed to logon to the DB.",
            RDDB::E_RDDB_DATA_INTEGRITY_VIOLATION => "DB operation failed due to violation of data integrity constraint.",
            RDDB::E_RDDB_FATAL_ERR => "DB internal error.",
            RDDB::E_RDDB_DB_OFFLINE => "DB is offline.",
            RDDB::E_RDDB_INVALID_ARG => "Invalid argument was supplied.",
            RDDB::E_RDDB_SYNCH_CONFLICT => "This error is returned to RD Connection Broker runtime, when there is a synch conflict. RD Connection Broker is expected to re-read the record from the DB & call update with latest record.",
            RDDB::E_RDDB_VM_ALREADY_ASSIGNED => "This VM is already assigned to a another user.",
            RDDB::E_RDDB_USER_ALREADY_ASSIGNED => "This user has already been assigned another VM.",
            RDDB::E_RDDB_USER_CONN_PENDING => "This error is returned to RD Connection Broker runtime, when the same user connects to the same farm more than once simultaneously.",
            RDDB::E_RDDB_TOO_MANY_RECORDS => "Number of records in the DB exceeds the maximum limit allowed for.",
            RDDB::E_RDDB_NO_RESOURCE_AVAILABLE => "No more resource available in Pool for connection.",
            RDDB::E_RDDB_TARGET_ENDPOINT_DOWN => "Target endpoint is not running currently.",
            RDDB::E_RDDB_UNKNOWN_ERR => "Generic database operation error.",
            RDDB::E_RDDB_ALREADY_EXISTS => "Value already exists.",
            RDDB::E_RDDB_KEYVALUE_MISMATCH => "Value mismatched.",
        }
    }

    pub fn from_name(name: &str) -> RDDB {
        return match name {
            "E_RDDB_LOGIN_FAILED" => RDDB::E_RDDB_LOGIN_FAILED,
            "E_RDDB_DATA_INTEGRITY_VIOLATION" => RDDB::E_RDDB_DATA_INTEGRITY_VIOLATION,
            "E_RDDB_FATAL_ERR" => RDDB::E_RDDB_FATAL_ERR,
            "E_RDDB_DB_OFFLINE" => RDDB::E_RDDB_DB_OFFLINE,
            "E_RDDB_INVALID_ARG" => RDDB::E_RDDB_INVALID_ARG,
            "E_RDDB_SYNCH_CONFLICT" => RDDB::E_RDDB_SYNCH_CONFLICT,
            "E_RDDB_VM_ALREADY_ASSIGNED" => RDDB::E_RDDB_VM_ALREADY_ASSIGNED,
            "E_RDDB_USER_ALREADY_ASSIGNED" => RDDB::E_RDDB_USER_ALREADY_ASSIGNED,
            "E_RDDB_USER_CONN_PENDING" => RDDB::E_RDDB_USER_CONN_PENDING,
            "E_RDDB_TOO_MANY_RECORDS" => RDDB::E_RDDB_TOO_MANY_RECORDS,
            "E_RDDB_NO_RESOURCE_AVAILABLE" => RDDB::E_RDDB_NO_RESOURCE_AVAILABLE,
            "E_RDDB_TARGET_ENDPOINT_DOWN" => RDDB::E_RDDB_TARGET_ENDPOINT_DOWN,
            "E_RDDB_UNKNOWN_ERR" => RDDB::E_RDDB_UNKNOWN_ERR,
            "E_RDDB_ALREADY_EXISTS" => RDDB::E_RDDB_ALREADY_EXISTS,
            "E_RDDB_KEYVALUE_MISMATCH" => RDDB::E_RDDB_KEYVALUE_MISMATCH,
        }
    }
    pub fn from_code(code: u32) -> RDDB {
        return match code {
            0x88250001 => RDDB::E_RDDB_LOGIN_FAILED,
            0x88250002 => RDDB::E_RDDB_DATA_INTEGRITY_VIOLATION,
            0x88250003 => RDDB::E_RDDB_FATAL_ERR,
            0x88250004 => RDDB::E_RDDB_DB_OFFLINE,
            0x88250005 => RDDB::E_RDDB_INVALID_ARG,
            0x88250006 => RDDB::E_RDDB_SYNCH_CONFLICT,
            0x88250007 => RDDB::E_RDDB_VM_ALREADY_ASSIGNED,
            0x88250008 => RDDB::E_RDDB_USER_ALREADY_ASSIGNED,
            0x88250009 => RDDB::E_RDDB_USER_CONN_PENDING,
            0x8825000A => RDDB::E_RDDB_TOO_MANY_RECORDS,
            0x8825000B => RDDB::E_RDDB_NO_RESOURCE_AVAILABLE,
            0x8825000C => RDDB::E_RDDB_TARGET_ENDPOINT_DOWN,
            0x8825000D => RDDB::E_RDDB_UNKNOWN_ERR,
            0x8825000E => RDDB::E_RDDB_ALREADY_EXISTS,
            0x8825000F => RDDB::E_RDDB_KEYVALUE_MISMATCH,
        }
    }
}
