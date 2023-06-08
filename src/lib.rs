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

use std::error::Error;
// use repr_bitpacked::Repr;
use std::fmt::{Debug, Display, Formatter};

#[cfg(target_os = "tvos")]
mod AppleTVOS;

#[cfg(target_os = "ios")]
mod iPhoneOS;

#[cfg(target_os = "linux")]
mod Linux;

#[cfg(target_vendor = "apple")]
mod Darwin;

#[cfg(any(feature = "watchos"))]
mod WatchOS;

#[cfg(any(feature = "windows"))]
mod Windows;

macro_rules! import_oserror {
    ($os:ident) => {
        pub use self::$os::OSError;
    };
}

#[cfg(target_os = "tvos")]
import_oserror!(AppleTVOS);

#[cfg(target_os = "ios")]
import_oserror!(iPhoneOS);

#[cfg(target_os = "linux")]
import_oserror!(Linux);

#[cfg(target_vendor = "apple")]
import_oserror!(Darwin);

#[cfg(any(feature = "watchos"))]
import_oserror!(WatchOS);

#[cfg(any(feature = "windows"))]
import_oserror!(Windows);
#[repr(align(4))]
#[derive(Debug)]
// pub(crate) struct SimpleMessage {
pub struct SimpleMessage {
    pub kind: OSError,
    pub message: &'static str,
}

pub enum ErrorData {
    Os(u8),
    Simple(OSError),
    SimpleMessage(&'static SimpleMessage)
}

impl Debug for ErrorData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.description(), f)
    }
}

impl Display for ErrorData {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorData::Os(code) => {
                let detail = OSError::kind_from_code(code);
                write!(fmt, "{detail} (os error {code})")
            }
            ErrorData::Simple(kind) => write!(fmt, "{}", *kind),
            ErrorData::SimpleMessage(msg) => std::fmt::Display::fmt(&msg.message, fmt),
            // ErrorData::SimpleMessage(msg) => **msg.message.fmt(fmt),
        }
    }
}

impl std::error::Error for ErrorData {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ErrorData::Os(..) => None,
            ErrorData::Simple(..) => None,
            ErrorData::SimpleMessage(..) => None
        }
    }

    fn description(&self) -> &str {
        match self {
            ErrorData::Os(code) => OSError::kind_from_code(code).description(),
            ErrorData::Simple(kind) => kind.description(),
            ErrorData::SimpleMessage(msg) => msg.message
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        match self {
            ErrorData::Os(..) => None,
            ErrorData::Simple(..) => None,
            ErrorData::SimpleMessage(..) => None
        }
    }
}

impl std::fmt::Display for OSError {
    /// Shows readable description of the `OSError`.
    ///
    /// # Examples
    /// ```
    /// use OSEXave::OSError;
    /// assert_eq!("Operation not permitted", OSError::EPERM.to_string());
    /// ```
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        fmt.write_str(self.description())
    }
}