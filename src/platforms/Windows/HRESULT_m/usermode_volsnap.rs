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

pub enum USERMODE_VOLSNAP {
    ERROR_VOLSNAP_BOOTFILE_NOT_VALID = 0x80820001,
    ERROR_VOLSNAP_ACTIVATION_TIMEOUT = 0x80820002,
    ERROR_VOLSNAP_NO_BYPASSIO_WITH_SNAPSHOT = 0x80820003,
}

impl USERMODE_VOLSNAP {
    pub fn description(&self) -> &'static str {
        match self {
            USERMODE_VOLSNAP::ERROR_VOLSNAP_BOOTFILE_NOT_VALID => "The bootfile is too small to support persistent snapshots.",
            USERMODE_VOLSNAP::ERROR_VOLSNAP_ACTIVATION_TIMEOUT => "Activation of persistent snapshots on this volume took longer than was allowed.",
            USERMODE_VOLSNAP::ERROR_VOLSNAP_NO_BYPASSIO_WITH_SNAPSHOT => "BypassIO cannot be enabled while a volume snapshot exists.",
        }
    }
}
