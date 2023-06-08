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

use super::{OSError};

pub struct OSErrorDetails {
    code: u32,
    kind: OSError,
    description: &'static str
}

impl OSErrorDetails {
    pub fn new(code: u16, kind: OSError, description: &'static str) -> OSErrorDetails {
        return OSErrorDetails {
            code: code,
            kind: kind,
            description: description,
        }
    }

    pub fn code(&self) -> &u32 {
        return &self.code;
    }

    pub fn kind(&self) -> &OSError {
        return &self.kind;
    }

    pub fn description(&self) -> &'static str {
        return &self.description;
    }
}
