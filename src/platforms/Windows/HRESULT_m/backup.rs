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
pub enum BACKUP {
    hrInvalidParam,
    hrError,
    hrInvalidHandle,
    hrRestoreInProgress,
    hrAlreadyOpen,
    hrInvalidRecips,
    hrCouldNotConnect,
    hrRestoreMapExists,
    hrIncrementalBackupDisabled,
    hrLogFileNotFound,
    hrCircularLogging,
    hrNoFullRestore,
    hrCommunicationError,
    hrFullBackupNotTaken,
    hrMissingExpiryToken,
    hrUnknownExpiryTokenFormat,
    hrContentsExpired,
}

impl BACKUP {
    pub fn code(&self) -> u32 {
        return match self {
            BACKUP::hrInvalidParam => 0xC7FF0001 as u32,
            BACKUP::hrError => 0xC7FF0002 as u32,
            BACKUP::hrInvalidHandle => 0xC7FF0003 as u32,
            BACKUP::hrRestoreInProgress => 0xC7FF0004 as u32,
            BACKUP::hrAlreadyOpen => 0xC7FF0005 as u32,
            BACKUP::hrInvalidRecips => 0xC7FF0006 as u32,
            BACKUP::hrCouldNotConnect => 0xC7FF0007 as u32,
            BACKUP::hrRestoreMapExists => 0xC7FF0008 as u32,
            BACKUP::hrIncrementalBackupDisabled => 0xC7FF0009 as u32,
            BACKUP::hrLogFileNotFound => 0xC7FF000A as u32,
            BACKUP::hrCircularLogging => 0xC7FF000B as u32,
            BACKUP::hrNoFullRestore => 0xC7FF000C as u32,
            BACKUP::hrCommunicationError => 0xC7FF000D as u32,
            BACKUP::hrFullBackupNotTaken => 0xC7FF000E as u32,
            BACKUP::hrMissingExpiryToken => 0xC7FF000F as u32,
            BACKUP::hrUnknownExpiryTokenFormat => 0xC7FF0010 as u32,
            BACKUP::hrContentsExpired => 0xC7FF0011 as u32,
        }
    }

    pub fn error(&self) -> RawError {
        return match self {
            BACKUP::hrInvalidParam => RawError::Kind(BACKUP::hrInvalidParam),
            BACKUP::hrError => RawError::Kind(BACKUP::hrError),
            BACKUP::hrInvalidHandle => RawError::Kind(BACKUP::hrInvalidHandle),
            BACKUP::hrRestoreInProgress => RawError::Kind(BACKUP::hrRestoreInProgress),
            BACKUP::hrAlreadyOpen => RawError::Kind(BACKUP::hrAlreadyOpen),
            BACKUP::hrInvalidRecips => RawError::Kind(BACKUP::hrInvalidRecips),
            BACKUP::hrCouldNotConnect => RawError::Kind(BACKUP::hrCouldNotConnect),
            BACKUP::hrRestoreMapExists => RawError::Kind(BACKUP::hrRestoreMapExists),
            BACKUP::hrIncrementalBackupDisabled => RawError::Kind(BACKUP::hrIncrementalBackupDisabled),
            BACKUP::hrLogFileNotFound => RawError::Kind(BACKUP::hrLogFileNotFound),
            BACKUP::hrCircularLogging => RawError::Kind(BACKUP::hrCircularLogging),
            BACKUP::hrNoFullRestore => RawError::Kind(BACKUP::hrNoFullRestore),
            BACKUP::hrCommunicationError => RawError::Kind(BACKUP::hrCommunicationError),
            BACKUP::hrFullBackupNotTaken => RawError::Kind(BACKUP::hrFullBackupNotTaken),
            BACKUP::hrMissingExpiryToken => RawError::Kind(BACKUP::hrMissingExpiryToken),
            BACKUP::hrUnknownExpiryTokenFormat => RawError::Kind(BACKUP::hrUnknownExpiryTokenFormat),
            BACKUP::hrContentsExpired => RawError::Kind(BACKUP::hrContentsExpired),
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            BACKUP::hrInvalidParam => "The parameter is not valid.",
            BACKUP::hrError => "An internal error has occurred.",
            BACKUP::hrInvalidHandle => "The handle is not valid.",
            BACKUP::hrRestoreInProgress => "The Restore process is already in progress.",
            BACKUP::hrAlreadyOpen => "The file specified is already open.",
            BACKUP::hrInvalidRecips => "The recipients are invalid.",
            BACKUP::hrCouldNotConnect => "Unable to perform the backup. Either you are not connected to the specified backup server or the service you are trying to backup is not running.",
            BACKUP::hrRestoreMapExists => "A restore map already exists for the specified component. You can only specify a restore map when performing a full restore.",
            BACKUP::hrIncrementalBackupDisabled => "Another application has modified the specified Windows NT Directory Service database such that any subsequent backups will fail. You must perform a full backup to fix this problem.",
            BACKUP::hrLogFileNotFound => "Unable to perform an incremental backup because a required Windows NT Directory Service database log file could not be found.",
            BACKUP::hrCircularLogging => "The Windows NT Directory Service component specified is configured to use circular database logs. It cannot be backed up without a full backup.",
            BACKUP::hrNoFullRestore => "The databases have not been restored to this machine. You cannot restore an incremental backup until a full backup has been restored.",
            BACKUP::hrCommunicationError => "A communications error occurred while attempting to perform a local backup.",
            BACKUP::hrFullBackupNotTaken => "You must perform a full backup before you can perform an incremental backup.",
            BACKUP::hrMissingExpiryToken => "Expiry token is missing. Cannot restore without knowing the expiry information.",
            BACKUP::hrUnknownExpiryTokenFormat => "Expiry token is in unrecognizable format.",
            BACKUP::hrContentsExpired => "DS Contents in the backup copy are out of date. Try restoring with a more recent copy.",
        }
    }

