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
pub enum WINDOWS_SETUP_PLA {
    PLA_S_PROPERTY_IGNORED,
    PLA_E_DCS_NOT_FOUND,
    PLA_E_TOO_MANY_FOLDERS,
    PLA_E_NO_MIN_DISK,
    PLA_E_DCS_IN_USE,
    PLA_E_DCS_ALREADY_EXISTS,
    PLA_E_PROPERTY_CONFLICT,
    PLA_E_DCS_SINGLETON_REQUIRED,
    PLA_E_CREDENTIALS_REQUIRED,
    PLA_E_DCS_NOT_RUNNING,
    PLA_E_CONFLICT_INCL_EXCL_API,
    PLA_E_NETWORK_EXE_NOT_VALID,
    PLA_E_EXE_ALREADY_CONFIGURED,
    PLA_E_EXE_PATH_NOT_VALID,
    PLA_E_DC_ALREADY_EXISTS,
    PLA_E_DCS_START_WAIT_TIMEOUT,
    PLA_E_DC_START_WAIT_TIMEOUT,
    PLA_E_REPORT_WAIT_TIMEOUT,
    PLA_E_NO_DUPLICATES,
    PLA_E_EXE_FULL_PATH_REQUIRED,
    PLA_E_INVALID_SESSION_NAME,
    PLA_E_PLA_CHANNEL_NOT_ENABLED,
    PLA_E_TASKSCHED_CHANNEL_NOT_ENABLED,
    PLA_E_RULES_MANAGER_FAILED,
    PLA_E_CABAPI_FAILURE,
}

impl WINDOWS_SETUP_PLA {
    pub fn code(&self) -> u32 {
        return match self {
            WINDOWS_SETUP_PLA::PLA_S_PROPERTY_IGNORED => 0x00300100 as u32,
            WINDOWS_SETUP_PLA::PLA_E_DCS_NOT_FOUND => 0x80300002 as u32,
            WINDOWS_SETUP_PLA::PLA_E_TOO_MANY_FOLDERS => 0x80300045 as u32,
            WINDOWS_SETUP_PLA::PLA_E_NO_MIN_DISK => 0x80300070 as u32,
            WINDOWS_SETUP_PLA::PLA_E_DCS_IN_USE => 0x803000AA as u32,
            WINDOWS_SETUP_PLA::PLA_E_DCS_ALREADY_EXISTS => 0x803000B7 as u32,
            WINDOWS_SETUP_PLA::PLA_E_PROPERTY_CONFLICT => 0x80300101 as u32,
            WINDOWS_SETUP_PLA::PLA_E_DCS_SINGLETON_REQUIRED => 0x80300102 as u32,
            WINDOWS_SETUP_PLA::PLA_E_CREDENTIALS_REQUIRED => 0x80300103 as u32,
            WINDOWS_SETUP_PLA::PLA_E_DCS_NOT_RUNNING => 0x80300104 as u32,
            WINDOWS_SETUP_PLA::PLA_E_CONFLICT_INCL_EXCL_API => 0x80300105 as u32,
            WINDOWS_SETUP_PLA::PLA_E_NETWORK_EXE_NOT_VALID => 0x80300106 as u32,
            WINDOWS_SETUP_PLA::PLA_E_EXE_ALREADY_CONFIGURED => 0x80300107 as u32,
            WINDOWS_SETUP_PLA::PLA_E_EXE_PATH_NOT_VALID => 0x80300108 as u32,
            WINDOWS_SETUP_PLA::PLA_E_DC_ALREADY_EXISTS => 0x80300109 as u32,
            WINDOWS_SETUP_PLA::PLA_E_DCS_START_WAIT_TIMEOUT => 0x8030010A as u32,
            WINDOWS_SETUP_PLA::PLA_E_DC_START_WAIT_TIMEOUT => 0x8030010B as u32,
            WINDOWS_SETUP_PLA::PLA_E_REPORT_WAIT_TIMEOUT => 0x8030010C as u32,
            WINDOWS_SETUP_PLA::PLA_E_NO_DUPLICATES => 0x8030010D as u32,
            WINDOWS_SETUP_PLA::PLA_E_EXE_FULL_PATH_REQUIRED => 0x8030010E as u32,
            WINDOWS_SETUP_PLA::PLA_E_INVALID_SESSION_NAME => 0x8030010F as u32,
            WINDOWS_SETUP_PLA::PLA_E_PLA_CHANNEL_NOT_ENABLED => 0x80300110 as u32,
            WINDOWS_SETUP_PLA::PLA_E_TASKSCHED_CHANNEL_NOT_ENABLED => 0x80300111 as u32,
            WINDOWS_SETUP_PLA::PLA_E_RULES_MANAGER_FAILED => 0x80300112 as u32,
            WINDOWS_SETUP_PLA::PLA_E_CABAPI_FAILURE => 0x80300113 as u32,
        }
    }

