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

pub enum WDS_MC_SERVER {
    EVT_WDSMCS_S_PARAMETERS_READ = 0x41210200,
    EVT_WDSMCS_W_CP_DLL_LOAD_FAILED_NOT_CRITICAL = 0x8121025A,
    WDSMCS_E_SESSION_SHUTDOWN_IN_PROGRESS = 0xC1210100,
    WDSMCS_E_REQCALLBACKS_NOT_REG = 0xC1210101,
    WDSMCS_E_INCOMPATIBLE_VERSION = 0xC1210102,
    WDSMCS_E_CONTENT_NOT_FOUND = 0xC1210103,
    WDSMCS_E_CLIENT_NOT_FOUND = 0xC1210104,
    WDSMCS_E_NAMESPACE_NOT_FOUND = 0xC1210105,
    WDSMCS_E_CONTENT_PROVIDER_NOT_FOUND = 0xC1210106,
    WDSMCS_E_NAMESPACE_ALREADY_EXISTS = 0xC1210107,
    WDSMCS_E_NAMESPACE_SHUTDOWN_IN_PROGRESS = 0xC1210108,
    WDSMCS_E_NAMESPACE_ALREADY_STARTED = 0xC1210109,
    WDSMCS_E_NS_START_FAILED_NO_CLIENTS = 0xC121010A,
    WDSMCS_E_START_TIME_IN_PAST = 0xC121010B,
    WDSMCS_E_PACKET_NOT_HASHED = 0xC121010C,
    WDSMCS_E_PACKET_NOT_SIGNED = 0xC121010D,
    WDSMCS_E_PACKET_HAS_SECURITY = 0xC121010E,
    WDSMCS_E_PACKET_NOT_CHECKSUMED = 0xC121010F,
    WDSMCS_E_CLIENT_DOESNOT_SUPPORT_SECURITY_MODE = 0xC1210110,
    EVT_WDSMCS_E_PARAMETERS_READ_FAILED = 0xC1210201,
    EVT_WDSMCS_E_DUPLICATE_MULTICAST_ADDR = 0xC1210202,
    EVT_WDSMCS_E_NON_WDS_DUPLICATE_MULTICAST_ADDR = 0xC1210203,
    EVT_WDSMCS_E_CP_DLL_LOAD_FAILED = 0xC1210250,
    EVT_WDSMCS_E_CP_INIT_FUNC_MISSING = 0xC1210251,
    EVT_WDSMCS_E_CP_INIT_FUNC_FAILED = 0xC1210252,
    EVT_WDSMCS_E_CP_INCOMPATIBLE_SERVER_VERSION = 0xC1210253,
    EVT_WDSMCS_E_CP_CALLBACKS_NOT_REG = 0xC1210254,
    EVT_WDSMCS_E_CP_SHUTDOWN_FUNC_FAILED = 0xC1210255,
    EVT_WDSMCS_E_CP_MEMORY_LEAK = 0xC1210256,
    EVT_WDSMCS_E_CP_OPEN_INSTANCE_FAILED = 0xC1210257,
    EVT_WDSMCS_E_CP_CLOSE_INSTANCE_FAILED = 0xC1210258,
    EVT_WDSMCS_E_CP_OPEN_CONTENT_FAILED = 0xC1210259,
    EVT_WDSMCS_E_CP_DLL_LOAD_FAILED_CRITICAL = 0xC121025B,
    EVT_WDSMCS_E_NSREG_START_TIME_IN_PAST = 0xC1210300,
    EVT_WDSMCS_E_NSREG_CONTENT_PROVIDER_NOT_REG = 0xC1210301,
    EVT_WDSMCS_E_NSREG_NAMESPACE_EXISTS = 0xC1210302,
    EVT_WDSMCS_E_NSREG_FAILURE = 0xC1210303,
}

