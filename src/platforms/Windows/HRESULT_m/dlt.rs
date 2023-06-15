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

pub enum DLT {
    TRK_S_OUT_OF_SYNC = 0x0DEAD100,
    TRK_VOLUME_NOT_FOUND = 0x0DEAD102,
    TRK_VOLUME_NOT_OWNED = 0x0DEAD103,
    TRK_S_NOTIFICATION_QUOTA_EXCEEDED = 0x0DEAD107,
    TRK_E_NOT_FOUND = 0x8DEAD01B,
    TRK_E_VOLUME_QUOTA_EXCEEDED = 0x8DEAD01C,
    TRK_SERVER_TOO_BUSY = 0x8DEAD01E,
}

impl DLT {
    pub fn description(&self) -> &'static str {
        match self {
            DLT::TRK_S_OUT_OF_SYNC => "The VolumeSequenceNumber of a MOVE_NOTIFICATION request is incorrect.",
            DLT::TRK_VOLUME_NOT_FOUND => "The VolumeID in a request was not found in the server's ServerVolumeTable.",
            DLT::TRK_VOLUME_NOT_OWNED => "A notification was sent to the LnkSvrMessage method, but the RequestMachine for the request was not the VolumeOwner for a VolumeID in the request.",
            DLT::TRK_S_NOTIFICATION_QUOTA_EXCEEDED => "The server received a MOVE_NOTIFICATION request, but the FileTable size limit has already been reached.",
            DLT::TRK_E_NOT_FOUND => "A requested object was not found.",
            DLT::TRK_E_VOLUME_QUOTA_EXCEEDED => "The server received a CREATE_VOLUME subrequest of a SYNC_VOLUMES request, but the ServerVolumeTable size limit for the RequestMachine has already been reached.",
            DLT::TRK_SERVER_TOO_BUSY => "The server is busy, and the client should retry the request at a later time.",
        }
    }
}
