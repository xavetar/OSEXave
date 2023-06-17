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
pub enum TIERING {
    ERROR_TIERING_NOT_SUPPORTED_ON_VOLUME,
    ERROR_TIERING_VOLUME_DISMOUNT_IN_PROGRESS,
    ERROR_TIERING_STORAGE_TIER_NOT_FOUND,
    ERROR_TIERING_INVALID_FILE_ID,
    ERROR_TIERING_WRONG_CLUSTER_NODE,
    ERROR_TIERING_ALREADY_PROCESSING,
    ERROR_TIERING_CANNOT_PIN_OBJECT,
    ERROR_TIERING_FILE_IS_NOT_PINNED,
    ERROR_NOT_A_TIERED_VOLUME,
    ERROR_ATTRIBUTE_NOT_PRESENT,
}

impl TIERING {
    pub fn code(&self) -> u32 {
        return match self {
            TIERING::ERROR_TIERING_NOT_SUPPORTED_ON_VOLUME => 0x80830001 as u32,
            TIERING::ERROR_TIERING_VOLUME_DISMOUNT_IN_PROGRESS => 0x80830002 as u32,
            TIERING::ERROR_TIERING_STORAGE_TIER_NOT_FOUND => 0x80830003 as u32,
            TIERING::ERROR_TIERING_INVALID_FILE_ID => 0x80830004 as u32,
            TIERING::ERROR_TIERING_WRONG_CLUSTER_NODE => 0x80830005 as u32,
            TIERING::ERROR_TIERING_ALREADY_PROCESSING => 0x80830006 as u32,
            TIERING::ERROR_TIERING_CANNOT_PIN_OBJECT => 0x80830007 as u32,
            TIERING::ERROR_TIERING_FILE_IS_NOT_PINNED => 0x80830008 as u32,
            TIERING::ERROR_NOT_A_TIERED_VOLUME => 0x80830009 as u32,
            TIERING::ERROR_ATTRIBUTE_NOT_PRESENT => 0x8083000A as u32,
        }
    }

    pub fn error(&self) -> RawError {
        return match self {
            TIERING::ERROR_TIERING_NOT_SUPPORTED_ON_VOLUME => RawError::Kind(TIERING::ERROR_TIERING_NOT_SUPPORTED_ON_VOLUME),
            TIERING::ERROR_TIERING_VOLUME_DISMOUNT_IN_PROGRESS => RawError::Kind(TIERING::ERROR_TIERING_VOLUME_DISMOUNT_IN_PROGRESS),
            TIERING::ERROR_TIERING_STORAGE_TIER_NOT_FOUND => RawError::Kind(TIERING::ERROR_TIERING_STORAGE_TIER_NOT_FOUND),
            TIERING::ERROR_TIERING_INVALID_FILE_ID => RawError::Kind(TIERING::ERROR_TIERING_INVALID_FILE_ID),
            TIERING::ERROR_TIERING_WRONG_CLUSTER_NODE => RawError::Kind(TIERING::ERROR_TIERING_WRONG_CLUSTER_NODE),
            TIERING::ERROR_TIERING_ALREADY_PROCESSING => RawError::Kind(TIERING::ERROR_TIERING_ALREADY_PROCESSING),
            TIERING::ERROR_TIERING_CANNOT_PIN_OBJECT => RawError::Kind(TIERING::ERROR_TIERING_CANNOT_PIN_OBJECT),
            TIERING::ERROR_TIERING_FILE_IS_NOT_PINNED => RawError::Kind(TIERING::ERROR_TIERING_FILE_IS_NOT_PINNED),
            TIERING::ERROR_NOT_A_TIERED_VOLUME => RawError::Kind(TIERING::ERROR_NOT_A_TIERED_VOLUME),
            TIERING::ERROR_ATTRIBUTE_NOT_PRESENT => RawError::Kind(TIERING::ERROR_ATTRIBUTE_NOT_PRESENT),
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            TIERING::ERROR_TIERING_NOT_SUPPORTED_ON_VOLUME => "The specified volume does not support storage tiers.",
            TIERING::ERROR_TIERING_VOLUME_DISMOUNT_IN_PROGRESS => "The Storage Tiers Management service detected that the specified volume is in the process of being dismounted.",
            TIERING::ERROR_TIERING_STORAGE_TIER_NOT_FOUND => "The specified storage tier could not be found on the volume. Confirm that the storage tier name is valid.",
            TIERING::ERROR_TIERING_INVALID_FILE_ID => "The file identifier specified is not valid on the volume.",
            TIERING::ERROR_TIERING_WRONG_CLUSTER_NODE => "Storage tier operations must be called on the clustering node that owns the metadata volume.",
            TIERING::ERROR_TIERING_ALREADY_PROCESSING => "The Storage Tiers Management service is already optimizing the storage tiers on the specified volume.",
            TIERING::ERROR_TIERING_CANNOT_PIN_OBJECT => "The requested object type cannot be assigned to a storage tier.",
            TIERING::ERROR_TIERING_FILE_IS_NOT_PINNED => "The requested file is not pinned to a tier.",
            TIERING::ERROR_NOT_A_TIERED_VOLUME => "The volume is not a tiered volume.",
            TIERING::ERROR_ATTRIBUTE_NOT_PRESENT => "The requested attribute is not present on the specified file or directory.",
        }
    }

