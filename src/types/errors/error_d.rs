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
    /// Details about raw error code.
    ///
    /// # Debug information about error:
    ///
    /// ```
    /// use OSEXave::{RawError};
    /// assert_eq!("RawOSError { code: 1, kind: EPERM, description: 'Operation not permitted' }", RawError::RawOSError(1));
    /// ```
    ///
    /// # Return error:
    ///
    /// ```
    /// use OSEXave::{OSError};
    /// use OSEXave::{RawError};
    ///
    /// fn raise() -> Result<String, RawError> {
    ///     Err(OSError::EACCES.error())
    /// }
    ///
    /// fn main() {
    ///     let test = raise().unwrap(); // thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: RawOSError { code: 13, kind: EACCES, description: "Permission denied" }'
    /// }
    /// ```
    ///
    /// # Handle std::io::RawOsError:
    ///
    /// ```
    /// use OSEXave::{OSError};
    /// use OSEXave::{RawError};
    ///
    /// fn raise_error() -> Result<String, RawError> {
    ///     Err(OSError::EACCES.error())
    /// }
    ///
    /// fn main() {
    ///     let result: String = match raise_error() {
    ///         Ok(test) => String::from("This is not work"),
    ///         Err(error) => {
    ///             match OSError::from_code(error.raw_os_error() as u32) {
    ///                 OSError::EACCES => OSError::EACCES.description().to_string(),
    ///                 _ => std::process::exit(-1)
    ///             }
    ///         }
    ///     };
    ///     assert_eq!("Permission denied", result);
    /// }
    /// ```
    /// 
    /// # Handle OSError:
    ///
    /// ```
    /// use OSEXave::{OSError};
    /// use OSEXave::{RawError};
    ///
    /// fn raise_error() -> Result<String, RawError> {
    ///     Err(OSError::EACCES.error())
    /// }
    ///
    /// fn main() {
    ///     let result: String = match raise_error() {
    ///         Ok(test) => String::from("This is not work"),
    ///         Err(error) => {
    ///             match error {
    ///                 RawError::Kind(kind) => {
    ///                     match kind {
    ///                         OSError::EACCES => kind.description().to_string(),
    ///                         _ => std::process::exit(-1)
    ///                     }
    ///                 },
    ///                 RawError::RawOSError(os_error) => {
    ///                     match os_error { _ => os_error.to_string() }
    ///                 }
    ///             }
    ///         }
    ///     };
    ///     assert_eq!("Permission denied", result);
    /// }
    /// ```
    ///
    Kind(OSError),
    RawOSError(u32)
}

impl Error for RawError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            RawError::Kind(..) => None,
            RawError::RawOSError(..) => None
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match self {
            RawError::Kind(..) => None,
            RawError::RawOSError(..) => None
        }
    }
}

impl Debug for RawError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        match self {
            RawError::Kind(kind) => {
                write!(fmt, "{}", kind.description())
            },
            RawError::RawOSError(code) => {
                let kind = OSError::from_code(*code);
                fmt.debug_struct("RawOSError")
                    .field("code", &code)
                    .field("kind", &kind)
                    .field("description", &kind.description())
                    .finish()
            }
        }
    }
}

impl Display for RawError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        match self {
            RawError::Kind(kind) => {
                write!(fmt, "{}", kind.description())
            },
            RawError::RawOSError(code) => {
                let detail = OSError::from_code(*code).description();
                write!(fmt, "{detail} (os error {code})")
            }
        }
    }
}
