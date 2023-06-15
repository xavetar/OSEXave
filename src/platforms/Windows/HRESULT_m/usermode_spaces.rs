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

pub enum USERMODE_SPACES {
    ERROR_SPACES_POOL_WAS_DELETED = 0x00E70001,
    ERROR_SPACES_FAULT_DOMAIN_TYPE_INVALID = 0x80E70001,
    ERROR_SPACES_INTERNAL_ERROR = 0x80E70002,
    ERROR_SPACES_RESILIENCY_TYPE_INVALID = 0x80E70003,
    ERROR_SPACES_DRIVE_SECTOR_SIZE_INVALID = 0x80E70004,
    ERROR_SPACES_DRIVE_REDUNDANCY_INVALID = 0x80E70006,
    ERROR_SPACES_NUMBER_OF_DATA_COPIES_INVALID = 0x80E70007,
    ERROR_SPACES_PARITY_LAYOUT_INVALID = 0x80E70008,
    ERROR_SPACES_INTERLEAVE_LENGTH_INVALID = 0x80E70009,
    ERROR_SPACES_NUMBER_OF_COLUMNS_INVALID = 0x80E7000A,
    ERROR_SPACES_NOT_ENOUGH_DRIVES = 0x80E7000B,
    ERROR_SPACES_EXTENDED_ERROR = 0x80E7000C,
    ERROR_SPACES_PROVISIONING_TYPE_INVALID = 0x80E7000D,
    ERROR_SPACES_ALLOCATION_SIZE_INVALID = 0x80E7000E,
    ERROR_SPACES_ENCLOSURE_AWARE_INVALID = 0x80E7000F,
    ERROR_SPACES_WRITE_CACHE_SIZE_INVALID = 0x80E70010,
    ERROR_SPACES_NUMBER_OF_GROUPS_INVALID = 0x80E70011,
    ERROR_SPACES_DRIVE_OPERATIONAL_STATE_INVALID = 0x80E70012,
    ERROR_SPACES_ENTRY_INCOMPLETE = 0x80E70013,
    ERROR_SPACES_ENTRY_INVALID = 0x80E70014,
    ERROR_SPACES_UPDATE_COLUMN_STATE = 0x80E70015,
    ERROR_SPACES_MAP_REQUIRED = 0x80E70016,
    ERROR_SPACES_UNSUPPORTED_VERSION = 0x80E70017,
    ERROR_SPACES_CORRUPT_METADATA = 0x80E70018,
    ERROR_SPACES_DRT_FULL = 0x80E70019,
    ERROR_SPACES_INCONSISTENCY = 0x80E7001A,
    ERROR_SPACES_LOG_NOT_READY = 0x80E7001B,
    ERROR_SPACES_NO_REDUNDANCY = 0x80E7001C,
    ERROR_SPACES_DRIVE_NOT_READY = 0x80E7001D,
    ERROR_SPACES_DRIVE_SPLIT = 0x80E7001E,
    ERROR_SPACES_DRIVE_LOST_DATA = 0x80E7001F,
    ERROR_SPACES_MARK_DIRTY = 0x80E70020,
    ERROR_SPACES_FLUSH_METADATA = 0x80E70025,
    ERROR_SPACES_CACHE_FULL = 0x80E70026,
    ERROR_SPACES_REPAIR_IN_PROGRESS = 0x80E70027,
}

