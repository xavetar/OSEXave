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

pub enum USERMODE_VHD {
    ERROR_QUERY_STORAGE_ERROR = 0x803A0001,
    ERROR_VHD_DRIVE_FOOTER_MISSING = 0xC03A0001,
    ERROR_VHD_DRIVE_FOOTER_CHECKSUM_MISMATCH = 0xC03A0002,
    ERROR_VHD_DRIVE_FOOTER_CORRUPT = 0xC03A0003,
    ERROR_VHD_FORMAT_UNKNOWN = 0xC03A0004,
    ERROR_VHD_FORMAT_UNSUPPORTED_VERSION = 0xC03A0005,
    ERROR_VHD_SPARSE_HEADER_CHECKSUM_MISMATCH = 0xC03A0006,
    ERROR_VHD_SPARSE_HEADER_UNSUPPORTED_VERSION = 0xC03A0007,
    ERROR_VHD_SPARSE_HEADER_CORRUPT = 0xC03A0008,
    ERROR_VHD_BLOCK_ALLOCATION_FAILURE = 0xC03A0009,
    ERROR_VHD_BLOCK_ALLOCATION_TABLE_CORRUPT = 0xC03A000A,
    ERROR_VHD_INVALID_BLOCK_SIZE = 0xC03A000B,
    ERROR_VHD_BITMAP_MISMATCH = 0xC03A000C,
    ERROR_VHD_PARENT_VHD_NOT_FOUND = 0xC03A000D,
    ERROR_VHD_CHILD_PARENT_ID_MISMATCH = 0xC03A000E,
    ERROR_VHD_CHILD_PARENT_TIMESTAMP_MISMATCH = 0xC03A000F,
    ERROR_VHD_METADATA_READ_FAILURE = 0xC03A0010,
    ERROR_VHD_METADATA_WRITE_FAILURE = 0xC03A0011,
    ERROR_VHD_INVALID_SIZE = 0xC03A0012,
    ERROR_VHD_INVALID_FILE_SIZE = 0xC03A0013,
    ERROR_VIRTDISK_PROVIDER_NOT_FOUND = 0xC03A0014,
    ERROR_VIRTDISK_NOT_VIRTUAL_DISK = 0xC03A0015,
    ERROR_VHD_PARENT_VHD_ACCESS_DENIED = 0xC03A0016,
    ERROR_VHD_CHILD_PARENT_SIZE_MISMATCH = 0xC03A0017,
    ERROR_VHD_DIFFERENCING_CHAIN_CYCLE_DETECTED = 0xC03A0018,
    ERROR_VHD_DIFFERENCING_CHAIN_ERROR_IN_PARENT = 0xC03A0019,
    ERROR_VIRTUAL_DISK_LIMITATION = 0xC03A001A,
    ERROR_VHD_INVALID_TYPE = 0xC03A001B,
    ERROR_VHD_INVALID_STATE = 0xC03A001C,
    ERROR_VIRTDISK_UNSUPPORTED_DISK_SECTOR_SIZE = 0xC03A001D,
    ERROR_VIRTDISK_DISK_ALREADY_OWNED = 0xC03A001E,
    ERROR_VIRTDISK_DISK_ONLINE_AND_WRITABLE = 0xC03A001F,
    ERROR_CTLOG_TRACKING_NOT_INITIALIZED = 0xC03A0020,
    ERROR_CTLOG_LOGFILE_SIZE_EXCEEDED_MAXSIZE = 0xC03A0021,
    ERROR_CTLOG_VHD_CHANGED_OFFLINE = 0xC03A0022,
    ERROR_CTLOG_INVALID_TRACKING_STATE = 0xC03A0023,
    ERROR_CTLOG_INCONSISTENT_TRACKING_FILE = 0xC03A0024,
    ERROR_VHD_RESIZE_WOULD_TRUNCATE_DATA = 0xC03A0025,
    ERROR_VHD_COULD_NOT_COMPUTE_MINIMUM_VIRTUAL_SIZE = 0xC03A0026,
    ERROR_VHD_ALREADY_AT_OR_BELOW_MINIMUM_VIRTUAL_SIZE = 0xC03A0027,
    ERROR_VHD_METADATA_FULL = 0xC03A0028,
    ERROR_VHD_INVALID_CHANGE_TRACKING_ID = 0xC03A0029,
    ERROR_VHD_CHANGE_TRACKING_DISABLED = 0xC03A002A,
    ERROR_VHD_MISSING_CHANGE_TRACKING_INFORMATION = 0xC03A0030,
}

