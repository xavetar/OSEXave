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
pub enum WINDOWS_DEFENDER {
    ERROR_MP_UI_CONSOLIDATION_BASE,
    ERROR_MP_ACTIONS_FAILED,
    ERROR_MP_NOENGINE,
    ERROR_MP_ACTIVE_THREATS,
    ERROR_MP_NO_INTERNET_CONN,
    MP_ERROR_CODE_LUA_CANCELLED,
    MP_ERROR_CODE_ALREADY_SHUTDOWN,
    MP_ERROR_CODE_RDEVICE_S_ASYNC_CALL_PENDING,
    MP_ERROR_CODE_CANCELLED,
    MP_ERROR_CODE_NO_TARGETOS,
    MP_ERROR_CODE_BAD_REGEXP,
    MP_ERROR_TEST_INDUCED_ERROR,
    MP_ERROR_SIG_BACKUP_DISABLED,
    ERR_MP_BAD_INIT_MODULES,
    ERR_MP_BAD_DATABASE,
    ERR_MP_BAD_UFS,
    ERR_MP_NO_MEMORY,
    ERR_MP_BAD_INPUT_DATA,
    ERR_MP_BAD_GLOBAL_STORAGE,
    ERR_MP_OBSOLETE,
    ERR_MP_NOT_SUPPORTED,
    ERR_MP_NO_MORE_ITEMS,
    ERR_MP_DUPLICATE_SCANID,
    ERR_MP_BAD_SCANID,
    ERR_MP_BAD_USERDB_VERSION,
    ERR_MP_RESTORE_FAILED,
    ERROR_MP_FAILED_TO_SPYNET,
    ERR_MP_BAD_ACTION,
    ERR_MP_REMOVE_FAILED,
    ERR_MP_SCAN_ABORTED,
    ERR_MP_NOT_FOUND,
    ERR_MP_BAD_CONFIGURATION,
    ERR_MP_QUARANTINE_FAILED,
    ERR_MP_REBOOT_REQUIRED,
    ERR_MP_THREAT_NOT_FOUND,
    ERR_MP_FULL_SCAN_REQUIRED,
    ERR_MP_MANUAL_STEPS_REQUIRED,
    ERR_MP_REMOVE_NOT_SUPPORTED,
    ERR_MP_REMOVE_LOW_MEDIUM_DISABLED,
    ERR_MP_UNREFERENCED_8028,
    ERROR_MP_RESCAN_REQUIRED,
    ERROR_MP_CALLISTO_REQUIRED,
    ERROR_MP_PLATFORM_OUTDATED,
    ERR_RELO_BAD_EHANDLE,
    ERR_RELO_KERNEL_NOT_LOADED,
    ERR_MP_BADDB_OPEN,
    ERR_MP_BADDB_HEADER,
    ERR_MP_BADDB_OLDENGINE,
    ERR_MP_BADDB_CONTENT,
    ERR_MP_BADDB_NOTSIGNED,
}

