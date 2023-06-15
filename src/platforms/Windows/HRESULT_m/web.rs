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

pub enum WEB {
    WEB_E_UNSUPPORTED_FORMAT = 0x83750001,
    WEB_E_INVALID_XML = 0x83750002,
    WEB_E_MISSING_REQUIRED_ELEMENT = 0x83750003,
    WEB_E_MISSING_REQUIRED_ATTRIBUTE = 0x83750004,
    WEB_E_UNEXPECTED_CONTENT = 0x83750005,
    WEB_E_RESOURCE_TOO_LARGE = 0x83750006,
    WEB_E_INVALID_JSON_STRING = 0x83750007,
    WEB_E_INVALID_JSON_NUMBER = 0x83750008,
    WEB_E_JSON_VALUE_NOT_FOUND = 0x83750009,
}

impl WEB {
    pub fn description(&self) -> &'static str {
        match self {
            WEB::WEB_E_UNSUPPORTED_FORMAT => "Unsupported format.",
            WEB::WEB_E_INVALID_XML => "Invalid XML.",
            WEB::WEB_E_MISSING_REQUIRED_ELEMENT => "Missing required element.",
            WEB::WEB_E_MISSING_REQUIRED_ATTRIBUTE => "Missing required attribute.",
            WEB::WEB_E_UNEXPECTED_CONTENT => "Unexpected content.",
            WEB::WEB_E_RESOURCE_TOO_LARGE => "Resource too large.",
            WEB::WEB_E_INVALID_JSON_STRING => "Invalid JSON string.",
            WEB::WEB_E_INVALID_JSON_NUMBER => "Invalid JSON number.",
            WEB::WEB_E_JSON_VALUE_NOT_FOUND => "JSON value not found.",
        }
    }
}
