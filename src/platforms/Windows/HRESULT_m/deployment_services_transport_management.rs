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

pub enum DEPLOYMENT_SERVICES_TRANSPORT_MANAGEMENT {
    WDSTPTMGMT_E_INVALID_PROPERTY = 0xC1100100,
    WDSTPTMGMT_E_INVALID_OPERATION = 0xC1100101,
    WDSTPTMGMT_E_INVALID_CLASS = 0xC1100102,
    WDSTPTMGMT_E_CONTENT_PROVIDER_ALREADY_REGISTERED = 0xC1100103,
    WDSTPTMGMT_E_CONTENT_PROVIDER_NOT_REGISTERED = 0xC1100104,
    WDSTPTMGMT_E_INVALID_CONTENT_PROVIDER_NAME = 0xC1100105,
    WDSTPTMGMT_E_TRANSPORT_SERVER_ROLE_NOT_CONFIGURED = 0xC1100106,
    WDSTPTMGMT_E_NAMESPACE_ALREADY_REGISTERED = 0xC1100107,
    WDSTPTMGMT_E_NAMESPACE_NOT_REGISTERED = 0xC1100108,
    WDSTPTMGMT_E_CANNOT_REINITIALIZE_OBJECT = 0xC1100109,
    WDSTPTMGMT_E_INVALID_NAMESPACE_NAME = 0xC110010A,
    WDSTPTMGMT_E_INVALID_NAMESPACE_DATA = 0xC110010B,
    WDSTPTMGMT_E_NAMESPACE_READ_ONLY = 0xC110010C,
    WDSTPTMGMT_E_INVALID_NAMESPACE_START_TIME = 0xC110010D,
    WDSTPTMGMT_E_INVALID_DIAGNOSTICS_COMPONENTS = 0xC110010E,
    WDSTPTMGMT_E_CANNOT_REFRESH_DIRTY_OBJECT = 0xC110010F,
    WDSTPTMGMT_E_INVALID_SERVICE_IP_ADDRESS_RANGE = 0xC1100110,
    WDSTPTMGMT_E_INVALID_SERVICE_PORT_RANGE = 0xC1100111,
    WDSTPTMGMT_E_INVALID_NAMESPACE_START_PARAMETERS = 0xC1100112,
    WDSTPTMGMT_E_TRANSPORT_SERVER_UNAVAILABLE = 0xC1100113,
    WDSTPTMGMT_E_NAMESPACE_NOT_ON_SERVER = 0xC1100114,
    WDSTPTMGMT_E_NAMESPACE_REMOVED_FROM_SERVER = 0xC1100115,
    WDSTPTMGMT_E_INVALID_IP_ADDRESS = 0xC1100116,
    WDSTPTMGMT_E_INVALID_IPV4_MULTICAST_ADDRESS = 0xC1100117,
    WDSTPTMGMT_E_INVALID_IPV6_MULTICAST_ADDRESS = 0xC1100118,
    WDSTPTMGMT_E_IPV6_NOT_SUPPORTED = 0xC1100119,
    WDSTPTMGMT_E_INVALID_IPV6_MULTICAST_ADDRESS_SOURCE = 0xC110011A,
    WDSTPTMGMT_E_INVALID_MULTISTREAM_STREAM_COUNT = 0xC110011B,
    WDSTPTMGMT_E_INVALID_AUTO_DISCONNECT_THRESHOLD = 0xC110011C,
    WDSTPTMGMT_E_MULTICAST_SESSION_POLICY_NOT_SUPPORTED = 0xC110011D,
    WDSTPTMGMT_E_INVALID_SLOW_CLIENT_HANDLING_TYPE = 0xC110011E,
    WDSTPTMGMT_E_NETWORK_PROFILES_NOT_SUPPORTED = 0xC110011F,
    WDSTPTMGMT_E_UDP_PORT_POLICY_NOT_SUPPORTED = 0xC1100120,
    WDSTPTMGMT_E_TFTP_MAX_BLOCKSIZE_NOT_SUPPORTED = 0xC1100121,
    WDSTPTMGMT_E_TFTP_VAR_WINDOW_NOT_SUPPORTED = 0xC1100122,
    WDSTPTMGMT_E_INVALID_TFTP_MAX_BLOCKSIZE = 0xC1100123,
}

