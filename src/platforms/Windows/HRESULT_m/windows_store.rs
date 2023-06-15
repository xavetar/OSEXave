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

pub enum WINDOWS_STORE {
    WINDOWS_STORE_ERROR_UNDOCUMENTED_7001 = 0x803F7001,
    WINDOWS_STORE_ERROR_UNDOCUMENTED_7002 = 0x803F7002,
    WINDOWS_STORE_ERROR_UNDOCUMENTED_7003 = 0x803F7003,
    WINDOWS_STORE_ERROR_UNDOCUMENTED_7004 = 0x803F7004,
    WINDOWS_STORE_ERROR_UNDOCUMENTED_7005 = 0x803F7005,
    WINDOWS_STORE_ERROR_UNDOCUMENTED_7006 = 0x803F7006,
    WINDOWS_STORE_ERROR_UNDOCUMENTED_7007 = 0x803F7007,
    WINDOWS_STORE_ERROR_UNDOCUMENTED_7008 = 0x803F7008,
    WINDOWS_STORE_ERROR_UNDOCUMENTED_7009 = 0x803F7009,
    StoreAgentAcquireLicenseFailure1 = 0x803F8001,
    WINDOWS_STORE_ERROR_UNDOCUMENTED_8002 = 0x803F8002,
    WINDOWS_STORE_ERROR_UNDOCUMENTED_8003 = 0x803F8003,
    WINDOWS_STORE_ERROR_UNDOCUMENTED_8004 = 0x803F8004,
    WINDOWS_STORE_ERROR_UNDOCUMENTED_812C = 0x803F812C,
    WINDOWS_STORE_ERROR_UNDOCUMENTED_8131 = 0x803F8131,
    WINDOWS_STORE_ERROR_UNDOCUMENTED_8132 = 0x803F8132,
    WINDOWS_STORE_ERROR_UNDOCUMENTED_81FB = 0x803F81FB,
    WINDOWS_STORE_ERROR_UNDOCUMENTED_81FC = 0x803F81FC,
    WINDOWS_STORE_ERROR_UNDOCUMENTED_81FD = 0x803F81FD,
    WINDOWS_STORE_ERROR_UNDOCUMENTED_9008 = 0x803F9008,
    WINDOWS_STORE_ERROR_UNDOCUMENTED_900B = 0x803F900B,
    WINDOWS_STORE_ERROR_UNDOCUMENTED_900D = 0x803F900D,
}

impl WINDOWS_STORE {
    pub fn description(&self) -> &'static str {
        match self {
            WINDOWS_STORE::WINDOWS_STORE_ERROR_UNDOCUMENTED_7001 => "Activation Failed Because This Device Doesn’t Have A Valid Digital Entitlement Or Product Key",
            WINDOWS_STORE::WINDOWS_STORE_ERROR_UNDOCUMENTED_7002 => "Description is missing",
            WINDOWS_STORE::WINDOWS_STORE_ERROR_UNDOCUMENTED_7003 => "Intermittent network connection error.",
            WINDOWS_STORE::WINDOWS_STORE_ERROR_UNDOCUMENTED_7004 => "Description is missing",
            WINDOWS_STORE::WINDOWS_STORE_ERROR_UNDOCUMENTED_7005 => "Description is missing",
            WINDOWS_STORE::WINDOWS_STORE_ERROR_UNDOCUMENTED_7006 => "Description is missing",
            WINDOWS_STORE::WINDOWS_STORE_ERROR_UNDOCUMENTED_7007 => "Description is missing",
            WINDOWS_STORE::WINDOWS_STORE_ERROR_UNDOCUMENTED_7008 => "Description is missing",
            WINDOWS_STORE::WINDOWS_STORE_ERROR_UNDOCUMENTED_7009 => "Description is missing",
            WINDOWS_STORE::StoreAgentAcquireLicenseFailure1 => "The Store Agent can't acquire license for this app",
            WINDOWS_STORE::WINDOWS_STORE_ERROR_UNDOCUMENTED_8002 => "Description is missing",
            WINDOWS_STORE::WINDOWS_STORE_ERROR_UNDOCUMENTED_8003 => "Description is missing",
            WINDOWS_STORE::WINDOWS_STORE_ERROR_UNDOCUMENTED_8004 => "Description is missing",
            WINDOWS_STORE::WINDOWS_STORE_ERROR_UNDOCUMENTED_812C => "Description is missing",
            WINDOWS_STORE::WINDOWS_STORE_ERROR_UNDOCUMENTED_8131 => "Description is missing",
            WINDOWS_STORE::WINDOWS_STORE_ERROR_UNDOCUMENTED_8132 => "Description is missing",
            WINDOWS_STORE::WINDOWS_STORE_ERROR_UNDOCUMENTED_81FB => "Description is missing",
            WINDOWS_STORE::WINDOWS_STORE_ERROR_UNDOCUMENTED_81FC => "Description is missing",
            WINDOWS_STORE::WINDOWS_STORE_ERROR_UNDOCUMENTED_81FD => "Description is missing",
            WINDOWS_STORE::WINDOWS_STORE_ERROR_UNDOCUMENTED_9008 => "Data has changed.",
            WINDOWS_STORE::WINDOWS_STORE_ERROR_UNDOCUMENTED_900B => "Description is missing",
            WINDOWS_STORE::WINDOWS_STORE_ERROR_UNDOCUMENTED_900D => "Description is missing",
        }
    }
}
