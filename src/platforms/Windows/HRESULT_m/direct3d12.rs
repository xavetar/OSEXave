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

pub enum DIRECT3D12 {
    D3D12_ERROR_ADAPTER_NOT_FOUND = 0x887E0001,
    D3D12_ERROR_DRIVER_VERSION_MISMATCH = 0x887E0002,
    D3D12_ERROR_INVALID_REDIST = 0x887E0003,
}

impl DIRECT3D12 {
    pub fn description(&self) -> &'static str {
        match self {
            DIRECT3D12::D3D12_ERROR_ADAPTER_NOT_FOUND => "The blob provided does not match the adapter that the device was created on.",
            DIRECT3D12::D3D12_ERROR_DRIVER_VERSION_MISMATCH => "The blob provided was created for a different version of the driver, and must be re-created.",
            DIRECT3D12::D3D12_ERROR_INVALID_REDIST => "The D3D12 SDK version configuration of the host exe is invalid.",
        }
    }
}
