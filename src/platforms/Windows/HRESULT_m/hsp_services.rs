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

pub enum HSP_SERVICES {
    HSP_E_ERROR_MASK = 0x81280000,
    HSP_E_INTERNAL_ERROR = 0x81280FFF,
    HSP_BS_ERROR_MASK = 0x81281000,
    HSP_BS_INTERNAL_ERROR = 0x812810FF,
}

impl HSP_SERVICES {
    pub fn description(&self) -> &'static str {
        match self {
            HSP_SERVICES::HSP_E_ERROR_MASK => "This is an error mask to convert HSP hardware errors to Win errors.",
            HSP_SERVICES::HSP_E_INTERNAL_ERROR => "Catastrophic internal failure in the HSP hardware.",
            HSP_SERVICES::HSP_BS_ERROR_MASK => "This is an error mask to convert HSP base services errors to Win errors.",
            HSP_SERVICES::HSP_BS_INTERNAL_ERROR => "Catastrophic internal failure in the HSP base services.",
        }
    }
}
