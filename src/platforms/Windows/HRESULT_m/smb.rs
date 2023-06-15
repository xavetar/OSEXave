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

pub enum SMB {
    ERROR_SMB_NO_PREAUTH_INTEGRITY_HASH_OVERLAP = 0xC05D0000,
    ERROR_SMB_BAD_CLUSTER_DIALECT = 0xC05D0001,
    ERROR_SMB_NO_SIGNING_ALGORITHM_OVERLAP = 0xC05D0002,
}

impl SMB {
    pub fn description(&self) -> &'static str {
        match self {
            SMB::ERROR_SMB_NO_PREAUTH_INTEGRITY_HASH_OVERLAP => "Failed to negotiate a preauthentication integrity hash function.",
            SMB::ERROR_SMB_BAD_CLUSTER_DIALECT => "The current cluster functional level does not support this SMB dialect.",
            SMB::ERROR_SMB_NO_SIGNING_ALGORITHM_OVERLAP => "Failed to negotiate a signing hash function.",
        }
    }
}
