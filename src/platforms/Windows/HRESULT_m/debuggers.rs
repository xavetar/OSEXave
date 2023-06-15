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

pub enum DEBUGGERS {
    ERROR_DBG_CREATE_PROCESS_FAILURE_LOCKDOWN = 0x80B00001,
    ERROR_DBG_ATTACH_PROCESS_FAILURE_LOCKDOWN = 0x80B00002,
    ERROR_DBG_CONNECT_SERVER_FAILURE_LOCKDOWN = 0x80B00003,
    ERROR_DBG_START_SERVER_FAILURE_LOCKDOWN = 0x80B00004,
}

impl DEBUGGERS {
    pub fn description(&self) -> &'static str {
        match self {
            DEBUGGERS::ERROR_DBG_CREATE_PROCESS_FAILURE_LOCKDOWN => "Could not create new process from ARM architecture device.",
            DEBUGGERS::ERROR_DBG_ATTACH_PROCESS_FAILURE_LOCKDOWN => "Could not attach to the application process from ARM architecture device.",
            DEBUGGERS::ERROR_DBG_CONNECT_SERVER_FAILURE_LOCKDOWN => "Could not connect to dbgsrv server from ARM architecture device.",
            DEBUGGERS::ERROR_DBG_START_SERVER_FAILURE_LOCKDOWN => "Could not start dbgsrv server from ARM architecture device.",
        }
    }
}
