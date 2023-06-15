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

pub enum TIERING {
    ERROR_TIERING_NOT_SUPPORTED_ON_VOLUME = 0x80830001,
    ERROR_TIERING_VOLUME_DISMOUNT_IN_PROGRESS = 0x80830002,
    ERROR_TIERING_STORAGE_TIER_NOT_FOUND = 0x80830003,
    ERROR_TIERING_INVALID_FILE_ID = 0x80830004,
    ERROR_TIERING_WRONG_CLUSTER_NODE = 0x80830005,
    ERROR_TIERING_ALREADY_PROCESSING = 0x80830006,
    ERROR_TIERING_CANNOT_PIN_OBJECT = 0x80830007,
    ERROR_TIERING_FILE_IS_NOT_PINNED = 0x80830008,
    ERROR_NOT_A_TIERED_VOLUME = 0x80830009,
    ERROR_ATTRIBUTE_NOT_PRESENT = 0x8083000A,
}

impl TIERING {
    pub fn description(&self) -> &'static str {
        match self {
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
}
