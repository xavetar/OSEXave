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

pub enum DXGI {
    DXGI_STATUS_OCCLUDED = 0x087A0001,
    DXGI_STATUS_CLIPPED = 0x087A0002,
    DXGI_STATUS_NO_REDIRECTION = 0x087A0004,
    DXGI_STATUS_NO_DESKTOP_ACCESS = 0x087A0005,
    DXGI_STATUS_GRAPHICS_VIDPN_SOURCE_IN_USE = 0x087A0006,
    DXGI_STATUS_MODE_CHANGED = 0x087A0007,
    DXGI_STATUS_MODE_CHANGE_IN_PROGRESS = 0x087A0008,
    DXGI_STATUS_UNOCCLUDED = 0x087A0009,
    DXGI_STATUS_DDA_WAS_STILL_DRAWING = 0x087A000A,
    DXGI_STATUS_PRESENT_REQUIRED = 0x087A002F,
    DXGI_ERROR_INVALID_CALL = 0x887A0001,
    DXGI_ERROR_NOT_FOUND = 0x887A0002,
    DXGI_ERROR_MORE_DATA = 0x887A0003,
    DXGI_ERROR_UNSUPPORTED = 0x887A0004,
    DXGI_ERROR_DEVICE_REMOVED = 0x887A0005,
    DXGI_ERROR_DEVICE_HUNG = 0x887A0006,
    DXGI_ERROR_DEVICE_RESET = 0x887A0007,
    DXGI_ERROR_WAS_STILL_DRAWING = 0x887A000A,
    DXGI_ERROR_FRAME_STATISTICS_DISJOINT = 0x887A000B,
    DXGI_ERROR_GRAPHICS_VIDPN_SOURCE_IN_USE = 0x887A000C,
    DXGI_ERROR_DRIVER_INTERNAL_ERROR = 0x887A0020,
    DXGI_ERROR_NONEXCLUSIVE = 0x887A0021,
    DXGI_ERROR_NOT_CURRENTLY_AVAILABLE = 0x887A0022,
    DXGI_ERROR_REMOTE_CLIENT_DISCONNECTED = 0x887A0023,
    DXGI_ERROR_REMOTE_OUTOFMEMORY = 0x887A0024,
    DXGI_ERROR_MODE_CHANGE_IN_PROGRESS = 0x887A0025,
    DXGI_ERROR_ACCESS_LOST = 0x887A0026,
    DXGI_ERROR_WAIT_TIMEOUT = 0x887A0027,
    DXGI_ERROR_SESSION_DISCONNECTED = 0x887A0028,
    DXGI_ERROR_RESTRICT_TO_OUTPUT_STALE = 0x887A0029,
    DXGI_ERROR_CANNOT_PROTECT_CONTENT = 0x887A002A,
    DXGI_ERROR_ACCESS_DENIED = 0x887A002B,
    DXGI_ERROR_NAME_ALREADY_EXISTS = 0x887A002C,
    DXGI_ERROR_SDK_COMPONENT_MISSING = 0x887A002D,
    DXGI_ERROR_NOT_CURRENT = 0x887A002E,
    DXGI_ERROR_HW_PROTECTION_OUTOFMEMORY = 0x887A0030,
    DXGI_ERROR_DYNAMIC_CODE_POLICY_VIOLATION = 0x887A0031,
    DXGI_ERROR_NON_COMPOSITED_UI = 0x887A0032,
    DXGI_ERROR_CACHE_CORRUPT = 0x887A0033,
    DXGI_ERROR_CACHE_FULL = 0x887A0034,
    DXGI_ERROR_CACHE_HASH_COLLISION = 0x887A0035,
    DXGI_ERROR_ALREADY_EXISTS = 0x887A0036,
    DXGI_ERROR_MPO_UNPINNED = 0x887A0064,
}

