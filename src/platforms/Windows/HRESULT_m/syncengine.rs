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

pub enum SYNCENGINE {
    E_SYNCENGINE_FILE_SIZE_OVER_LIMIT = 0x8802B001,
    E_SYNCENGINE_FILE_SIZE_EXCEEDS_REMAINING_QUOTA = 0x8802B002,
    E_SYNCENGINE_UNSUPPORTED_FILE_NAME = 0x8802B003,
    E_SYNCENGINE_FOLDER_ITEM_COUNT_LIMIT_EXCEEDED = 0x8802B004,
    E_SYNCENGINE_FILE_SYNC_PARTNER_ERROR = 0x8802B005,
    E_SYNCENGINE_SYNC_PAUSED_BY_SERVICE = 0x8802B006,
    E_SYNCENGINE_FILE_IDENTIFIER_UNKNOWN = 0x8802C002,
    E_SYNCENGINE_SERVICE_AUTHENTICATION_FAILED = 0x8802C003,
    E_SYNCENGINE_UNKNOWN_SERVICE_ERROR = 0x8802C004,
    E_SYNCENGINE_SERVICE_RETURNED_UNEXPECTED_SIZE = 0x8802C005,
    E_SYNCENGINE_REQUEST_BLOCKED_BY_SERVICE = 0x8802C006,
    E_SYNCENGINE_REQUEST_BLOCKED_DUE_TO_CLIENT_ERROR = 0x8802C007,
    E_SYNCENGINE_FOLDER_INACCESSIBLE = 0x8802D001,
    E_SYNCENGINE_UNSUPPORTED_FOLDER_NAME = 0x8802D002,
    E_SYNCENGINE_UNSUPPORTED_MARKET = 0x8802D003,
    E_SYNCENGINE_PATH_LENGTH_LIMIT_EXCEEDED = 0x8802D004,
    E_SYNCENGINE_REMOTE_PATH_LENGTH_LIMIT_EXCEEDED = 0x8802D005,
    E_SYNCENGINE_CLIENT_UPDATE_NEEDED = 0x8802D006,
    E_SYNCENGINE_PROXY_AUTHENTICATION_REQUIRED = 0x8802D007,
    E_SYNCENGINE_STORAGE_SERVICE_PROVISIONING_FAILED = 0x8802D008,
    E_SYNCENGINE_UNSUPPORTED_REPARSE_POINT = 0x8802D009,
    E_SYNCENGINE_STORAGE_SERVICE_BLOCKED = 0x8802D00A,
    E_SYNCENGINE_FOLDER_IN_REDIRECTION = 0x8802D00B,
}

impl SYNCENGINE {
    pub fn description(&self) -> &'static str {
        match self {
            SYNCENGINE::E_SYNCENGINE_FILE_SIZE_OVER_LIMIT => "The file size is larger than supported by the sync engine.",
            SYNCENGINE::E_SYNCENGINE_FILE_SIZE_EXCEEDS_REMAINING_QUOTA => "The file cannot be uploaded because it doesn't fit in the user's available service provided storage space.",
            SYNCENGINE::E_SYNCENGINE_UNSUPPORTED_FILE_NAME => "The file name contains invalid characters.",
            SYNCENGINE::E_SYNCENGINE_FOLDER_ITEM_COUNT_LIMIT_EXCEEDED => "The maximum file count has been reached for this folder in the sync engine.",
            SYNCENGINE::E_SYNCENGINE_FILE_SYNC_PARTNER_ERROR => "The file sync has been delegated to another program and has run into an issue.",
            SYNCENGINE::E_SYNCENGINE_SYNC_PAUSED_BY_SERVICE => "Sync has been delayed due to a throttling request from the service.",
            SYNCENGINE::E_SYNCENGINE_FILE_IDENTIFIER_UNKNOWN => "We can't seem to find that file. Please try again later.",
            SYNCENGINE::E_SYNCENGINE_SERVICE_AUTHENTICATION_FAILED => "The account you're signed in with doesn't have permission to open this file.",
            SYNCENGINE::E_SYNCENGINE_UNKNOWN_SERVICE_ERROR => "There was a problem connecting to the service. Please try again later.",
            SYNCENGINE::E_SYNCENGINE_SERVICE_RETURNED_UNEXPECTED_SIZE => "Sorry, there was a problem downloading the file.",
            SYNCENGINE::E_SYNCENGINE_REQUEST_BLOCKED_BY_SERVICE => "We're having trouble downloading the file right now. Please try again later.",
            SYNCENGINE::E_SYNCENGINE_REQUEST_BLOCKED_DUE_TO_CLIENT_ERROR => "We're having trouble downloading the file right now. Please try again later.",
            SYNCENGINE::E_SYNCENGINE_FOLDER_INACCESSIBLE => "The sync engine does not have permissions to access a local folder under the sync root.",
            SYNCENGINE::E_SYNCENGINE_UNSUPPORTED_FOLDER_NAME => "The folder name contains invalid characters.",
            SYNCENGINE::E_SYNCENGINE_UNSUPPORTED_MARKET => "The sync engine is not allowed to run in your current market.",
            SYNCENGINE::E_SYNCENGINE_PATH_LENGTH_LIMIT_EXCEEDED => "All files and folders can't be uploaded because a path of a file or folder is too long.",
            SYNCENGINE::E_SYNCENGINE_REMOTE_PATH_LENGTH_LIMIT_EXCEEDED => "All file and folders cannot be synchronized because a path of a file or folder would exceed the local path limit.",
            SYNCENGINE::E_SYNCENGINE_CLIENT_UPDATE_NEEDED => "Updates are needed in order to use the sync engine.",
            SYNCENGINE::E_SYNCENGINE_PROXY_AUTHENTICATION_REQUIRED => "The sync engine needs to authenticate with a proxy server.",
            SYNCENGINE::E_SYNCENGINE_STORAGE_SERVICE_PROVISIONING_FAILED => "There was a problem setting up the storage services for the account.",
            SYNCENGINE::E_SYNCENGINE_UNSUPPORTED_REPARSE_POINT => "Files can't be uploaded because there's an unsupported reparse point.",
            SYNCENGINE::E_SYNCENGINE_STORAGE_SERVICE_BLOCKED => "The service has blocked your account from accessing the storage service.",
            SYNCENGINE::E_SYNCENGINE_FOLDER_IN_REDIRECTION => "The action can't be performed right now because this folder is being moved. Please try again later.",
        }
    }
}
