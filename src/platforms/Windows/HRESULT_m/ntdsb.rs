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

pub enum NTDSB {
    FHCFG_E_INVALID_REHYDRATION_STATE = 0x0800430A,
    hrBFNotSynchronous = 0x880000C8,
    hrBFPageNotFound = 0x880000C9,
    hrRemainingVersions = 0x88000141,
    hrFLDKeyTooBig = 0x88000190,
    hrFLDNullKey = 0x88000192,
    hrExistingLogFileHasBadSignature = 0x8800022E,
    hrExistingLogFileIsNotContiguous = 0x8800022F,
    hrColumnNull = 0x880003EC,
    hrBufferTruncated = 0x880003EE,
    hrDatabaseAttached = 0x880003EF,
    hrSeekNotEqual = 0x8800040F,
    hrNoIdleActivity = 0x88000422,
    hrNoWriteLock = 0x8800042B,
    hrColumnSetNull = 0x8800042C,
    hrTableEmpty = 0x88000515,
    hrCreateIndexFailed = 0x88000581,
    hrColumnMaxTruncated = 0x880005E8,
    hrwrnDataHasChanged = 0x8800064A,
    hrKeyChanged = 0x88000652,
    hrFileOpenReadOnly = 0x88000715,
    hrFileClose = 0xC8000066,
    hrOutOfThreads = 0xC8000067,
    hrTooManyIO = 0xC8000069,
    hrBFInUse = 0xC80000CA,
    hrPMRecDeleted = 0xC800012E,
    hrFLDTooManySegments = 0xC8000191,
    hrLogFileCorrupt = 0xC80001F5,
    hrNoBackupDirectory = 0xC80001F7,
    hrBackupDirectoryNotEmpty = 0xC80001F8,
    hrBackupInProgress = 0xC80001F9,
    hrMissingPreviousLogFile = 0xC80001FD,
    hrLogWriteFail = 0xC80001FE,
    hrBadLogVersion = 0xC8000202,
    hrInvalidLogSequence = 0xC8000203,
    hrLoggingDisabled = 0xC8000204,
    hrLogBufferTooSmall = 0xC8000205,
    hrLogSequenceEnd = 0xC8000207,
    hrNoBackup = 0xC8000208,
    hrInvalidBackupSequence = 0xC8000209,
    hrBackupNotAllowedYet = 0xC800020B,
    hrDeleteBackupFileFail = 0xC800020C,
    hrMakeBackupDirectoryFail = 0xC800020D,
    hrInvalidBackup = 0xC800020E,
    hrRecoveredWithErrors = 0xC800020F,
    hrMissingLogFile = 0xC8000210,
    hrLogDiskFull = 0xC8000211,
    hrBadLogSignature = 0xC8000212,
    hrBadDbSignature = 0xC8000213,
    hrBadCheckpointSignature = 0xC8000214,
    hrCheckpointCorrupt = 0xC8000215,
    hrDatabaseInconsistent = 0xC8000226,
    hrConsistentTimeMismatch = 0xC8000227,
    hrPatchFileMismatch = 0xC8000228,
    hrRestoreLogTooLow = 0xC8000229,
    hrRestoreLogTooHigh = 0xC800022A,
    hrGivenLogFileHasBadSignature = 0xC800022B,
    hrGivenLogFileIsNotContiguous = 0xC800022C,
    hrMissingRestoreLogFiles = 0xC800022D,
    hrMissingFullBackup = 0xC8000230,
    hrBadBackupDatabaseSize = 0xC8000231,
    hrTermInProgress = 0xC80003E8,
    hrFeatureNotAvailable = 0xC80003E9,
    hrInvalidName = 0xC80003EA,
    hrInvalidParameter = 0xC80003EB,
    hrInvalidDatabaseId = 0xC80003F2,
    hrOutOfMemory = 0xC80003F3,
    hrOutOfDatabaseSpace = 0xC80003F4,
    hrOutOfCursors = 0xC80003F5,
    hrOutOfBuffers = 0xC80003F6,
    hrTooManyIndexes = 0xC80003F7,
    hrTooManyKeys = 0xC80003F8,
    hrRecordDeleted = 0xC80003F9,
    hrReadVerifyFailure = 0xC80003FA,
    hrOutOfFileHandles = 0xC80003FC,
    hrDiskIO = 0xC80003FE,
    hrInvalidPath = 0xC80003FF,
    hrRecordTooBig = 0xC8000402,
    hrTooManyOpenDatabases = 0xC8000403,
    hrInvalidDatabase = 0xC8000404,
    hrNotInitialized = 0xC8000405,
    hrAlreadyInitialized = 0xC8000406,
    hrFileAccessDenied = 0xC8000408,
    hrBufferTooSmall = 0xC800040E,
    hrTooManyColumns = 0xC8000410,
    hrContainerNotEmpty = 0xC8000413,
    hrInvalidFilename = 0xC8000414,
    hrInvalidBookmark = 0xC8000415,
    hrColumnInUse = 0xC8000416,
    hrInvalidBufferSize = 0xC8000417,
    hrColumnNotUpdatable = 0xC8000418,
    hrIndexInUse = 0xC800041B,
    hrNullKeyDisallowed = 0xC800041D,
    hrNotInTransaction = 0xC800041E,
    hrTooManyActiveUsers = 0xC8000423,
    hrInvalidCountry = 0xC8000425,
    hrInvalidLanguageId = 0xC8000426,
    hrInvalidCodePage = 0xC8000427,
    hrVersionStoreOutOfMemory = 0xC800042D,
    hrCurrencyStackOutOfMemory = 0xC800042E,
    hrOutOfSessions = 0xC800044D,
    hrWriteConflict = 0xC800044E,
    hrTransTooDeep = 0xC800044F,
    hrInvalidSesid = 0xC8000450,
    hrSessionWriteConflict = 0xC8000453,
    hrInTransaction = 0xC8000454,
    hrDatabaseDuplicate = 0xC80004B1,
    hrDatabaseInUse = 0xC80004B2,
    hrDatabaseNotFound = 0xC80004B3,
    hrDatabaseInvalidName = 0xC80004B4,
    hrDatabaseInvalidPages = 0xC80004B5,
    hrDatabaseCorrupted = 0xC80004B6,
    hrDatabaseLocked = 0xC80004B7,
    hrTableLocked = 0xC8000516,
    hrTableDuplicate = 0xC8000517,
    hrTableInUse = 0xC8000518,
    hrObjectNotFound = 0xC8000519,
    hrCannotRename = 0xC800051A,
    hrDensityInvalid = 0xC800051B,
    hrTableNotEmpty = 0xC800051C,
    hrInvalidTableId = 0xC800051E,
    hrTooManyOpenTables = 0xC800051F,
    hrIllegalOperation = 0xC8000520,
    hrObjectDuplicate = 0xC8000522,
    hrInvalidObject = 0xC8000524,
    hrIndexCantBuild = 0xC8000579,
    hrIndexHasPrimary = 0xC800057A,
    hrIndexDuplicate = 0xC800057B,
    hrIndexNotFound = 0xC800057C,
    hrIndexMustStay = 0xC800057D,
    hrIndexInvalidDef = 0xC800057E,
    hrIndexHasClustered = 0xC8000580,
    hrTooManyOpenIndexes = 0xC8000582,
    hrColumnLong = 0xC80005DD,
    hrColumnDoesNotFit = 0xC80005DF,
    hrNullInvalid = 0xC80005E0,
    hrColumnIndexed = 0xC80005E1,
    hrColumnTooBig = 0xC80005E2,
    hrColumnNotFound = 0xC80005E3,
    hrColumnDuplicate = 0xC80005E4,
    hrColumn2ndSysMaint = 0xC80005E6,
    hrInvalidColumnType = 0xC80005E7,
    hrColumnCannotIndex = 0xC80005E9,
    hrTaggedNotNULL = 0xC80005EA,
    hrNoCurrentIndex = 0xC80005EB,
    hrKeyIsMade = 0xC80005EC,
    hrBadColumnId = 0xC80005ED,
    hrBadItagSequence = 0xC80005EE,
    hrCannotBeTagged = 0xC80005F1,
    hrRecordNotFound = 0xC8000641,
    hrNoCurrentRecord = 0xC8000643,
    hrRecordClusteredChanged = 0xC8000644,
    hrKeyDuplicate = 0xC8000645,
    hrAlreadyPrepared = 0xC8000647,
    hrKeyNotMade = 0xC8000648,
    hrUpdateNotPrepared = 0xC8000649,
    hrerrDataHasChanged = 0xC800064B,
    hrTooManySorts = 0xC80006A5,
    hrInvalidOnSort = 0xC80006A6,
    hrTempFileOpenError = 0xC800070B,
    hrTooManyAttachedDatabases = 0xC800070D,
    hrDiskFull = 0xC8000710,
    hrPermissionDenied = 0xC8000711,
    hrFileNotFound = 0xC8000713,
    hrAfterInitialization = 0xC800073A,
    hrLogCorrupted = 0xC800073C,
    hrInvalidOperation = 0xC8000772,
    hrAccessDenied = 0xC8000773,
}

