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

pub enum IMAPI2 {
    S_IMAPI_SPEEDADJUSTED = 0x00AA0004,
    S_IMAPI_ROTATIONADJUSTED = 0x00AA0005,
    S_IMAPI_BOTHADJUSTED = 0x00AA0006,
    S_IMAPI_COMMAND_HAS_SENSE_DATA = 0x00AA0200,
    S_IMAPI_WRITE_NOT_IN_PROGRESS = 0x00AA0302,
    S_IMAPI_RAW_IMAGE_TRACK_INDEX_ALREADY_EXISTS = 0x00AA0A08,
    IMAPI_S_IMAGE_FEATURE_NOT_SUPPORTED = 0x00AAB15F,
    E_IMAPI_ERASE_RECORDER_IN_USE = 0x80AA0900,
    E_IMAPI_ERASE_ONLY_ONE_RECORDER_SUPPORTED = 0x80AA0901,
    E_IMAPI_ERASE_DISC_INFORMATION_TOO_SMALL = 0x80AA0902,
    E_IMAPI_ERASE_MODE_PAGE_2A_TOO_SMALL = 0x80AA0903,
    E_IMAPI_ERASE_MEDIA_IS_NOT_ERASABLE = 0x80AA0904,
    E_IMAPI_ERASE_DRIVE_FAILED_ERASE_COMMAND = 0x80AA0905,
    E_IMAPI_ERASE_TOOK_LONGER_THAN_ONE_HOUR = 0x80AA0906,
    E_IMAPI_ERASE_UNEXPECTED_DRIVE_RESPONSE_DURING_ERASE = 0x80AA0907,
    E_IMAPI_ERASE_DRIVE_FAILED_SPINUP_COMMAND = 0x80AA0908,
    E_IMAPI_RAW_IMAGE_IS_READ_ONLY = 0x80AA0A00,
    E_IMAPI_RAW_IMAGE_TOO_MANY_TRACKS = 0x80AA0A01,
    E_IMAPI_RAW_IMAGE_SECTOR_TYPE_NOT_SUPPORTED = 0x80AA0A02,
    E_IMAPI_RAW_IMAGE_NO_TRACKS = 0x80AA0A03,
    E_IMAPI_RAW_IMAGE_TRACKS_ALREADY_ADDED = 0x80AA0A04,
    E_IMAPI_RAW_IMAGE_INSUFFICIENT_SPACE = 0x80AA0A05,
    E_IMAPI_RAW_IMAGE_TOO_MANY_TRACK_INDEXES = 0x80AA0A06,
    E_IMAPI_RAW_IMAGE_TRACK_INDEX_NOT_FOUND = 0x80AA0A07,
    E_IMAPI_RAW_IMAGE_TRACK_INDEX_OFFSET_ZERO_CANNOT_BE_CLEARED = 0x80AA0A09,
    E_IMAPI_RAW_IMAGE_TRACK_INDEX_TOO_CLOSE_TO_OTHER_INDEX = 0x80AA0A0A,
    E_IMAPI_REQUEST_CANCELLED = 0xC0AA0002,
    E_IMAPI_RECORDER_REQUIRED = 0xC0AA0003,
    E_IMAPI_BURN_VERIFICATION_FAILED = 0xC0AA0007,
    E_IMAPI_RECORDER_NO_SUCH_MODE_PAGE = 0xC0AA0201,
    E_IMAPI_RECORDER_MEDIA_NO_MEDIA = 0xC0AA0202,
    E_IMAPI_RECORDER_MEDIA_INCOMPATIBLE = 0xC0AA0203,
    E_IMAPI_RECORDER_MEDIA_UPSIDE_DOWN = 0xC0AA0204,
    E_IMAPI_RECORDER_MEDIA_BECOMING_READY = 0xC0AA0205,
    E_IMAPI_RECORDER_MEDIA_FORMAT_IN_PROGRESS = 0xC0AA0206,
    E_IMAPI_RECORDER_MEDIA_BUSY = 0xC0AA0207,
    E_IMAPI_RECORDER_INVALID_MODE_PARAMETERS = 0xC0AA0208,
    E_IMAPI_RECORDER_MEDIA_WRITE_PROTECTED = 0xC0AA0209,
    E_IMAPI_RECORDER_NO_SUCH_FEATURE = 0xC0AA020A,
    E_IMAPI_RECORDER_FEATURE_IS_NOT_CURRENT = 0xC0AA020B,
    E_IMAPI_RECORDER_GET_CONFIGURATION_NOT_SUPPORTED = 0xC0AA020C,
    E_IMAPI_RECORDER_COMMAND_TIMEOUT = 0xC0AA020D,
    E_IMAPI_RECORDER_DVD_STRUCTURE_NOT_PRESENT = 0xC0AA020E,
    E_IMAPI_RECORDER_MEDIA_SPEED_MISMATCH = 0xC0AA020F,
    E_IMAPI_RECORDER_LOCKED = 0xC0AA0210,
    E_IMAPI_RECORDER_CLIENT_NAME_IS_NOT_VALID = 0xC0AA0211,
    E_IMAPI_RECORDER_MEDIA_NOT_FORMATTED = 0xC0AA0212,
    E_IMAPI_RECORDER_INVALID_RESPONSE_FROM_DEVICE = 0xC0AA02FF,
    E_IMAPI_LOSS_OF_STREAMING = 0xC0AA0300,
    E_IMAPI_UNEXPECTED_RESPONSE_FROM_DEVICE = 0xC0AA0301,
    E_IMAPI_DF2DATA_WRITE_IN_PROGRESS = 0xC0AA0400,
    E_IMAPI_DF2DATA_WRITE_NOT_IN_PROGRESS = 0xC0AA0401,
    E_IMAPI_DF2DATA_INVALID_MEDIA_STATE = 0xC0AA0402,
    E_IMAPI_DF2DATA_STREAM_NOT_SUPPORTED = 0xC0AA0403,
    E_IMAPI_DF2DATA_STREAM_TOO_LARGE_FOR_CURRENT_MEDIA = 0xC0AA0404,
    E_IMAPI_DF2DATA_MEDIA_NOT_BLANK = 0xC0AA0405,
    E_IMAPI_DF2DATA_MEDIA_IS_NOT_SUPPORTED = 0xC0AA0406,
    E_IMAPI_DF2DATA_RECORDER_NOT_SUPPORTED = 0xC0AA0407,
    E_IMAPI_DF2DATA_CLIENT_NAME_IS_NOT_VALID = 0xC0AA0408,
    E_IMAPI_DF2TAO_WRITE_IN_PROGRESS = 0xC0AA0500,
    E_IMAPI_DF2TAO_WRITE_NOT_IN_PROGRESS = 0xC0AA0501,
    E_IMAPI_DF2TAO_MEDIA_IS_NOT_PREPARED = 0xC0AA0502,
    E_IMAPI_DF2TAO_MEDIA_IS_PREPARED = 0xC0AA0503,
    E_IMAPI_DF2TAO_PROPERTY_FOR_BLANK_MEDIA_ONLY = 0xC0AA0504,
    E_IMAPI_DF2TAO_TABLE_OF_CONTENTS_EMPTY_DISC = 0xC0AA0505,
    E_IMAPI_DF2TAO_MEDIA_IS_NOT_BLANK = 0xC0AA0506,
    E_IMAPI_DF2TAO_MEDIA_IS_NOT_SUPPORTED = 0xC0AA0507,
    E_IMAPI_DF2TAO_TRACK_LIMIT_REACHED = 0xC0AA0508,
    E_IMAPI_DF2TAO_NOT_ENOUGH_SPACE = 0xC0AA0509,
    E_IMAPI_DF2TAO_NO_RECORDER_SPECIFIED = 0xC0AA050A,
    E_IMAPI_DF2TAO_INVALID_ISRC = 0xC0AA050B,
    E_IMAPI_DF2TAO_INVALID_MCN = 0xC0AA050C,
    E_IMAPI_DF2TAO_STREAM_NOT_SUPPORTED = 0xC0AA050D,
    E_IMAPI_DF2TAO_RECORDER_NOT_SUPPORTED = 0xC0AA050E,
    E_IMAPI_DF2TAO_CLIENT_NAME_IS_NOT_VALID = 0xC0AA050F,
    E_IMAPI_DF2RAW_WRITE_IN_PROGRESS = 0xC0AA0600,
    E_IMAPI_DF2RAW_WRITE_NOT_IN_PROGRESS = 0xC0AA0601,
    E_IMAPI_DF2RAW_MEDIA_IS_NOT_PREPARED = 0xC0AA0602,
    E_IMAPI_DF2RAW_MEDIA_IS_PREPARED = 0xC0AA0603,
    E_IMAPI_DF2RAW_CLIENT_NAME_IS_NOT_VALID = 0xC0AA0604,
    E_IMAPI_DF2RAW_MEDIA_IS_NOT_BLANK = 0xC0AA0606,
    E_IMAPI_DF2RAW_MEDIA_IS_NOT_SUPPORTED = 0xC0AA0607,
    E_IMAPI_DF2RAW_NOT_ENOUGH_SPACE = 0xC0AA0609,
    E_IMAPI_DF2RAW_NO_RECORDER_SPECIFIED = 0xC0AA060A,
    E_IMAPI_DF2RAW_STREAM_NOT_SUPPORTED = 0xC0AA060D,
    E_IMAPI_DF2RAW_DATA_BLOCK_TYPE_NOT_SUPPORTED = 0xC0AA060E,
    E_IMAPI_DF2RAW_STREAM_LEADIN_TOO_SHORT = 0xC0AA060F,
    E_IMAPI_DF2RAW_RECORDER_NOT_SUPPORTED = 0xC0AA0610,
    E_IMAPI_ERASE_MEDIA_IS_NOT_SUPPORTED = 0xC0AA0909,
    E_IMAPI_ERASE_RECORDER_NOT_SUPPORTED = 0xC0AA090A,
    E_IMAPI_ERASE_CLIENT_NAME_IS_NOT_VALID = 0xC0AA090B,
    IMAPI_E_FSI_INTERNAL_ERROR = 0xC0AAB100,
    IMAPI_E_INVALID_PARAM = 0xC0AAB101,
    IMAPI_E_READONLY = 0xC0AAB102,
    IMAPI_E_NO_OUTPUT = 0xC0AAB103,
    IMAPI_E_INVALID_VOLUME_NAME = 0xC0AAB104,
    IMAPI_E_INVALID_DATE = 0xC0AAB105,
    IMAPI_E_FILE_SYSTEM_NOT_EMPTY = 0xC0AAB106,
    IMAPI_E_NOT_FILE = 0xC0AAB108,
    IMAPI_E_NOT_DIR = 0xC0AAB109,
    IMAPI_E_DIR_NOT_EMPTY = 0xC0AAB10A,
    IMAPI_E_NOT_IN_FILE_SYSTEM = 0xC0AAB10B,
    IMAPI_E_INVALID_PATH = 0xC0AAB110,
    IMAPI_E_RESTRICTED_NAME_VIOLATION = 0xC0AAB111,
    IMAPI_E_DUP_NAME = 0xC0AAB112,
    IMAPI_E_NO_UNIQUE_NAME = 0xC0AAB113,
    IMAPI_E_ITEM_NOT_FOUND = 0xC0AAB118,
    IMAPI_E_FILE_NOT_FOUND = 0xC0AAB119,
    IMAPI_E_DIR_NOT_FOUND = 0xC0AAB11A,
    IMAPI_E_IMAGE_SIZE_LIMIT = 0xC0AAB120,
    IMAPI_E_IMAGE_TOO_BIG = 0xC0AAB121,
    IMAPI_E_DATA_STREAM_INCONSISTENCY = 0xC0AAB128,
    IMAPI_E_DATA_STREAM_READ_FAILURE = 0xC0AAB129,
    IMAPI_E_DATA_STREAM_CREATE_FAILURE = 0xC0AAB12A,
    IMAPI_E_DIRECTORY_READ_FAILURE = 0xC0AAB12B,
    IMAPI_E_TOO_MANY_DIRS = 0xC0AAB130,
    IMAPI_E_ISO9660_LEVELS = 0xC0AAB131,
    IMAPI_E_DATA_TOO_BIG = 0xC0AAB132,
    IMAPI_E_INCOMPATIBLE_PREVIOUS_SESSION = 0xC0AAB133,
    IMAPI_E_STASHFILE_OPEN_FAILURE = 0xC0AAB138,
    IMAPI_E_STASHFILE_SEEK_FAILURE = 0xC0AAB139,
    IMAPI_E_STASHFILE_WRITE_FAILURE = 0xC0AAB13A,
    IMAPI_E_STASHFILE_READ_FAILURE = 0xC0AAB13B,
    IMAPI_E_INVALID_WORKING_DIRECTORY = 0xC0AAB140,
    IMAPI_E_WORKING_DIRECTORY_SPACE = 0xC0AAB141,
    IMAPI_E_STASHFILE_MOVE = 0xC0AAB142,
    IMAPI_E_BOOT_IMAGE_DATA = 0xC0AAB148,
    IMAPI_E_BOOT_OBJECT_CONFLICT = 0xC0AAB149,
    IMAPI_E_BOOT_EMULATION_IMAGE_SIZE_MISMATCH = 0xC0AAB14A,
    IMAPI_E_EMPTY_DISC = 0xC0AAB150,
    IMAPI_E_NO_SUPPORTED_FILE_SYSTEM = 0xC0AAB151,
    IMAPI_E_FILE_SYSTEM_NOT_FOUND = 0xC0AAB152,
    IMAPI_E_FILE_SYSTEM_READ_CONSISTENCY_ERROR = 0xC0AAB153,
    IMAPI_E_FILE_SYSTEM_FEATURE_NOT_SUPPORTED = 0xC0AAB154,
    IMAPI_E_IMPORT_TYPE_COLLISION_FILE_EXISTS_AS_DIRECTORY = 0xC0AAB155,
    IMAPI_E_IMPORT_SEEK_FAILURE = 0xC0AAB156,
    IMAPI_E_IMPORT_READ_FAILURE = 0xC0AAB157,
    IMAPI_E_DISC_MISMATCH = 0xC0AAB158,
    IMAPI_E_IMPORT_MEDIA_NOT_ALLOWED = 0xC0AAB159,
    IMAPI_E_UDF_NOT_WRITE_COMPATIBLE = 0xC0AAB15A,
    IMAPI_E_INCOMPATIBLE_MULTISESSION_TYPE = 0xC0AAB15B,
    IMAPI_E_NO_COMPATIBLE_MULTISESSION_TYPE = 0xC0AAB15C,
    IMAPI_E_MULTISESSION_NOT_SET = 0xC0AAB15D,
    IMAPI_E_IMPORT_TYPE_COLLISION_DIRECTORY_EXISTS_AS_FILE = 0xC0AAB15E,
    IMAPI_E_PROPERTY_NOT_ACCESSIBLE = 0xC0AAB160,
    IMAPI_E_UDF_REVISION_CHANGE_NOT_ALLOWED = 0xC0AAB161,
    IMAPI_E_BAD_MULTISESSION_PARAMETER = 0xC0AAB162,
    IMAPI_E_FILE_SYSTEM_CHANGE_NOT_ALLOWED = 0xC0AAB163,
    IMAPI_E_IMAGEMANAGER_IMAGE_NOT_ALIGNED = 0xC0AAB200,
    IMAPI_E_IMAGEMANAGER_NO_VALID_VD_FOUND = 0xC0AAB201,
    IMAPI_E_IMAGEMANAGER_NO_IMAGE = 0xC0AAB202,
    IMAPI_E_IMAGEMANAGER_IMAGE_TOO_BIG = 0xC0AAB203,
}