impl WINDOWS_DEFENDER {
    pub fn code(&self) -> u32 {
        return match self {
            WINDOWS_DEFENDER::ERROR_MP_UI_CONSOLIDATION_BASE => 0x80501000 as u32,
            WINDOWS_DEFENDER::ERROR_MP_ACTIONS_FAILED => 0x80501001 as u32,
            WINDOWS_DEFENDER::ERROR_MP_NOENGINE => 0x80501002 as u32,
            WINDOWS_DEFENDER::ERROR_MP_ACTIVE_THREATS => 0x80501003 as u32,
            WINDOWS_DEFENDER::ERROR_MP_NO_INTERNET_CONN => 0x80501004 as u32,
            WINDOWS_DEFENDER::MP_ERROR_CODE_LUA_CANCELLED => 0x80501101 as u32,
            WINDOWS_DEFENDER::MP_ERROR_CODE_ALREADY_SHUTDOWN => 0x80501102 as u32,
            WINDOWS_DEFENDER::MP_ERROR_CODE_RDEVICE_S_ASYNC_CALL_PENDING => 0x80501103 as u32,
            WINDOWS_DEFENDER::MP_ERROR_CODE_CANCELLED => 0x80501104 as u32,
            WINDOWS_DEFENDER::MP_ERROR_CODE_NO_TARGETOS => 0x80501105 as u32,
            WINDOWS_DEFENDER::MP_ERROR_CODE_BAD_REGEXP => 0x80501106 as u32,
            WINDOWS_DEFENDER::MP_ERROR_TEST_INDUCED_ERROR => 0x80501107 as u32,
            WINDOWS_DEFENDER::MP_ERROR_SIG_BACKUP_DISABLED => 0x80501108 as u32,
            WINDOWS_DEFENDER::ERR_MP_BAD_INIT_MODULES => 0x80508001 as u32,
            WINDOWS_DEFENDER::ERR_MP_BAD_DATABASE => 0x80508002 as u32,
            WINDOWS_DEFENDER::ERR_MP_BAD_UFS => 0x80508004 as u32,
            WINDOWS_DEFENDER::ERR_MP_NO_MEMORY => 0x80508007 as u32,
            WINDOWS_DEFENDER::ERR_MP_BAD_INPUT_DATA => 0x8050800C as u32,
            WINDOWS_DEFENDER::ERR_MP_BAD_GLOBAL_STORAGE => 0x8050800D as u32,
            WINDOWS_DEFENDER::ERR_MP_OBSOLETE => 0x8050800E as u32,
            WINDOWS_DEFENDER::ERR_MP_NOT_SUPPORTED => 0x8050800F as u32,
            WINDOWS_DEFENDER::ERR_MP_NO_MORE_ITEMS => 0x80508010 as u32,
            WINDOWS_DEFENDER::ERR_MP_DUPLICATE_SCANID => 0x80508011 as u32,
            WINDOWS_DEFENDER::ERR_MP_BAD_SCANID => 0x80508012 as u32,
            WINDOWS_DEFENDER::ERR_MP_BAD_USERDB_VERSION => 0x80508013 as u32,
            WINDOWS_DEFENDER::ERR_MP_RESTORE_FAILED => 0x80508014 as u32,
            WINDOWS_DEFENDER::ERROR_MP_FAILED_TO_SPYNET => 0x80508015 as u32,
            WINDOWS_DEFENDER::ERR_MP_BAD_ACTION => 0x80508016 as u32,
            WINDOWS_DEFENDER::ERR_MP_REMOVE_FAILED => 0x80508017 as u32,
            WINDOWS_DEFENDER::ERR_MP_SCAN_ABORTED => 0x80508018 as u32,
            WINDOWS_DEFENDER::ERR_MP_NOT_FOUND => 0x80508019 as u32,
            WINDOWS_DEFENDER::ERR_MP_BAD_CONFIGURATION => 0x80508020 as u32,
            WINDOWS_DEFENDER::ERR_MP_QUARANTINE_FAILED => 0x80508021 as u32,
            WINDOWS_DEFENDER::ERR_MP_REBOOT_REQUIRED => 0x80508022 as u32,
            WINDOWS_DEFENDER::ERR_MP_THREAT_NOT_FOUND => 0x80508023 as u32,
            WINDOWS_DEFENDER::ERR_MP_FULL_SCAN_REQUIRED => 0x80508024 as u32,
            WINDOWS_DEFENDER::ERR_MP_MANUAL_STEPS_REQUIRED => 0x80508025 as u32,
            WINDOWS_DEFENDER::ERR_MP_REMOVE_NOT_SUPPORTED => 0x80508026 as u32,
            WINDOWS_DEFENDER::ERR_MP_REMOVE_LOW_MEDIUM_DISABLED => 0x80508027 as u32,
            WINDOWS_DEFENDER::ERR_MP_UNREFERENCED_8028 => 0x80508028 as u32,
            WINDOWS_DEFENDER::ERROR_MP_RESCAN_REQUIRED => 0x80508029 as u32,
            WINDOWS_DEFENDER::ERROR_MP_CALLISTO_REQUIRED => 0x80508030 as u32,
            WINDOWS_DEFENDER::ERROR_MP_PLATFORM_OUTDATED => 0x80508031 as u32,
            WINDOWS_DEFENDER::ERR_RELO_BAD_EHANDLE => 0x80509001 as u32,
            WINDOWS_DEFENDER::ERR_RELO_KERNEL_NOT_LOADED => 0x80509003 as u32,
            WINDOWS_DEFENDER::ERR_MP_BADDB_OPEN => 0x8050A001 as u32,
            WINDOWS_DEFENDER::ERR_MP_BADDB_HEADER => 0x8050A002 as u32,
            WINDOWS_DEFENDER::ERR_MP_BADDB_OLDENGINE => 0x8050A003 as u32,
            WINDOWS_DEFENDER::ERR_MP_BADDB_CONTENT => 0x8050A004 as u32,
            WINDOWS_DEFENDER::ERR_MP_BADDB_NOTSIGNED => 0x8050A005 as u32,
        }
    }

