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

pub enum DIRECT3D10 {
    D3D10_ERROR_TOO_MANY_UNIQUE_STATE_OBJECTS = 0x88790001,
    D3D10_ERROR_FILE_NOT_FOUND = 0x88790002,
}

impl DIRECT3D10 {
    pub fn description(&self) -> &'static str {
        match self {
            DIRECT3D10::D3D10_ERROR_TOO_MANY_UNIQUE_STATE_OBJECTS => "The application has exceeded the maximum number of unique state objects per Direct3D device.",
            DIRECT3D10::D3D10_ERROR_FILE_NOT_FOUND => "The specified file was not found.",
        }
    }
}
