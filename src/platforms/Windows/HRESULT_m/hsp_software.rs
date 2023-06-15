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

pub enum HSP_SOFTWARE {
    HSP_DRV_ERROR_MASK = 0x81290000,
    HSP_DRV_INTERNAL_ERROR = 0x812900FF,
    HSP_BASE_ERROR_MASK = 0x81290100,
    HSP_BASE_INTERNAL_ERROR = 0x812901FF,
    HSP_KSP_ERROR_MASK = 0x81290200,
    HSP_KSP_DEVICE_NOT_READY = 0x81290201,
    HSP_KSP_INVALID_PROVIDER_HANDLE = 0x81290202,
    HSP_KSP_INVALID_KEY_HANDLE = 0x81290203,
    HSP_KSP_INVALID_PARAMETER = 0x81290204,
    HSP_KSP_BUFFER_TOO_SMALL = 0x81290205,
    HSP_KSP_NOT_SUPPORTED = 0x81290206,
    HSP_KSP_INVALID_DATA = 0x81290207,
    HSP_KSP_INVALID_FLAGS = 0x81290208,
    HSP_KSP_ALGORITHM_NOT_SUPPORTED = 0x81290209,
    HSP_KSP_KEY_ALREADY_FINALIZED = 0x8129020A,
    HSP_KSP_KEY_NOT_FINALIZED = 0x8129020B,
    HSP_KSP_INVALID_KEY_TYPE = 0x8129020C,
    HSP_KSP_NO_MEMORY = 0x81290210,
    HSP_KSP_PARAMETER_NOT_SET = 0x81290211,
    HSP_KSP_KEY_EXISTS = 0x81290215,
    HSP_KSP_KEY_MISSING = 0x81290216,
    HSP_KSP_KEY_LOAD_FAIL = 0x81290217,
    HSP_KSP_NO_MORE_ITEMS = 0x81290218,
    HSP_KSP_INTERNAL_ERROR = 0x812902FF,
}

impl HSP_SOFTWARE {
    pub fn description(&self) -> &'static str {
        match self {
            HSP_SOFTWARE::HSP_DRV_ERROR_MASK => "This is an error mask to convert HSP driver errors to Win errors.",
            HSP_SOFTWARE::HSP_DRV_INTERNAL_ERROR => "Catastrophic internal failure in the HSP driver.",
            HSP_SOFTWARE::HSP_BASE_ERROR_MASK => "This is an error mask to convert HSP base class errors to Win errors.",
            HSP_SOFTWARE::HSP_BASE_INTERNAL_ERROR => "Catastrophic internal failure in the HSP base class.",
            HSP_SOFTWARE::HSP_KSP_ERROR_MASK => "This is an error mask to convert HSP KSP errors to Win errors.",
            HSP_SOFTWARE::HSP_KSP_DEVICE_NOT_READY => "The Pluton processor is currently not ready for use.",
            HSP_SOFTWARE::HSP_KSP_INVALID_PROVIDER_HANDLE => "The handle to the HSP KSP is invalid.",
            HSP_SOFTWARE::HSP_KSP_INVALID_KEY_HANDLE => "The handle to a key stored by the HSP KSP is invalid.",
            HSP_SOFTWARE::HSP_KSP_INVALID_PARAMETER => "A parameter to the HSP KSP was invalid.",
            HSP_SOFTWARE::HSP_KSP_BUFFER_TOO_SMALL => "The supplied buffer is too small.",
            HSP_SOFTWARE::HSP_KSP_NOT_SUPPORTED => "The requested operation is not supported.",
            HSP_SOFTWARE::HSP_KSP_INVALID_DATA => "The provided data is invalid.",
            HSP_SOFTWARE::HSP_KSP_INVALID_FLAGS => "The provided flags are invalid.",
            HSP_SOFTWARE::HSP_KSP_ALGORITHM_NOT_SUPPORTED => "The algorithm identifier is not supported.",
            HSP_SOFTWARE::HSP_KSP_KEY_ALREADY_FINALIZED => "The key has already been finalized.",
            HSP_SOFTWARE::HSP_KSP_KEY_NOT_FINALIZED => "The key has not been finalized.",
            HSP_SOFTWARE::HSP_KSP_INVALID_KEY_TYPE => "The key does not support the requested operation.",
            HSP_SOFTWARE::HSP_KSP_NO_MEMORY => "There is not enough memory for the operation.",
            HSP_SOFTWARE::HSP_KSP_PARAMETER_NOT_SET => "The parameter has not been set and has no default value.",
            HSP_SOFTWARE::HSP_KSP_KEY_EXISTS => "Key object already exists.",
            HSP_SOFTWARE::HSP_KSP_KEY_MISSING => "The requsted key object does not exist.",
            HSP_SOFTWARE::HSP_KSP_KEY_LOAD_FAIL => "Failed to load the requested key.",
            HSP_SOFTWARE::HSP_KSP_NO_MORE_ITEMS => "No more data is available.",
            HSP_SOFTWARE::HSP_KSP_INTERNAL_ERROR => "Catastrophic internal failure in the HSP KSP.",
        }
    }
}
