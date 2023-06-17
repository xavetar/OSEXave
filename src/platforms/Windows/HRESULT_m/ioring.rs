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
pub enum IORING {
    IORING_E_REQUIRED_FLAG_NOT_SUPPORTED,
    IORING_E_SUBMISSION_QUEUE_FULL,
    IORING_E_VERSION_NOT_SUPPORTED,
    IORING_E_SUBMISSION_QUEUE_TOO_BIG,
    IORING_E_COMPLETION_QUEUE_TOO_BIG,
    IORING_E_SUBMIT_IN_PROGRESS,
    IORING_E_CORRUPT,
    IORING_E_COMPLETION_QUEUE_TOO_FULL,
}

impl IORING {
    pub fn code(&self) -> u32 {
        return match self {
            IORING::IORING_E_REQUIRED_FLAG_NOT_SUPPORTED => 0x80460001 as u32,
            IORING::IORING_E_SUBMISSION_QUEUE_FULL => 0x80460002 as u32,
            IORING::IORING_E_VERSION_NOT_SUPPORTED => 0x80460003 as u32,
            IORING::IORING_E_SUBMISSION_QUEUE_TOO_BIG => 0x80460004 as u32,
            IORING::IORING_E_COMPLETION_QUEUE_TOO_BIG => 0x80460005 as u32,
            IORING::IORING_E_SUBMIT_IN_PROGRESS => 0x80460006 as u32,
            IORING::IORING_E_CORRUPT => 0x80460007 as u32,
            IORING::IORING_E_COMPLETION_QUEUE_TOO_FULL => 0x80460008 as u32,
        }
    }

    pub fn error(&self) -> RawError {
        return match self {
            IORING::IORING_E_REQUIRED_FLAG_NOT_SUPPORTED => RawError::Kind(IORING::IORING_E_REQUIRED_FLAG_NOT_SUPPORTED),
            IORING::IORING_E_SUBMISSION_QUEUE_FULL => RawError::Kind(IORING::IORING_E_SUBMISSION_QUEUE_FULL),
            IORING::IORING_E_VERSION_NOT_SUPPORTED => RawError::Kind(IORING::IORING_E_VERSION_NOT_SUPPORTED),
            IORING::IORING_E_SUBMISSION_QUEUE_TOO_BIG => RawError::Kind(IORING::IORING_E_SUBMISSION_QUEUE_TOO_BIG),
            IORING::IORING_E_COMPLETION_QUEUE_TOO_BIG => RawError::Kind(IORING::IORING_E_COMPLETION_QUEUE_TOO_BIG),
            IORING::IORING_E_SUBMIT_IN_PROGRESS => RawError::Kind(IORING::IORING_E_SUBMIT_IN_PROGRESS),
            IORING::IORING_E_CORRUPT => RawError::Kind(IORING::IORING_E_CORRUPT),
            IORING::IORING_E_COMPLETION_QUEUE_TOO_FULL => RawError::Kind(IORING::IORING_E_COMPLETION_QUEUE_TOO_FULL),
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            IORING::IORING_E_REQUIRED_FLAG_NOT_SUPPORTED => "One or more of the required flags provided is unknown by the implementation.",
            IORING::IORING_E_SUBMISSION_QUEUE_FULL => "The submission queue is full.",
            IORING::IORING_E_VERSION_NOT_SUPPORTED => "The version specified is not known or supported.",
            IORING::IORING_E_SUBMISSION_QUEUE_TOO_BIG => "The submission queue size specified for the IoRing is too big.",
            IORING::IORING_E_COMPLETION_QUEUE_TOO_BIG => "The completion queue size specified for the IoRing is too big.",
            IORING::IORING_E_SUBMIT_IN_PROGRESS => "A submit operation is already in progress for this IoRing on another thread.",
            IORING::IORING_E_CORRUPT => "The shared ring buffers of the IoRing are corrupt.",
            IORING::IORING_E_COMPLETION_QUEUE_TOO_FULL => "The completion queue does not have enough free space, to post completions, for all entries being submitted.",
        }
    }

    pub fn from_name(name: &str) -> IORING {
        return match name {
            "IORING_E_REQUIRED_FLAG_NOT_SUPPORTED" => IORING::IORING_E_REQUIRED_FLAG_NOT_SUPPORTED,
            "IORING_E_SUBMISSION_QUEUE_FULL" => IORING::IORING_E_SUBMISSION_QUEUE_FULL,
            "IORING_E_VERSION_NOT_SUPPORTED" => IORING::IORING_E_VERSION_NOT_SUPPORTED,
            "IORING_E_SUBMISSION_QUEUE_TOO_BIG" => IORING::IORING_E_SUBMISSION_QUEUE_TOO_BIG,
            "IORING_E_COMPLETION_QUEUE_TOO_BIG" => IORING::IORING_E_COMPLETION_QUEUE_TOO_BIG,
            "IORING_E_SUBMIT_IN_PROGRESS" => IORING::IORING_E_SUBMIT_IN_PROGRESS,
            "IORING_E_CORRUPT" => IORING::IORING_E_CORRUPT,
            "IORING_E_COMPLETION_QUEUE_TOO_FULL" => IORING::IORING_E_COMPLETION_QUEUE_TOO_FULL,
        }
    }
    pub fn from_code(code: u32) -> IORING {
        return match code {
            0x80460001 => IORING::IORING_E_REQUIRED_FLAG_NOT_SUPPORTED,
            0x80460002 => IORING::IORING_E_SUBMISSION_QUEUE_FULL,
            0x80460003 => IORING::IORING_E_VERSION_NOT_SUPPORTED,
            0x80460004 => IORING::IORING_E_SUBMISSION_QUEUE_TOO_BIG,
            0x80460005 => IORING::IORING_E_COMPLETION_QUEUE_TOO_BIG,
            0x80460006 => IORING::IORING_E_SUBMIT_IN_PROGRESS,
            0x80460007 => IORING::IORING_E_CORRUPT,
            0x80460008 => IORING::IORING_E_COMPLETION_QUEUE_TOO_FULL,
        }
    }
}
