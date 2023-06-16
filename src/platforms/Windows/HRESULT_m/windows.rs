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

pub enum WINDOWS {
    CO_S_NOTALLINTERFACES = 0x00080012,
    CO_S_MACHINENAMENOTFOUND = 0x00080013,
    CO_E_CLASS_CREATE_FAILED = 0x80080001,
    CO_E_SCM_ERROR = 0x80080002,
    CO_E_SCM_RPC_FAILURE = 0x80080003,
    CO_E_BAD_PATH = 0x80080004,
    CO_E_SERVER_EXEC_FAILURE = 0x80080005,
    CO_E_OBJSRV_RPC_FAILURE = 0x80080006,
    MK_E_NO_NORMALIZED = 0x80080007,
    CO_E_SERVER_STOPPING = 0x80080008,
    MEM_E_INVALID_ROOT = 0x80080009,
    MEM_E_INVALID_LINK = 0x80080010,
    MEM_E_INVALID_SIZE = 0x80080011,
    CO_E_MISSING_DISPLAYNAME = 0x80080015,
    CO_E_RUNAS_VALUE_MUST_BE_AAA = 0x80080016,
    CO_E_ELEVATION_DISABLED = 0x80080017,
    APPX_E_PACKAGING_INTERNAL = 0x80080200,
    APPX_E_INTERLEAVING_NOT_ALLOWED = 0x80080201,
    APPX_E_RELATIONSHIPS_NOT_ALLOWED = 0x80080202,
    APPX_E_MISSING_REQUIRED_FILE = 0x80080203,
    APPX_E_INVALID_MANIFEST = 0x80080204,
    APPX_E_INVALID_BLOCKMAP = 0x80080205,
    APPX_E_CORRUPT_CONTENT = 0x80080206,
    APPX_E_BLOCK_HASH_INVALID = 0x80080207,
    APPX_E_REQUESTED_RANGE_TOO_LARGE = 0x80080208,
    APPX_E_INVALID_SIP_CLIENT_DATA = 0x80080209,
    APPX_E_INVALID_KEY_INFO = 0x8008020A,
    APPX_E_INVALID_CONTENTGROUPMAP = 0x8008020B,
    APPX_E_INVALID_APPINSTALLER = 0x8008020C,
    APPX_E_DELTA_BASELINE_VERSION_MISMATCH = 0x8008020D,
    APPX_E_DELTA_PACKAGE_MISSING_FILE = 0x8008020E,
    APPX_E_INVALID_DELTA_PACKAGE = 0x8008020F,
    APPX_E_DELTA_APPENDED_PACKAGE_NOT_ALLOWED = 0x80080210,
    APPX_E_INVALID_PACKAGING_LAYOUT = 0x80080211,
    APPX_E_INVALID_PACKAGESIGNCONFIG = 0x80080212,
    APPX_E_RESOURCESPRI_NOT_ALLOWED = 0x80080213,
    APPX_E_FILE_COMPRESSION_MISMATCH = 0x80080214,
    APPX_E_INVALID_PAYLOAD_PACKAGE_EXTENSION = 0x80080215,
    APPX_E_INVALID_ENCRYPTION_EXCLUSION_FILE_LIST = 0x80080216,
    APPX_E_INVALID_PACKAGE_FOLDER_ACLS = 0x80080217,
    APPX_E_INVALID_PUBLISHER_BRIDGING = 0x80080218,
    APPX_E_DIGEST_MISMATCH = 0x80080219,
    BT_E_SPURIOUS_ACTIVATION = 0x80080300,
}