impl DXGI {
    pub fn description(&self) -> &'static str {
        match self {
            DXGI::DXGI_STATUS_OCCLUDED => "The Present operation was invisible to the user.",
            DXGI::DXGI_STATUS_CLIPPED => "The Present operation was partially invisible to the user.",
            DXGI::DXGI_STATUS_NO_REDIRECTION => "The driver is requesting that the DXGI runtime not use shared resources to communicate with the Desktop Window Manager.",
            DXGI::DXGI_STATUS_NO_DESKTOP_ACCESS => "The Present operation was not visible because the Windows session has switched to another desktop (for example, ctrl-alt-del).",
            DXGI::DXGI_STATUS_GRAPHICS_VIDPN_SOURCE_IN_USE => "The Present operation was not visible because the target monitor was being used for some other purpose.",
            DXGI::DXGI_STATUS_MODE_CHANGED => "The Present operation was not visible because the display mode changed. DXGI will have re-attempted the presentation.",
            DXGI::DXGI_STATUS_MODE_CHANGE_IN_PROGRESS => "The Present operation was not visible because another Direct3D device was attempting to take fullscreen mode at the time.",
            DXGI::DXGI_STATUS_UNOCCLUDED => "The swapchain has become unoccluded.",
            DXGI::DXGI_STATUS_DDA_WAS_STILL_DRAWING => "The adapter did not have access to the required resources to complete the Desktop Duplication Present() call, the Present() call needs to be made again",
            DXGI::DXGI_STATUS_PRESENT_REQUIRED => "The present succeeded but the caller should present again on the next V-sync, even if there are no changes to the content.",
            DXGI::DXGI_ERROR_INVALID_CALL => "The application made a call that is invalid. Either the parameters of the call or the state of some object was incorrect.",
            DXGI::DXGI_ERROR_NOT_FOUND => "The object was not found. If calling IDXGIFactory::EnumAdaptes, there is no adapter with the specified ordinal.",
            DXGI::DXGI_ERROR_MORE_DATA => "The caller did not supply a sufficiently large buffer.",
            DXGI::DXGI_ERROR_UNSUPPORTED => "The specified device interface or feature level is not supported on this system.",
            DXGI::DXGI_ERROR_DEVICE_REMOVED => "The GPU device instance has been suspended. Use GetDeviceRemovedReason to determine the appropriate action.",
            DXGI::DXGI_ERROR_DEVICE_HUNG => "The GPU will not respond to more commands, most likely because of an invalid command passed by the calling application.",
            DXGI::DXGI_ERROR_DEVICE_RESET => "The GPU will not respond to more commands, most likely because some other application submitted invalid commands.",
            DXGI::DXGI_ERROR_WAS_STILL_DRAWING => "The GPU was busy at the moment when the call was made, and the call was neither executed nor scheduled.",
            DXGI::DXGI_ERROR_FRAME_STATISTICS_DISJOINT => "An event (such as power cycle) interrupted the gathering of presentation statistics. Any previous statistics should be",
            DXGI::DXGI_ERROR_GRAPHICS_VIDPN_SOURCE_IN_USE => "Fullscreen mode could not be achieved because the specified output was already in use.",
            DXGI::DXGI_ERROR_DRIVER_INTERNAL_ERROR => "An internal issue prevented the driver from carrying out the specified operation. The driver's state is probably suspect,",
            DXGI::DXGI_ERROR_NONEXCLUSIVE => "A global counter resource was in use, and the specified counter cannot be used by this Direct3D device at this time.",
            DXGI::DXGI_ERROR_NOT_CURRENTLY_AVAILABLE => "A resource is not available at the time of the call, but may become available later.",
            DXGI::DXGI_ERROR_REMOTE_CLIENT_DISCONNECTED => "The application's remote device has been removed due to session disconnect or network disconnect.",
            DXGI::DXGI_ERROR_REMOTE_OUTOFMEMORY => "The device has been removed during a remote session because the remote computer ran out of memory.",
            DXGI::DXGI_ERROR_MODE_CHANGE_IN_PROGRESS => "An on-going mode change prevented completion of the call. The call may succeed if attempted later.",
            DXGI::DXGI_ERROR_ACCESS_LOST => "The keyed mutex was abandoned.",
            DXGI::DXGI_ERROR_WAIT_TIMEOUT => "The timeout value has elapsed and the resource is not yet available.",
            DXGI::DXGI_ERROR_SESSION_DISCONNECTED => "The output duplication has been turned off because the Windows session ended or was disconnected.",
            DXGI::DXGI_ERROR_RESTRICT_TO_OUTPUT_STALE => "The DXGI output (monitor) to which the swapchain content was restricted, has been disconnected or changed.",
            DXGI::DXGI_ERROR_CANNOT_PROTECT_CONTENT => "DXGI is unable to provide content protection on the swapchain. This is typically caused by an older driver,",
            DXGI::DXGI_ERROR_ACCESS_DENIED => "The application is trying to use a resource to which it does not have the required access privileges.",
            DXGI::DXGI_ERROR_NAME_ALREADY_EXISTS => "The application is trying to create a shared handle using a name that is already associated with some other resource.",
            DXGI::DXGI_ERROR_SDK_COMPONENT_MISSING => "The application requested an operation that depends on an SDK component that is missing or mismatched.",
            DXGI::DXGI_ERROR_NOT_CURRENT => "The DXGI objects that the application has created are no longer current & need to be recreated for this operation to be performed.",
            DXGI::DXGI_ERROR_HW_PROTECTION_OUTOFMEMORY => "Insufficient HW protected memory exits for proper function.",
            DXGI::DXGI_ERROR_DYNAMIC_CODE_POLICY_VIOLATION => "Creating this device would violate the process's dynamic code policy.",
            DXGI::DXGI_ERROR_NON_COMPOSITED_UI => "The operation failed because the compositor is not in control of the output.",
            DXGI::DXGI_ERROR_CACHE_CORRUPT => "The cache is corrupt and either could not be opened or could not be reset.",
            DXGI::DXGI_ERROR_CACHE_FULL => "This entry would cause the cache to exceed its quota. On a load operation, this may indicate exceeding the maximum in-memory size.",
            DXGI::DXGI_ERROR_CACHE_HASH_COLLISION => "A cache entry was found, but the key provided does not match the key stored in the entry.",
            DXGI::DXGI_ERROR_ALREADY_EXISTS => "The desired element already exists.",
            DXGI::DXGI_ERROR_MPO_UNPINNED => "The allocation of the MPO plane has been unpinned",
        }
    }
}
