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

pub enum IORING {
    IORING_E_REQUIRED_FLAG_NOT_SUPPORTED = 0x80460001,
    IORING_E_SUBMISSION_QUEUE_FULL = 0x80460002,
    IORING_E_VERSION_NOT_SUPPORTED = 0x80460003,
    IORING_E_SUBMISSION_QUEUE_TOO_BIG = 0x80460004,
    IORING_E_COMPLETION_QUEUE_TOO_BIG = 0x80460005,
    IORING_E_SUBMIT_IN_PROGRESS = 0x80460006,
    IORING_E_CORRUPT = 0x80460007,
    IORING_E_COMPLETION_QUEUE_TOO_FULL = 0x80460008,
}

impl IORING {
    pub fn description(&self) -> &'static str {
        match self {
            IORING::IORING_E_REQUIRED_FLAG_NOT_SUPPORTED => "One or more of the required flags provided is unknown by the implementation.",
            IORING::IORING_E_SUBMISSION_QUEUE_FULL => "The submission queue is full.",
            IORING::IORING_E_VERSION_NOT_SUPPORTED => "The version specified is not known or supported.",
            IORING::IORING_E_SUBMISSION_QUEUE_TOO_BIG => "The submission queue size specified for the IoRing is too big.",
            IORING::IORING_E_COMPLETION_QUEUE_TOO_BIG => "The completion queue size specified for the IoRing is too big.",
            IORING::IORING_E_SUBMIT_IN_PROGRESS => "A submit operation is already in progress for this IoRing on another thread.",
            IORING::IORING_E_CORRUPT => "The shared ring buffers of the IoRing are corrupt.",
            IORING::IORING_E_COMPLETION_QUEUE_TOO_FULL => "The completion queue does not have enough free space, to post completions, for all entries being submitted.",
        }
    }
}