    pub fn error(&self) -> RawError {
        return match self {
            WINDOWS_DEFENDER::ERROR_MP_UI_CONSOLIDATION_BASE => RawError::Kind(WINDOWS_DEFENDER::ERROR_MP_UI_CONSOLIDATION_BASE),
            WINDOWS_DEFENDER::ERROR_MP_ACTIONS_FAILED => RawError::Kind(WINDOWS_DEFENDER::ERROR_MP_ACTIONS_FAILED),
            WINDOWS_DEFENDER::ERROR_MP_NOENGINE => RawError::Kind(WINDOWS_DEFENDER::ERROR_MP_NOENGINE),
            WINDOWS_DEFENDER::ERROR_MP_ACTIVE_THREATS => RawError::Kind(WINDOWS_DEFENDER::ERROR_MP_ACTIVE_THREATS),
            WINDOWS_DEFENDER::ERROR_MP_NO_INTERNET_CONN => RawError::Kind(WINDOWS_DEFENDER::ERROR_MP_NO_INTERNET_CONN),
            WINDOWS_DEFENDER::MP_ERROR_CODE_LUA_CANCELLED => RawError::Kind(WINDOWS_DEFENDER::MP_ERROR_CODE_LUA_CANCELLED),
            WINDOWS_DEFENDER::MP_ERROR_CODE_ALREADY_SHUTDOWN => RawError::Kind(WINDOWS_DEFENDER::MP_ERROR_CODE_ALREADY_SHUTDOWN),
            WINDOWS_DEFENDER::MP_ERROR_CODE_RDEVICE_S_ASYNC_CALL_PENDING => RawError::Kind(WINDOWS_DEFENDER::MP_ERROR_CODE_RDEVICE_S_ASYNC_CALL_PENDING),
            WINDOWS_DEFENDER::MP_ERROR_CODE_CANCELLED => RawError::Kind(WINDOWS_DEFENDER::MP_ERROR_CODE_CANCELLED),
            WINDOWS_DEFENDER::MP_ERROR_CODE_NO_TARGETOS => RawError::Kind(WINDOWS_DEFENDER::MP_ERROR_CODE_NO_TARGETOS),
            WINDOWS_DEFENDER::MP_ERROR_CODE_BAD_REGEXP => RawError::Kind(WINDOWS_DEFENDER::MP_ERROR_CODE_BAD_REGEXP),
            WINDOWS_DEFENDER::MP_ERROR_TEST_INDUCED_ERROR => RawError::Kind(WINDOWS_DEFENDER::MP_ERROR_TEST_INDUCED_ERROR),
            WINDOWS_DEFENDER::MP_ERROR_SIG_BACKUP_DISABLED => RawError::Kind(WINDOWS_DEFENDER::MP_ERROR_SIG_BACKUP_DISABLED),
            WINDOWS_DEFENDER::ERR_MP_BAD_INIT_MODULES => RawError::Kind(WINDOWS_DEFENDER::ERR_MP_BAD_INIT_MODULES),
            WINDOWS_DEFENDER::ERR_MP_BAD_DATABASE => RawError::Kind(WINDOWS_DEFENDER::ERR_MP_BAD_DATABASE),
            WINDOWS_DEFENDER::ERR_MP_BAD_UFS => RawError::Kind(WINDOWS_DEFENDER::ERR_MP_BAD_UFS),
            WINDOWS_DEFENDER::ERR_MP_NO_MEMORY => RawError::Kind(WINDOWS_DEFENDER::ERR_MP_NO_MEMORY),
            WINDOWS_DEFENDER::ERR_MP_BAD_INPUT_DATA => RawError::Kind(WINDOWS_DEFENDER::ERR_MP_BAD_INPUT_DATA),
            WINDOWS_DEFENDER::ERR_MP_BAD_GLOBAL_STORAGE => RawError::Kind(WINDOWS_DEFENDER::ERR_MP_BAD_GLOBAL_STORAGE),
            WINDOWS_DEFENDER::ERR_MP_OBSOLETE => RawError::Kind(WINDOWS_DEFENDER::ERR_MP_OBSOLETE),
            WINDOWS_DEFENDER::ERR_MP_NOT_SUPPORTED => RawError::Kind(WINDOWS_DEFENDER::ERR_MP_NOT_SUPPORTED),
            WINDOWS_DEFENDER::ERR_MP_NO_MORE_ITEMS => RawError::Kind(WINDOWS_DEFENDER::ERR_MP_NO_MORE_ITEMS),
            WINDOWS_DEFENDER::ERR_MP_DUPLICATE_SCANID => RawError::Kind(WINDOWS_DEFENDER::ERR_MP_DUPLICATE_SCANID),
            WINDOWS_DEFENDER::ERR_MP_BAD_SCANID => RawError::Kind(WINDOWS_DEFENDER::ERR_MP_BAD_SCANID),
            WINDOWS_DEFENDER::ERR_MP_BAD_USERDB_VERSION => RawError::Kind(WINDOWS_DEFENDER::ERR_MP_BAD_USERDB_VERSION),
            WINDOWS_DEFENDER::ERR_MP_RESTORE_FAILED => RawError::Kind(WINDOWS_DEFENDER::ERR_MP_RESTORE_FAILED),
            WINDOWS_DEFENDER::ERROR_MP_FAILED_TO_SPYNET => RawError::Kind(WINDOWS_DEFENDER::ERROR_MP_FAILED_TO_SPYNET),
            WINDOWS_DEFENDER::ERR_MP_BAD_ACTION => RawError::Kind(WINDOWS_DEFENDER::ERR_MP_BAD_ACTION),
            WINDOWS_DEFENDER::ERR_MP_REMOVE_FAILED => RawError::Kind(WINDOWS_DEFENDER::ERR_MP_REMOVE_FAILED),
            WINDOWS_DEFENDER::ERR_MP_SCAN_ABORTED => RawError::Kind(WINDOWS_DEFENDER::ERR_MP_SCAN_ABORTED),
            WINDOWS_DEFENDER::ERR_MP_NOT_FOUND => RawError::Kind(WINDOWS_DEFENDER::ERR_MP_NOT_FOUND),
            WINDOWS_DEFENDER::ERR_MP_BAD_CONFIGURATION => RawError::Kind(WINDOWS_DEFENDER::ERR_MP_BAD_CONFIGURATION),
            WINDOWS_DEFENDER::ERR_MP_QUARANTINE_FAILED => RawError::Kind(WINDOWS_DEFENDER::ERR_MP_QUARANTINE_FAILED),
            WINDOWS_DEFENDER::ERR_MP_REBOOT_REQUIRED => RawError::Kind(WINDOWS_DEFENDER::ERR_MP_REBOOT_REQUIRED),
            WINDOWS_DEFENDER::ERR_MP_THREAT_NOT_FOUND => RawError::Kind(WINDOWS_DEFENDER::ERR_MP_THREAT_NOT_FOUND),
            WINDOWS_DEFENDER::ERR_MP_FULL_SCAN_REQUIRED => RawError::Kind(WINDOWS_DEFENDER::ERR_MP_FULL_SCAN_REQUIRED),
            WINDOWS_DEFENDER::ERR_MP_MANUAL_STEPS_REQUIRED => RawError::Kind(WINDOWS_DEFENDER::ERR_MP_MANUAL_STEPS_REQUIRED),
            WINDOWS_DEFENDER::ERR_MP_REMOVE_NOT_SUPPORTED => RawError::Kind(WINDOWS_DEFENDER::ERR_MP_REMOVE_NOT_SUPPORTED),
            WINDOWS_DEFENDER::ERR_MP_REMOVE_LOW_MEDIUM_DISABLED => RawError::Kind(WINDOWS_DEFENDER::ERR_MP_REMOVE_LOW_MEDIUM_DISABLED),
            WINDOWS_DEFENDER::ERR_MP_UNREFERENCED_8028 => RawError::Kind(WINDOWS_DEFENDER::ERR_MP_UNREFERENCED_8028),
            WINDOWS_DEFENDER::ERROR_MP_RESCAN_REQUIRED => RawError::Kind(WINDOWS_DEFENDER::ERROR_MP_RESCAN_REQUIRED),
            WINDOWS_DEFENDER::ERROR_MP_CALLISTO_REQUIRED => RawError::Kind(WINDOWS_DEFENDER::ERROR_MP_CALLISTO_REQUIRED),
            WINDOWS_DEFENDER::ERROR_MP_PLATFORM_OUTDATED => RawError::Kind(WINDOWS_DEFENDER::ERROR_MP_PLATFORM_OUTDATED),
            WINDOWS_DEFENDER::ERR_RELO_BAD_EHANDLE => RawError::Kind(WINDOWS_DEFENDER::ERR_RELO_BAD_EHANDLE),
            WINDOWS_DEFENDER::ERR_RELO_KERNEL_NOT_LOADED => RawError::Kind(WINDOWS_DEFENDER::ERR_RELO_KERNEL_NOT_LOADED),
            WINDOWS_DEFENDER::ERR_MP_BADDB_OPEN => RawError::Kind(WINDOWS_DEFENDER::ERR_MP_BADDB_OPEN),
            WINDOWS_DEFENDER::ERR_MP_BADDB_HEADER => RawError::Kind(WINDOWS_DEFENDER::ERR_MP_BADDB_HEADER),
            WINDOWS_DEFENDER::ERR_MP_BADDB_OLDENGINE => RawError::Kind(WINDOWS_DEFENDER::ERR_MP_BADDB_OLDENGINE),
            WINDOWS_DEFENDER::ERR_MP_BADDB_CONTENT => RawError::Kind(WINDOWS_DEFENDER::ERR_MP_BADDB_CONTENT),
            WINDOWS_DEFENDER::ERR_MP_BADDB_NOTSIGNED => RawError::Kind(WINDOWS_DEFENDER::ERR_MP_BADDB_NOTSIGNED),
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            WINDOWS_DEFENDER::ERROR_MP_UI_CONSOLIDATION_BASE => "This is an internal error. The cause is not clearly defined.",
            WINDOWS_DEFENDER::ERROR_MP_ACTIONS_FAILED => "Description is missing",
            WINDOWS_DEFENDER::ERROR_MP_NOENGINE => "No engine is loaded in antimalware service to perform the requested operation.",
            WINDOWS_DEFENDER::ERROR_MP_ACTIVE_THREATS => "Description is missing",
            WINDOWS_DEFENDER::ERROR_MP_NO_INTERNET_CONN => "Check your Internet connection, then run the scan again.",
            WINDOWS_DEFENDER::MP_ERROR_CODE_LUA_CANCELLED => "Description is missing",
            WINDOWS_DEFENDER::MP_ERROR_CODE_ALREADY_SHUTDOWN => "Description is missing",
            WINDOWS_DEFENDER::MP_ERROR_CODE_RDEVICE_S_ASYNC_CALL_PENDING => "Description is missing",
            WINDOWS_DEFENDER::MP_ERROR_CODE_CANCELLED => "Description is missing",
            WINDOWS_DEFENDER::MP_ERROR_CODE_NO_TARGETOS => "Description is missing",
            WINDOWS_DEFENDER::MP_ERROR_CODE_BAD_REGEXP => "Description is missing",
            WINDOWS_DEFENDER::MP_ERROR_TEST_INDUCED_ERROR => "Description is missing",
            WINDOWS_DEFENDER::MP_ERROR_SIG_BACKUP_DISABLED => "Description is missing",
            WINDOWS_DEFENDER::ERR_MP_BAD_INIT_MODULES => "Description is missing",
            WINDOWS_DEFENDER::ERR_MP_BAD_DATABASE => "The digital signature on the definition update file is not valid. You should try to update the definitions again.",
            WINDOWS_DEFENDER::ERR_MP_BAD_UFS => "The digital signature of the definition update file is not valid. You should try to update the definitions again.",
            WINDOWS_DEFENDER::ERR_MP_NO_MEMORY => "The antimalware engine has encountered a no memory situation.",
            WINDOWS_DEFENDER::ERR_MP_BAD_INPUT_DATA => "There might be a problem with your security product.",
            WINDOWS_DEFENDER::ERR_MP_BAD_GLOBAL_STORAGE => "Description is missing",
            WINDOWS_DEFENDER::ERR_MP_OBSOLETE => "Description is missing",
            WINDOWS_DEFENDER::ERR_MP_NOT_SUPPORTED => "Description is missing",
            WINDOWS_DEFENDER::ERR_MP_NO_MORE_ITEMS => "Description is missing",
            WINDOWS_DEFENDER::ERR_MP_DUPLICATE_SCANID => "Description is missing",
            WINDOWS_DEFENDER::ERR_MP_BAD_SCANID => "Description is missing",
            WINDOWS_DEFENDER::ERR_MP_BAD_USERDB_VERSION => "Description is missing",
            WINDOWS_DEFENDER::ERR_MP_RESTORE_FAILED => "Description is missing",
            WINDOWS_DEFENDER::ERROR_MP_FAILED_TO_SPYNET => "Description is missing",
            WINDOWS_DEFENDER::ERR_MP_BAD_ACTION => "Description is missing",
            WINDOWS_DEFENDER::ERR_MP_REMOVE_FAILED => "It might be triggered when malware removal is not successful.",
            WINDOWS_DEFENDER::ERR_MP_SCAN_ABORTED => "It might have triggered when a scan fails to complete.",
            WINDOWS_DEFENDER::ERR_MP_NOT_FOUND => "Description is missing",
            WINDOWS_DEFENDER::ERR_MP_BAD_CONFIGURATION => "There might be an engine configuration error; commonly, this is related to input data that does not allow the engine to function properly.",
            WINDOWS_DEFENDER::ERR_MP_QUARANTINE_FAILED => "Quarantine operation failed for a specific threat.",
            WINDOWS_DEFENDER::ERR_MP_REBOOT_REQUIRED => "A reboot is required to complete threat removal.",
            WINDOWS_DEFENDER::ERR_MP_THREAT_NOT_FOUND => "The threat might no longer be present on the media, or malware might be stopping you from scanning your device.",
            WINDOWS_DEFENDER::ERR_MP_FULL_SCAN_REQUIRED => "A full system scan might be required.",
            WINDOWS_DEFENDER::ERR_MP_MANUAL_STEPS_REQUIRED => "Manual steps are required to complete threat removal.",
            WINDOWS_DEFENDER::ERR_MP_REMOVE_NOT_SUPPORTED => "Remove operation for a specific threat inside the container type is not supported.",
            WINDOWS_DEFENDER::ERR_MP_REMOVE_LOW_MEDIUM_DISABLED => "Removal of low and medium threats might be disabled.",
            WINDOWS_DEFENDER::ERR_MP_UNREFERENCED_8028 => "Windows Defender has encountered an error when taking action on spyware or other potentially unwanted software.",
            WINDOWS_DEFENDER::ERROR_MP_RESCAN_REQUIRED => "A rescan of the threat is required.",
            WINDOWS_DEFENDER::ERROR_MP_CALLISTO_REQUIRED => "An offline scan is required.",
            WINDOWS_DEFENDER::ERROR_MP_PLATFORM_OUTDATED => "Windows Defender does not support the current version of the platform and requires a new version of the platform.",
            WINDOWS_DEFENDER::ERR_RELO_BAD_EHANDLE => "Description is missing",
            WINDOWS_DEFENDER::ERR_RELO_KERNEL_NOT_LOADED => "Description is missing",
            WINDOWS_DEFENDER::ERR_MP_BADDB_OPEN => "The scanning engine did not load because it could not load the definition database.",
            WINDOWS_DEFENDER::ERR_MP_BADDB_HEADER => "The digital signature of the definition update file is not valid. You should try to update the definitions again.",
            WINDOWS_DEFENDER::ERR_MP_BADDB_OLDENGINE => "Signature update request provided an older engine or signature files(s).",
            WINDOWS_DEFENDER::ERR_MP_BADDB_CONTENT => "The digital signature of the definition update file is not valid. You should try to update the definitions again.",
            WINDOWS_DEFENDER::ERR_MP_BADDB_NOTSIGNED => "The digital signature of the definition update file is not valid. You should try to update the definitions again.",
        }
    }

