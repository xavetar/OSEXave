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

pub enum USERMODE_FILTER_MANAGER {
    ERROR_FLT_IO_COMPLETE = 0x001F0001,
    ERROR_FLT_NO_HANDLER_DEFINED = 0x801F0001,
    ERROR_FLT_CONTEXT_ALREADY_DEFINED = 0x801F0002,
    ERROR_FLT_INVALID_ASYNCHRONOUS_REQUEST = 0x801F0003,
    ERROR_FLT_DISALLOW_FAST_IO = 0x801F0004,
    ERROR_FLT_INVALID_NAME_REQUEST = 0x801F0005,
    ERROR_FLT_NOT_SAFE_TO_POST_OPERATION = 0x801F0006,
    ERROR_FLT_NOT_INITIALIZED = 0x801F0007,
    ERROR_FLT_FILTER_NOT_READY = 0x801F0008,
    ERROR_FLT_POST_OPERATION_CLEANUP = 0x801F0009,
    ERROR_FLT_INTERNAL_ERROR = 0x801F000A,
    ERROR_FLT_DELETING_OBJECT = 0x801F000B,
    ERROR_FLT_MUST_BE_NONPAGED_POOL = 0x801F000C,
    ERROR_FLT_DUPLICATE_ENTRY = 0x801F000D,
    ERROR_FLT_CBDQ_DISABLED = 0x801F000E,
    ERROR_FLT_DO_NOT_ATTACH = 0x801F000F,
    ERROR_FLT_DO_NOT_DETACH = 0x801F0010,
    ERROR_FLT_INSTANCE_ALTITUDE_COLLISION = 0x801F0011,
    ERROR_FLT_INSTANCE_NAME_COLLISION = 0x801F0012,
    ERROR_FLT_FILTER_NOT_FOUND = 0x801F0013,
    ERROR_FLT_VOLUME_NOT_FOUND = 0x801F0014,
    ERROR_FLT_INSTANCE_NOT_FOUND = 0x801F0015,
    ERROR_FLT_CONTEXT_ALLOCATION_NOT_FOUND = 0x801F0016,
    ERROR_FLT_INVALID_CONTEXT_REGISTRATION = 0x801F0017,
    ERROR_FLT_NAME_CACHE_MISS = 0x801F0018,
    ERROR_FLT_NO_DEVICE_OBJECT = 0x801F0019,
    ERROR_FLT_VOLUME_ALREADY_MOUNTED = 0x801F001A,
    ERROR_FLT_ALREADY_ENLISTED = 0x801F001B,
    ERROR_FLT_CONTEXT_ALREADY_LINKED = 0x801F001C,
    ERROR_FLT_NO_WAITER_FOR_REPLY = 0x801F0020,
    ERROR_FLT_REGISTRATION_BUSY = 0x801F0023,
    ERROR_FLT_WCOS_NOT_SUPPORTED = 0x801F0024,
}

