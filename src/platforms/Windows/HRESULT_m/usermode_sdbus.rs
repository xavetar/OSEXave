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

pub enum USERMODE_SDBUS {
    ERROR_IO_PREEMPTED = 0x89010001,
}

impl USERMODE_SDBUS {
    pub fn description(&self) -> &'static str {
        match self {
            USERMODE_SDBUS::ERROR_IO_PREEMPTED => "The operation was preempted by a higher priority operation. It must be resumed later.",
        }
    }
}