impl IMAPI2 {
    pub fn description(&self) -> &'static str {
        match self {
            IMAPI2::S_IMAPI_SPEEDADJUSTED => "The requested write speed was not supported by the drive and the speed was adjusted.",
            IMAPI2::S_IMAPI_ROTATIONADJUSTED => "The requested rotation type was not supported by the drive and the rotation type was adjusted.",
            IMAPI2::S_IMAPI_BOTHADJUSTED => "The requested write speed and rotation type were not supported by the drive and they were both adjusted.",
            IMAPI2::S_IMAPI_COMMAND_HAS_SENSE_DATA => "The device accepted the command, but returned sense data, indicating an error.",
            IMAPI2::S_IMAPI_WRITE_NOT_IN_PROGRESS => "There is no write operation currently in progress.",
            IMAPI2::S_IMAPI_RAW_IMAGE_TRACK_INDEX_ALREADY_EXISTS => "The specified LBA offset is already in the list of track indexes.",
            IMAPI2::IMAPI_S_IMAGE_FEATURE_NOT_SUPPORTED => "Feature is not supported for the current file system revision, image will be created without this feature.",
            IMAPI2::E_IMAPI_ERASE_RECORDER_IN_USE => "The format is currently using the disc recorder for an erase operation. Please wait for the erase to complete before attempting to set or clear the current disc recorder.",
            IMAPI2::E_IMAPI_ERASE_ONLY_ONE_RECORDER_SUPPORTED => "The erase format only supports one recorder. You must clear the current recorder before setting a new one.",
            IMAPI2::E_IMAPI_ERASE_DISC_INFORMATION_TOO_SMALL => "The drive did not report sufficient data for a READ DISC INFORMATION command. The drive may not be supported, or the media may not be correct.",
            IMAPI2::E_IMAPI_ERASE_MODE_PAGE_2A_TOO_SMALL => "The drive did not report sufficient data for a MODE SENSE (page 0x2A) command. The drive may not be supported, or the media may not be correct.",
            IMAPI2::E_IMAPI_ERASE_MEDIA_IS_NOT_ERASABLE => "The drive reported that the media is not erasable.",
            IMAPI2::E_IMAPI_ERASE_DRIVE_FAILED_ERASE_COMMAND => "The drive failed the erase command.",
            IMAPI2::E_IMAPI_ERASE_TOOK_LONGER_THAN_ONE_HOUR => "The drive did not complete the erase in one hour. The drive may require a power cycle, media removal, or other manual intervention to resume proper operation.",
            IMAPI2::E_IMAPI_ERASE_UNEXPECTED_DRIVE_RESPONSE_DURING_ERASE => "The drive returned an unexpected error during the erase. The the media may be unusable, the erase may be complete, or the drive may still be in the process of erasing the disc.",
            IMAPI2::E_IMAPI_ERASE_DRIVE_FAILED_SPINUP_COMMAND => "The drive returned an error for a START UNIT (spinup) command. Manual intervention may be required.",
            IMAPI2::E_IMAPI_RAW_IMAGE_IS_READ_ONLY => "The image has become read-only from a call to CreateResultImage(). The object can no longer be modified.",
            IMAPI2::E_IMAPI_RAW_IMAGE_TOO_MANY_TRACKS => "No more tracks may be added, as CD media is restricted to track numbers between 1 and 99.",
            IMAPI2::E_IMAPI_RAW_IMAGE_SECTOR_TYPE_NOT_SUPPORTED => "The requested sector type is not supported.",
            IMAPI2::E_IMAPI_RAW_IMAGE_NO_TRACKS => "Tracks must be added to the image before using this function.",
            IMAPI2::E_IMAPI_RAW_IMAGE_TRACKS_ALREADY_ADDED => "Tracks may not be added to the image prior to the use of this function.",
            IMAPI2::E_IMAPI_RAW_IMAGE_INSUFFICIENT_SPACE => "Adding the track would result in exceeding the limit for the start of the leadout.",
            IMAPI2::E_IMAPI_RAW_IMAGE_TOO_MANY_TRACK_INDEXES => "Adding the track index would result in exceeding the 99 index limit.",
            IMAPI2::E_IMAPI_RAW_IMAGE_TRACK_INDEX_NOT_FOUND => "The specified LBA offset is not in the list of track indexes.",
            IMAPI2::E_IMAPI_RAW_IMAGE_TRACK_INDEX_OFFSET_ZERO_CANNOT_BE_CLEARED => "Index 1 (LBA offset zero) may not be cleared.",
            IMAPI2::E_IMAPI_RAW_IMAGE_TRACK_INDEX_TOO_CLOSE_TO_OTHER_INDEX => "Each index must have a minimum size of ten sectors.",
            IMAPI2::E_IMAPI_REQUEST_CANCELLED => "The request was cancelled.",
            IMAPI2::E_IMAPI_RECORDER_REQUIRED => "The request requires a current disc recorder to be selected.",
            IMAPI2::E_IMAPI_BURN_VERIFICATION_FAILED => "The disc did not pass burn verification and may contain corrupt data or be unusable.",
            IMAPI2::E_IMAPI_RECORDER_NO_SUCH_MODE_PAGE => "The device reported that the requested mode page (and type) is not present.",
            IMAPI2::E_IMAPI_RECORDER_MEDIA_NO_MEDIA => "There is no media in the device.",
            IMAPI2::E_IMAPI_RECORDER_MEDIA_INCOMPATIBLE => "The media is not compatible or of unknown physical format.",
            IMAPI2::E_IMAPI_RECORDER_MEDIA_UPSIDE_DOWN => "The media is inserted upside down.",
            IMAPI2::E_IMAPI_RECORDER_MEDIA_BECOMING_READY => "The drive reported that it is in the process of becoming ready. Please try the request again later.",
            IMAPI2::E_IMAPI_RECORDER_MEDIA_FORMAT_IN_PROGRESS => "The media is currently being formatted. Please wait for the format to complete before attempting to use the media.",
            IMAPI2::E_IMAPI_RECORDER_MEDIA_BUSY => "The drive reported that it is performing a long-running operation, such as finishing a write. The drive may be unusable for a long period of time.",
            IMAPI2::E_IMAPI_RECORDER_INVALID_MODE_PARAMETERS => "The drive reported that the combination of parameters provided in the mode page for a MODE SELECT command were not supported.",
            IMAPI2::E_IMAPI_RECORDER_MEDIA_WRITE_PROTECTED => "The drive reported that the media is write protected.",
            IMAPI2::E_IMAPI_RECORDER_NO_SUCH_FEATURE => "The feature page requested is not supported by the device.",
            IMAPI2::E_IMAPI_RECORDER_FEATURE_IS_NOT_CURRENT => "The feature page requested is supported, but is not marked as current.",
            IMAPI2::E_IMAPI_RECORDER_GET_CONFIGURATION_NOT_SUPPORTED => "The drive does not support the GET CONFIGURATION command.",
            IMAPI2::E_IMAPI_RECORDER_COMMAND_TIMEOUT => "The device failed to accept the command within the timeout period. This may be caused by the device having entered an inconsistent state, or the timeout value for the command may need to be increased.",
            IMAPI2::E_IMAPI_RECORDER_DVD_STRUCTURE_NOT_PRESENT => "The DVD structure is not present. This may be caused by incompatible drive/medium used.",
            IMAPI2::E_IMAPI_RECORDER_MEDIA_SPEED_MISMATCH => "The media's speed is incompatible with the device. This may be caused by using higher or lower speed media than the range of speeds supported by the device.",
            IMAPI2::E_IMAPI_RECORDER_LOCKED => "The device associated with this recorder during the last operation has been exclusively locked, causing this operation to failed.",
            IMAPI2::E_IMAPI_RECORDER_CLIENT_NAME_IS_NOT_VALID => "The client name is not valid.",
            IMAPI2::E_IMAPI_RECORDER_MEDIA_NOT_FORMATTED => "The media is not formatted. Please format the media before attempting to use it.",
            IMAPI2::E_IMAPI_RECORDER_INVALID_RESPONSE_FROM_DEVICE => "The device reported unexpected or invalid data for a command.",
            IMAPI2::E_IMAPI_LOSS_OF_STREAMING => "The write failed because the drive did not receive data quickly enough to continue writing. Moving the source data to the local computer, reducing the write speed, or enabling a 'buffer underrun free' setting may resolve this issue.",
            IMAPI2::E_IMAPI_UNEXPECTED_RESPONSE_FROM_DEVICE => "The write failed because the drive returned error information that could not be recovered from.",
            IMAPI2::E_IMAPI_DF2DATA_WRITE_IN_PROGRESS => "There is currently a write operation in progress.",
            IMAPI2::E_IMAPI_DF2DATA_WRITE_NOT_IN_PROGRESS => "There is no write operation currently in progress.",
            IMAPI2::E_IMAPI_DF2DATA_INVALID_MEDIA_STATE => "The requested operation is only valid with supported media.",
            IMAPI2::E_IMAPI_DF2DATA_STREAM_NOT_SUPPORTED => "The provided stream to write is not supported.",
            IMAPI2::E_IMAPI_DF2DATA_STREAM_TOO_LARGE_FOR_CURRENT_MEDIA => "The provided stream to write is too large for the currently inserted media.",
            IMAPI2::E_IMAPI_DF2DATA_MEDIA_NOT_BLANK => "Overwriting non-blank media is not allowed without the ForceOverwrite property set to VARIANT_TRUE.",
            IMAPI2::E_IMAPI_DF2DATA_MEDIA_IS_NOT_SUPPORTED => "The current media type is unsupported.",
            IMAPI2::E_IMAPI_DF2DATA_RECORDER_NOT_SUPPORTED => "This device does not support the operations required by this disc format.",
            IMAPI2::E_IMAPI_DF2DATA_CLIENT_NAME_IS_NOT_VALID => "The client name is not valid.",
            IMAPI2::E_IMAPI_DF2TAO_WRITE_IN_PROGRESS => "There is currently a write operation in progress.",
            IMAPI2::E_IMAPI_DF2TAO_WRITE_NOT_IN_PROGRESS => "There is no write operation currently in progress.",
            IMAPI2::E_IMAPI_DF2TAO_MEDIA_IS_NOT_PREPARED => "The requested operation is only valid when media has been 'prepared'.",
            IMAPI2::E_IMAPI_DF2TAO_MEDIA_IS_PREPARED => "The requested operation is not valid when media has been 'prepared' but not released.",
            IMAPI2::E_IMAPI_DF2TAO_PROPERTY_FOR_BLANK_MEDIA_ONLY => "The property cannot be changed once the media has been written to.",
            IMAPI2::E_IMAPI_DF2TAO_TABLE_OF_CONTENTS_EMPTY_DISC => "The table of contents cannot be retrieved from an empty disc.",
            IMAPI2::E_IMAPI_DF2TAO_MEDIA_IS_NOT_BLANK => "Only blank CD-R/RW media is supported.",
            IMAPI2::E_IMAPI_DF2TAO_MEDIA_IS_NOT_SUPPORTED => "Only blank CD-R/RW media is supported.",
            IMAPI2::E_IMAPI_DF2TAO_TRACK_LIMIT_REACHED => "CD-R and CD-RW media support a maximum of 99 audio tracks.",
            IMAPI2::E_IMAPI_DF2TAO_NOT_ENOUGH_SPACE => "There is not enough space left on the media to add the provided audio track.",
            IMAPI2::E_IMAPI_DF2TAO_NO_RECORDER_SPECIFIED => "You cannot prepare the media until you choose a recorder to use.",
            IMAPI2::E_IMAPI_DF2TAO_INVALID_ISRC => "The ISRC provided is not valid.",
            IMAPI2::E_IMAPI_DF2TAO_INVALID_MCN => "The Media Catalog Number provided is not valid.",
            IMAPI2::E_IMAPI_DF2TAO_STREAM_NOT_SUPPORTED => "The provided audio stream is not valid.",
            IMAPI2::E_IMAPI_DF2TAO_RECORDER_NOT_SUPPORTED => "This device does not support the operations required by this disc format.",
            IMAPI2::E_IMAPI_DF2TAO_CLIENT_NAME_IS_NOT_VALID => "The client name is not valid.",
            IMAPI2::E_IMAPI_DF2RAW_WRITE_IN_PROGRESS => "There is currently a write operation in progress.",
            IMAPI2::E_IMAPI_DF2RAW_WRITE_NOT_IN_PROGRESS => "There is no write operation currently in progress.",
            IMAPI2::E_IMAPI_DF2RAW_MEDIA_IS_NOT_PREPARED => "The requested operation is only valid when media has been 'prepared'.",
            IMAPI2::E_IMAPI_DF2RAW_MEDIA_IS_PREPARED => "The requested operation is not valid when media has been 'prepared' but not released.",
            IMAPI2::E_IMAPI_DF2RAW_CLIENT_NAME_IS_NOT_VALID => "The client name is not valid.",
            IMAPI2::E_IMAPI_DF2RAW_MEDIA_IS_NOT_BLANK => "Only blank CD-R/RW media is supported.",
            IMAPI2::E_IMAPI_DF2RAW_MEDIA_IS_NOT_SUPPORTED => "Only blank CD-R/RW media is supported.",
            IMAPI2::E_IMAPI_DF2RAW_NOT_ENOUGH_SPACE => "There is not enough space on the media to add the provided session.",
            IMAPI2::E_IMAPI_DF2RAW_NO_RECORDER_SPECIFIED => "You cannot prepare the media until you choose a recorder to use.",
            IMAPI2::E_IMAPI_DF2RAW_STREAM_NOT_SUPPORTED => "The provided audio stream is not valid.",
            IMAPI2::E_IMAPI_DF2RAW_DATA_BLOCK_TYPE_NOT_SUPPORTED => "The requested data block type is not supported by the current device.",
            IMAPI2::E_IMAPI_DF2RAW_STREAM_LEADIN_TOO_SHORT => "The stream does not contain a sufficient number of sectors in the leadin for the current media.",
            IMAPI2::E_IMAPI_DF2RAW_RECORDER_NOT_SUPPORTED => "This device does not support the operations required by this disc format.",
            IMAPI2::E_IMAPI_ERASE_MEDIA_IS_NOT_SUPPORTED => "The current media type is unsupported.",
            IMAPI2::E_IMAPI_ERASE_RECORDER_NOT_SUPPORTED => "This device does not support the operations required by this disc format.",
            IMAPI2::E_IMAPI_ERASE_CLIENT_NAME_IS_NOT_VALID => "The client name is not valid.",
            IMAPI2::IMAPI_E_FSI_INTERNAL_ERROR => "Internal file system error occurred.",
            IMAPI2::IMAPI_E_INVALID_PARAM => "The value specified for parameter '%1!ls!' is not valid.",
            IMAPI2::IMAPI_E_READONLY => "FileSystemImage object is in read only mode.",
            IMAPI2::IMAPI_E_NO_OUTPUT => "No output file system specified.",
            IMAPI2::IMAPI_E_INVALID_VOLUME_NAME => "The specified Volume Identifier is either too long or contains one or more invalid characters.",
            IMAPI2::IMAPI_E_INVALID_DATE => "Invalid file dates. %1!ls! time is earlier than %2!ls! time.",
            IMAPI2::IMAPI_E_FILE_SYSTEM_NOT_EMPTY => "The file system must be empty for this function.",
            IMAPI2::IMAPI_E_NOT_FILE => "Specified path '%1!ls!' does not identify a file.",
            IMAPI2::IMAPI_E_NOT_DIR => "Specified path '%1!ls!' does not identify a directory.",
            IMAPI2::IMAPI_E_DIR_NOT_EMPTY => "The directory '%1!s!' is not empty.",
            IMAPI2::IMAPI_E_NOT_IN_FILE_SYSTEM => "'%1!ls!' is not part of the file system. It must be added to complete this operation.",
            IMAPI2::IMAPI_E_INVALID_PATH => "Path '%1!s!' is badly formed or contains invalid characters.",
            IMAPI2::IMAPI_E_RESTRICTED_NAME_VIOLATION => "The name '%1!ls!' specified is not legal: Name of file or directory object created while the UseRestrictedCharacterSet property is set may only contain ANSI characters.",
            IMAPI2::IMAPI_E_DUP_NAME => "'%1!ls!' name already exists.",
            IMAPI2::IMAPI_E_NO_UNIQUE_NAME => "Attempt to add '%1!ls!' failed: cannot create a file-system-specific unique name for the %2!ls! file system.",
            IMAPI2::IMAPI_E_ITEM_NOT_FOUND => "Cannot find item '%1!ls!' in FileSystemImage hierarchy.",
            IMAPI2::IMAPI_E_FILE_NOT_FOUND => "The file '%1!s!' not found in FileSystemImage hierarchy.",
            IMAPI2::IMAPI_E_DIR_NOT_FOUND => "The directory '%1!s!' not found in FileSystemImage hierarchy.",
            IMAPI2::IMAPI_E_IMAGE_SIZE_LIMIT => "Adding '%1!ls!' would result in a result image having a size larger than the current configured limit.",
            IMAPI2::IMAPI_E_IMAGE_TOO_BIG => "Value specified for FreeMediaBlocks property is too small for estimated image size based on current data.",
            IMAPI2::IMAPI_E_DATA_STREAM_INCONSISTENCY => "Data stream supplied for file '%1!ls!' is inconsistent: expected %2!I64d! bytes, found %3!I64d!.",
            IMAPI2::IMAPI_E_DATA_STREAM_READ_FAILURE => "Cannot read data from stream supplied for file '%1!ls!'.",
            IMAPI2::IMAPI_E_DATA_STREAM_CREATE_FAILURE => "The following error was encountered when trying to create data stream for '%1!ls!':",
            IMAPI2::IMAPI_E_DIRECTORY_READ_FAILURE => "The following error was encountered when trying to enumerate files in directory '%1!ls!':",
            IMAPI2::IMAPI_E_TOO_MANY_DIRS => "This file system image has too many directories for the %1!ls! file system.",
            IMAPI2::IMAPI_E_ISO9660_LEVELS => "ISO9660 is limited to 8 levels of directories.",
            IMAPI2::IMAPI_E_DATA_TOO_BIG => "Data file is too large for '%1!ls!' file system.",
            IMAPI2::IMAPI_E_INCOMPATIBLE_PREVIOUS_SESSION => "Operation failed because of incompatible layout of the previous session imported from the medium.",
            IMAPI2::IMAPI_E_STASHFILE_OPEN_FAILURE => "Cannot initialize %1!ls! stash file.",
            IMAPI2::IMAPI_E_STASHFILE_SEEK_FAILURE => "Error seeking in '%1!ls!' stash file.",
            IMAPI2::IMAPI_E_STASHFILE_WRITE_FAILURE => "Error encountered writing to '%1!ls!' stash file.",
            IMAPI2::IMAPI_E_STASHFILE_READ_FAILURE => "Error encountered reading from '%1!ls!' stash file.",
            IMAPI2::IMAPI_E_INVALID_WORKING_DIRECTORY => "The working directory '%1!ls!' is not valid.",
            IMAPI2::IMAPI_E_WORKING_DIRECTORY_SPACE => "Cannot set working directory to '%1!ls!'. Space available is %2!I64d! bytes, approximately %3!I64d! bytes required.",
            IMAPI2::IMAPI_E_STASHFILE_MOVE => "Attempt to move the data stash file to directory '%1!ls!' was not successful.",
            IMAPI2::IMAPI_E_BOOT_IMAGE_DATA => "The boot object could not be added to the image.",
            IMAPI2::IMAPI_E_BOOT_OBJECT_CONFLICT => "A boot object can only be included in an initial disc image.",
            IMAPI2::IMAPI_E_BOOT_EMULATION_IMAGE_SIZE_MISMATCH => "The emulation type requested does not match the boot image size.",
            IMAPI2::IMAPI_E_EMPTY_DISC => "Optical media is empty.",
            IMAPI2::IMAPI_E_NO_SUPPORTED_FILE_SYSTEM => "The specified disc does not contain one of the supported file systems.",
            IMAPI2::IMAPI_E_FILE_SYSTEM_NOT_FOUND => "The specified disc does not contain a '%1!ls!' file system.",
            IMAPI2::IMAPI_E_FILE_SYSTEM_READ_CONSISTENCY_ERROR => "Consistency error encountered while importing the '%1!ls!' file system.",
            IMAPI2::IMAPI_E_FILE_SYSTEM_FEATURE_NOT_SUPPORTED => "The '%1!ls!'file system on the selected disc contains a feature not supported for import.",
            IMAPI2::IMAPI_E_IMPORT_TYPE_COLLISION_FILE_EXISTS_AS_DIRECTORY => "Could not import %2!ls! file system from disc. The file '%1!ls!' already exists within the image hierarchy as a directory.",
            IMAPI2::IMAPI_E_IMPORT_SEEK_FAILURE => "Cannot seek to block %1!I64d! on source disc.",
            IMAPI2::IMAPI_E_IMPORT_READ_FAILURE => "Import from previous session failed due to an error reading a block on the media (most likely block %1!u!).",
            IMAPI2::IMAPI_E_DISC_MISMATCH => "Current disc is not the same one from which file system was imported.",
            IMAPI2::IMAPI_E_IMPORT_MEDIA_NOT_ALLOWED => "IMAPI does not allow multi-session with the current media type.",
            IMAPI2::IMAPI_E_UDF_NOT_WRITE_COMPATIBLE => "IMAPI can not do multi-session with the current media because it does not support a compatible UDF revision for write.",
            IMAPI2::IMAPI_E_INCOMPATIBLE_MULTISESSION_TYPE => "IMAPI does not support the multisession type requested.",
            IMAPI2::IMAPI_E_NO_COMPATIBLE_MULTISESSION_TYPE => "IMAPI supports none of the multisession type(s) provided on the current media.",
            IMAPI2::IMAPI_E_MULTISESSION_NOT_SET => "MultisessionInterfaces property must be set prior calling this method.",
            IMAPI2::IMAPI_E_IMPORT_TYPE_COLLISION_DIRECTORY_EXISTS_AS_FILE => "Could not import %2!ls! file system from disc. The directory '%1!ls!' already exists within the image hierarchy as a file.",
            IMAPI2::IMAPI_E_PROPERTY_NOT_ACCESSIBLE => "Property '%1!ls!' is not accessible",
            IMAPI2::IMAPI_E_UDF_REVISION_CHANGE_NOT_ALLOWED => "UDF revision cannot be changed because of the previously imported session",
            IMAPI2::IMAPI_E_BAD_MULTISESSION_PARAMETER => "One of the multisession parameters cannot be retrieved or has a wrong value.",
            IMAPI2::IMAPI_E_FILE_SYSTEM_CHANGE_NOT_ALLOWED => "You cannot change the file system to be created, because the file system in the imported session and the one in the new session must match.",
            IMAPI2::IMAPI_E_IMAGEMANAGER_IMAGE_NOT_ALIGNED => "The image is not 2kb aligned. Only 2048 bytes aligned images are supported.",
            IMAPI2::IMAPI_E_IMAGEMANAGER_NO_VALID_VD_FOUND => "No valid file system Volume Descriptor was found in the iso image. This image format is not supported and the resulting disc might not be readable.",
            IMAPI2::IMAPI_E_IMAGEMANAGER_NO_IMAGE => "No image was set (neither path nor stream was given).",
            IMAPI2::IMAPI_E_IMAGEMANAGER_IMAGE_TOO_BIG => "Image size exceeds MAXLONG sectors - too big.",
        }
    }
}
