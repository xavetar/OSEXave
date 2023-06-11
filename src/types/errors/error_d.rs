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

pub enum RawError {
    /// Convert raw error code to `OSError`.
    ///
    /// # Examples
    /// ```
    /// use OSEXave::RawError;
    /// assert_eq!("RawOSError { code: 1, kind: EPERM, description: 'Operation not permitted' }", RawError::RawOSError(1));
    /// ```
    ///
    /// Return raw error code.
    ///
    /// # Examples
    /// ```
    /// use OSEXave::RawError;
    /// assert_eq!("RawOSError { code: 1, kind: EPERM, description: 'Operation not permitted' }", RawError::RawOSError(1));
    /// ```
    RawOSError(u32),
    Kind(OSError)
}

impl Error for RawError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            RawError::RawOSError(..) => None,
            RawError::Kind(..) => None
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match self {
            RawError::RawOSError(..) => None,
            RawError::Kind(..) => None
        }
    }
}

impl Debug for RawError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        match self {
            RawError::RawOSError(code) => {
                let kind = OSError::kind_from_code(code);
                fmt.debug_struct("RawOSError")
                    .field("code", &code)
                    .field("kind", &kind)
                    .field("description", &kind.description())
                    .finish()
            }
            RawError::Kind(kind) => {
                write!(fmt, "{}", *kind)
            }
        }
    }
}

impl Display for RawError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        match self {
            RawError::RawOSError(code) => {
                let detail = OSError::kind_from_code(code).description();
                write!(fmt, "{detail} (os error {code})")
            }
            RawError::Kind(kind) => {
                write!(fmt, "{}", *kind)
            }
        }
    }
}
