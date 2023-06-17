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
pub enum JsDEBUG {
    E_JsDEBUG_MISMATCHED_RUNTIME,
    E_JsDEBUG_UNKNOWN_THREAD,
    E_JsDEBUG_OUTSIDE_OF_VM,
    E_JsDEBUG_INVALID_MEMORY_ADDRESS,
    E_JsDEBUG_SOURCE_LOCATION_NOT_FOUND,
    E_JsDEBUG_RUNTIME_NOT_IN_DEBUG_MODE,
}

impl JsDEBUG {
    pub fn code(&self) -> u32 {
        return match self {
            JsDEBUG::E_JsDEBUG_MISMATCHED_RUNTIME => 0x8DC70001 as u32,
            JsDEBUG::E_JsDEBUG_UNKNOWN_THREAD => 0x8DC70002 as u32,
            JsDEBUG::E_JsDEBUG_OUTSIDE_OF_VM => 0x8DC70004 as u32,
            JsDEBUG::E_JsDEBUG_INVALID_MEMORY_ADDRESS => 0x8DC70005 as u32,
            JsDEBUG::E_JsDEBUG_SOURCE_LOCATION_NOT_FOUND => 0x8DC70006 as u32,
            JsDEBUG::E_JsDEBUG_RUNTIME_NOT_IN_DEBUG_MODE => 0x8DC70007 as u32,
        }
    }

    pub fn error(&self) -> RawError {
        return match self {
            JsDEBUG::E_JsDEBUG_MISMATCHED_RUNTIME => RawError::Kind(JsDEBUG::E_JsDEBUG_MISMATCHED_RUNTIME),
            JsDEBUG::E_JsDEBUG_UNKNOWN_THREAD => RawError::Kind(JsDEBUG::E_JsDEBUG_UNKNOWN_THREAD),
            JsDEBUG::E_JsDEBUG_OUTSIDE_OF_VM => RawError::Kind(JsDEBUG::E_JsDEBUG_OUTSIDE_OF_VM),
            JsDEBUG::E_JsDEBUG_INVALID_MEMORY_ADDRESS => RawError::Kind(JsDEBUG::E_JsDEBUG_INVALID_MEMORY_ADDRESS),
            JsDEBUG::E_JsDEBUG_SOURCE_LOCATION_NOT_FOUND => RawError::Kind(JsDEBUG::E_JsDEBUG_SOURCE_LOCATION_NOT_FOUND),
            JsDEBUG::E_JsDEBUG_RUNTIME_NOT_IN_DEBUG_MODE => RawError::Kind(JsDEBUG::E_JsDEBUG_RUNTIME_NOT_IN_DEBUG_MODE),
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            JsDEBUG::E_JsDEBUG_MISMATCHED_RUNTIME => "The Js runtime and the Js diag do not match.",
            JsDEBUG::E_JsDEBUG_UNKNOWN_THREAD => "Thread is not known to have any JS code, and will have no frames.",
            JsDEBUG::E_JsDEBUG_OUTSIDE_OF_VM => "Frame is outside of the interpreter. For example, portions of the Js dll which are logically not part of the interpreter.",
            JsDEBUG::E_JsDEBUG_INVALID_MEMORY_ADDRESS => "Specified memory address could not be written/read from",
            JsDEBUG::E_JsDEBUG_SOURCE_LOCATION_NOT_FOUND => "No source location found to bind the breakpoint",
            JsDEBUG::E_JsDEBUG_RUNTIME_NOT_IN_DEBUG_MODE => "Runtime not in debug mode",
        }
    }

    pub fn from_name(name: &str) -> JsDEBUG {
        return match name {
            "E_JsDEBUG_MISMATCHED_RUNTIME" => JsDEBUG::E_JsDEBUG_MISMATCHED_RUNTIME,
            "E_JsDEBUG_UNKNOWN_THREAD" => JsDEBUG::E_JsDEBUG_UNKNOWN_THREAD,
            "E_JsDEBUG_OUTSIDE_OF_VM" => JsDEBUG::E_JsDEBUG_OUTSIDE_OF_VM,
            "E_JsDEBUG_INVALID_MEMORY_ADDRESS" => JsDEBUG::E_JsDEBUG_INVALID_MEMORY_ADDRESS,
            "E_JsDEBUG_SOURCE_LOCATION_NOT_FOUND" => JsDEBUG::E_JsDEBUG_SOURCE_LOCATION_NOT_FOUND,
            "E_JsDEBUG_RUNTIME_NOT_IN_DEBUG_MODE" => JsDEBUG::E_JsDEBUG_RUNTIME_NOT_IN_DEBUG_MODE,
        }
    }
    pub fn from_code(code: u32) -> JsDEBUG {
        return match code {
            0x8DC70001 => JsDEBUG::E_JsDEBUG_MISMATCHED_RUNTIME,
            0x8DC70002 => JsDEBUG::E_JsDEBUG_UNKNOWN_THREAD,
            0x8DC70004 => JsDEBUG::E_JsDEBUG_OUTSIDE_OF_VM,
            0x8DC70005 => JsDEBUG::E_JsDEBUG_INVALID_MEMORY_ADDRESS,
            0x8DC70006 => JsDEBUG::E_JsDEBUG_SOURCE_LOCATION_NOT_FOUND,
            0x8DC70007 => JsDEBUG::E_JsDEBUG_RUNTIME_NOT_IN_DEBUG_MODE,
        }
    }
}