impl USERMODE_SPACES {
    pub fn description(&self) -> &'static str {
        match self {
            USERMODE_SPACES::ERROR_SPACES_POOL_WAS_DELETED => "The storage pool was deleted by the driver. The object cache should be updated.",
            USERMODE_SPACES::ERROR_SPACES_FAULT_DOMAIN_TYPE_INVALID => "The specified fault domain type or combination of minimum / maximum fault domain type is not valid.",
            USERMODE_SPACES::ERROR_SPACES_INTERNAL_ERROR => "A Storage Spaces internal error occurred.",
            USERMODE_SPACES::ERROR_SPACES_RESILIENCY_TYPE_INVALID => "The specified resiliency type is not valid.",
            USERMODE_SPACES::ERROR_SPACES_DRIVE_SECTOR_SIZE_INVALID => "The physical disk's sector size is not supported by the storage pool.",
            USERMODE_SPACES::ERROR_SPACES_DRIVE_REDUNDANCY_INVALID => "The requested redundancy is outside of the supported range of values.",
            USERMODE_SPACES::ERROR_SPACES_NUMBER_OF_DATA_COPIES_INVALID => "The number of data copies requested is outside of the supported range of values.",
            USERMODE_SPACES::ERROR_SPACES_PARITY_LAYOUT_INVALID => "The value for ParityLayout is outside of the supported range of values.",
            USERMODE_SPACES::ERROR_SPACES_INTERLEAVE_LENGTH_INVALID => "The value for interleave length is outside of the supported range of values or is not a power of 2.",
            USERMODE_SPACES::ERROR_SPACES_NUMBER_OF_COLUMNS_INVALID => "The number of columns specified is outside of the supported range of values.",
            USERMODE_SPACES::ERROR_SPACES_NOT_ENOUGH_DRIVES => "There were not enough physical disks to complete the requested operation.",
            USERMODE_SPACES::ERROR_SPACES_EXTENDED_ERROR => "Extended error information is available.",
            USERMODE_SPACES::ERROR_SPACES_PROVISIONING_TYPE_INVALID => "The specified provisioning type is not valid.",
            USERMODE_SPACES::ERROR_SPACES_ALLOCATION_SIZE_INVALID => "The allocation size is outside of the supported range of values.",
            USERMODE_SPACES::ERROR_SPACES_ENCLOSURE_AWARE_INVALID => "Enclosure awareness is not supported for this virtual disk.",
            USERMODE_SPACES::ERROR_SPACES_WRITE_CACHE_SIZE_INVALID => "The write cache size is outside of the supported range of values.",
            USERMODE_SPACES::ERROR_SPACES_NUMBER_OF_GROUPS_INVALID => "The value for number of groups is outside of the supported range of values.",
            USERMODE_SPACES::ERROR_SPACES_DRIVE_OPERATIONAL_STATE_INVALID => "The OperationalState of the physical disk is invalid for this operation.",
            USERMODE_SPACES::ERROR_SPACES_ENTRY_INCOMPLETE => "The specified log entry is not complete.",
            USERMODE_SPACES::ERROR_SPACES_ENTRY_INVALID => "The specified log entry is not valid.",
            USERMODE_SPACES::ERROR_SPACES_UPDATE_COLUMN_STATE => "A column's state needs to be updated.",
            USERMODE_SPACES::ERROR_SPACES_MAP_REQUIRED => "An extent needs to be allocated.",
            USERMODE_SPACES::ERROR_SPACES_UNSUPPORTED_VERSION => "The metadata version is unsupported.",
            USERMODE_SPACES::ERROR_SPACES_CORRUPT_METADATA => "The metadata read was corrupt.",
            USERMODE_SPACES::ERROR_SPACES_DRT_FULL => "The DRT is full.",
            USERMODE_SPACES::ERROR_SPACES_INCONSISTENCY => "An inconsistency was found.",
            USERMODE_SPACES::ERROR_SPACES_LOG_NOT_READY => "The log is not ready.",
            USERMODE_SPACES::ERROR_SPACES_NO_REDUNDANCY => "No good copy of data was available.",
            USERMODE_SPACES::ERROR_SPACES_DRIVE_NOT_READY => "The drive is not ready.",
            USERMODE_SPACES::ERROR_SPACES_DRIVE_SPLIT => "The data on this drive is stale.",
            USERMODE_SPACES::ERROR_SPACES_DRIVE_LOST_DATA => "The data on this drive has been lost.",
            USERMODE_SPACES::ERROR_SPACES_MARK_DIRTY => "A slab needs to be marked dirty.",
            USERMODE_SPACES::ERROR_SPACES_FLUSH_METADATA => "The cache metadata needs to be written and flushed.",
            USERMODE_SPACES::ERROR_SPACES_CACHE_FULL => "The cache is full.",
            USERMODE_SPACES::ERROR_SPACES_REPAIR_IN_PROGRESS => "Repair is in progress.",
        }
    }
}
