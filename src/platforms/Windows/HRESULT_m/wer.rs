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
pub enum WER {
    WER_S_REPORT_DEBUG,
    WER_S_REPORT_UPLOADED,
    WER_S_REPORT_QUEUED,
    WER_S_DISABLED,
    WER_S_SUSPENDED_UPLOAD,
    WER_S_DISABLED_QUEUE,
    WER_S_DISABLED_ARCHIVE,
    WER_S_REPORT_ASYNC,
    WER_S_IGNORE_ASSERT_INSTANCE,
    WER_S_IGNORE_ALL_ASSERTS,
    WER_S_ASSERT_CONTINUE,
    WER_S_THROTTLED,
    WER_S_REPORT_UPLOADED_CAB,
    WER_E_CRASH_FAILURE,
    WER_E_CANCELED,
    WER_E_NETWORK_FAILURE,
    WER_E_NOT_INITIALIZED,
    WER_E_ALREADY_REPORTING,
    WER_E_DUMP_THROTTLED,
    WER_E_INSUFFICIENT_CONSENT,
    WER_E_TOO_HEAVY,
}

impl WER {
    pub fn code(&self) -> u32 {
        return match self {
            WER::WER_S_REPORT_DEBUG => 0x001B0000 as u32,
            WER::WER_S_REPORT_UPLOADED => 0x001B0001 as u32,
            WER::WER_S_REPORT_QUEUED => 0x001B0002 as u32,
            WER::WER_S_DISABLED => 0x001B0003 as u32,
            WER::WER_S_SUSPENDED_UPLOAD => 0x001B0004 as u32,
            WER::WER_S_DISABLED_QUEUE => 0x001B0005 as u32,
            WER::WER_S_DISABLED_ARCHIVE => 0x001B0006 as u32,
            WER::WER_S_REPORT_ASYNC => 0x001B0007 as u32,
            WER::WER_S_IGNORE_ASSERT_INSTANCE => 0x001B0008 as u32,
            WER::WER_S_IGNORE_ALL_ASSERTS => 0x001B0009 as u32,
            WER::WER_S_ASSERT_CONTINUE => 0x001B000A as u32,
            WER::WER_S_THROTTLED => 0x001B000B as u32,
            WER::WER_S_REPORT_UPLOADED_CAB => 0x001B000C as u32,
            WER::WER_E_CRASH_FAILURE => 0x801B8000 as u32,
            WER::WER_E_CANCELED => 0x801B8001 as u32,
            WER::WER_E_NETWORK_FAILURE => 0x801B8002 as u32,
            WER::WER_E_NOT_INITIALIZED => 0x801B8003 as u32,
            WER::WER_E_ALREADY_REPORTING => 0x801B8004 as u32,
            WER::WER_E_DUMP_THROTTLED => 0x801B8005 as u32,
            WER::WER_E_INSUFFICIENT_CONSENT => 0x801B8006 as u32,
            WER::WER_E_TOO_HEAVY => 0x801B8007 as u32,
        }
    }

    pub fn error(&self) -> RawError {
        return match self {
            WER::WER_S_REPORT_DEBUG => RawError::Kind(WER::WER_S_REPORT_DEBUG),
            WER::WER_S_REPORT_UPLOADED => RawError::Kind(WER::WER_S_REPORT_UPLOADED),
            WER::WER_S_REPORT_QUEUED => RawError::Kind(WER::WER_S_REPORT_QUEUED),
            WER::WER_S_DISABLED => RawError::Kind(WER::WER_S_DISABLED),
            WER::WER_S_SUSPENDED_UPLOAD => RawError::Kind(WER::WER_S_SUSPENDED_UPLOAD),
            WER::WER_S_DISABLED_QUEUE => RawError::Kind(WER::WER_S_DISABLED_QUEUE),
            WER::WER_S_DISABLED_ARCHIVE => RawError::Kind(WER::WER_S_DISABLED_ARCHIVE),
            WER::WER_S_REPORT_ASYNC => RawError::Kind(WER::WER_S_REPORT_ASYNC),
            WER::WER_S_IGNORE_ASSERT_INSTANCE => RawError::Kind(WER::WER_S_IGNORE_ASSERT_INSTANCE),
            WER::WER_S_IGNORE_ALL_ASSERTS => RawError::Kind(WER::WER_S_IGNORE_ALL_ASSERTS),
            WER::WER_S_ASSERT_CONTINUE => RawError::Kind(WER::WER_S_ASSERT_CONTINUE),
            WER::WER_S_THROTTLED => RawError::Kind(WER::WER_S_THROTTLED),
            WER::WER_S_REPORT_UPLOADED_CAB => RawError::Kind(WER::WER_S_REPORT_UPLOADED_CAB),
            WER::WER_E_CRASH_FAILURE => RawError::Kind(WER::WER_E_CRASH_FAILURE),
            WER::WER_E_CANCELED => RawError::Kind(WER::WER_E_CANCELED),
            WER::WER_E_NETWORK_FAILURE => RawError::Kind(WER::WER_E_NETWORK_FAILURE),
            WER::WER_E_NOT_INITIALIZED => RawError::Kind(WER::WER_E_NOT_INITIALIZED),
            WER::WER_E_ALREADY_REPORTING => RawError::Kind(WER::WER_E_ALREADY_REPORTING),
            WER::WER_E_DUMP_THROTTLED => RawError::Kind(WER::WER_E_DUMP_THROTTLED),
            WER::WER_E_INSUFFICIENT_CONSENT => RawError::Kind(WER::WER_E_INSUFFICIENT_CONSENT),
            WER::WER_E_TOO_HEAVY => RawError::Kind(WER::WER_E_TOO_HEAVY),
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            WER::WER_S_REPORT_DEBUG => "Debugger was attached.",
            WER::WER_S_REPORT_UPLOADED => "Report was uploaded.",
            WER::WER_S_REPORT_QUEUED => "Report was queued.",
            WER::WER_S_DISABLED => "Reporting was disabled.",
            WER::WER_S_SUSPENDED_UPLOAD => "Reporting was temporarily suspended.",
            WER::WER_S_DISABLED_QUEUE => "Report was not queued to queuing being disabled.",
            WER::WER_S_DISABLED_ARCHIVE => "Report was uploaded, but not archived due to archiving being disabled.",
            WER::WER_S_REPORT_ASYNC => "Reporting was successfully spun off as an asynchronous operation.",
            WER::WER_S_IGNORE_ASSERT_INSTANCE => "The assertion was handled.",
            WER::WER_S_IGNORE_ALL_ASSERTS => "The assertion was handled and added to a permanent ignore list.",
            WER::WER_S_ASSERT_CONTINUE => "The assertion was resumed as unhandled.",
            WER::WER_S_THROTTLED => "Report was throttled.",
            WER::WER_S_REPORT_UPLOADED_CAB => "Report was uploaded with cab.",
            WER::WER_E_CRASH_FAILURE => "Crash reporting failed.",
            WER::WER_E_CANCELED => "Report aborted due to user cancellation.",
            WER::WER_E_NETWORK_FAILURE => "Report aborted due to network failure.",
            WER::WER_E_NOT_INITIALIZED => "Report not initialized.",
            WER::WER_E_ALREADY_REPORTING => "Reporting is already in progress for the specified process.",
            WER::WER_E_DUMP_THROTTLED => "Dump not generated due to a throttle.",
            WER::WER_E_INSUFFICIENT_CONSENT => "Operation failed due to insufficient user consent.",
            WER::WER_E_TOO_HEAVY => "Report aborted due to performance criteria.",
        }
    }

