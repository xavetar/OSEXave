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
pub enum PRESENTATION {
    PRESENTATION_ERROR_LOST,
}

impl PRESENTATION {
    pub fn code(&self) -> u32 {
        return match self {
            PRESENTATION::PRESENTATION_ERROR_LOST => 0x88810001 as u32,
        }
    }

    pub fn error(&self) -> RawError {
        return match self {
            PRESENTATION::PRESENTATION_ERROR_LOST => RawError::Kind(PRESENTATION::PRESENTATION_ERROR_LOST),
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            PRESENTATION::PRESENTATION_ERROR_LOST => "The presentation manager has been lost and can no longer be used. The application should destroy this presentation manager and create a new presentation manager to use.",
        }
    }

    pub fn from_name(name: &str) -> PRESENTATION {
        return match name {
            "PRESENTATION_ERROR_LOST" => PRESENTATION::PRESENTATION_ERROR_LOST,
        }
    }
    pub fn from_code(code: u32) -> PRESENTATION {
        return match code {
            0x88810001 => PRESENTATION::PRESENTATION_ERROR_LOST,
        }
    }
}