impl WDS_MC_SERVER {
    pub fn description(&self) -> &'static str {
        match self {
            WDS_MC_SERVER::EVT_WDSMCS_S_PARAMETERS_READ => "The Windows Deployment Multicast server successfully read its configuration settings. The Windows Deployment Multicast server will now process incoming client requests.",
            WDS_MC_SERVER::EVT_WDSMCS_W_CP_DLL_LOAD_FAILED_NOT_CRITICAL => "The Content Provider %1 failed to initialize. The provider is marked as non-critical. WDS Multicast server will continue to start.%n %n Error Information: %2 %n.",
            WDS_MC_SERVER::WDSMCS_E_SESSION_SHUTDOWN_IN_PROGRESS => "The request for content was rejected because shutdown is in progress for specified session.",
            WDS_MC_SERVER::WDSMCS_E_REQCALLBACKS_NOT_REG => "The Content Provider did not register required callbacks.",
            WDS_MC_SERVER::WDSMCS_E_INCOMPATIBLE_VERSION => "The supported version reported by Content Provider is incompatible.",
            WDS_MC_SERVER::WDSMCS_E_CONTENT_NOT_FOUND => "The specified content was not found.",
            WDS_MC_SERVER::WDSMCS_E_CLIENT_NOT_FOUND => "The specified client was not found.",
            WDS_MC_SERVER::WDSMCS_E_NAMESPACE_NOT_FOUND => "The specified namespace was not found.",
            WDS_MC_SERVER::WDSMCS_E_CONTENT_PROVIDER_NOT_FOUND => "The specified content provider was not found.",
            WDS_MC_SERVER::WDSMCS_E_NAMESPACE_ALREADY_EXISTS => "The specified namespace already exists.",
            WDS_MC_SERVER::WDSMCS_E_NAMESPACE_SHUTDOWN_IN_PROGRESS => "Namespace is in pending close state.",
            WDS_MC_SERVER::WDSMCS_E_NAMESPACE_ALREADY_STARTED => "The namespace has already started.",
            WDS_MC_SERVER::WDSMCS_E_NS_START_FAILED_NO_CLIENTS => "The namespace cannot be started because there are no clients in the namespace.",
            WDS_MC_SERVER::WDSMCS_E_START_TIME_IN_PAST => "The specified start time is in the past.",
            WDS_MC_SERVER::WDSMCS_E_PACKET_NOT_HASHED => "A packet was received without a hash.",
            WDS_MC_SERVER::WDSMCS_E_PACKET_NOT_SIGNED => "A packet was received without a signature.",
            WDS_MC_SERVER::WDSMCS_E_PACKET_HAS_SECURITY => "A signed or hashed packet was received.",
            WDS_MC_SERVER::WDSMCS_E_PACKET_NOT_CHECKSUMED => "A packet was received without checksum.",
            WDS_MC_SERVER::WDSMCS_E_CLIENT_DOESNOT_SUPPORT_SECURITY_MODE => "The client does not support the required security mode.",
            WDS_MC_SERVER::EVT_WDSMCS_E_PARAMETERS_READ_FAILED => "An error occurred while trying to read the configuration information. The Windows Deployment Multicast server will not process incoming client requests. %n %n Error Information: %1 %n.",
            WDS_MC_SERVER::EVT_WDSMCS_E_DUPLICATE_MULTICAST_ADDR => "The multicast IP address %1 is being used by another Windows Deployment Services server, which has IP %2. Use the Windows Deployment Services management tools to configure your multicast IP address range to avoid this multicast IP address. If you allow this overlap to continue, your network usage will be increased.",
            WDS_MC_SERVER::EVT_WDSMCS_E_NON_WDS_DUPLICATE_MULTICAST_ADDR => "The multicast IP address %1 is being used by another multicast server, which has IP %2. Use the Windows Deployment Services management tools to configure your multicast IP address range to avoid this multicast IP address. If you allow this overlap to continue, your network usage will be increased.",
            WDS_MC_SERVER::EVT_WDSMCS_E_CP_DLL_LOAD_FAILED => "An error occurred while trying to load the module %1 for Content Provider %2. %n %n Error Information: %3 %n.",
            WDS_MC_SERVER::EVT_WDSMCS_E_CP_INIT_FUNC_MISSING => "The module %1 for Content Provider %2 does not export the initialization function %3. %n %n Error Information: %4 %n.",
            WDS_MC_SERVER::EVT_WDSMCS_E_CP_INIT_FUNC_FAILED => "The Content Provider %1 loaded from %2 failed to initialize. %n %n Error Information: %3 %n.",
            WDS_MC_SERVER::EVT_WDSMCS_E_CP_INCOMPATIBLE_SERVER_VERSION => "The Content Provider %1 is incompatible with this version of Windows Deployment Multicast Server. %n.",
            WDS_MC_SERVER::EVT_WDSMCS_E_CP_CALLBACKS_NOT_REG => "The Content Provider %1 did not register required callbacks during its initialization. %n.",
            WDS_MC_SERVER::EVT_WDSMCS_E_CP_SHUTDOWN_FUNC_FAILED => "The shutdown function for Content Provider %1 failed. %n %n Error Information: %2 %n.",
            WDS_MC_SERVER::EVT_WDSMCS_E_CP_MEMORY_LEAK => "The Content Provider %1 did not release %2 memory allocation(s) after its shutdown function has successfully returned.",
            WDS_MC_SERVER::EVT_WDSMCS_E_CP_OPEN_INSTANCE_FAILED => "The Content Provider %1 returned an error while trying to open a new instance. %n %n Configuration: %2 %n Error Information: %3 %n.",
            WDS_MC_SERVER::EVT_WDSMCS_E_CP_CLOSE_INSTANCE_FAILED => "The Content Provider %1 returned an error while trying to close an instance. %n %n Error Information: %2 %n.",
            WDS_MC_SERVER::EVT_WDSMCS_E_CP_OPEN_CONTENT_FAILED => "The Content Provider %1 returned an error while trying to open content %2. %n %n Error Information: %3 %n.",
            WDS_MC_SERVER::EVT_WDSMCS_E_CP_DLL_LOAD_FAILED_CRITICAL => "The Content Provider %1 failed to initialize. The provider is marked as critical. WDS Multicast server will fail to start.%n %n Error Information: %2 %n.",
            WDS_MC_SERVER::EVT_WDSMCS_E_NSREG_START_TIME_IN_PAST => "The ScheduledCast namespace %1 using content provider %2 could not be registered as it starts in the past. The namespace has been removed from the namespace store. %n %n Error Information: %3 %n.",
            WDS_MC_SERVER::EVT_WDSMCS_E_NSREG_CONTENT_PROVIDER_NOT_REG => "The namespace %1 using content provider %2 could not be registered as the content provider is not registered. The namespace has been removed from the namespace store. %n %n Error Information: %3 %n.",
            WDS_MC_SERVER::EVT_WDSMCS_E_NSREG_NAMESPACE_EXISTS => "The namespace %1 using content provider %2 could not be registered as a namespace by that name already exists. The namespace has been removed from the namespace store. %n %n Error Information: %3 %n.",
            WDS_MC_SERVER::EVT_WDSMCS_E_NSREG_FAILURE => "The namespace %1 using content provider %2 could not be registered due to an unknown error. %n %n Error Information: %3 %n.",
        }
    }
}