    pub fn from_name(name: &str) -> TIERING {
        return match name {
            "ERROR_TIERING_NOT_SUPPORTED_ON_VOLUME" => TIERING::ERROR_TIERING_NOT_SUPPORTED_ON_VOLUME,
            "ERROR_TIERING_VOLUME_DISMOUNT_IN_PROGRESS" => TIERING::ERROR_TIERING_VOLUME_DISMOUNT_IN_PROGRESS,
            "ERROR_TIERING_STORAGE_TIER_NOT_FOUND" => TIERING::ERROR_TIERING_STORAGE_TIER_NOT_FOUND,
            "ERROR_TIERING_INVALID_FILE_ID" => TIERING::ERROR_TIERING_INVALID_FILE_ID,
            "ERROR_TIERING_WRONG_CLUSTER_NODE" => TIERING::ERROR_TIERING_WRONG_CLUSTER_NODE,
            "ERROR_TIERING_ALREADY_PROCESSING" => TIERING::ERROR_TIERING_ALREADY_PROCESSING,
            "ERROR_TIERING_CANNOT_PIN_OBJECT" => TIERING::ERROR_TIERING_CANNOT_PIN_OBJECT,
            "ERROR_TIERING_FILE_IS_NOT_PINNED" => TIERING::ERROR_TIERING_FILE_IS_NOT_PINNED,
            "ERROR_NOT_A_TIERED_VOLUME" => TIERING::ERROR_NOT_A_TIERED_VOLUME,
            "ERROR_ATTRIBUTE_NOT_PRESENT" => TIERING::ERROR_ATTRIBUTE_NOT_PRESENT,
        }
    }
    pub fn from_code(code: u32) -> TIERING {
        return match code {
            0x80830001 => TIERING::ERROR_TIERING_NOT_SUPPORTED_ON_VOLUME,
            0x80830002 => TIERING::ERROR_TIERING_VOLUME_DISMOUNT_IN_PROGRESS,
            0x80830003 => TIERING::ERROR_TIERING_STORAGE_TIER_NOT_FOUND,
            0x80830004 => TIERING::ERROR_TIERING_INVALID_FILE_ID,
            0x80830005 => TIERING::ERROR_TIERING_WRONG_CLUSTER_NODE,
            0x80830006 => TIERING::ERROR_TIERING_ALREADY_PROCESSING,
            0x80830007 => TIERING::ERROR_TIERING_CANNOT_PIN_OBJECT,
            0x80830008 => TIERING::ERROR_TIERING_FILE_IS_NOT_PINNED,
            0x80830009 => TIERING::ERROR_NOT_A_TIERED_VOLUME,
            0x8083000A => TIERING::ERROR_ATTRIBUTE_NOT_PRESENT,
        }
    }
}
