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
pub enum JSCRIPT {
    JSCRIPT_E_CANTEXECUTE,
}

impl JSCRIPT {
    pub fn code(&self) -> u32 {
        return match self {
            JSCRIPT::JSCRIPT_E_CANTEXECUTE => 0x89020001 as u32,
        }
    }

    pub fn error(&self) -> RawError {
        return match self {
            JSCRIPT::JSCRIPT_E_CANTEXECUTE => RawError::Kind(JSCRIPT::JSCRIPT_E_CANTEXECUTE),
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            JSCRIPT::JSCRIPT_E_CANTEXECUTE => "Function could not execute because it was deleted or garbage collected.",
        }
    }

    pub fn from_name(name: &str) -> JSCRIPT {
        return match name {
            "JSCRIPT_E_CANTEXECUTE" => JSCRIPT::JSCRIPT_E_CANTEXECUTE,
        }
    }
    pub fn from_code(code: u32) -> JSCRIPT {
        return match code {
            0x89020001 => JSCRIPT::JSCRIPT_E_CANTEXECUTE,
        }
    }
}
