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
pub enum WEP {
    WEP_E_NOT_PROVISIONED_ON_ALL_VOLUMES,
    WEP_E_FIXED_DATA_NOT_SUPPORTED,
    WEP_E_HARDWARE_NOT_COMPLIANT,
    WEP_E_LOCK_NOT_CONFIGURED,
    WEP_E_PROTECTION_SUSPENDED,
    WEP_E_NO_LICENSE,
    WEP_E_OS_NOT_PROTECTED,
    WEP_E_UNEXPECTED_FAIL,
    WEP_E_BUFFER_TOO_LARGE,
}

impl WEP {
    pub fn code(&self) -> u32 {
        return match self {
            WEP::WEP_E_NOT_PROVISIONED_ON_ALL_VOLUMES => 0x88010001 as u32,
            WEP::WEP_E_FIXED_DATA_NOT_SUPPORTED => 0x88010002 as u32,
            WEP::WEP_E_HARDWARE_NOT_COMPLIANT => 0x88010003 as u32,
            WEP::WEP_E_LOCK_NOT_CONFIGURED => 0x88010004 as u32,
            WEP::WEP_E_PROTECTION_SUSPENDED => 0x88010005 as u32,
            WEP::WEP_E_NO_LICENSE => 0x88010006 as u32,
            WEP::WEP_E_OS_NOT_PROTECTED => 0x88010007 as u32,
            WEP::WEP_E_UNEXPECTED_FAIL => 0x88010008 as u32,
            WEP::WEP_E_BUFFER_TOO_LARGE => 0x88010009 as u32,
        }
    }

    pub fn error(&self) -> RawError {
        return match self {
            WEP::WEP_E_NOT_PROVISIONED_ON_ALL_VOLUMES => RawError::Kind(WEP::WEP_E_NOT_PROVISIONED_ON_ALL_VOLUMES),
            WEP::WEP_E_FIXED_DATA_NOT_SUPPORTED => RawError::Kind(WEP::WEP_E_FIXED_DATA_NOT_SUPPORTED),
            WEP::WEP_E_HARDWARE_NOT_COMPLIANT => RawError::Kind(WEP::WEP_E_HARDWARE_NOT_COMPLIANT),
            WEP::WEP_E_LOCK_NOT_CONFIGURED => RawError::Kind(WEP::WEP_E_LOCK_NOT_CONFIGURED),
            WEP::WEP_E_PROTECTION_SUSPENDED => RawError::Kind(WEP::WEP_E_PROTECTION_SUSPENDED),
            WEP::WEP_E_NO_LICENSE => RawError::Kind(WEP::WEP_E_NO_LICENSE),
            WEP::WEP_E_OS_NOT_PROTECTED => RawError::Kind(WEP::WEP_E_OS_NOT_PROTECTED),
            WEP::WEP_E_UNEXPECTED_FAIL => RawError::Kind(WEP::WEP_E_UNEXPECTED_FAIL),
            WEP::WEP_E_BUFFER_TOO_LARGE => RawError::Kind(WEP::WEP_E_BUFFER_TOO_LARGE),
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            WEP::WEP_E_NOT_PROVISIONED_ON_ALL_VOLUMES => "One or more fixed volumes are not provisioned with the 3rd party encryption providers to support device encryption. Enable encryption with the 3rd party provider to comply with policy.",
            WEP::WEP_E_FIXED_DATA_NOT_SUPPORTED => "This computer is not fully encrypted. There are fixed volumes present which are not supported for encryption.",
            WEP::WEP_E_HARDWARE_NOT_COMPLIANT => "This computer does not meet the hardware requirements to support device encryption with the installed 3rd party provider.",
            WEP::WEP_E_LOCK_NOT_CONFIGURED => "This computer cannot support device encryption because the requisites for the device lock feature are not configured.",
            WEP::WEP_E_PROTECTION_SUSPENDED => "Protection is enabled on this volume but is not in the active state.",
            WEP::WEP_E_NO_LICENSE => "The 3rd party provider has been installed, but cannot activate encryption because a license has not been activated.",
            WEP::WEP_E_OS_NOT_PROTECTED => "The operating system drive is not protected by 3rd party drive encryption.",
            WEP::WEP_E_UNEXPECTED_FAIL => "Unexpected failure was encountered while calling into the 3rd Party drive encryption plugin.",
            WEP::WEP_E_BUFFER_TOO_LARGE => "The input buffer size for the lockout metadata used by the 3rd party drive encryption is too large.",
        }
    }

    pub fn from_name(name: &str) -> WEP {
        return match name {
            "WEP_E_NOT_PROVISIONED_ON_ALL_VOLUMES" => WEP::WEP_E_NOT_PROVISIONED_ON_ALL_VOLUMES,
            "WEP_E_FIXED_DATA_NOT_SUPPORTED" => WEP::WEP_E_FIXED_DATA_NOT_SUPPORTED,
            "WEP_E_HARDWARE_NOT_COMPLIANT" => WEP::WEP_E_HARDWARE_NOT_COMPLIANT,
            "WEP_E_LOCK_NOT_CONFIGURED" => WEP::WEP_E_LOCK_NOT_CONFIGURED,
            "WEP_E_PROTECTION_SUSPENDED" => WEP::WEP_E_PROTECTION_SUSPENDED,
            "WEP_E_NO_LICENSE" => WEP::WEP_E_NO_LICENSE,
            "WEP_E_OS_NOT_PROTECTED" => WEP::WEP_E_OS_NOT_PROTECTED,
            "WEP_E_UNEXPECTED_FAIL" => WEP::WEP_E_UNEXPECTED_FAIL,
            "WEP_E_BUFFER_TOO_LARGE" => WEP::WEP_E_BUFFER_TOO_LARGE,
        }
    }
    pub fn from_code(code: u32) -> WEP {
        return match code {
            0x88010001 => WEP::WEP_E_NOT_PROVISIONED_ON_ALL_VOLUMES,
            0x88010002 => WEP::WEP_E_FIXED_DATA_NOT_SUPPORTED,
            0x88010003 => WEP::WEP_E_HARDWARE_NOT_COMPLIANT,
            0x88010004 => WEP::WEP_E_LOCK_NOT_CONFIGURED,
            0x88010005 => WEP::WEP_E_PROTECTION_SUSPENDED,
            0x88010006 => WEP::WEP_E_NO_LICENSE,
            0x88010007 => WEP::WEP_E_OS_NOT_PROTECTED,
            0x88010008 => WEP::WEP_E_UNEXPECTED_FAIL,
            0x88010009 => WEP::WEP_E_BUFFER_TOO_LARGE,
        }
    }
}
