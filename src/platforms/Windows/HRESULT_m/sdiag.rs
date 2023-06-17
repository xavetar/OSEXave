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
pub enum SDIAG {
    SDIAG_S_CANNOTRUN,
    SDIAG_E_CANCELLED,
    SDIAG_E_SCRIPT,
    SDIAG_E_POWERSHELL,
    SDIAG_E_MANAGEDHOST,
    SDIAG_E_NOVERIFIER,
    SDIAG_E_DISABLED,
    SDIAG_E_TRUST,
    SDIAG_E_CANNOTRUN,
    SDIAG_E_VERSION,
    SDIAG_E_RESOURCE,
    SDIAG_E_ROOTCAUSE,
}

impl SDIAG {
    pub fn code(&self) -> u32 {
        return match self {
            SDIAG::SDIAG_S_CANNOTRUN => 0x003C0105 as u32,
            SDIAG::SDIAG_E_CANCELLED => 0x803C0100 as u32,
            SDIAG::SDIAG_E_SCRIPT => 0x803C0101 as u32,
            SDIAG::SDIAG_E_POWERSHELL => 0x803C0102 as u32,
            SDIAG::SDIAG_E_MANAGEDHOST => 0x803C0103 as u32,
            SDIAG::SDIAG_E_NOVERIFIER => 0x803C0104 as u32,
            SDIAG::SDIAG_E_DISABLED => 0x803C0106 as u32,
            SDIAG::SDIAG_E_TRUST => 0x803C0107 as u32,
            SDIAG::SDIAG_E_CANNOTRUN => 0x803C0108 as u32,
            SDIAG::SDIAG_E_VERSION => 0x803C0109 as u32,
            SDIAG::SDIAG_E_RESOURCE => 0x803C010A as u32,
            SDIAG::SDIAG_E_ROOTCAUSE => 0x803C010B as u32,
        }
    }

    pub fn error(&self) -> RawError {
        return match self {
            SDIAG::SDIAG_S_CANNOTRUN => RawError::Kind(SDIAG::SDIAG_S_CANNOTRUN),
            SDIAG::SDIAG_E_CANCELLED => RawError::Kind(SDIAG::SDIAG_E_CANCELLED),
            SDIAG::SDIAG_E_SCRIPT => RawError::Kind(SDIAG::SDIAG_E_SCRIPT),
            SDIAG::SDIAG_E_POWERSHELL => RawError::Kind(SDIAG::SDIAG_E_POWERSHELL),
            SDIAG::SDIAG_E_MANAGEDHOST => RawError::Kind(SDIAG::SDIAG_E_MANAGEDHOST),
            SDIAG::SDIAG_E_NOVERIFIER => RawError::Kind(SDIAG::SDIAG_E_NOVERIFIER),
            SDIAG::SDIAG_E_DISABLED => RawError::Kind(SDIAG::SDIAG_E_DISABLED),
            SDIAG::SDIAG_E_TRUST => RawError::Kind(SDIAG::SDIAG_E_TRUST),
            SDIAG::SDIAG_E_CANNOTRUN => RawError::Kind(SDIAG::SDIAG_E_CANNOTRUN),
            SDIAG::SDIAG_E_VERSION => RawError::Kind(SDIAG::SDIAG_E_VERSION),
            SDIAG::SDIAG_E_RESOURCE => RawError::Kind(SDIAG::SDIAG_E_RESOURCE),
            SDIAG::SDIAG_E_ROOTCAUSE => RawError::Kind(SDIAG::SDIAG_E_ROOTCAUSE),
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            SDIAG::SDIAG_S_CANNOTRUN => "The troubleshooting pack cannot be executed on this system.",
            SDIAG::SDIAG_E_CANCELLED => "The operation was cancelled.",
            SDIAG::SDIAG_E_SCRIPT => "An error occurred when running a PowerShell script.",
            SDIAG::SDIAG_E_POWERSHELL => "An error occurred when interacting with PowerShell runtime.",
            SDIAG::SDIAG_E_MANAGEDHOST => "An error occurred in the Scripted Diagnostic Managed Host.",
            SDIAG::SDIAG_E_NOVERIFIER => "The troubleshooting pack does not contain a required verifier to complete the verification.",
            SDIAG::SDIAG_E_DISABLED => "Scripted diagnostics is disabled by group policy.",
            SDIAG::SDIAG_E_TRUST => "Trust validation of the troubleshooting pack failed.",
            SDIAG::SDIAG_E_CANNOTRUN => "The troubleshooting pack cannot be executed on this system.",
            SDIAG::SDIAG_E_VERSION => "This version of the troubleshooting pack is not supported.",
            SDIAG::SDIAG_E_RESOURCE => "A required resource cannot be loaded.",
            SDIAG::SDIAG_E_ROOTCAUSE => "The troubleshooting pack reported information for a root cause without adding the root cause.",
        }
    }

    pub fn from_name(name: &str) -> SDIAG {
        return match name {
            "SDIAG_S_CANNOTRUN" => SDIAG::SDIAG_S_CANNOTRUN,
            "SDIAG_E_CANCELLED" => SDIAG::SDIAG_E_CANCELLED,
            "SDIAG_E_SCRIPT" => SDIAG::SDIAG_E_SCRIPT,
            "SDIAG_E_POWERSHELL" => SDIAG::SDIAG_E_POWERSHELL,
            "SDIAG_E_MANAGEDHOST" => SDIAG::SDIAG_E_MANAGEDHOST,
            "SDIAG_E_NOVERIFIER" => SDIAG::SDIAG_E_NOVERIFIER,
            "SDIAG_E_DISABLED" => SDIAG::SDIAG_E_DISABLED,
            "SDIAG_E_TRUST" => SDIAG::SDIAG_E_TRUST,
            "SDIAG_E_CANNOTRUN" => SDIAG::SDIAG_E_CANNOTRUN,
            "SDIAG_E_VERSION" => SDIAG::SDIAG_E_VERSION,
            "SDIAG_E_RESOURCE" => SDIAG::SDIAG_E_RESOURCE,
            "SDIAG_E_ROOTCAUSE" => SDIAG::SDIAG_E_ROOTCAUSE,
        }
    }
    pub fn from_code(code: u32) -> SDIAG {
        return match code {
            0x003C0105 => SDIAG::SDIAG_S_CANNOTRUN,
            0x803C0100 => SDIAG::SDIAG_E_CANCELLED,
            0x803C0101 => SDIAG::SDIAG_E_SCRIPT,
            0x803C0102 => SDIAG::SDIAG_E_POWERSHELL,
            0x803C0103 => SDIAG::SDIAG_E_MANAGEDHOST,
            0x803C0104 => SDIAG::SDIAG_E_NOVERIFIER,
            0x803C0106 => SDIAG::SDIAG_E_DISABLED,
            0x803C0107 => SDIAG::SDIAG_E_TRUST,
            0x803C0108 => SDIAG::SDIAG_E_CANNOTRUN,
            0x803C0109 => SDIAG::SDIAG_E_VERSION,
            0x803C010A => SDIAG::SDIAG_E_RESOURCE,
            0x803C010B => SDIAG::SDIAG_E_ROOTCAUSE,
        }
    }
}