    pub fn error(&self) -> RawError {
        return match self {
            WINDOWS_SETUP_PLA::PLA_S_PROPERTY_IGNORED => RawError::Kind(WINDOWS_SETUP_PLA::PLA_S_PROPERTY_IGNORED),
            WINDOWS_SETUP_PLA::PLA_E_DCS_NOT_FOUND => RawError::Kind(WINDOWS_SETUP_PLA::PLA_E_DCS_NOT_FOUND),
            WINDOWS_SETUP_PLA::PLA_E_TOO_MANY_FOLDERS => RawError::Kind(WINDOWS_SETUP_PLA::PLA_E_TOO_MANY_FOLDERS),
            WINDOWS_SETUP_PLA::PLA_E_NO_MIN_DISK => RawError::Kind(WINDOWS_SETUP_PLA::PLA_E_NO_MIN_DISK),
            WINDOWS_SETUP_PLA::PLA_E_DCS_IN_USE => RawError::Kind(WINDOWS_SETUP_PLA::PLA_E_DCS_IN_USE),
            WINDOWS_SETUP_PLA::PLA_E_DCS_ALREADY_EXISTS => RawError::Kind(WINDOWS_SETUP_PLA::PLA_E_DCS_ALREADY_EXISTS),
            WINDOWS_SETUP_PLA::PLA_E_PROPERTY_CONFLICT => RawError::Kind(WINDOWS_SETUP_PLA::PLA_E_PROPERTY_CONFLICT),
            WINDOWS_SETUP_PLA::PLA_E_DCS_SINGLETON_REQUIRED => RawError::Kind(WINDOWS_SETUP_PLA::PLA_E_DCS_SINGLETON_REQUIRED),
            WINDOWS_SETUP_PLA::PLA_E_CREDENTIALS_REQUIRED => RawError::Kind(WINDOWS_SETUP_PLA::PLA_E_CREDENTIALS_REQUIRED),
            WINDOWS_SETUP_PLA::PLA_E_DCS_NOT_RUNNING => RawError::Kind(WINDOWS_SETUP_PLA::PLA_E_DCS_NOT_RUNNING),
            WINDOWS_SETUP_PLA::PLA_E_CONFLICT_INCL_EXCL_API => RawError::Kind(WINDOWS_SETUP_PLA::PLA_E_CONFLICT_INCL_EXCL_API),
            WINDOWS_SETUP_PLA::PLA_E_NETWORK_EXE_NOT_VALID => RawError::Kind(WINDOWS_SETUP_PLA::PLA_E_NETWORK_EXE_NOT_VALID),
            WINDOWS_SETUP_PLA::PLA_E_EXE_ALREADY_CONFIGURED => RawError::Kind(WINDOWS_SETUP_PLA::PLA_E_EXE_ALREADY_CONFIGURED),
            WINDOWS_SETUP_PLA::PLA_E_EXE_PATH_NOT_VALID => RawError::Kind(WINDOWS_SETUP_PLA::PLA_E_EXE_PATH_NOT_VALID),
            WINDOWS_SETUP_PLA::PLA_E_DC_ALREADY_EXISTS => RawError::Kind(WINDOWS_SETUP_PLA::PLA_E_DC_ALREADY_EXISTS),
            WINDOWS_SETUP_PLA::PLA_E_DCS_START_WAIT_TIMEOUT => RawError::Kind(WINDOWS_SETUP_PLA::PLA_E_DCS_START_WAIT_TIMEOUT),
            WINDOWS_SETUP_PLA::PLA_E_DC_START_WAIT_TIMEOUT => RawError::Kind(WINDOWS_SETUP_PLA::PLA_E_DC_START_WAIT_TIMEOUT),
            WINDOWS_SETUP_PLA::PLA_E_REPORT_WAIT_TIMEOUT => RawError::Kind(WINDOWS_SETUP_PLA::PLA_E_REPORT_WAIT_TIMEOUT),
            WINDOWS_SETUP_PLA::PLA_E_NO_DUPLICATES => RawError::Kind(WINDOWS_SETUP_PLA::PLA_E_NO_DUPLICATES),
            WINDOWS_SETUP_PLA::PLA_E_EXE_FULL_PATH_REQUIRED => RawError::Kind(WINDOWS_SETUP_PLA::PLA_E_EXE_FULL_PATH_REQUIRED),
            WINDOWS_SETUP_PLA::PLA_E_INVALID_SESSION_NAME => RawError::Kind(WINDOWS_SETUP_PLA::PLA_E_INVALID_SESSION_NAME),
            WINDOWS_SETUP_PLA::PLA_E_PLA_CHANNEL_NOT_ENABLED => RawError::Kind(WINDOWS_SETUP_PLA::PLA_E_PLA_CHANNEL_NOT_ENABLED),
            WINDOWS_SETUP_PLA::PLA_E_TASKSCHED_CHANNEL_NOT_ENABLED => RawError::Kind(WINDOWS_SETUP_PLA::PLA_E_TASKSCHED_CHANNEL_NOT_ENABLED),
            WINDOWS_SETUP_PLA::PLA_E_RULES_MANAGER_FAILED => RawError::Kind(WINDOWS_SETUP_PLA::PLA_E_RULES_MANAGER_FAILED),
            WINDOWS_SETUP_PLA::PLA_E_CABAPI_FAILURE => RawError::Kind(WINDOWS_SETUP_PLA::PLA_E_CABAPI_FAILURE),
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            WINDOWS_SETUP_PLA::PLA_S_PROPERTY_IGNORED => "Property value will be ignored.",
            WINDOWS_SETUP_PLA::PLA_E_DCS_NOT_FOUND => "Data Collector Set was not found.",
            WINDOWS_SETUP_PLA::PLA_E_TOO_MANY_FOLDERS => "Unable to start Data Collector Set because there are too many folders.",
            WINDOWS_SETUP_PLA::PLA_E_NO_MIN_DISK => "Not enough free disk space to start Data Collector Set.",
            WINDOWS_SETUP_PLA::PLA_E_DCS_IN_USE => "The Data Collector Set or one of its dependencies is already in use.",
            WINDOWS_SETUP_PLA::PLA_E_DCS_ALREADY_EXISTS => "Data Collector Set already exists.",
            WINDOWS_SETUP_PLA::PLA_E_PROPERTY_CONFLICT => "Property value conflict.",
            WINDOWS_SETUP_PLA::PLA_E_DCS_SINGLETON_REQUIRED => "The current configuration for this Data Collector Set requires that it contain exactly one Data Collector.",
            WINDOWS_SETUP_PLA::PLA_E_CREDENTIALS_REQUIRED => "A user account is required in order to commit the current Data Collector Set properties.",
            WINDOWS_SETUP_PLA::PLA_E_DCS_NOT_RUNNING => "Data Collector Set is not running.",
            WINDOWS_SETUP_PLA::PLA_E_CONFLICT_INCL_EXCL_API => "A conflict was detected in the list of include/exclude APIs. Do not specify the same API in both the include list and the exclude list.",
            WINDOWS_SETUP_PLA::PLA_E_NETWORK_EXE_NOT_VALID => "The executable path you have specified refers to a network share or UNC path.",
            WINDOWS_SETUP_PLA::PLA_E_EXE_ALREADY_CONFIGURED => "The executable path you have specified is already configured for API tracing.",
            WINDOWS_SETUP_PLA::PLA_E_EXE_PATH_NOT_VALID => "The executable path you have specified does not exist. Verify that the specified path is correct.",
            WINDOWS_SETUP_PLA::PLA_E_DC_ALREADY_EXISTS => "Data Collector already exists.",
            WINDOWS_SETUP_PLA::PLA_E_DCS_START_WAIT_TIMEOUT => "The wait for the Data Collector Set start notification has timed out.",
            WINDOWS_SETUP_PLA::PLA_E_DC_START_WAIT_TIMEOUT => "The wait for the Data Collector to start has timed out.",
            WINDOWS_SETUP_PLA::PLA_E_REPORT_WAIT_TIMEOUT => "The wait for the report generation tool to finish has timed out.",
            WINDOWS_SETUP_PLA::PLA_E_NO_DUPLICATES => "Duplicate items are not allowed.",
            WINDOWS_SETUP_PLA::PLA_E_EXE_FULL_PATH_REQUIRED => "When specifying the executable that you want to trace, you must specify a full path to the executable and not just a filename.",
            WINDOWS_SETUP_PLA::PLA_E_INVALID_SESSION_NAME => "The session name provided is invalid.",
            WINDOWS_SETUP_PLA::PLA_E_PLA_CHANNEL_NOT_ENABLED => "The Event Log channel Microsoft-Windows-Diagnosis-PLA/Operational must be enabled to perform this operation.",
            WINDOWS_SETUP_PLA::PLA_E_TASKSCHED_CHANNEL_NOT_ENABLED => "The Event Log channel Microsoft-Windows-TaskScheduler must be enabled to perform this operation.",
            WINDOWS_SETUP_PLA::PLA_E_RULES_MANAGER_FAILED => "The execution of the Rules Manager failed.",
            WINDOWS_SETUP_PLA::PLA_E_CABAPI_FAILURE => "An error occurred while attempting to compress or extract the data.",
        }
    }

