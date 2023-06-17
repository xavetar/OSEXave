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

pub use types::{RawError};

#[cfg(target_os = "linux")]
use platforms::Linux;

#[cfg(target_vendor = "apple")]
use platforms::Darwin;

#[cfg(target_os = "windows")]
use platforms::Windows::{
    TPM_SERVICES, XAUDIO2, ONLINE_ID, SQLITE, NTDSB, MSMQ, WIN32, USERMODE_VIRTUALIZATION, NULL,
    SMB, MBN_POWERSHELL, JSCRIPT, DISPATCH, USERMODE_HYPERVISOR, XPS, DXGI, WDS_CONTENT_PROVIDER,
    UI, JsDEBUG, FVE, ITF, WINDOWSUPDATE, WINDOWS_SETUP_PLA, WEP, CONTROL, TIERING, XACTENGINE,
    DXCORE, INTERNET, QUIC, SDIAG, WINDOWS_STORE, USER_MODE_SECURITY_CORE, WINDOWS_CE, P2P,
    USERMODE_VHD, RDDB, HSP_SERVICES, MCA, DEBUGGERS, BLUETOOTH_ATT, DIRECT3D11, CONFIGURATION,
    USERMODE_HNS, USERMODE_VOLSNAP, SETUPAPI, GRAPHICS, EAP, DIRECT_DRAW, USERMODE_SDBUS,
    DIRECT3D10, WEB, LEAP, SCARD, BACKUP, SHARED_VHDX, OPC, SB, NDIS, WINML, SHELL_NAP, WINRM,
    WDS_MC_SERVER, XAPO, EAS, RTC_INTERFACE, USERMODE_VOLMGR, STATEREPOSITORY, FWP, BCD, IMAPI2,
    USERMODE_LICENSING, WINDOWS, UTC, AUDIO, P2P_INT, XAML, DLT, HSP_SOFTWARE, IORING,
    PINT_STATUS_CODE, DXGI_DDI, DEFRAG, DIRECT3D12, STORAGE, HTTP, INPUT, SYNCENGINE, WDS_MC_CLIENT,
    WINDOWS_DEFENDER, WEBSERVICES_WINPE, MEDIASERVER, SIP_STATUS_CODE, COMPLUS, WEB_SOCKET,
    USERMODE_FILTER_MANAGER, D2D, PRESENTATION, AUDCLNT, URT, TPM_SOFTWARE, DIRECTMUSIC,
    USERMODE_SPACES, WPN, WER, WINCODEC_DWRITE_DWM, CERT, DIRECT3D11_DEBUG, WDS_TM, RPC,
    SECURITY_SSPI
};

macro_rules! import_oserror {
    ($os:ident) => {
        pub use self::platforms::$os::OSError;
    };
}

#[cfg(target_os = "linux")]
import_oserror!(Linux);

#[cfg(target_vendor = "apple")]
import_oserror!(Darwin);

#[cfg(any(target_os = "linux", target_vendor = "apple"))]
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
