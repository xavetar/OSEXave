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

use super::{Error};
use super::{OSError};
use super::{Debug, Display, Formatter, Result};

pub enum RawErrorData {
    /// Convert raw error code to `OSError`.
    ///
    /// # Examples
    /// ```
    /// use OSEXave::RawErrorData;
    /// assert_eq!("RawOSError { code: 1, kind: EPERM, description: 'Operation not permitted' }", RawErrorData::RawOSError(1));
    /// ```
    RawOSError(u32),
    Kind(OSError)
}

impl Error for RawErrorData {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            RawErrorData::RawOSError(..) => None,
            RawErrorData::Kind(..) => None
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match self {
            RawErrorData::RawOSError(..) => None,
            RawErrorData::Kind(..) => None
        }
    }
}

impl Debug for RawErrorData {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        match self {
            RawErrorData::RawOSError(code) => {
                let kind = OSError::kind_from_code(code);
                fmt.debug_struct("RawOSError")
                    .field("code", &code)
                    .field("kind", &kind)
                    .field("description", &kind.description())
                    .finish()
            }
            RawErrorData::Kind(kind) => {
                write!(fmt, "{}", *kind)
            }
        }
    }
}

impl Display for RawErrorData {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        match self {
            RawErrorData::RawOSError(code) => {
                let kind = OSError::kind_from_code(code);
                fmt.debug_struct("RawOSError")
                    .field("code", &code)
                    .field("kind", &kind)
                    .field("description", &kind.description())
                    .finish()
            }
            RawErrorData::Kind(kind) => {
                write!(fmt, "{}", *kind)
            }
        }
    }
}
