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

pub enum USERMODE_VIRTUALIZATION {
    ERROR_VID_REMOTE_NODE_PARENT_GPA_PAGES_USED = 0x80370001,
    HCS_E_TERMINATED_DURING_START = 0x80370100,
    HCS_E_IMAGE_MISMATCH = 0x80370101,
    HCS_E_HYPERV_NOT_INSTALLED = 0x80370102,
    HCS_E_INVALID_STATE = 0x80370105,
    HCS_E_UNEXPECTED_EXIT = 0x80370106,
    HCS_E_TERMINATED = 0x80370107,
    HCS_E_CONNECT_FAILED = 0x80370108,
    HCS_E_CONNECTION_TIMEOUT = 0x80370109,
    HCS_E_CONNECTION_CLOSED = 0x8037010A,
    HCS_E_UNKNOWN_MESSAGE = 0x8037010B,
    HCS_E_UNSUPPORTED_PROTOCOL_VERSION = 0x8037010C,
    HCS_E_INVALID_JSON = 0x8037010D,
    HCS_E_SYSTEM_NOT_FOUND = 0x8037010E,
    HCS_E_SYSTEM_ALREADY_EXISTS = 0x8037010F,
    HCS_E_SYSTEM_ALREADY_STOPPED = 0x80370110,
    HCS_E_PROTOCOL_ERROR = 0x80370111,
    HCS_E_INVALID_LAYER = 0x80370112,
    HCS_E_WINDOWS_INSIDER_REQUIRED = 0x80370113,
    HCS_E_SERVICE_NOT_AVAILABLE = 0x80370114,
    HCS_E_OPERATION_NOT_STARTED = 0x80370115,
    HCS_E_OPERATION_ALREADY_STARTED = 0x80370116,
    HCS_E_OPERATION_PENDING = 0x80370117,
    HCS_E_OPERATION_TIMEOUT = 0x80370118,
    HCS_E_OPERATION_SYSTEM_CALLBACK_ALREADY_SET = 0x80370119,
    HCS_E_OPERATION_RESULT_ALLOCATION_FAILED = 0x8037011A,
    HCS_E_ACCESS_DENIED = 0x8037011B,
    HCS_E_GUEST_CRITICAL_ERROR = 0x8037011C,
    HCS_E_PROCESS_INFO_NOT_AVAILABLE = 0x8037011D,
    HCS_E_SERVICE_DISCONNECT = 0x8037011E,
    HCS_E_PROCESS_ALREADY_STOPPED = 0x8037011F,
    HCS_E_SYSTEM_NOT_CONFIGURED_FOR_OPERATION = 0x80370120,
    HCS_E_OPERATION_ALREADY_CANCELLED = 0x80370121,
    WHV_E_UNKNOWN_CAPABILITY = 0x80370300,
    WHV_E_INSUFFICIENT_BUFFER = 0x80370301,
    WHV_E_UNKNOWN_PROPERTY = 0x80370302,
    WHV_E_UNSUPPORTED_HYPERVISOR_CONFIG = 0x80370303,
    WHV_E_INVALID_PARTITION_CONFIG = 0x80370304,
    WHV_E_GPA_RANGE_NOT_FOUND = 0x80370305,
    WHV_E_VP_ALREADY_EXISTS = 0x80370306,
    WHV_E_VP_DOES_NOT_EXIST = 0x80370307,
    WHV_E_INVALID_VP_STATE = 0x80370308,
    WHV_E_INVALID_VP_REGISTER_NAME = 0x80370309,
    WHV_E_UNSUPPORTED_PROCESSOR_CONFIG = 0x80370310,
    ERROR_VID_DUPLICATE_HANDLER = 0xC0370001,
    ERROR_VID_TOO_MANY_HANDLERS = 0xC0370002,
    ERROR_VID_QUEUE_FULL = 0xC0370003,
    ERROR_VID_HANDLER_NOT_PRESENT = 0xC0370004,
    ERROR_VID_INVALID_OBJECT_NAME = 0xC0370005,
    ERROR_VID_PARTITION_NAME_TOO_LONG = 0xC0370006,
    ERROR_VID_MESSAGE_QUEUE_NAME_TOO_LONG = 0xC0370007,
    ERROR_VID_PARTITION_ALREADY_EXISTS = 0xC0370008,
    ERROR_VID_PARTITION_DOES_NOT_EXIST = 0xC0370009,
    ERROR_VID_PARTITION_NAME_NOT_FOUND = 0xC037000A,
    ERROR_VID_MESSAGE_QUEUE_ALREADY_EXISTS = 0xC037000B,
    ERROR_VID_EXCEEDED_MBP_ENTRY_MAP_LIMIT = 0xC037000C,
    ERROR_VID_MB_STILL_REFERENCED = 0xC037000D,
    ERROR_VID_CHILD_GPA_PAGE_SET_CORRUPTED = 0xC037000E,
    ERROR_VID_INVALID_NUMA_SETTINGS = 0xC037000F,
    ERROR_VID_INVALID_NUMA_NODE_INDEX = 0xC0370010,
    ERROR_VID_NOTIFICATION_QUEUE_ALREADY_ASSOCIATED = 0xC0370011,
    ERROR_VID_INVALID_MEMORY_BLOCK_HANDLE = 0xC0370012,
    ERROR_VID_PAGE_RANGE_OVERFLOW = 0xC0370013,
    ERROR_VID_INVALID_MESSAGE_QUEUE_HANDLE = 0xC0370014,
    ERROR_VID_INVALID_GPA_RANGE_HANDLE = 0xC0370015,
    ERROR_VID_NO_MEMORY_BLOCK_NOTIFICATION_QUEUE = 0xC0370016,
    ERROR_VID_MEMORY_BLOCK_LOCK_COUNT_EXCEEDED = 0xC0370017,
    ERROR_VID_INVALID_PPM_HANDLE = 0xC0370018,
    ERROR_VID_MBPS_ARE_LOCKED = 0xC0370019,
    ERROR_VID_MESSAGE_QUEUE_CLOSED = 0xC037001A,
    ERROR_VID_VIRTUAL_PROCESSOR_LIMIT_EXCEEDED = 0xC037001B,
    ERROR_VID_STOP_PENDING = 0xC037001C,
    ERROR_VID_INVALID_PROCESSOR_STATE = 0xC037001D,
    ERROR_VID_EXCEEDED_KM_CONTEXT_COUNT_LIMIT = 0xC037001E,
    ERROR_VID_KM_INTERFACE_ALREADY_INITIALIZED = 0xC037001F,
    ERROR_VID_MB_PROPERTY_ALREADY_SET_RESET = 0xC0370020,
    ERROR_VID_MMIO_RANGE_DESTROYED = 0xC0370021,
    ERROR_VID_INVALID_CHILD_GPA_PAGE_SET = 0xC0370022,
    ERROR_VID_RESERVE_PAGE_SET_IS_BEING_USED = 0xC0370023,
    ERROR_VID_RESERVE_PAGE_SET_TOO_SMALL = 0xC0370024,
    ERROR_VID_MBP_ALREADY_LOCKED_USING_RESERVED_PAGE = 0xC0370025,
    ERROR_VID_MBP_COUNT_EXCEEDED_LIMIT = 0xC0370026,
    ERROR_VID_SAVED_STATE_CORRUPT = 0xC0370027,
    ERROR_VID_SAVED_STATE_UNRECOGNIZED_ITEM = 0xC0370028,
    ERROR_VID_SAVED_STATE_INCOMPATIBLE = 0xC0370029,
    ERROR_VID_VTL_ACCESS_DENIED = 0xC037002A,
    ERROR_VID_INSUFFICIENT_RESOURCES_RESERVE = 0xC037002B,
    ERROR_VID_INSUFFICIENT_RESOURCES_PHYSICAL_BUFFER = 0xC037002C,
    ERROR_VID_INSUFFICIENT_RESOURCES_HV_DEPOSIT = 0xC037002D,
    ERROR_VID_MEMORY_TYPE_NOT_SUPPORTED = 0xC037002E,
    ERROR_VID_INSUFFICIENT_RESOURCES_WITHDRAW = 0xC037002F,
    ERROR_VID_PROCESS_ALREADY_SET = 0xC0370030,
    ERROR_VMCOMPUTE_TERMINATED_DURING_START = 0xC0370100,
    ERROR_VMCOMPUTE_IMAGE_MISMATCH = 0xC0370101,
    ERROR_VMCOMPUTE_HYPERV_NOT_INSTALLED = 0xC0370102,
    ERROR_VMCOMPUTE_OPERATION_PENDING = 0xC0370103,
    ERROR_VMCOMPUTE_TOO_MANY_NOTIFICATIONS = 0xC0370104,
    ERROR_VMCOMPUTE_INVALID_STATE = 0xC0370105,
    ERROR_VMCOMPUTE_UNEXPECTED_EXIT = 0xC0370106,
    ERROR_VMCOMPUTE_TERMINATED = 0xC0370107,
    ERROR_VMCOMPUTE_CONNECT_FAILED = 0xC0370108,
    ERROR_VMCOMPUTE_TIMEOUT = 0xC0370109,
    ERROR_VMCOMPUTE_CONNECTION_CLOSED = 0xC037010A,
    ERROR_VMCOMPUTE_UNKNOWN_MESSAGE = 0xC037010B,
    ERROR_VMCOMPUTE_UNSUPPORTED_PROTOCOL_VERSION = 0xC037010C,
    ERROR_VMCOMPUTE_INVALID_JSON = 0xC037010D,
    ERROR_VMCOMPUTE_SYSTEM_NOT_FOUND = 0xC037010E,
    ERROR_VMCOMPUTE_SYSTEM_ALREADY_EXISTS = 0xC037010F,
    ERROR_VMCOMPUTE_SYSTEM_ALREADY_STOPPED = 0xC0370110,
    ERROR_VMCOMPUTE_PROTOCOL_ERROR = 0xC0370111,
    ERROR_VMCOMPUTE_INVALID_LAYER = 0xC0370112,
    ERROR_VMCOMPUTE_WINDOWS_INSIDER_REQUIRED = 0xC0370113,
    ERROR_VNET_VIRTUAL_SWITCH_NAME_NOT_FOUND = 0xC0370200,
    ERROR_VSMB_SAVED_STATE_FILE_NOT_FOUND = 0xC0370400,
    ERROR_VSMB_SAVED_STATE_CORRUPT = 0xC0370401,
    VM_SAVED_STATE_DUMP_E_PARTITION_STATE_NOT_FOUND = 0xC0370500,
    VM_SAVED_STATE_DUMP_E_GUEST_MEMORY_NOT_FOUND = 0xC0370501,
    VM_SAVED_STATE_DUMP_E_NO_VP_FOUND_IN_PARTITION_STATE = 0xC0370502,
    VM_SAVED_STATE_DUMP_E_NESTED_VIRTUALIZATION_NOT_SUPPORTED = 0xC0370503,
    VM_SAVED_STATE_DUMP_E_WINDOWS_KERNEL_IMAGE_NOT_FOUND = 0xC0370504,
    VM_SAVED_STATE_DUMP_E_VA_NOT_MAPPED = 0xC0370505,
    VM_SAVED_STATE_DUMP_E_INVALID_VP_STATE = 0xC0370506,
    VM_SAVED_STATE_DUMP_E_VP_VTL_NOT_ENABLED = 0xC0370509,
    ERROR_DM_OPERATION_LIMIT_EXCEEDED = 0xC0370600,
}