impl USERMODE_VHD {
    pub fn description(&self) -> &'static str {
        match self {
            USERMODE_VHD::ERROR_QUERY_STORAGE_ERROR => "The virtualization storage subsystem has generated an error.",
            USERMODE_VHD::ERROR_VHD_DRIVE_FOOTER_MISSING => "The virtual hard disk is corrupted. The virtual hard disk drive footer is missing.",
            USERMODE_VHD::ERROR_VHD_DRIVE_FOOTER_CHECKSUM_MISMATCH => "The virtual hard disk is corrupted. The virtual hard disk drive footer checksum does not match the on-disk checksum.",
            USERMODE_VHD::ERROR_VHD_DRIVE_FOOTER_CORRUPT => "The virtual hard disk is corrupted. The virtual hard disk drive footer in the virtual hard disk is corrupted.",
            USERMODE_VHD::ERROR_VHD_FORMAT_UNKNOWN => "The system does not recognize the file format of this virtual hard disk.",
            USERMODE_VHD::ERROR_VHD_FORMAT_UNSUPPORTED_VERSION => "The version does not support this version of the file format.",
            USERMODE_VHD::ERROR_VHD_SPARSE_HEADER_CHECKSUM_MISMATCH => "The virtual hard disk is corrupted. The sparse header checksum does not match the on-disk checksum.",
            USERMODE_VHD::ERROR_VHD_SPARSE_HEADER_UNSUPPORTED_VERSION => "The system does not support this version of the virtual hard disk.This version of the sparse header is not supported.",
            USERMODE_VHD::ERROR_VHD_SPARSE_HEADER_CORRUPT => "The virtual hard disk is corrupted. The sparse header in the virtual hard disk is corrupt.",
            USERMODE_VHD::ERROR_VHD_BLOCK_ALLOCATION_FAILURE => "Failed to write to the virtual hard disk failed because the system failed to allocate a new block in the virtual hard disk.",
            USERMODE_VHD::ERROR_VHD_BLOCK_ALLOCATION_TABLE_CORRUPT => "The virtual hard disk is corrupted. The block allocation table in the virtual hard disk is corrupt.",
            USERMODE_VHD::ERROR_VHD_INVALID_BLOCK_SIZE => "The system does not support this version of the virtual hard disk. The block size is invalid.",
            USERMODE_VHD::ERROR_VHD_BITMAP_MISMATCH => "The virtual hard disk is corrupted. The block bitmap does not match with the block data present in the virtual hard disk.",
            USERMODE_VHD::ERROR_VHD_PARENT_VHD_NOT_FOUND => "The chain of virtual hard disks is broken. The system cannot locate the parent virtual hard disk for the differencing disk.",
            USERMODE_VHD::ERROR_VHD_CHILD_PARENT_ID_MISMATCH => "The chain of virtual hard disks is corrupted. There is a mismatch in the identifiers of the parent virtual hard disk and differencing disk.",
            USERMODE_VHD::ERROR_VHD_CHILD_PARENT_TIMESTAMP_MISMATCH => "The chain of virtual hard disks is corrupted. The time stamp of the parent virtual hard disk does not match the time stamp of the differencing disk.",
            USERMODE_VHD::ERROR_VHD_METADATA_READ_FAILURE => "Failed to read the metadata of the virtual hard disk.",
            USERMODE_VHD::ERROR_VHD_METADATA_WRITE_FAILURE => "Failed to write to the metadata of the virtual hard disk.",
            USERMODE_VHD::ERROR_VHD_INVALID_SIZE => "The size of the virtual hard disk is not valid.",
            USERMODE_VHD::ERROR_VHD_INVALID_FILE_SIZE => "The file size of this virtual hard disk is not valid.",
            USERMODE_VHD::ERROR_VIRTDISK_PROVIDER_NOT_FOUND => "A virtual disk support provider for the specified file was not found.",
            USERMODE_VHD::ERROR_VIRTDISK_NOT_VIRTUAL_DISK => "The specified disk is not a virtual disk.",
            USERMODE_VHD::ERROR_VHD_PARENT_VHD_ACCESS_DENIED => "The chain of virtual hard disks is inaccessible. The process has not been granted access rights to the parent virtual hard disk for the differencing disk.",
            USERMODE_VHD::ERROR_VHD_CHILD_PARENT_SIZE_MISMATCH => "The chain of virtual hard disks is corrupted. There is a mismatch in the virtual sizes of the parent virtual hard disk and differencing disk.",
            USERMODE_VHD::ERROR_VHD_DIFFERENCING_CHAIN_CYCLE_DETECTED => "The chain of virtual hard disks is corrupted. A differencing disk is indicated in its own parent chain.",
            USERMODE_VHD::ERROR_VHD_DIFFERENCING_CHAIN_ERROR_IN_PARENT => "The chain of virtual hard disks is inaccessible. There was an error opening a virtual hard disk further up the chain.",
            USERMODE_VHD::ERROR_VIRTUAL_DISK_LIMITATION => "The requested operation could not be completed due to a virtual disk system limitation. Virtual hard disk files must be uncompressed and unencrypted and must not be sparse.",
            USERMODE_VHD::ERROR_VHD_INVALID_TYPE => "The requested operation cannot be performed on a virtual disk of this type.",
            USERMODE_VHD::ERROR_VHD_INVALID_STATE => "The requested operation cannot be performed on the virtual disk in its current state.",
            USERMODE_VHD::ERROR_VIRTDISK_UNSUPPORTED_DISK_SECTOR_SIZE => "The sector size of the physical disk on which the virtual disk resides is not supported.",
            USERMODE_VHD::ERROR_VIRTDISK_DISK_ALREADY_OWNED => "The disk is already owned by a different owner.",
            USERMODE_VHD::ERROR_VIRTDISK_DISK_ONLINE_AND_WRITABLE => "The disk must be offline or read-only.",
            USERMODE_VHD::ERROR_CTLOG_TRACKING_NOT_INITIALIZED => "Change Tracking is not initialized for this virtual disk.",
            USERMODE_VHD::ERROR_CTLOG_LOGFILE_SIZE_EXCEEDED_MAXSIZE => "Size of change tracking file exceeded the maximum size limit.",
            USERMODE_VHD::ERROR_CTLOG_VHD_CHANGED_OFFLINE => "VHD file is changed due to compaction, expansion, or offline updates.",
            USERMODE_VHD::ERROR_CTLOG_INVALID_TRACKING_STATE => "Change Tracking for the virtual disk is not in a valid state to perform this request. Change tracking could be discontinued or already in the requested state.",
            USERMODE_VHD::ERROR_CTLOG_INCONSISTENT_TRACKING_FILE => "Change Tracking file for the virtual disk is not in a valid state.",
            USERMODE_VHD::ERROR_VHD_RESIZE_WOULD_TRUNCATE_DATA => "The requested resize operation could not be completed because it might truncate user data residing on the virtual disk.",
            USERMODE_VHD::ERROR_VHD_COULD_NOT_COMPUTE_MINIMUM_VIRTUAL_SIZE => "The requested operation could not be completed because the virtual disk's minimum safe size could not be determined.",
            USERMODE_VHD::ERROR_VHD_ALREADY_AT_OR_BELOW_MINIMUM_VIRTUAL_SIZE => "The requested operation could not be completed because the virtual disk's size cannot be safely reduced further.",
            USERMODE_VHD::ERROR_VHD_METADATA_FULL => "There is not enough space in the virtual disk file for the provided metadata item.",
            USERMODE_VHD::ERROR_VHD_INVALID_CHANGE_TRACKING_ID => "The specified change tracking identifier is not valid.",
            USERMODE_VHD::ERROR_VHD_CHANGE_TRACKING_DISABLED => "Change tracking is disabled for the specified virtual hard disk, so no change tracking information is available.",
            USERMODE_VHD::ERROR_VHD_MISSING_CHANGE_TRACKING_INFORMATION => "There is no change tracking data available associated with the specified change tracking identifier.",
        }
    }
}
