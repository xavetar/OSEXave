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

pub enum WER_VIDEO {
    WER_S_REPORT_DEBUG = 0x001B0000,
    WER_S_REPORT_UPLOADED = 0x001B0001,
    WER_S_REPORT_QUEUED = 0x001B0002,
    WER_S_DISABLED = 0x001B0003,
    WER_S_SUSPENDED_UPLOAD = 0x001B0004,
    WER_S_DISABLED_QUEUE = 0x001B0005,
    WER_S_DISABLED_ARCHIVE = 0x001B0006,
    WER_S_REPORT_ASYNC = 0x001B0007,
    WER_S_IGNORE_ASSERT_INSTANCE = 0x001B0008,
    WER_S_IGNORE_ALL_ASSERTS = 0x001B0009,
    WER_S_ASSERT_CONTINUE = 0x001B000A,
    WER_S_THROTTLED = 0x001B000B,
    WER_S_REPORT_UPLOADED_CAB = 0x001B000C,
    WER_E_CRASH_FAILURE = 0x801B8000,
    WER_E_CANCELED = 0x801B8001,
    WER_E_NETWORK_FAILURE = 0x801B8002,
    WER_E_NOT_INITIALIZED = 0x801B8003,
    WER_E_ALREADY_REPORTING = 0x801B8004,
    WER_E_DUMP_THROTTLED = 0x801B8005,
    WER_E_INSUFFICIENT_CONSENT = 0x801B8006,
    WER_E_TOO_HEAVY = 0x801B8007,
}

impl WER_VIDEO {
    pub fn description(&self) -> &'static str {
        match self {
            WER_VIDEO::WER_S_REPORT_DEBUG => "Debugger was attached.",
            WER_VIDEO::WER_S_REPORT_UPLOADED => "Report was uploaded.",
            WER_VIDEO::WER_S_REPORT_QUEUED => "Report was queued.",
            WER_VIDEO::WER_S_DISABLED => "Reporting was disabled.",
            WER_VIDEO::WER_S_SUSPENDED_UPLOAD => "Reporting was temporarily suspended.",
            WER_VIDEO::WER_S_DISABLED_QUEUE => "Report was not queued to queuing being disabled.",
            WER_VIDEO::WER_S_DISABLED_ARCHIVE => "Report was uploaded, but not archived due to archiving being disabled.",
            WER_VIDEO::WER_S_REPORT_ASYNC => "Reporting was successfully spun off as an asynchronous operation.",
            WER_VIDEO::WER_S_IGNORE_ASSERT_INSTANCE => "The assertion was handled.",
            WER_VIDEO::WER_S_IGNORE_ALL_ASSERTS => "The assertion was handled and added to a permanent ignore list.",
            WER_VIDEO::WER_S_ASSERT_CONTINUE => "The assertion was resumed as unhandled.",
            WER_VIDEO::WER_S_THROTTLED => "Report was throttled.",
            WER_VIDEO::WER_S_REPORT_UPLOADED_CAB => "Report was uploaded with cab.",
            WER_VIDEO::WER_E_CRASH_FAILURE => "Crash reporting failed.",
            WER_VIDEO::WER_E_CANCELED => "Report aborted due to user cancellation.",
            WER_VIDEO::WER_E_NETWORK_FAILURE => "Report aborted due to network failure.",
            WER_VIDEO::WER_E_NOT_INITIALIZED => "Report not initialized.",
            WER_VIDEO::WER_E_ALREADY_REPORTING => "Reporting is already in progress for the specified process.",
            WER_VIDEO::WER_E_DUMP_THROTTLED => "Dump not generated due to a throttle.",
            WER_VIDEO::WER_E_INSUFFICIENT_CONSENT => "Operation failed due to insufficient user consent.",
            WER_VIDEO::WER_E_TOO_HEAVY => "Report aborted due to performance criteria.",
        }
    }
}
