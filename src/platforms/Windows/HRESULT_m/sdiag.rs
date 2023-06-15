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

pub enum SDIAG {
    SDIAG_S_CANNOTRUN = 0x003C0105,
    SDIAG_E_CANCELLED = 0x803C0100,
    SDIAG_E_SCRIPT = 0x803C0101,
    SDIAG_E_POWERSHELL = 0x803C0102,
    SDIAG_E_MANAGEDHOST = 0x803C0103,
    SDIAG_E_NOVERIFIER = 0x803C0104,
    SDIAG_E_DISABLED = 0x803C0106,
    SDIAG_E_TRUST = 0x803C0107,
    SDIAG_E_CANNOTRUN = 0x803C0108,
    SDIAG_E_VERSION = 0x803C0109,
    SDIAG_E_RESOURCE = 0x803C010A,
    SDIAG_E_ROOTCAUSE = 0x803C010B,
}

impl SDIAG {
    pub fn description(&self) -> &'static str {
        match self {
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
}