impl USERMODE_VIRTUALIZATION {
    pub fn description(&self) -> &'static str {
        match self {
            USERMODE_VIRTUALIZATION::ERROR_VID_REMOTE_NODE_PARENT_GPA_PAGES_USED => "A virtual machine is running with its memory allocated across multiple NUMA nodes. This does not indicate a problem unless the performance of your virtual machine is unusually slow. If you are experiencing performance problems, you may need to modify the NUMA configuration.",
            USERMODE_VIRTUALIZATION::HCS_E_TERMINATED_DURING_START => "The virtual machine or container exited unexpectedly while starting.",
            USERMODE_VIRTUALIZATION::HCS_E_IMAGE_MISMATCH => "The container operating system does not match the host operating system.",
            USERMODE_VIRTUALIZATION::HCS_E_HYPERV_NOT_INSTALLED => "The virtual machine could not be started because a required feature is not installed.",
            USERMODE_VIRTUALIZATION::HCS_E_INVALID_STATE => "The requested virtual machine or container operation is not valid in the current state.",
            USERMODE_VIRTUALIZATION::HCS_E_UNEXPECTED_EXIT => "The virtual machine or container exited unexpectedly.",
            USERMODE_VIRTUALIZATION::HCS_E_TERMINATED => "The virtual machine or container was forcefully exited.",
            USERMODE_VIRTUALIZATION::HCS_E_CONNECT_FAILED => "A connection could not be established with the container or virtual machine.",
            USERMODE_VIRTUALIZATION::HCS_E_CONNECTION_TIMEOUT => "The operation timed out because a response was not received from the virtual machine or container.",
            USERMODE_VIRTUALIZATION::HCS_E_CONNECTION_CLOSED => "The connection with the virtual machine or container was closed.",
            USERMODE_VIRTUALIZATION::HCS_E_UNKNOWN_MESSAGE => "An unknown internal message was received by the virtual machine or container.",
            USERMODE_VIRTUALIZATION::HCS_E_UNSUPPORTED_PROTOCOL_VERSION => "The virtual machine or container does not support an available version of the communication protocol with the host.",
            USERMODE_VIRTUALIZATION::HCS_E_INVALID_JSON => "The virtual machine or container JSON document is invalid.",
            USERMODE_VIRTUALIZATION::HCS_E_SYSTEM_NOT_FOUND => "A virtual machine or container with the specified identifier does not exist.",
            USERMODE_VIRTUALIZATION::HCS_E_SYSTEM_ALREADY_EXISTS => "A virtual machine or container with the specified identifier already exists.",
            USERMODE_VIRTUALIZATION::HCS_E_SYSTEM_ALREADY_STOPPED => "The virtual machine or container with the specified identifier is not running.",
            USERMODE_VIRTUALIZATION::HCS_E_PROTOCOL_ERROR => "A communication protocol error has occurred between the virtual machine or container and the host.",
            USERMODE_VIRTUALIZATION::HCS_E_INVALID_LAYER => "The container image contains a layer with an unrecognized format.",
            USERMODE_VIRTUALIZATION::HCS_E_WINDOWS_INSIDER_REQUIRED => "To use this container image, you must join the Windows Insider Program. Please see https://go.microsoft.com/fwlink/?linkid=850659 for more information.",
            USERMODE_VIRTUALIZATION::HCS_E_SERVICE_NOT_AVAILABLE => "The operation could not be started because a required feature is not installed.",
            USERMODE_VIRTUALIZATION::HCS_E_OPERATION_NOT_STARTED => "The operation has not started.",
            USERMODE_VIRTUALIZATION::HCS_E_OPERATION_ALREADY_STARTED => "The operation is already running.",
            USERMODE_VIRTUALIZATION::HCS_E_OPERATION_PENDING => "The operation is still running.",
            USERMODE_VIRTUALIZATION::HCS_E_OPERATION_TIMEOUT => "The operation did not complete in time.",
            USERMODE_VIRTUALIZATION::HCS_E_OPERATION_SYSTEM_CALLBACK_ALREADY_SET => "An event callback has already been registered on this handle.",
            USERMODE_VIRTUALIZATION::HCS_E_OPERATION_RESULT_ALLOCATION_FAILED => "Not enough memory available to return the result of the operation.",
            USERMODE_VIRTUALIZATION::HCS_E_ACCESS_DENIED => "Insufficient privileges. Only administrators or users that are members of the Hyper-V Administrators user group are permitted to access virtual machines or containers. To add yourself to the Hyper-V Administrators user group, please see https://aka.ms/hcsadmin for more information.",
            USERMODE_VIRTUALIZATION::HCS_E_GUEST_CRITICAL_ERROR => "The virtual machine or container reported a critical error and was stopped or restarted.",
            USERMODE_VIRTUALIZATION::HCS_E_PROCESS_INFO_NOT_AVAILABLE => "The process information is not available.",
            USERMODE_VIRTUALIZATION::HCS_E_SERVICE_DISCONNECT => "The host compute system service has disconnected unexpectedly.",
            USERMODE_VIRTUALIZATION::HCS_E_PROCESS_ALREADY_STOPPED => "The process has already exited.",
            USERMODE_VIRTUALIZATION::HCS_E_SYSTEM_NOT_CONFIGURED_FOR_OPERATION => "The virtual machine or container is not configured to perform the operation.",
            USERMODE_VIRTUALIZATION::HCS_E_OPERATION_ALREADY_CANCELLED => "The operation has already been cancelled.",
            USERMODE_VIRTUALIZATION::WHV_E_UNKNOWN_CAPABILITY => "The specified capability does not exist.",
            USERMODE_VIRTUALIZATION::WHV_E_INSUFFICIENT_BUFFER => "The specified buffer is too small for the requested data.",
            USERMODE_VIRTUALIZATION::WHV_E_UNKNOWN_PROPERTY => "The specified property does not exist.",
            USERMODE_VIRTUALIZATION::WHV_E_UNSUPPORTED_HYPERVISOR_CONFIG => "The configuration of the hypervisor on this system is not supported.",
            USERMODE_VIRTUALIZATION::WHV_E_INVALID_PARTITION_CONFIG => "The configuration of the partition is not valid.",
            USERMODE_VIRTUALIZATION::WHV_E_GPA_RANGE_NOT_FOUND => "The specified GPA range was not found.",
            USERMODE_VIRTUALIZATION::WHV_E_VP_ALREADY_EXISTS => "A virtual processor with the specified index already exists.",
            USERMODE_VIRTUALIZATION::WHV_E_VP_DOES_NOT_EXIST => "A virtual processor with the specified index does not exist.",
            USERMODE_VIRTUALIZATION::WHV_E_INVALID_VP_STATE => "The virtual processor is not in the correct state to perform the requested operation.",
            USERMODE_VIRTUALIZATION::WHV_E_INVALID_VP_REGISTER_NAME => "A virtual processor register with the specified name does not exist.",
            USERMODE_VIRTUALIZATION::WHV_E_UNSUPPORTED_PROCESSOR_CONFIG => "The Windows Hypervisor Platform is not supported due to a processor limitation.",
            USERMODE_VIRTUALIZATION::ERROR_VID_DUPLICATE_HANDLER => "The handler for the virtualization infrastructure driver is already registered. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.",
            USERMODE_VIRTUALIZATION::ERROR_VID_TOO_MANY_HANDLERS => "The number of registered handlers for the virtualization infrastructure driver exceeded the maximum. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.",
            USERMODE_VIRTUALIZATION::ERROR_VID_QUEUE_FULL => "The message queue for the virtualization infrastructure driver is full and cannot accept new messages. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.",
            USERMODE_VIRTUALIZATION::ERROR_VID_HANDLER_NOT_PRESENT => "No handler exists to handle the message for the virtualization infrastructure driver. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.",
            USERMODE_VIRTUALIZATION::ERROR_VID_INVALID_OBJECT_NAME => "The name of the partition or message queue for the virtualization infrastructure driver is invalid. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.",
            USERMODE_VIRTUALIZATION::ERROR_VID_PARTITION_NAME_TOO_LONG => "The partition name of the virtualization infrastructure driver exceeds the maximum.",
            USERMODE_VIRTUALIZATION::ERROR_VID_MESSAGE_QUEUE_NAME_TOO_LONG => "The message queue name of the virtualization infrastructure driver exceeds the maximum.",
            USERMODE_VIRTUALIZATION::ERROR_VID_PARTITION_ALREADY_EXISTS => "Cannot create the partition for the virtualization infrastructure driver because another partition with the same name already exists.",
            USERMODE_VIRTUALIZATION::ERROR_VID_PARTITION_DOES_NOT_EXIST => "The virtualization infrastructure driver has encountered an error. The requested partition does not exist. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.",
            USERMODE_VIRTUALIZATION::ERROR_VID_PARTITION_NAME_NOT_FOUND => "The virtualization infrastructure driver has encountered an error. Could not find the requested partition. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.",
            USERMODE_VIRTUALIZATION::ERROR_VID_MESSAGE_QUEUE_ALREADY_EXISTS => "A message queue with the same name already exists for the virtualization infrastructure driver.",
            USERMODE_VIRTUALIZATION::ERROR_VID_EXCEEDED_MBP_ENTRY_MAP_LIMIT => "The memory block page for the virtualization infrastructure driver cannot be mapped because the page map limit has been reached. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.",
            USERMODE_VIRTUALIZATION::ERROR_VID_MB_STILL_REFERENCED => "The memory block for the virtualization infrastructure driver is still being used and cannot be destroyed.",
            USERMODE_VIRTUALIZATION::ERROR_VID_CHILD_GPA_PAGE_SET_CORRUPTED => "Cannot unlock the page array for the guest operating system memory address because it does not match a previous lock request. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.",
            USERMODE_VIRTUALIZATION::ERROR_VID_INVALID_NUMA_SETTINGS => "The non-uniform memory access (NUMA) node settings do not match the system NUMA topology. In order to start the virtual machine, you will need to modify the NUMA configuration.",
            USERMODE_VIRTUALIZATION::ERROR_VID_INVALID_NUMA_NODE_INDEX => "The non-uniform memory access (NUMA) node index does not match a valid index in the system NUMA topology.",
            USERMODE_VIRTUALIZATION::ERROR_VID_NOTIFICATION_QUEUE_ALREADY_ASSOCIATED => "The memory block for the virtualization infrastructure driver is already associated with a message queue.",
            USERMODE_VIRTUALIZATION::ERROR_VID_INVALID_MEMORY_BLOCK_HANDLE => "The handle is not a valid memory block handle for the virtualization infrastructure driver.",
            USERMODE_VIRTUALIZATION::ERROR_VID_PAGE_RANGE_OVERFLOW => "The request exceeded the memory block page limit for the virtualization infrastructure driver. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.",
            USERMODE_VIRTUALIZATION::ERROR_VID_INVALID_MESSAGE_QUEUE_HANDLE => "The handle is not a valid message queue handle for the virtualization infrastructure driver.",
            USERMODE_VIRTUALIZATION::ERROR_VID_INVALID_GPA_RANGE_HANDLE => "The handle is not a valid page range handle for the virtualization infrastructure driver.",
            USERMODE_VIRTUALIZATION::ERROR_VID_NO_MEMORY_BLOCK_NOTIFICATION_QUEUE => "Cannot install client notifications because no message queue for the virtualization infrastructure driver is associated with the memory block.",
            USERMODE_VIRTUALIZATION::ERROR_VID_MEMORY_BLOCK_LOCK_COUNT_EXCEEDED => "The request to lock or map a memory block page failed because the virtualization infrastructure driver memory block limit has been reached. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.",
            USERMODE_VIRTUALIZATION::ERROR_VID_INVALID_PPM_HANDLE => "The handle is not a valid parent partition mapping handle for the virtualization infrastructure driver.",
            USERMODE_VIRTUALIZATION::ERROR_VID_MBPS_ARE_LOCKED => "Notifications cannot be created on the memory block because it is use.",
            USERMODE_VIRTUALIZATION::ERROR_VID_MESSAGE_QUEUE_CLOSED => "The message queue for the virtualization infrastructure driver has been closed. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.",
            USERMODE_VIRTUALIZATION::ERROR_VID_VIRTUAL_PROCESSOR_LIMIT_EXCEEDED => "Cannot add a virtual processor to the partition because the maximum has been reached.",
            USERMODE_VIRTUALIZATION::ERROR_VID_STOP_PENDING => "Cannot stop the virtual processor immediately because of a pending intercept.",
            USERMODE_VIRTUALIZATION::ERROR_VID_INVALID_PROCESSOR_STATE => "Invalid state for the virtual processor. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.",
            USERMODE_VIRTUALIZATION::ERROR_VID_EXCEEDED_KM_CONTEXT_COUNT_LIMIT => "The maximum number of kernel mode clients for the virtualization infrastructure driver has been reached. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.",
            USERMODE_VIRTUALIZATION::ERROR_VID_KM_INTERFACE_ALREADY_INITIALIZED => "This kernel mode interface for the virtualization infrastructure driver has already been initialized. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.",
            USERMODE_VIRTUALIZATION::ERROR_VID_MB_PROPERTY_ALREADY_SET_RESET => "Cannot set or reset the memory block property more than once for the virtualization infrastructure driver. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.",
            USERMODE_VIRTUALIZATION::ERROR_VID_MMIO_RANGE_DESTROYED => "The memory mapped I/O for this page range no longer exists. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.",
            USERMODE_VIRTUALIZATION::ERROR_VID_INVALID_CHILD_GPA_PAGE_SET => "The lock or unlock request uses an invalid guest operating system memory address. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.",
            USERMODE_VIRTUALIZATION::ERROR_VID_RESERVE_PAGE_SET_IS_BEING_USED => "Cannot destroy or reuse the reserve page set for the virtualization infrastructure driver because it is in use. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.",
            USERMODE_VIRTUALIZATION::ERROR_VID_RESERVE_PAGE_SET_TOO_SMALL => "The reserve page set for the virtualization infrastructure driver is too small to use in the lock request. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.",
            USERMODE_VIRTUALIZATION::ERROR_VID_MBP_ALREADY_LOCKED_USING_RESERVED_PAGE => "Cannot lock or map the memory block page for the virtualization infrastructure driver because it has already been locked using a reserve page set page. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.",
            USERMODE_VIRTUALIZATION::ERROR_VID_MBP_COUNT_EXCEEDED_LIMIT => "Cannot create the memory block for the virtualization infrastructure driver because the requested number of pages exceeded the limit. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.",
            USERMODE_VIRTUALIZATION::ERROR_VID_SAVED_STATE_CORRUPT => "Cannot restore this virtual machine because the saved state data cannot be read. Delete the saved state data and then try to start the virtual machine.",
            USERMODE_VIRTUALIZATION::ERROR_VID_SAVED_STATE_UNRECOGNIZED_ITEM => "Cannot restore this virtual machine because an item read from the saved state data is not recognized. Delete the saved state data and then try to start the virtual machine.",
            USERMODE_VIRTUALIZATION::ERROR_VID_SAVED_STATE_INCOMPATIBLE => "Cannot restore this virtual machine to the saved state because of hypervisor incompatibility. Delete the saved state data and then try to start the virtual machine.",
            USERMODE_VIRTUALIZATION::ERROR_VID_VTL_ACCESS_DENIED => "The specified VTL does not have the permission to access the resource.",
            USERMODE_VIRTUALIZATION::ERROR_VID_INSUFFICIENT_RESOURCES_RESERVE => "Failed to allocate backing memory due to insufficient memory resources.",
            USERMODE_VIRTUALIZATION::ERROR_VID_INSUFFICIENT_RESOURCES_PHYSICAL_BUFFER => "Failed to allocate memory for the physical buffer used to back certain internal structures.",
            USERMODE_VIRTUALIZATION::ERROR_VID_INSUFFICIENT_RESOURCES_HV_DEPOSIT => "Failed to allocate memory to be deposited in the hypervisor.",
            USERMODE_VIRTUALIZATION::ERROR_VID_MEMORY_TYPE_NOT_SUPPORTED => "Memory type not supported for requested operation.",
            USERMODE_VIRTUALIZATION::ERROR_VID_INSUFFICIENT_RESOURCES_WITHDRAW => "Failed to withdraw memory.",
            USERMODE_VIRTUALIZATION::ERROR_VID_PROCESS_ALREADY_SET => "The process has already been set.",
            USERMODE_VIRTUALIZATION::ERROR_VMCOMPUTE_TERMINATED_DURING_START => "The virtual machine or container exited unexpectedly while starting.",
            USERMODE_VIRTUALIZATION::ERROR_VMCOMPUTE_IMAGE_MISMATCH => "The container operating system does not match the host operating system.",
            USERMODE_VIRTUALIZATION::ERROR_VMCOMPUTE_HYPERV_NOT_INSTALLED => "The virtual machine could not be started because a required feature is not installed.",
            USERMODE_VIRTUALIZATION::ERROR_VMCOMPUTE_OPERATION_PENDING => "The call to start an asynchronous operation succeeded and the operation is performed in the background.",
            USERMODE_VIRTUALIZATION::ERROR_VMCOMPUTE_TOO_MANY_NOTIFICATIONS => "The supported number of notification callbacks has been exceeded.",
            USERMODE_VIRTUALIZATION::ERROR_VMCOMPUTE_INVALID_STATE => "The requested virtual machine or container operation is not valid in the current state.",
            USERMODE_VIRTUALIZATION::ERROR_VMCOMPUTE_UNEXPECTED_EXIT => "The virtual machine or container exited unexpectedly.",
            USERMODE_VIRTUALIZATION::ERROR_VMCOMPUTE_TERMINATED => "The virtual machine or container was forcefully exited.",
            USERMODE_VIRTUALIZATION::ERROR_VMCOMPUTE_CONNECT_FAILED => "A connection could not be established with the container or virtual machine.",
            USERMODE_VIRTUALIZATION::ERROR_VMCOMPUTE_TIMEOUT => "The operation timed out because a response was not received from the virtual machine or container.",
            USERMODE_VIRTUALIZATION::ERROR_VMCOMPUTE_CONNECTION_CLOSED => "The connection with the virtual machine or container was closed.",
            USERMODE_VIRTUALIZATION::ERROR_VMCOMPUTE_UNKNOWN_MESSAGE => "An unknown internal message was received by the virtual machine or container.",
            USERMODE_VIRTUALIZATION::ERROR_VMCOMPUTE_UNSUPPORTED_PROTOCOL_VERSION => "The virtual machine or container does not support an available version of the communication protocol with the host.",
            USERMODE_VIRTUALIZATION::ERROR_VMCOMPUTE_INVALID_JSON => "The virtual machine or container JSON document is invalid.",
            USERMODE_VIRTUALIZATION::ERROR_VMCOMPUTE_SYSTEM_NOT_FOUND => "A virtual machine or container with the specified identifier does not exist.",
            USERMODE_VIRTUALIZATION::ERROR_VMCOMPUTE_SYSTEM_ALREADY_EXISTS => "A virtual machine or container with the specified identifier already exists.",
            USERMODE_VIRTUALIZATION::ERROR_VMCOMPUTE_SYSTEM_ALREADY_STOPPED => "The virtual machine or container with the specified identifier is not running.",
            USERMODE_VIRTUALIZATION::ERROR_VMCOMPUTE_PROTOCOL_ERROR => "A communication protocol error has occurred between the virtual machine or container and the host.",
            USERMODE_VIRTUALIZATION::ERROR_VMCOMPUTE_INVALID_LAYER => "The container image contains a layer with an unrecognized format.",
            USERMODE_VIRTUALIZATION::ERROR_VMCOMPUTE_WINDOWS_INSIDER_REQUIRED => "To use this container image, you must join the Windows Insider Program. Please see https://go.microsoft.com/fwlink/?linkid=850659 for more information.",
            USERMODE_VIRTUALIZATION::ERROR_VNET_VIRTUAL_SWITCH_NAME_NOT_FOUND => "A virtual switch with the given name was not found.",
            USERMODE_VIRTUALIZATION::ERROR_VSMB_SAVED_STATE_FILE_NOT_FOUND => "Cannot restore this virtual machine because a file read from the vSMB saved state data could not be found. Delete the saved state data and then try to start the virtual machine.",
            USERMODE_VIRTUALIZATION::ERROR_VSMB_SAVED_STATE_CORRUPT => "Cannot restore this virtual machine because the vSMB saved state data cannot be read. Delete the saved state data and then try to start the virtual machine.",
            USERMODE_VIRTUALIZATION::VM_SAVED_STATE_DUMP_E_PARTITION_STATE_NOT_FOUND => "Partition state blob not found. Make sure the virtual machine is saved for this content to be included in the saved state file(s).",
            USERMODE_VIRTUALIZATION::VM_SAVED_STATE_DUMP_E_GUEST_MEMORY_NOT_FOUND => "Guest memory not found. Make sure the virtual machine is saved for this content to be included in the saved state file(s).",
            USERMODE_VIRTUALIZATION::VM_SAVED_STATE_DUMP_E_NO_VP_FOUND_IN_PARTITION_STATE => "No virtual processor information found in the saved partition blob. Make sure the virtual machine is saved successfully for this content to be included in the partition state.",
            USERMODE_VIRTUALIZATION::VM_SAVED_STATE_DUMP_E_NESTED_VIRTUALIZATION_NOT_SUPPORTED => "A virtual processor has been detected to have nested virtualization enabled. Nested Virtualization is not supported yet by VmSavedStateDumpProvider.",
            USERMODE_VIRTUALIZATION::VM_SAVED_STATE_DUMP_E_WINDOWS_KERNEL_IMAGE_NOT_FOUND => "The Windows kernel image address could not be found in the virtual machine saved state.",
            USERMODE_VIRTUALIZATION::VM_SAVED_STATE_DUMP_E_VA_NOT_MAPPED => "The given virtual address is not mapped to a physical address.",
            USERMODE_VIRTUALIZATION::VM_SAVED_STATE_DUMP_E_INVALID_VP_STATE => "The virtual processor is not in the correct state for the operation.",
            USERMODE_VIRTUALIZATION::VM_SAVED_STATE_DUMP_E_VP_VTL_NOT_ENABLED => "The active virtual trust level is not enabled on the specified virtual processor.",
            USERMODE_VIRTUALIZATION::ERROR_DM_OPERATION_LIMIT_EXCEEDED => "The attempted DM / resize operation exceeds the supported size.",
        }
    }
}
