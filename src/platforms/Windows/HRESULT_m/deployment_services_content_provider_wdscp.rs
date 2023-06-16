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

pub enum DEPLOYMENT_SERVICES_CONTENT_PROVIDER_WDSCP {
    WDSCP_E_INVALID_CONFIGURATION = 0xC1250100,
    WDSCP_E_NOT_A_DIRECTORY = 0xC1250101,
    WDSCP_E_CONFIG_STRING_REQUIRED = 0xC1250102,
}

impl DEPLOYMENT_SERVICES_CONTENT_PROVIDER_WDSCP {
    pub fn description(&self) -> &'static str {
        match self {
            DEPLOYMENT_SERVICES_CONTENT_PROVIDER_WDSCP::WDSCP_E_INVALID_CONFIGURATION => "The configuration string was invalid or empty.",
            DEPLOYMENT_SERVICES_CONTENT_PROVIDER_WDSCP::WDSCP_E_NOT_A_DIRECTORY => "The path specified in the configuration string was not a directory.",
            DEPLOYMENT_SERVICES_CONTENT_PROVIDER_WDSCP::WDSCP_E_CONFIG_STRING_REQUIRED => "The WDS Content Provider requires that a configuration string be provided.",
        }
    }
}