    pub fn from_name(name: &str) -> WINDOWS_SETUP_PLA {
        return match name {
            "PLA_S_PROPERTY_IGNORED" => WINDOWS_SETUP_PLA::PLA_S_PROPERTY_IGNORED,
            "PLA_E_DCS_NOT_FOUND" => WINDOWS_SETUP_PLA::PLA_E_DCS_NOT_FOUND,
            "PLA_E_TOO_MANY_FOLDERS" => WINDOWS_SETUP_PLA::PLA_E_TOO_MANY_FOLDERS,
            "PLA_E_NO_MIN_DISK" => WINDOWS_SETUP_PLA::PLA_E_NO_MIN_DISK,
            "PLA_E_DCS_IN_USE" => WINDOWS_SETUP_PLA::PLA_E_DCS_IN_USE,
            "PLA_E_DCS_ALREADY_EXISTS" => WINDOWS_SETUP_PLA::PLA_E_DCS_ALREADY_EXISTS,
            "PLA_E_PROPERTY_CONFLICT" => WINDOWS_SETUP_PLA::PLA_E_PROPERTY_CONFLICT,
            "PLA_E_DCS_SINGLETON_REQUIRED" => WINDOWS_SETUP_PLA::PLA_E_DCS_SINGLETON_REQUIRED,
            "PLA_E_CREDENTIALS_REQUIRED" => WINDOWS_SETUP_PLA::PLA_E_CREDENTIALS_REQUIRED,
            "PLA_E_DCS_NOT_RUNNING" => WINDOWS_SETUP_PLA::PLA_E_DCS_NOT_RUNNING,
            "PLA_E_CONFLICT_INCL_EXCL_API" => WINDOWS_SETUP_PLA::PLA_E_CONFLICT_INCL_EXCL_API,
            "PLA_E_NETWORK_EXE_NOT_VALID" => WINDOWS_SETUP_PLA::PLA_E_NETWORK_EXE_NOT_VALID,
            "PLA_E_EXE_ALREADY_CONFIGURED" => WINDOWS_SETUP_PLA::PLA_E_EXE_ALREADY_CONFIGURED,
            "PLA_E_EXE_PATH_NOT_VALID" => WINDOWS_SETUP_PLA::PLA_E_EXE_PATH_NOT_VALID,
            "PLA_E_DC_ALREADY_EXISTS" => WINDOWS_SETUP_PLA::PLA_E_DC_ALREADY_EXISTS,
            "PLA_E_DCS_START_WAIT_TIMEOUT" => WINDOWS_SETUP_PLA::PLA_E_DCS_START_WAIT_TIMEOUT,
            "PLA_E_DC_START_WAIT_TIMEOUT" => WINDOWS_SETUP_PLA::PLA_E_DC_START_WAIT_TIMEOUT,
            "PLA_E_REPORT_WAIT_TIMEOUT" => WINDOWS_SETUP_PLA::PLA_E_REPORT_WAIT_TIMEOUT,
            "PLA_E_NO_DUPLICATES" => WINDOWS_SETUP_PLA::PLA_E_NO_DUPLICATES,
            "PLA_E_EXE_FULL_PATH_REQUIRED" => WINDOWS_SETUP_PLA::PLA_E_EXE_FULL_PATH_REQUIRED,
            "PLA_E_INVALID_SESSION_NAME" => WINDOWS_SETUP_PLA::PLA_E_INVALID_SESSION_NAME,
            "PLA_E_PLA_CHANNEL_NOT_ENABLED" => WINDOWS_SETUP_PLA::PLA_E_PLA_CHANNEL_NOT_ENABLED,
            "PLA_E_TASKSCHED_CHANNEL_NOT_ENABLED" => WINDOWS_SETUP_PLA::PLA_E_TASKSCHED_CHANNEL_NOT_ENABLED,
            "PLA_E_RULES_MANAGER_FAILED" => WINDOWS_SETUP_PLA::PLA_E_RULES_MANAGER_FAILED,
            "PLA_E_CABAPI_FAILURE" => WINDOWS_SETUP_PLA::PLA_E_CABAPI_FAILURE,
        }
    }
    pub fn from_code(code: u32) -> WINDOWS_SETUP_PLA {
        return match code {
            0x00300100 => WINDOWS_SETUP_PLA::PLA_S_PROPERTY_IGNORED,
            0x80300002 => WINDOWS_SETUP_PLA::PLA_E_DCS_NOT_FOUND,
            0x80300045 => WINDOWS_SETUP_PLA::PLA_E_TOO_MANY_FOLDERS,
            0x80300070 => WINDOWS_SETUP_PLA::PLA_E_NO_MIN_DISK,
            0x803000AA => WINDOWS_SETUP_PLA::PLA_E_DCS_IN_USE,
            0x803000B7 => WINDOWS_SETUP_PLA::PLA_E_DCS_ALREADY_EXISTS,
            0x80300101 => WINDOWS_SETUP_PLA::PLA_E_PROPERTY_CONFLICT,
            0x80300102 => WINDOWS_SETUP_PLA::PLA_E_DCS_SINGLETON_REQUIRED,
            0x80300103 => WINDOWS_SETUP_PLA::PLA_E_CREDENTIALS_REQUIRED,
            0x80300104 => WINDOWS_SETUP_PLA::PLA_E_DCS_NOT_RUNNING,
            0x80300105 => WINDOWS_SETUP_PLA::PLA_E_CONFLICT_INCL_EXCL_API,
            0x80300106 => WINDOWS_SETUP_PLA::PLA_E_NETWORK_EXE_NOT_VALID,
            0x80300107 => WINDOWS_SETUP_PLA::PLA_E_EXE_ALREADY_CONFIGURED,
            0x80300108 => WINDOWS_SETUP_PLA::PLA_E_EXE_PATH_NOT_VALID,
            0x80300109 => WINDOWS_SETUP_PLA::PLA_E_DC_ALREADY_EXISTS,
            0x8030010A => WINDOWS_SETUP_PLA::PLA_E_DCS_START_WAIT_TIMEOUT,
            0x8030010B => WINDOWS_SETUP_PLA::PLA_E_DC_START_WAIT_TIMEOUT,
            0x8030010C => WINDOWS_SETUP_PLA::PLA_E_REPORT_WAIT_TIMEOUT,
            0x8030010D => WINDOWS_SETUP_PLA::PLA_E_NO_DUPLICATES,
            0x8030010E => WINDOWS_SETUP_PLA::PLA_E_EXE_FULL_PATH_REQUIRED,
            0x8030010F => WINDOWS_SETUP_PLA::PLA_E_INVALID_SESSION_NAME,
            0x80300110 => WINDOWS_SETUP_PLA::PLA_E_PLA_CHANNEL_NOT_ENABLED,
            0x80300111 => WINDOWS_SETUP_PLA::PLA_E_TASKSCHED_CHANNEL_NOT_ENABLED,
            0x80300112 => WINDOWS_SETUP_PLA::PLA_E_RULES_MANAGER_FAILED,
            0x80300113 => WINDOWS_SETUP_PLA::PLA_E_CABAPI_FAILURE,
        }
    }
}