impl WINDOWS {
    pub fn description(&self) -> &'static str {
        match self {
            WINDOWS::CO_S_NOTALLINTERFACES => "Not all the requested interfaces were available",
            WINDOWS::CO_S_MACHINENAMENOTFOUND => "The specified machine name was not found in the cache.",
            WINDOWS::CO_E_CLASS_CREATE_FAILED => "Attempt to create a class object failed",
            WINDOWS::CO_E_SCM_ERROR => "OLE service could not bind object",
            WINDOWS::CO_E_SCM_RPC_FAILURE => "RPC communication failed with OLE service",
            WINDOWS::CO_E_BAD_PATH => "Bad path to object",
            WINDOWS::CO_E_SERVER_EXEC_FAILURE => "Server execution failed",
            WINDOWS::CO_E_OBJSRV_RPC_FAILURE => "OLE service could not communicate with the object server",
            WINDOWS::MK_E_NO_NORMALIZED => "Moniker path could not be normalized",
            WINDOWS::CO_E_SERVER_STOPPING => "Object server is stopping when OLE service contacts it",
            WINDOWS::MEM_E_INVALID_ROOT => "An invalid root block pointer was specified",
            WINDOWS::MEM_E_INVALID_LINK => "An allocation chain contained an invalid link pointer",
            WINDOWS::MEM_E_INVALID_SIZE => "The requested allocation size was too large",
            WINDOWS::CO_E_MISSING_DISPLAYNAME => "The activation requires a display name to be present under the CLSID key.",
            WINDOWS::CO_E_RUNAS_VALUE_MUST_BE_AAA => "The activation requires that the RunAs value for the application is Activate As Activator.",
            WINDOWS::CO_E_ELEVATION_DISABLED => "The class is not configured to support Elevated activation.",
            WINDOWS::APPX_E_PACKAGING_INTERNAL => "Appx packaging API has encountered an internal error.",
            WINDOWS::APPX_E_INTERLEAVING_NOT_ALLOWED => "The file is not a valid Appx package because its contents are interleaved.",
            WINDOWS::APPX_E_RELATIONSHIPS_NOT_ALLOWED => "The file is not a valid Appx package because it contains OPC relationships.",
            WINDOWS::APPX_E_MISSING_REQUIRED_FILE => "The file is not a valid Appx package because it is missing a manifest or block map, or missing a signature file when the code integrity file is present.",
            WINDOWS::APPX_E_INVALID_MANIFEST => "The Appx package's manifest is invalid.",
            WINDOWS::APPX_E_INVALID_BLOCKMAP => "The Appx package's block map is invalid.",
            WINDOWS::APPX_E_CORRUPT_CONTENT => "The Appx package's content cannot be read because it is corrupt.",
            WINDOWS::APPX_E_BLOCK_HASH_INVALID => "The computed hash value of the block does not match the one stored in the block map.",
            WINDOWS::APPX_E_REQUESTED_RANGE_TOO_LARGE => "The requested byte range is over 4GB when translated to byte range of blocks.",
            WINDOWS::APPX_E_INVALID_SIP_CLIENT_DATA => "The SIP_SUBJECTINFO structure used to sign the package didn't contain the required data.",
            WINDOWS::APPX_E_INVALID_KEY_INFO => "The APPX_KEY_INFO structure used to encrypt or decrypt the package contains invalid data.",
            WINDOWS::APPX_E_INVALID_CONTENTGROUPMAP => "The Appx package's content group map is invalid.",
            WINDOWS::APPX_E_INVALID_APPINSTALLER => "The .appinstaller file is invalid.",
            WINDOWS::APPX_E_DELTA_BASELINE_VERSION_MISMATCH => "The baseline package version in delta package does not match the version in the baseline package to be updated.",
            WINDOWS::APPX_E_DELTA_PACKAGE_MISSING_FILE => "The delta package is missing a file from the updated package.",
            WINDOWS::APPX_E_INVALID_DELTA_PACKAGE => "The delta package is invalid.",
            WINDOWS::APPX_E_DELTA_APPENDED_PACKAGE_NOT_ALLOWED => "The delta appended package is not allowed for the current operation.",
            WINDOWS::APPX_E_INVALID_PACKAGING_LAYOUT => "The packaging layout file is invalid.",
            WINDOWS::APPX_E_INVALID_PACKAGESIGNCONFIG => "The packageSignConfig file is invalid.",
            WINDOWS::APPX_E_RESOURCESPRI_NOT_ALLOWED => "The resources.pri file is not allowed when there are no resource elements in the package manifest.",
            WINDOWS::APPX_E_FILE_COMPRESSION_MISMATCH => "The compression state of file in baseline and updated package does not match.",
            WINDOWS::APPX_E_INVALID_PAYLOAD_PACKAGE_EXTENSION => "Non appx extensions are not allowed for payload packages targeting older platforms.",
            WINDOWS::APPX_E_INVALID_ENCRYPTION_EXCLUSION_FILE_LIST => "The encryptionExclusionFileList file is invalid.",
            WINDOWS::APPX_E_INVALID_PACKAGE_FOLDER_ACLS => "The package folder ACLs are invalid.",
            WINDOWS::APPX_E_INVALID_PUBLISHER_BRIDGING => "The publisher bridging artifact is invalid.",
            WINDOWS::APPX_E_DIGEST_MISMATCH => "The expected digest value did not match the actual digest value of the content.",
            WINDOWS::BT_E_SPURIOUS_ACTIVATION => "The background task activation is spurious.",
        }
    }
}
