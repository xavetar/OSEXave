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

pub enum BACKUP {
    hrInvalidParam = 0xC7FF0001,
    hrError = 0xC7FF0002,
    hrInvalidHandle = 0xC7FF0003,
    hrRestoreInProgress = 0xC7FF0004,
    hrAlreadyOpen = 0xC7FF0005,
    hrInvalidRecips = 0xC7FF0006,
    hrCouldNotConnect = 0xC7FF0007,
    hrRestoreMapExists = 0xC7FF0008,
    hrIncrementalBackupDisabled = 0xC7FF0009,
    hrLogFileNotFound = 0xC7FF000A,
    hrCircularLogging = 0xC7FF000B,
    hrNoFullRestore = 0xC7FF000C,
    hrCommunicationError = 0xC7FF000D,
    hrFullBackupNotTaken = 0xC7FF000E,
    hrMissingExpiryToken = 0xC7FF000F,
    hrUnknownExpiryTokenFormat = 0xC7FF0010,
    hrContentsExpired = 0xC7FF0011,
}

impl BACKUP {
    pub fn description(&self) -> &'static str {
        match self {
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
}