    pub fn from_name(name: &str) -> WER {
        return match name {
            "WER_S_REPORT_DEBUG" => WER::WER_S_REPORT_DEBUG,
            "WER_S_REPORT_UPLOADED" => WER::WER_S_REPORT_UPLOADED,
            "WER_S_REPORT_QUEUED" => WER::WER_S_REPORT_QUEUED,
            "WER_S_DISABLED" => WER::WER_S_DISABLED,
            "WER_S_SUSPENDED_UPLOAD" => WER::WER_S_SUSPENDED_UPLOAD,
            "WER_S_DISABLED_QUEUE" => WER::WER_S_DISABLED_QUEUE,
            "WER_S_DISABLED_ARCHIVE" => WER::WER_S_DISABLED_ARCHIVE,
            "WER_S_REPORT_ASYNC" => WER::WER_S_REPORT_ASYNC,
            "WER_S_IGNORE_ASSERT_INSTANCE" => WER::WER_S_IGNORE_ASSERT_INSTANCE,
            "WER_S_IGNORE_ALL_ASSERTS" => WER::WER_S_IGNORE_ALL_ASSERTS,
            "WER_S_ASSERT_CONTINUE" => WER::WER_S_ASSERT_CONTINUE,
            "WER_S_THROTTLED" => WER::WER_S_THROTTLED,
            "WER_S_REPORT_UPLOADED_CAB" => WER::WER_S_REPORT_UPLOADED_CAB,
            "WER_E_CRASH_FAILURE" => WER::WER_E_CRASH_FAILURE,
            "WER_E_CANCELED" => WER::WER_E_CANCELED,
            "WER_E_NETWORK_FAILURE" => WER::WER_E_NETWORK_FAILURE,
            "WER_E_NOT_INITIALIZED" => WER::WER_E_NOT_INITIALIZED,
            "WER_E_ALREADY_REPORTING" => WER::WER_E_ALREADY_REPORTING,
            "WER_E_DUMP_THROTTLED" => WER::WER_E_DUMP_THROTTLED,
            "WER_E_INSUFFICIENT_CONSENT" => WER::WER_E_INSUFFICIENT_CONSENT,
            "WER_E_TOO_HEAVY" => WER::WER_E_TOO_HEAVY,
        }
    }
    pub fn from_code(code: u32) -> WER {
        return match code {
            0x001B0000 => WER::WER_S_REPORT_DEBUG,
            0x001B0001 => WER::WER_S_REPORT_UPLOADED,
            0x001B0002 => WER::WER_S_REPORT_QUEUED,
            0x001B0003 => WER::WER_S_DISABLED,
            0x001B0004 => WER::WER_S_SUSPENDED_UPLOAD,
            0x001B0005 => WER::WER_S_DISABLED_QUEUE,
            0x001B0006 => WER::WER_S_DISABLED_ARCHIVE,
            0x001B0007 => WER::WER_S_REPORT_ASYNC,
            0x001B0008 => WER::WER_S_IGNORE_ASSERT_INSTANCE,
            0x001B0009 => WER::WER_S_IGNORE_ALL_ASSERTS,
            0x001B000A => WER::WER_S_ASSERT_CONTINUE,
            0x001B000B => WER::WER_S_THROTTLED,
            0x001B000C => WER::WER_S_REPORT_UPLOADED_CAB,
            0x801B8000 => WER::WER_E_CRASH_FAILURE,
            0x801B8001 => WER::WER_E_CANCELED,
            0x801B8002 => WER::WER_E_NETWORK_FAILURE,
            0x801B8003 => WER::WER_E_NOT_INITIALIZED,
            0x801B8004 => WER::WER_E_ALREADY_REPORTING,
            0x801B8005 => WER::WER_E_DUMP_THROTTLED,
            0x801B8006 => WER::WER_E_INSUFFICIENT_CONSENT,
            0x801B8007 => WER::WER_E_TOO_HEAVY,
        }
    }
}
