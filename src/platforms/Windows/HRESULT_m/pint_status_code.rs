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
pub enum PINT_STATUS_CODE {
    RTC_E_PINT_STATUS_REJECTED_BUSY,
    RTC_E_PINT_STATUS_REJECTED_NO_ANSWER,
    RTC_E_PINT_STATUS_REJECTED_ALL_BUSY,
    RTC_E_PINT_STATUS_REJECTED_PL_FAILED,
    RTC_E_PINT_STATUS_REJECTED_SW_FAILED,
    RTC_E_PINT_STATUS_REJECTED_CANCELLED,
    RTC_E_PINT_STATUS_REJECTED_BADNUMBER,
}

impl PINT_STATUS_CODE {
    pub fn code(&self) -> u32 {
        return match self {
            PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_BUSY => 0x80F00005 as u32,
            PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_NO_ANSWER => 0x80F00006 as u32,
            PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_ALL_BUSY => 0x80F00007 as u32,
            PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_PL_FAILED => 0x80F00008 as u32,
            PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_SW_FAILED => 0x80F00009 as u32,
            PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_CANCELLED => 0x80F0000A as u32,
            PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_BADNUMBER => 0x80F0000B as u32,
        }
    }

    pub fn error(&self) -> RawError {
        return match self {
            PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_BUSY => RawError::Kind(PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_BUSY),
            PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_NO_ANSWER => RawError::Kind(PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_NO_ANSWER),
            PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_ALL_BUSY => RawError::Kind(PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_ALL_BUSY),
            PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_PL_FAILED => RawError::Kind(PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_PL_FAILED),
            PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_SW_FAILED => RawError::Kind(PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_SW_FAILED),
            PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_CANCELLED => RawError::Kind(PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_CANCELLED),
            PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_BADNUMBER => RawError::Kind(PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_BADNUMBER),
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_BUSY => "Busy",
            PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_NO_ANSWER => "No Answer",
            PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_ALL_BUSY => "All Busy",
            PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_PL_FAILED => "Primary Leg Failed",
            PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_SW_FAILED => "Switch Failed",
            PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_CANCELLED => "Cancelled",
            PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_BADNUMBER => "Bad Number",
        }
    }

    pub fn from_name(name: &str) -> PINT_STATUS_CODE {
        return match name {
            "RTC_E_PINT_STATUS_REJECTED_BUSY" => PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_BUSY,
            "RTC_E_PINT_STATUS_REJECTED_NO_ANSWER" => PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_NO_ANSWER,
            "RTC_E_PINT_STATUS_REJECTED_ALL_BUSY" => PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_ALL_BUSY,
            "RTC_E_PINT_STATUS_REJECTED_PL_FAILED" => PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_PL_FAILED,
            "RTC_E_PINT_STATUS_REJECTED_SW_FAILED" => PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_SW_FAILED,
            "RTC_E_PINT_STATUS_REJECTED_CANCELLED" => PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_CANCELLED,
            "RTC_E_PINT_STATUS_REJECTED_BADNUMBER" => PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_BADNUMBER,
        }
    }
    pub fn from_code(code: u32) -> PINT_STATUS_CODE {
        return match code {
            0x80F00005 => PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_BUSY,
            0x80F00006 => PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_NO_ANSWER,
            0x80F00007 => PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_ALL_BUSY,
            0x80F00008 => PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_PL_FAILED,
            0x80F00009 => PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_SW_FAILED,
            0x80F0000A => PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_CANCELLED,
            0x80F0000B => PINT_STATUS_CODE::RTC_E_PINT_STATUS_REJECTED_BADNUMBER,
        }
    }
}
