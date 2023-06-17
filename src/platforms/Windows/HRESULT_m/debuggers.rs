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
pub enum DEBUGGERS {
    ERROR_DBG_CREATE_PROCESS_FAILURE_LOCKDOWN,
    ERROR_DBG_ATTACH_PROCESS_FAILURE_LOCKDOWN,
    ERROR_DBG_CONNECT_SERVER_FAILURE_LOCKDOWN,
    ERROR_DBG_START_SERVER_FAILURE_LOCKDOWN,
}

impl DEBUGGERS {
    pub fn code(&self) -> u32 {
        return match self {
            DEBUGGERS::ERROR_DBG_CREATE_PROCESS_FAILURE_LOCKDOWN => 0x80B00001 as u32,
            DEBUGGERS::ERROR_DBG_ATTACH_PROCESS_FAILURE_LOCKDOWN => 0x80B00002 as u32,
            DEBUGGERS::ERROR_DBG_CONNECT_SERVER_FAILURE_LOCKDOWN => 0x80B00003 as u32,
            DEBUGGERS::ERROR_DBG_START_SERVER_FAILURE_LOCKDOWN => 0x80B00004 as u32,
        }
    }

    pub fn error(&self) -> RawError {
        return match self {
            DEBUGGERS::ERROR_DBG_CREATE_PROCESS_FAILURE_LOCKDOWN => RawError::Kind(DEBUGGERS::ERROR_DBG_CREATE_PROCESS_FAILURE_LOCKDOWN),
            DEBUGGERS::ERROR_DBG_ATTACH_PROCESS_FAILURE_LOCKDOWN => RawError::Kind(DEBUGGERS::ERROR_DBG_ATTACH_PROCESS_FAILURE_LOCKDOWN),
            DEBUGGERS::ERROR_DBG_CONNECT_SERVER_FAILURE_LOCKDOWN => RawError::Kind(DEBUGGERS::ERROR_DBG_CONNECT_SERVER_FAILURE_LOCKDOWN),
            DEBUGGERS::ERROR_DBG_START_SERVER_FAILURE_LOCKDOWN => RawError::Kind(DEBUGGERS::ERROR_DBG_START_SERVER_FAILURE_LOCKDOWN),
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            DEBUGGERS::ERROR_DBG_CREATE_PROCESS_FAILURE_LOCKDOWN => "Could not create new process from ARM architecture device.",
            DEBUGGERS::ERROR_DBG_ATTACH_PROCESS_FAILURE_LOCKDOWN => "Could not attach to the application process from ARM architecture device.",
            DEBUGGERS::ERROR_DBG_CONNECT_SERVER_FAILURE_LOCKDOWN => "Could not connect to dbgsrv server from ARM architecture device.",
            DEBUGGERS::ERROR_DBG_START_SERVER_FAILURE_LOCKDOWN => "Could not start dbgsrv server from ARM architecture device.",
        }
    }

    pub fn from_name(name: &str) -> DEBUGGERS {
        return match name {
            "ERROR_DBG_CREATE_PROCESS_FAILURE_LOCKDOWN" => DEBUGGERS::ERROR_DBG_CREATE_PROCESS_FAILURE_LOCKDOWN,
            "ERROR_DBG_ATTACH_PROCESS_FAILURE_LOCKDOWN" => DEBUGGERS::ERROR_DBG_ATTACH_PROCESS_FAILURE_LOCKDOWN,
            "ERROR_DBG_CONNECT_SERVER_FAILURE_LOCKDOWN" => DEBUGGERS::ERROR_DBG_CONNECT_SERVER_FAILURE_LOCKDOWN,
            "ERROR_DBG_START_SERVER_FAILURE_LOCKDOWN" => DEBUGGERS::ERROR_DBG_START_SERVER_FAILURE_LOCKDOWN,
        }
    }
    pub fn from_code(code: u32) -> DEBUGGERS {
        return match code {
            0x80B00001 => DEBUGGERS::ERROR_DBG_CREATE_PROCESS_FAILURE_LOCKDOWN,
            0x80B00002 => DEBUGGERS::ERROR_DBG_ATTACH_PROCESS_FAILURE_LOCKDOWN,
            0x80B00003 => DEBUGGERS::ERROR_DBG_CONNECT_SERVER_FAILURE_LOCKDOWN,
            0x80B00004 => DEBUGGERS::ERROR_DBG_START_SERVER_FAILURE_LOCKDOWN,
        }
    }
}