    pub fn from_name(name: &str) -> WINDOWS_DEFENDER {
        return match name {
            "ERROR_MP_UI_CONSOLIDATION_BASE" => WINDOWS_DEFENDER::ERROR_MP_UI_CONSOLIDATION_BASE,
            "ERROR_MP_ACTIONS_FAILED" => WINDOWS_DEFENDER::ERROR_MP_ACTIONS_FAILED,
            "ERROR_MP_NOENGINE" => WINDOWS_DEFENDER::ERROR_MP_NOENGINE,
            "ERROR_MP_ACTIVE_THREATS" => WINDOWS_DEFENDER::ERROR_MP_ACTIVE_THREATS,
            "ERROR_MP_NO_INTERNET_CONN" => WINDOWS_DEFENDER::ERROR_MP_NO_INTERNET_CONN,
            "MP_ERROR_CODE_LUA_CANCELLED" => WINDOWS_DEFENDER::MP_ERROR_CODE_LUA_CANCELLED,
            "MP_ERROR_CODE_ALREADY_SHUTDOWN" => WINDOWS_DEFENDER::MP_ERROR_CODE_ALREADY_SHUTDOWN,
            "MP_ERROR_CODE_RDEVICE_S_ASYNC_CALL_PENDING" => WINDOWS_DEFENDER::MP_ERROR_CODE_RDEVICE_S_ASYNC_CALL_PENDING,
            "MP_ERROR_CODE_CANCELLED" => WINDOWS_DEFENDER::MP_ERROR_CODE_CANCELLED,
            "MP_ERROR_CODE_NO_TARGETOS" => WINDOWS_DEFENDER::MP_ERROR_CODE_NO_TARGETOS,
            "MP_ERROR_CODE_BAD_REGEXP" => WINDOWS_DEFENDER::MP_ERROR_CODE_BAD_REGEXP,
            "MP_ERROR_TEST_INDUCED_ERROR" => WINDOWS_DEFENDER::MP_ERROR_TEST_INDUCED_ERROR,
            "MP_ERROR_SIG_BACKUP_DISABLED" => WINDOWS_DEFENDER::MP_ERROR_SIG_BACKUP_DISABLED,
            "ERR_MP_BAD_INIT_MODULES" => WINDOWS_DEFENDER::ERR_MP_BAD_INIT_MODULES,
            "ERR_MP_BAD_DATABASE" => WINDOWS_DEFENDER::ERR_MP_BAD_DATABASE,
            "ERR_MP_BAD_UFS" => WINDOWS_DEFENDER::ERR_MP_BAD_UFS,
            "ERR_MP_NO_MEMORY" => WINDOWS_DEFENDER::ERR_MP_NO_MEMORY,
            "ERR_MP_BAD_INPUT_DATA" => WINDOWS_DEFENDER::ERR_MP_BAD_INPUT_DATA,
            "ERR_MP_BAD_GLOBAL_STORAGE" => WINDOWS_DEFENDER::ERR_MP_BAD_GLOBAL_STORAGE,
            "ERR_MP_OBSOLETE" => WINDOWS_DEFENDER::ERR_MP_OBSOLETE,
            "ERR_MP_NOT_SUPPORTED" => WINDOWS_DEFENDER::ERR_MP_NOT_SUPPORTED,
            "ERR_MP_NO_MORE_ITEMS" => WINDOWS_DEFENDER::ERR_MP_NO_MORE_ITEMS,
            "ERR_MP_DUPLICATE_SCANID" => WINDOWS_DEFENDER::ERR_MP_DUPLICATE_SCANID,
            "ERR_MP_BAD_SCANID" => WINDOWS_DEFENDER::ERR_MP_BAD_SCANID,
            "ERR_MP_BAD_USERDB_VERSION" => WINDOWS_DEFENDER::ERR_MP_BAD_USERDB_VERSION,
            "ERR_MP_RESTORE_FAILED" => WINDOWS_DEFENDER::ERR_MP_RESTORE_FAILED,
            "ERROR_MP_FAILED_TO_SPYNET" => WINDOWS_DEFENDER::ERROR_MP_FAILED_TO_SPYNET,
            "ERR_MP_BAD_ACTION" => WINDOWS_DEFENDER::ERR_MP_BAD_ACTION,
            "ERR_MP_REMOVE_FAILED" => WINDOWS_DEFENDER::ERR_MP_REMOVE_FAILED,
            "ERR_MP_SCAN_ABORTED" => WINDOWS_DEFENDER::ERR_MP_SCAN_ABORTED,
            "ERR_MP_NOT_FOUND" => WINDOWS_DEFENDER::ERR_MP_NOT_FOUND,
            "ERR_MP_BAD_CONFIGURATION" => WINDOWS_DEFENDER::ERR_MP_BAD_CONFIGURATION,
            "ERR_MP_QUARANTINE_FAILED" => WINDOWS_DEFENDER::ERR_MP_QUARANTINE_FAILED,
            "ERR_MP_REBOOT_REQUIRED" => WINDOWS_DEFENDER::ERR_MP_REBOOT_REQUIRED,
            "ERR_MP_THREAT_NOT_FOUND" => WINDOWS_DEFENDER::ERR_MP_THREAT_NOT_FOUND,
            "ERR_MP_FULL_SCAN_REQUIRED" => WINDOWS_DEFENDER::ERR_MP_FULL_SCAN_REQUIRED,
            "ERR_MP_MANUAL_STEPS_REQUIRED" => WINDOWS_DEFENDER::ERR_MP_MANUAL_STEPS_REQUIRED,
            "ERR_MP_REMOVE_NOT_SUPPORTED" => WINDOWS_DEFENDER::ERR_MP_REMOVE_NOT_SUPPORTED,
            "ERR_MP_REMOVE_LOW_MEDIUM_DISABLED" => WINDOWS_DEFENDER::ERR_MP_REMOVE_LOW_MEDIUM_DISABLED,
            "ERR_MP_UNREFERENCED_8028" => WINDOWS_DEFENDER::ERR_MP_UNREFERENCED_8028,
            "ERROR_MP_RESCAN_REQUIRED" => WINDOWS_DEFENDER::ERROR_MP_RESCAN_REQUIRED,
            "ERROR_MP_CALLISTO_REQUIRED" => WINDOWS_DEFENDER::ERROR_MP_CALLISTO_REQUIRED,
            "ERROR_MP_PLATFORM_OUTDATED" => WINDOWS_DEFENDER::ERROR_MP_PLATFORM_OUTDATED,
            "ERR_RELO_BAD_EHANDLE" => WINDOWS_DEFENDER::ERR_RELO_BAD_EHANDLE,
            "ERR_RELO_KERNEL_NOT_LOADED" => WINDOWS_DEFENDER::ERR_RELO_KERNEL_NOT_LOADED,
            "ERR_MP_BADDB_OPEN" => WINDOWS_DEFENDER::ERR_MP_BADDB_OPEN,
            "ERR_MP_BADDB_HEADER" => WINDOWS_DEFENDER::ERR_MP_BADDB_HEADER,
            "ERR_MP_BADDB_OLDENGINE" => WINDOWS_DEFENDER::ERR_MP_BADDB_OLDENGINE,
            "ERR_MP_BADDB_CONTENT" => WINDOWS_DEFENDER::ERR_MP_BADDB_CONTENT,
            "ERR_MP_BADDB_NOTSIGNED" => WINDOWS_DEFENDER::ERR_MP_BADDB_NOTSIGNED,
        }
    }
    pub fn from_code(code: u32) -> WINDOWS_DEFENDER {
        return match code {
            0x80501000 => WINDOWS_DEFENDER::ERROR_MP_UI_CONSOLIDATION_BASE,
            0x80501001 => WINDOWS_DEFENDER::ERROR_MP_ACTIONS_FAILED,
            0x80501002 => WINDOWS_DEFENDER::ERROR_MP_NOENGINE,
            0x80501003 => WINDOWS_DEFENDER::ERROR_MP_ACTIVE_THREATS,
            0x80501004 => WINDOWS_DEFENDER::ERROR_MP_NO_INTERNET_CONN,
            0x80501101 => WINDOWS_DEFENDER::MP_ERROR_CODE_LUA_CANCELLED,
            0x80501102 => WINDOWS_DEFENDER::MP_ERROR_CODE_ALREADY_SHUTDOWN,
            0x80501103 => WINDOWS_DEFENDER::MP_ERROR_CODE_RDEVICE_S_ASYNC_CALL_PENDING,
            0x80501104 => WINDOWS_DEFENDER::MP_ERROR_CODE_CANCELLED,
            0x80501105 => WINDOWS_DEFENDER::MP_ERROR_CODE_NO_TARGETOS,
            0x80501106 => WINDOWS_DEFENDER::MP_ERROR_CODE_BAD_REGEXP,
            0x80501107 => WINDOWS_DEFENDER::MP_ERROR_TEST_INDUCED_ERROR,
            0x80501108 => WINDOWS_DEFENDER::MP_ERROR_SIG_BACKUP_DISABLED,
            0x80508001 => WINDOWS_DEFENDER::ERR_MP_BAD_INIT_MODULES,
            0x80508002 => WINDOWS_DEFENDER::ERR_MP_BAD_DATABASE,
            0x80508004 => WINDOWS_DEFENDER::ERR_MP_BAD_UFS,
            0x80508007 => WINDOWS_DEFENDER::ERR_MP_NO_MEMORY,
            0x8050800C => WINDOWS_DEFENDER::ERR_MP_BAD_INPUT_DATA,
            0x8050800D => WINDOWS_DEFENDER::ERR_MP_BAD_GLOBAL_STORAGE,
            0x8050800E => WINDOWS_DEFENDER::ERR_MP_OBSOLETE,
            0x8050800F => WINDOWS_DEFENDER::ERR_MP_NOT_SUPPORTED,
            0x80508010 => WINDOWS_DEFENDER::ERR_MP_NO_MORE_ITEMS,
            0x80508011 => WINDOWS_DEFENDER::ERR_MP_DUPLICATE_SCANID,
            0x80508012 => WINDOWS_DEFENDER::ERR_MP_BAD_SCANID,
            0x80508013 => WINDOWS_DEFENDER::ERR_MP_BAD_USERDB_VERSION,
            0x80508014 => WINDOWS_DEFENDER::ERR_MP_RESTORE_FAILED,
            0x80508015 => WINDOWS_DEFENDER::ERROR_MP_FAILED_TO_SPYNET,
            0x80508016 => WINDOWS_DEFENDER::ERR_MP_BAD_ACTION,
            0x80508017 => WINDOWS_DEFENDER::ERR_MP_REMOVE_FAILED,
            0x80508018 => WINDOWS_DEFENDER::ERR_MP_SCAN_ABORTED,
            0x80508019 => WINDOWS_DEFENDER::ERR_MP_NOT_FOUND,
            0x80508020 => WINDOWS_DEFENDER::ERR_MP_BAD_CONFIGURATION,
            0x80508021 => WINDOWS_DEFENDER::ERR_MP_QUARANTINE_FAILED,
            0x80508022 => WINDOWS_DEFENDER::ERR_MP_REBOOT_REQUIRED,
            0x80508023 => WINDOWS_DEFENDER::ERR_MP_THREAT_NOT_FOUND,
            0x80508024 => WINDOWS_DEFENDER::ERR_MP_FULL_SCAN_REQUIRED,
            0x80508025 => WINDOWS_DEFENDER::ERR_MP_MANUAL_STEPS_REQUIRED,
            0x80508026 => WINDOWS_DEFENDER::ERR_MP_REMOVE_NOT_SUPPORTED,
            0x80508027 => WINDOWS_DEFENDER::ERR_MP_REMOVE_LOW_MEDIUM_DISABLED,
            0x80508028 => WINDOWS_DEFENDER::ERR_MP_UNREFERENCED_8028,
            0x80508029 => WINDOWS_DEFENDER::ERROR_MP_RESCAN_REQUIRED,
            0x80508030 => WINDOWS_DEFENDER::ERROR_MP_CALLISTO_REQUIRED,
            0x80508031 => WINDOWS_DEFENDER::ERROR_MP_PLATFORM_OUTDATED,
            0x80509001 => WINDOWS_DEFENDER::ERR_RELO_BAD_EHANDLE,
            0x80509003 => WINDOWS_DEFENDER::ERR_RELO_KERNEL_NOT_LOADED,
            0x8050A001 => WINDOWS_DEFENDER::ERR_MP_BADDB_OPEN,
            0x8050A002 => WINDOWS_DEFENDER::ERR_MP_BADDB_HEADER,
            0x8050A003 => WINDOWS_DEFENDER::ERR_MP_BADDB_OLDENGINE,
            0x8050A004 => WINDOWS_DEFENDER::ERR_MP_BADDB_CONTENT,
            0x8050A005 => WINDOWS_DEFENDER::ERR_MP_BADDB_NOTSIGNED,
        }
    }
}