    pub fn from_name(name: &str) -> BACKUP {
        return match name {
            "hrInvalidParam" => BACKUP::hrInvalidParam,
            "hrError" => BACKUP::hrError,
            "hrInvalidHandle" => BACKUP::hrInvalidHandle,
            "hrRestoreInProgress" => BACKUP::hrRestoreInProgress,
            "hrAlreadyOpen" => BACKUP::hrAlreadyOpen,
            "hrInvalidRecips" => BACKUP::hrInvalidRecips,
            "hrCouldNotConnect" => BACKUP::hrCouldNotConnect,
            "hrRestoreMapExists" => BACKUP::hrRestoreMapExists,
            "hrIncrementalBackupDisabled" => BACKUP::hrIncrementalBackupDisabled,
            "hrLogFileNotFound" => BACKUP::hrLogFileNotFound,
            "hrCircularLogging" => BACKUP::hrCircularLogging,
            "hrNoFullRestore" => BACKUP::hrNoFullRestore,
            "hrCommunicationError" => BACKUP::hrCommunicationError,
            "hrFullBackupNotTaken" => BACKUP::hrFullBackupNotTaken,
            "hrMissingExpiryToken" => BACKUP::hrMissingExpiryToken,
            "hrUnknownExpiryTokenFormat" => BACKUP::hrUnknownExpiryTokenFormat,
            "hrContentsExpired" => BACKUP::hrContentsExpired,
        }
    }
    pub fn from_code(code: u32) -> BACKUP {
        return match code {
            0xC7FF0001 => BACKUP::hrInvalidParam,
            0xC7FF0002 => BACKUP::hrError,
            0xC7FF0003 => BACKUP::hrInvalidHandle,
            0xC7FF0004 => BACKUP::hrRestoreInProgress,
            0xC7FF0005 => BACKUP::hrAlreadyOpen,
            0xC7FF0006 => BACKUP::hrInvalidRecips,
            0xC7FF0007 => BACKUP::hrCouldNotConnect,
            0xC7FF0008 => BACKUP::hrRestoreMapExists,
            0xC7FF0009 => BACKUP::hrIncrementalBackupDisabled,
            0xC7FF000A => BACKUP::hrLogFileNotFound,
            0xC7FF000B => BACKUP::hrCircularLogging,
            0xC7FF000C => BACKUP::hrNoFullRestore,
            0xC7FF000D => BACKUP::hrCommunicationError,
            0xC7FF000E => BACKUP::hrFullBackupNotTaken,
            0xC7FF000F => BACKUP::hrMissingExpiryToken,
            0xC7FF0010 => BACKUP::hrUnknownExpiryTokenFormat,
            0xC7FF0011 => BACKUP::hrContentsExpired,
        }
    }
}