impl DEPLOYMENT_SERVICES_TRANSPORT_MANAGEMENT {
    pub fn description(&self) -> &'static str {
        match self {
            DEPLOYMENT_SERVICES_TRANSPORT_MANAGEMENT::WDSTPTMGMT_E_INVALID_PROPERTY => "The property is invalid.",
            DEPLOYMENT_SERVICES_TRANSPORT_MANAGEMENT::WDSTPTMGMT_E_INVALID_OPERATION => "The operation is invalid.",
            DEPLOYMENT_SERVICES_TRANSPORT_MANAGEMENT::WDSTPTMGMT_E_INVALID_CLASS => "The interface pointer passed to the method has an invalid underlying class. Only library classes are supported.",
            DEPLOYMENT_SERVICES_TRANSPORT_MANAGEMENT::WDSTPTMGMT_E_CONTENT_PROVIDER_ALREADY_REGISTERED => "A content provider with the specified name is already registered on the server.",
            DEPLOYMENT_SERVICES_TRANSPORT_MANAGEMENT::WDSTPTMGMT_E_CONTENT_PROVIDER_NOT_REGISTERED => "The specified content provider is not registered on the server.",
            DEPLOYMENT_SERVICES_TRANSPORT_MANAGEMENT::WDSTPTMGMT_E_INVALID_CONTENT_PROVIDER_NAME => "The specified content provider name is invalid. The name must be 1 to 255 characters long and cannot contain a backslash (\\) character.",
            DEPLOYMENT_SERVICES_TRANSPORT_MANAGEMENT::WDSTPTMGMT_E_TRANSPORT_SERVER_ROLE_NOT_CONFIGURED => "The Windows Deployment Services Transport Server role service has not been configured on the server.",
            DEPLOYMENT_SERVICES_TRANSPORT_MANAGEMENT::WDSTPTMGMT_E_NAMESPACE_ALREADY_REGISTERED => "The specified namespace is already registered on the server.",
            DEPLOYMENT_SERVICES_TRANSPORT_MANAGEMENT::WDSTPTMGMT_E_NAMESPACE_NOT_REGISTERED => "The specified namespace is not registered on the server.",
            DEPLOYMENT_SERVICES_TRANSPORT_MANAGEMENT::WDSTPTMGMT_E_CANNOT_REINITIALIZE_OBJECT => "The object has already been initialized and cannot be reinitialized.",
            DEPLOYMENT_SERVICES_TRANSPORT_MANAGEMENT::WDSTPTMGMT_E_INVALID_NAMESPACE_NAME => "The specified namespace name is invalid. The name must be 1 to 255 characters long and cannot contain a backslash (\\) character..",
            DEPLOYMENT_SERVICES_TRANSPORT_MANAGEMENT::WDSTPTMGMT_E_INVALID_NAMESPACE_DATA => "The namespace contains invalid or unknown data.",
            DEPLOYMENT_SERVICES_TRANSPORT_MANAGEMENT::WDSTPTMGMT_E_NAMESPACE_READ_ONLY => "The namespace data cannot be modified because it is or has been previously registered on the server.",
            DEPLOYMENT_SERVICES_TRANSPORT_MANAGEMENT::WDSTPTMGMT_E_INVALID_NAMESPACE_START_TIME => "The scheduled start time for the namespace is invalid.",
            DEPLOYMENT_SERVICES_TRANSPORT_MANAGEMENT::WDSTPTMGMT_E_INVALID_DIAGNOSTICS_COMPONENTS => "The specified diagnostics components mask contains invalid or unknown components.",
            DEPLOYMENT_SERVICES_TRANSPORT_MANAGEMENT::WDSTPTMGMT_E_CANNOT_REFRESH_DIRTY_OBJECT => "The object contains unsaved changes. The object data cannot be refreshed until you commit or discard the changes.",
            DEPLOYMENT_SERVICES_TRANSPORT_MANAGEMENT::WDSTPTMGMT_E_INVALID_SERVICE_IP_ADDRESS_RANGE => "The specified IP address range is invalid. The start and end IP addresses must be valid and the start IP address must be less than or equal to the end IP address.",
            DEPLOYMENT_SERVICES_TRANSPORT_MANAGEMENT::WDSTPTMGMT_E_INVALID_SERVICE_PORT_RANGE => "The specified service port range is invalid. The start port must be less than or equal to the end port, and both values must be between 1025 and 65536, inclusive.",
            DEPLOYMENT_SERVICES_TRANSPORT_MANAGEMENT::WDSTPTMGMT_E_INVALID_NAMESPACE_START_PARAMETERS => "The start parameters for the namespace are invalid.",
            DEPLOYMENT_SERVICES_TRANSPORT_MANAGEMENT::WDSTPTMGMT_E_TRANSPORT_SERVER_UNAVAILABLE => "The Windows Deployment Services Transport Server is unavailable.",
            DEPLOYMENT_SERVICES_TRANSPORT_MANAGEMENT::WDSTPTMGMT_E_NAMESPACE_NOT_ON_SERVER => "The specified namespace has never been registered on the server.",
            DEPLOYMENT_SERVICES_TRANSPORT_MANAGEMENT::WDSTPTMGMT_E_NAMESPACE_REMOVED_FROM_SERVER => "The specified namespace has been unregistered and removed from the server.",
            DEPLOYMENT_SERVICES_TRANSPORT_MANAGEMENT::WDSTPTMGMT_E_INVALID_IP_ADDRESS => "The specified IP address is invalid. The IP address must be a well formed value of the correct type.",
            DEPLOYMENT_SERVICES_TRANSPORT_MANAGEMENT::WDSTPTMGMT_E_INVALID_IPV4_MULTICAST_ADDRESS => "The specified IPv4 address is not valid for multicast. IPv4 multicast addresses must be in the range 224.0.1.0 to 239.255.255.255.",
            DEPLOYMENT_SERVICES_TRANSPORT_MANAGEMENT::WDSTPTMGMT_E_INVALID_IPV6_MULTICAST_ADDRESS => "The specified IPv6 address is not valid for multicast. IPv6 multicast addresses must start with FF (for example, FF15::FF).",
            DEPLOYMENT_SERVICES_TRANSPORT_MANAGEMENT::WDSTPTMGMT_E_IPV6_NOT_SUPPORTED => "The Windows Deployment Services Transport Server does not support IPv6.",
            DEPLOYMENT_SERVICES_TRANSPORT_MANAGEMENT::WDSTPTMGMT_E_INVALID_IPV6_MULTICAST_ADDRESS_SOURCE => "The specified IPv6 multicast address source is invalid. Transport Server only supports using IPv6 multicast addresses from a range.",
            DEPLOYMENT_SERVICES_TRANSPORT_MANAGEMENT::WDSTPTMGMT_E_INVALID_MULTISTREAM_STREAM_COUNT => "The specified multistream stream count is invalid. The stream count must be set to either 2 or 3.",
            DEPLOYMENT_SERVICES_TRANSPORT_MANAGEMENT::WDSTPTMGMT_E_INVALID_AUTO_DISCONNECT_THRESHOLD => "The specified AutoDisconnect threshold is invalid. The threshold must be set to a value between 1 KBps and 4194303 KBps, inclusive.",
            DEPLOYMENT_SERVICES_TRANSPORT_MANAGEMENT::WDSTPTMGMT_E_MULTICAST_SESSION_POLICY_NOT_SUPPORTED => "You cannot configure the multicast session policy on this Transport Server. This policy is only supported on Windows Server 2008 R2 and later.",
            DEPLOYMENT_SERVICES_TRANSPORT_MANAGEMENT::WDSTPTMGMT_E_INVALID_SLOW_CLIENT_HANDLING_TYPE => "The specified slow-client handling type is not valid.",
            DEPLOYMENT_SERVICES_TRANSPORT_MANAGEMENT::WDSTPTMGMT_E_NETWORK_PROFILES_NOT_SUPPORTED => "This Windows Deployment Services Transport Server does not support network profiles.",
            DEPLOYMENT_SERVICES_TRANSPORT_MANAGEMENT::WDSTPTMGMT_E_UDP_PORT_POLICY_NOT_SUPPORTED => "The Windows Deployment Services Transport Server does not support UDP port policy.",
            DEPLOYMENT_SERVICES_TRANSPORT_MANAGEMENT::WDSTPTMGMT_E_TFTP_MAX_BLOCKSIZE_NOT_SUPPORTED => "The Windows Deployment Services Transport Server does not support the TFTP maximum block size setting.",
            DEPLOYMENT_SERVICES_TRANSPORT_MANAGEMENT::WDSTPTMGMT_E_TFTP_VAR_WINDOW_NOT_SUPPORTED => "The Windows Deployment Services Transport Server does not support the TFTP variable window extension.",
            DEPLOYMENT_SERVICES_TRANSPORT_MANAGEMENT::WDSTPTMGMT_E_INVALID_TFTP_MAX_BLOCKSIZE => "The specified TFTP maximum block size is invalid. It must be either 0 or between 512 and 65531, inclusive.",
        }
    }
}