impl USERMODE_FILTER_MANAGER {
    pub fn description(&self) -> &'static str {
        match self {
            USERMODE_FILTER_MANAGER::ERROR_FLT_IO_COMPLETE => "The IO was completed by a filter.",
            USERMODE_FILTER_MANAGER::ERROR_FLT_NO_HANDLER_DEFINED => "A handler was not defined by the filter for this operation.",
            USERMODE_FILTER_MANAGER::ERROR_FLT_CONTEXT_ALREADY_DEFINED => "A context is already defined for this object.",
            USERMODE_FILTER_MANAGER::ERROR_FLT_INVALID_ASYNCHRONOUS_REQUEST => "Asynchronous requests are not valid for this operation.",
            USERMODE_FILTER_MANAGER::ERROR_FLT_DISALLOW_FAST_IO => "Disallow the Fast IO path for this operation.",
            USERMODE_FILTER_MANAGER::ERROR_FLT_INVALID_NAME_REQUEST => "An invalid name request was made. The name requested cannot be retrieved at this time.",
            USERMODE_FILTER_MANAGER::ERROR_FLT_NOT_SAFE_TO_POST_OPERATION => "Posting this operation to a worker thread for further processing is not safe at this time because it could lead to a system deadlock.",
            USERMODE_FILTER_MANAGER::ERROR_FLT_NOT_INITIALIZED => "The Filter Manager was not initialized when a filter tried to register. Make sure that the Filter Manager is getting loaded as a driver.",
            USERMODE_FILTER_MANAGER::ERROR_FLT_FILTER_NOT_READY => "The filter is not ready for attachment to volumes because it has not finished initializing (FltStartFiltering has not been called).",
            USERMODE_FILTER_MANAGER::ERROR_FLT_POST_OPERATION_CLEANUP => "The filter must cleanup any operation specific context at this time because it is being removed from the system before the operation is completed by the lower drivers.",
            USERMODE_FILTER_MANAGER::ERROR_FLT_INTERNAL_ERROR => "The Filter Manager had an internal error from which it cannot recover, therefore the operation has been failed. This is usually the result of a filter returning an invalid value from a pre-operation callback.",
            USERMODE_FILTER_MANAGER::ERROR_FLT_DELETING_OBJECT => "The object specified for this action is in the process of being deleted, therefore the action requested cannot be completed at this time.",
            USERMODE_FILTER_MANAGER::ERROR_FLT_MUST_BE_NONPAGED_POOL => "Non-paged pool must be used for this type of context.",
            USERMODE_FILTER_MANAGER::ERROR_FLT_DUPLICATE_ENTRY => "A duplicate handler definition has been provided for an operation.",
            USERMODE_FILTER_MANAGER::ERROR_FLT_CBDQ_DISABLED => "The callback data queue has been disabled.",
            USERMODE_FILTER_MANAGER::ERROR_FLT_DO_NOT_ATTACH => "Do not attach the filter to the volume at this time.",
            USERMODE_FILTER_MANAGER::ERROR_FLT_DO_NOT_DETACH => "Do not detach the filter from the volume at this time.",
            USERMODE_FILTER_MANAGER::ERROR_FLT_INSTANCE_ALTITUDE_COLLISION => "An instance already exists at this altitude on the volume specified.",
            USERMODE_FILTER_MANAGER::ERROR_FLT_INSTANCE_NAME_COLLISION => "An instance already exists with this name on the volume specified.",
            USERMODE_FILTER_MANAGER::ERROR_FLT_FILTER_NOT_FOUND => "The system could not find the filter specified.",
            USERMODE_FILTER_MANAGER::ERROR_FLT_VOLUME_NOT_FOUND => "The system could not find the volume specified.",
            USERMODE_FILTER_MANAGER::ERROR_FLT_INSTANCE_NOT_FOUND => "The system could not find the instance specified.",
            USERMODE_FILTER_MANAGER::ERROR_FLT_CONTEXT_ALLOCATION_NOT_FOUND => "No registered context allocation definition was found for the given request.",
            USERMODE_FILTER_MANAGER::ERROR_FLT_INVALID_CONTEXT_REGISTRATION => "An invalid parameter was specified during context registration.",
            USERMODE_FILTER_MANAGER::ERROR_FLT_NAME_CACHE_MISS => "The name requested was not found in Filter Manager's name cache and could not be retrieved from the file system.",
            USERMODE_FILTER_MANAGER::ERROR_FLT_NO_DEVICE_OBJECT => "The requested device object does not exist for the given volume.",
            USERMODE_FILTER_MANAGER::ERROR_FLT_VOLUME_ALREADY_MOUNTED => "The specified volume is already mounted.",
            USERMODE_FILTER_MANAGER::ERROR_FLT_ALREADY_ENLISTED => "The specified Transaction Context is already enlisted in a transaction",
            USERMODE_FILTER_MANAGER::ERROR_FLT_CONTEXT_ALREADY_LINKED => "The specified context is already attached to another object",
            USERMODE_FILTER_MANAGER::ERROR_FLT_NO_WAITER_FOR_REPLY => "No waiter is present for the filter's reply to this message.",
            USERMODE_FILTER_MANAGER::ERROR_FLT_REGISTRATION_BUSY => "The filesystem database resource is in use. Registration cannot complete at this time.",
            USERMODE_FILTER_MANAGER::ERROR_FLT_WCOS_NOT_SUPPORTED => "The filter is not allowed to attach because it has not declared compability with WCOS.",
        }
    }
}
