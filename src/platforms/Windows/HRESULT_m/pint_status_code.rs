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

pub enum PINT_STATUS_CODE {
    RTC_E_PINT_STATUS_REJECTED_BUSY = 0x80F00005,
    RTC_E_PINT_STATUS_REJECTED_NO_ANSWER = 0x80F00006,
    RTC_E_PINT_STATUS_REJECTED_ALL_BUSY = 0x80F00007,
    RTC_E_PINT_STATUS_REJECTED_PL_FAILED = 0x80F00008,
    RTC_E_PINT_STATUS_REJECTED_SW_FAILED = 0x80F00009,
    RTC_E_PINT_STATUS_REJECTED_CANCELLED = 0x80F0000A,
    RTC_E_PINT_STATUS_REJECTED_BADNUMBER = 0x80F0000B,
}

impl PINT_STATUS_CODE {
    pub fn description(&self) -> &'static str {
        match self {
            PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_BUSY => "Busy",
            PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_NO_ANSWER => "No Answer",
            PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_ALL_BUSY => "All Busy",
            PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_PL_FAILED => "Primary Leg Failed",
            PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_SW_FAILED => "Switch Failed",
            PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_CANCELLED => "Cancelled",
            PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_BADNUMBER => "Bad Number",
        }
    }
}
