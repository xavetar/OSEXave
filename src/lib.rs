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

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod platforms;
mod types;

pub use types::{RawErrorData};

#[cfg(target_os = "linux")]
use platforms::Linux;

#[cfg(target_vendor = "apple")]
use platforms::Darwin;

#[cfg(any(feature = "windows"))]
use platforms::Windows;

macro_rules! import_oserror {
    ($os:ident) => {
        pub use self::platforms::$os::OSError;
    };
}

#[cfg(target_os = "linux")]
import_oserror!(Linux);

#[cfg(target_vendor = "apple")]
import_oserror!(Darwin);

#[cfg(any(feature = "windows"))]
import_oserror!(Windows);

impl std::fmt::Display for OSError {
    /// Shows readable description of the `OSError`.
    ///
    /// # Examples
    /// ```
    /// use OSEXave::OSError;
    /// assert_eq!("Operation not permitted", OSError::EPERM);
    /// ```
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{:?}", self)
    }
}