impl NTDSB {
    pub fn description(&self) -> &'static str {
        match self {
            NTDSB::FHCFG_E_INVALID_REHYDRATION_STATE => "Configuration Manager: Rehydration can't be performed in the current configuration state.",
            NTDSB::hrBFNotSynchronous => "The buffer page has been evicted.",
            NTDSB::hrBFPageNotFound => "Unable to find the page.",
            NTDSB::hrRemainingVersions => "There is idle work remaining.",
            NTDSB::hrFLDKeyTooBig => "The key was truncated because it exceeded the maximum length.",
            NTDSB::hrFLDNullKey => "The key is NULL.",
            NTDSB::hrExistingLogFileHasBadSignature => "The log file in the log file path is damaged.",
            NTDSB::hrExistingLogFileIsNotContiguous => "Unable to find a mandatory log file in the log file path.",
            NTDSB::hrColumnNull => "The value of the column is null.",
            NTDSB::hrBufferTruncated => "The buffer is too small for data.",
            NTDSB::hrDatabaseAttached => "The database is already attached.",
            NTDSB::hrSeekNotEqual => "Either SeekLE or SeekGE did not find an exact match.",
            NTDSB::hrNoIdleActivity => "No idle activity occurred.",
            NTDSB::hrNoWriteLock => "There is no write lock at transaction level 0.",
            NTDSB::hrColumnSetNull => "The column value is set to null.",
            NTDSB::hrTableEmpty => "An empty table was opened.",
            NTDSB::hrCreateIndexFailed => "Unable to create the index because an error occurred while creating a table.",
            NTDSB::hrColumnMaxTruncated => "The column was truncated because it exceeded the maximum length.",
            NTDSB::hrwrnDataHasChanged => "Data has changed.",
            NTDSB::hrKeyChanged => "Moved to a new key.",
            NTDSB::hrFileOpenReadOnly => "The database file is read only.",
            NTDSB::hrFileClose => "Unable to close the DOS file.",
            NTDSB::hrOutOfThreads => "Unable to start a thread because there are none available.",
            NTDSB::hrTooManyIO => "The system is busy because there are too many I/Os.",
            NTDSB::hrBFInUse => "Unable to abandon the buffer.",
            NTDSB::hrPMRecDeleted => "The record has been deleted.",
            NTDSB::hrFLDTooManySegments => "There are too many key segments.",
            NTDSB::hrLogFileCorrupt => "The log file is damaged.",
            NTDSB::hrNoBackupDirectory => "No backup directory was given.",
            NTDSB::hrBackupDirectoryNotEmpty => "The backup directory is not empty.",
            NTDSB::hrBackupInProgress => "Backup is already active.",
            NTDSB::hrMissingPreviousLogFile => "A log file for the checkpoint is missing.",
            NTDSB::hrLogWriteFail => "Unable to write to the log file.",
            NTDSB::hrBadLogVersion => "The version of the log file is not compatible with the version of the Windows NT Directory Service database (NTDS).",
            NTDSB::hrInvalidLogSequence => "The time stamp in the next log does not match what was expected.",
            NTDSB::hrLoggingDisabled => "The log is not active.",
            NTDSB::hrLogBufferTooSmall => "The log buffer is too small to be recovered.",
            NTDSB::hrLogSequenceEnd => "The maximum number of log files has been exceeded.",
            NTDSB::hrNoBackup => "There is no backup in progress.",
            NTDSB::hrInvalidBackupSequence => "The backup call is out of sequence.",
            NTDSB::hrBackupNotAllowedYet => "Unable to perform a backup now.",
            NTDSB::hrDeleteBackupFileFail => "Unable to delete the backup file.",
            NTDSB::hrMakeBackupDirectoryFail => "Unable to make a backup temporary directory.",
            NTDSB::hrInvalidBackup => "An incremental backup cannot be performed when circular logging is enabled.",
            NTDSB::hrRecoveredWithErrors => "Errors were encountered during the repair process.",
            NTDSB::hrMissingLogFile => "The current log file is missing.",
            NTDSB::hrLogDiskFull => "The log disk is full.",
            NTDSB::hrBadLogSignature => "A log file is damaged.",
            NTDSB::hrBadDbSignature => "A database file is damaged.",
            NTDSB::hrBadCheckpointSignature => "A checkpoint file is damaged.",
            NTDSB::hrCheckpointCorrupt => "A checkpoint file either could not be found or is damaged.",
            NTDSB::hrDatabaseInconsistent => "The database is damaged.",
            NTDSB::hrConsistentTimeMismatch => "There is a mismatch in the database's last consistent time.",
            NTDSB::hrPatchFileMismatch => "The patch file is not generated from this backup.",
            NTDSB::hrRestoreLogTooLow => "The starting log number is too low for the restore.",
            NTDSB::hrRestoreLogTooHigh => "The starting log number is too high for the restore.",
            NTDSB::hrGivenLogFileHasBadSignature => "The log file downloaded from the tape is damaged.",
            NTDSB::hrGivenLogFileIsNotContiguous => "Unable to find a mandatory log file after the tape was downloaded.",
            NTDSB::hrMissingRestoreLogFiles => "The data is not fully restored because some log files are missing.",
            NTDSB::hrMissingFullBackup => "The database missed a previous full backup before the incremental backup.",
            NTDSB::hrBadBackupDatabaseSize => "The backup database size must be a multiple of 4K (4096 bytes).",
            NTDSB::hrTermInProgress => "The database is being shut down.",
            NTDSB::hrFeatureNotAvailable => "The feature is not available.",
            NTDSB::hrInvalidName => "The name is not valid.",
            NTDSB::hrInvalidParameter => "The parameter is not valid.",
            NTDSB::hrInvalidDatabaseId => "The database ID is not valid.",
            NTDSB::hrOutOfMemory => "The computer is out of memory.",
            NTDSB::hrOutOfDatabaseSpace => "The database has reached the maximum size of 16 GB.",
            NTDSB::hrOutOfCursors => "Out of table cursors.",
            NTDSB::hrOutOfBuffers => "Out of database page buffers.",
            NTDSB::hrTooManyIndexes => "There are too many indexes.",
            NTDSB::hrTooManyKeys => "There are too many columns in an index.",
            NTDSB::hrRecordDeleted => "The record has been deleted.",
            NTDSB::hrReadVerifyFailure => "A read verification error occurred.",
            NTDSB::hrOutOfFileHandles => "Out of file handles.",
            NTDSB::hrDiskIO => "A disk I/O error occurred.",
            NTDSB::hrInvalidPath => "The path to the file is not valid.",
            NTDSB::hrRecordTooBig => "The record has exceeded the maximum size.",
            NTDSB::hrTooManyOpenDatabases => "There are too many open databases.",
            NTDSB::hrInvalidDatabase => "The file is not a database file.",
            NTDSB::hrNotInitialized => "The database was not yet called.",
            NTDSB::hrAlreadyInitialized => "The database was already called.",
            NTDSB::hrFileAccessDenied => "Unable to access the file.",
            NTDSB::hrBufferTooSmall => "The buffer is too small.",
            NTDSB::hrTooManyColumns => "There are too many columns defined.",
            NTDSB::hrContainerNotEmpty => "The container is not empty.",
            NTDSB::hrInvalidFilename => "The filename is not valid.",
            NTDSB::hrInvalidBookmark => "The bookmark is not valid.",
            NTDSB::hrColumnInUse => "The column is used in an index.",
            NTDSB::hrInvalidBufferSize => "The data buffer does not match the column size.",
            NTDSB::hrColumnNotUpdatable => "Unable to set the column value.",
            NTDSB::hrIndexInUse => "The index is in use.",
            NTDSB::hrNullKeyDisallowed => "Null keys are not allowed on an index.",
            NTDSB::hrNotInTransaction => "The operation must be within a transaction.",
            NTDSB::hrTooManyActiveUsers => "There are too many active database users.",
            NTDSB::hrInvalidCountry => "The country code is either not known or is not valid.",
            NTDSB::hrInvalidLanguageId => "The language ID is either not known or is not valid.",
            NTDSB::hrInvalidCodePage => "The code page is either not known or is not valid.",
            NTDSB::hrVersionStoreOutOfMemory => "lMaxVerPages exceeded (XJET only).",
            NTDSB::hrCurrencyStackOutOfMemory => "Out of cursors.",
            NTDSB::hrOutOfSessions => "Out of sessions.",
            NTDSB::hrWriteConflict => "The write lock failed due to an outstanding write lock.",
            NTDSB::hrTransTooDeep => "The transactions are nested too deeply.",
            NTDSB::hrInvalidSesid => "The session handle is not valid.",
            NTDSB::hrSessionWriteConflict => "Another session has a private version of the page.",
            NTDSB::hrInTransaction => "The operation is not allowed within a transaction.",
            NTDSB::hrDatabaseDuplicate => "The database already exists.",
            NTDSB::hrDatabaseInUse => "The database is in use.",
            NTDSB::hrDatabaseNotFound => "The database does not exist.",
            NTDSB::hrDatabaseInvalidName => "The database name is not valid.",
            NTDSB::hrDatabaseInvalidPages => "The number of pages is not valid.",
            NTDSB::hrDatabaseCorrupted => "The database file is either damaged or cannot be found.",
            NTDSB::hrDatabaseLocked => "The database is locked.",
            NTDSB::hrTableLocked => "The table is locked.",
            NTDSB::hrTableDuplicate => "The table already exists.",
            NTDSB::hrTableInUse => "Unable to lock the table because it is already in use.",
            NTDSB::hrObjectNotFound => "The table or object does not exist.",
            NTDSB::hrCannotRename => "Unable to rename the temporary file.",
            NTDSB::hrDensityInvalid => "The file/index density is not valid.",
            NTDSB::hrTableNotEmpty => "Unable to define the clustered index.",
            NTDSB::hrInvalidTableId => "The table ID is not valid.",
            NTDSB::hrTooManyOpenTables => "Unable to open any more tables.",
            NTDSB::hrIllegalOperation => "The operation is not supported on tables.",
            NTDSB::hrObjectDuplicate => "The table or object name is already being used.",
            NTDSB::hrInvalidObject => "The object is not valid for operation.",
            NTDSB::hrIndexCantBuild => "Unable to build a clustered index.",
            NTDSB::hrIndexHasPrimary => "The primary index is already defined.",
            NTDSB::hrIndexDuplicate => "The index is already defined.",
            NTDSB::hrIndexNotFound => "The index does not exist.",
            NTDSB::hrIndexMustStay => "Unable to delete a clustered index.",
            NTDSB::hrIndexInvalidDef => "The index definition is illegal.",
            NTDSB::hrIndexHasClustered => "The clustered index is already defined.",
            NTDSB::hrTooManyOpenIndexes => "Out of index description blocks.",
            NTDSB::hrColumnLong => "The column value is too long.",
            NTDSB::hrColumnDoesNotFit => "The field will not fit in the record.",
            NTDSB::hrNullInvalid => "The value cannot be null.",
            NTDSB::hrColumnIndexed => "Unable to delete because the column is indexed.",
            NTDSB::hrColumnTooBig => "The length of the field exceeds the maximum length.",
            NTDSB::hrColumnNotFound => "Unable to find the column.",
            NTDSB::hrColumnDuplicate => "The field is already defined.",
            NTDSB::hrColumn2ndSysMaint => "Only one auto-increment or version column is allowed per table.",
            NTDSB::hrInvalidColumnType => "The column data type is not valid.",
            NTDSB::hrColumnCannotIndex => "Unable to index a long value column.",
            NTDSB::hrTaggedNotNULL => "Tagged columns cannot be null.",
            NTDSB::hrNoCurrentIndex => "The entry is not valid without a current index.",
            NTDSB::hrKeyIsMade => "The key is completely made.",
            NTDSB::hrBadColumnId => "The column ID is not correct.",
            NTDSB::hrBadItagSequence => "There is a bad instance identifier for a multivalued column.",
            NTDSB::hrCannotBeTagged => "AutoIncrement and Version cannot be multivalued.",
            NTDSB::hrRecordNotFound => "Unable to find the key.",
            NTDSB::hrNoCurrentRecord => "The currency is not on a record.",
            NTDSB::hrRecordClusteredChanged => "A clustered key cannot be changed.",
            NTDSB::hrKeyDuplicate => "The key already exists.",
            NTDSB::hrAlreadyPrepared => "The current entry has already been copied or cleared.",
            NTDSB::hrKeyNotMade => "No key was made.",
            NTDSB::hrUpdateNotPrepared => "Update was not prepared.",
            NTDSB::hrerrDataHasChanged => "The operation was abandoned because data has changed.",
            NTDSB::hrTooManySorts => "There are too many sort processes.",
            NTDSB::hrInvalidOnSort => "An operation that is not valid occurred in the sort.",
            NTDSB::hrTempFileOpenError => "Unable to open the temporary file.",
            NTDSB::hrTooManyAttachedDatabases => "There are too many databases open.",
            NTDSB::hrDiskFull => "The disk is full.",
            NTDSB::hrPermissionDenied => "Permission is denied.",
            NTDSB::hrFileNotFound => "Unable to find the file.",
            NTDSB::hrAfterInitialization => "Unable to restore after initialization.",
            NTDSB::hrLogCorrupted => "The database log files are damaged.",
            NTDSB::hrInvalidOperation => "The operation is not valid.",
            NTDSB::hrAccessDenied => "Access is denied.",
        }
    }
}