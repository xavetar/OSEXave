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

pub enum DXGI_DDI {
    DXGI_DDI_ERR_WASSTILLDRAWING = 0x887B0001,
    DXGI_DDI_ERR_UNSUPPORTED = 0x887B0002,
    DXGI_DDI_ERR_NONEXCLUSIVE = 0x887B0003,
}

impl DXGI_DDI {
    pub fn description(&self) -> &'static str {
        match self {
            DXGI_DDI::DXGI_DDI_ERR_WASSTILLDRAWING => "The GPU was busy when the operation was requested.",
            DXGI_DDI::DXGI_DDI_ERR_UNSUPPORTED => "The driver has rejected the creation of this resource.",
            DXGI_DDI::DXGI_DDI_ERR_NONEXCLUSIVE => "The GPU counter was in use by another process or d3d device when application requested access to it.",
        }
    }
}
