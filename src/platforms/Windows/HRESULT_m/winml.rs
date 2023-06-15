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

pub enum WINML {
    WINML_ERR_INVALID_DEVICE = 0x88900001,
    WINML_ERR_INVALID_BINDING = 0x88900002,
    WINML_ERR_VALUE_NOTFOUND = 0x88900003,
    WINML_ERR_SIZE_MISMATCH = 0x88900004,
}

impl WINML {
    pub fn description(&self) -> &'static str {
        match self {
            WINML::WINML_ERR_INVALID_DEVICE => "The device is invalid or does not support machine learning.",
            WINML::WINML_ERR_INVALID_BINDING => "The binding is incomplete or does not match the input/output description.",
            WINML::WINML_ERR_VALUE_NOTFOUND => "An attempt was made to bind an unknown input or output.",
            WINML::WINML_ERR_SIZE_MISMATCH => "The size of the buffer provided for a bound variable is invalid.",
        }
    }
}
