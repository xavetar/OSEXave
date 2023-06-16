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

pub enum SB {
    E_SB_PROV_DELETE_VM_AD_ACCOUNT_FAILED = 0x88130101,
    E_SB_PROV_INVALID_HyperV_HOST_NAME = 0x88130102,
    E_SB_PROV_HyperV_HOST_RPC_FAILED = 0x88130103,
    E_SB_PROV_VM_ALREADY_EXIST = 0x88130104,
    E_SB_PROV_VM_NAME_TOO_LONG = 0x88130105,
    E_SB_PROV_OPERATION_CANCELLED = 0x88130106,
    E_SB_PROV_NO_HyperV_HOST_AVAILABLE = 0x88130107,
    E_SB_PROV_SELECTED_VM_NOT_FOUND = 0x88130108,
    E_SB_PROV_POOL_NOT_FOUND = 0x88130109,
    E_SB_PROV_UNATTEND_XML_NOT_SUPPORTED = 0x8813010A,
    E_SB_PROV_INVALID_UNATTEND_XML = 0x8813010B,
    E_SB_PROV_VM_AD_ACCOUNT_ALREADY_EXISTS = 0x8813010C,
    E_SB_PROV_CREATE_VM_AD_ACCOUNT_FAILED = 0x8813010D,
    E_SB_PROV_INVALID_BASE_VM_PATH = 0x88130200,
    E_SB_PROV_INVALID_CACHE_PATH = 0x88130201,
    E_SB_PROV_INVALID_VM_CREATION_PATH = 0x88130202,
    E_SB_PROV_INVALID_SMB_SHARE_PATH = 0x88130203,
    E_SB_PROV_MASTER_VHD_CACHING_FAILED = 0x88130204,
    E_SB_PROV_CREATE_DIFF_VHD_FAILED = 0x88130205,
    E_SB_PROV_CREATE_CLONE_VHD_FAILED = 0x88130206,
    E_SB_PROV_VM_START_FAIL = 0x88130207,
    E_SB_PROV_VM_TURN_OFF_FAIL = 0x88130208,
    E_SB_PROV_VM_SNAPSHOT_FAIL = 0x88130209,
    E_SB_PROV_VM_BOOT_FAIL = 0x8813020A,
    E_SB_PROV_MIGRATE_STORAGE_FAILED = 0x8813020B,
    E_SB_PROV_IMPORT_FAILED = 0x8813020C,
    E_SB_PROV_CONNECT_VHD_FAILED = 0x8813020D,
    E_SB_PROV_DISCONNECT_VHD_FAILED = 0x8813020E,
    E_SB_PROV_SETUP_VHD_FAILED = 0x8813020F,
    E_SB_PROV_GEN_QUERY_OFFVM_FAILURE = 0x88130210,
    E_SB_PROV_VM_NOT_TURNED_OFF = 0x88130211,
    E_SB_PROV_MULTIPLE_VHD_DISKS = 0x88130212,
    E_SB_PROV_NO_VHD_FOUND = 0x88130213,
    E_SB_PROV_MULTIPLE_WINDOWS_FOLDERS = 0x88130214,
    E_SB_PROV_GOLDVM_NOT_GENERALIZED = 0x88130215,
    E_SB_PROV_GOLDVM_IC_UPGRADE_REQUIRED = 0x88130216,
    E_SB_PROV_GOLDVM_INVALID_EXPORT_SHARE = 0x88130217,
    E_SB_PROV_NO_NETWORK_ADAPTER_ATTACHED = 0x88130218,
    E_SB_PROV_NETWORK_NOT_CONNECTED = 0x88130219,
    E_VM_NEEDS_SCSI_CONTROLLER = 0x8813021A,
    E_SB_PROV_GOLDVM_NOT_ENOUGH_RAM = 0x8813021B,
    E_SB_PROV_SERVER_SKU_UNSUPPORTED = 0x8813021C,
    E_SB_PROV_VM_SYSPREP_SPECIALIZE_INCOMPLETE = 0x8813021D,
    E_SB_PROV_VM_SYSPREP_SPECIALIZE_COMPLETE = 0x8813021E,
    E_SB_PROV_VM_FAILED_DOMAIN_CHECK = 0x8813021F,
    E_SB_PROV_VM_FAILED_GUEST_COMMUNICATION = 0x88130220,
    E_SB_PROV_VM_NOT_DEPLOYABLE_STATE = 0x88130221,
    E_SB_PROV_INVALID_VM_STATE = 0x88130222,
    E_SB_PROV_VM_GENERATION_NOT_SUPPORTED = 0x88130243,
}

impl SB {
    pub fn description(&self) -> &'static str {
        match self {
            SB::E_SB_PROV_DELETE_VM_AD_ACCOUNT_FAILED => "Broker failed to delete M/C account for the VM at Active directory Probable causes: - Broker m/c account does not have required permission on the requested OU - AD DC unreachable Event log gives more detailed info about the failure.",
            SB::E_SB_PROV_INVALID_HyperV_HOST_NAME => "Broker failed to find the specified VMHost name in the list of currently connected VMHosts Probable causes: - Incorrect HyperV host name - specified HyperV host is not joined to VM host - specified HyperV host is not running/unreachable/VMHA service not running Event log gives more detailed info about the failure.",
            SB::E_SB_PROV_HyperV_HOST_RPC_FAILED => "RPC Call from Broker to HyperV host failed Probable causes: - network connectivity with the HyperV host is lost - hypreV host/VMHA service crashed Event log gives more detailed info about the failure.",
            SB::E_SB_PROV_VM_ALREADY_EXIST => "The VM specified in the CREATE pool request already exists in the deployment.",
            SB::E_SB_PROV_VM_NAME_TOO_LONG => "The VM name specified in the CREATE pool request is longer then supported Netbios name.",
            SB::E_SB_PROV_OPERATION_CANCELLED => "The provisioning operation was cancelled because  - provisioning op. for some other VM in the Job request failed and StopOnError was specified. - Job was cancelled by user.",
            SB::E_SB_PROV_NO_HyperV_HOST_AVAILABLE => "No HyperV host is connected to the broker.",
            SB::E_SB_PROV_SELECTED_VM_NOT_FOUND => "The VM specified in the selectedVMs list in the provisioning job is not present in broker Target database.",
            SB::E_SB_PROV_POOL_NOT_FOUND => "The Wmi object (win32_RdPublishedFarm) does not exist for the pool name in the Provisioning request.",
            SB::E_SB_PROV_UNATTEND_XML_NOT_SUPPORTED => "The specified unattend XML is not supported.",
            SB::E_SB_PROV_INVALID_UNATTEND_XML => "The specified unattend XML is not supported.",
            SB::E_SB_PROV_VM_AD_ACCOUNT_ALREADY_EXISTS => "The AD machine account already exists for the specified VM name.",
            SB::E_SB_PROV_CREATE_VM_AD_ACCOUNT_FAILED => "Broker failed to create M/C account for the VM at Active Directory. Probable causes: - Broker m/c account does not have required permission on the requested OU - AD DC unreachable - Machine account already exists in a different OU Event log gives more detailed info about the failure.",
            SB::E_SB_PROV_INVALID_BASE_VM_PATH => "VMHost Agent failed to access the base VM location Probable causes: - Invalid Base VM Location - No read/write permission to VMHA service/VMHA Machine account on Base VM location - if BaseVmLocation is on SMB, then there could N/W connectivity issues Event log gives more detailed info about the failure.",
            SB::E_SB_PROV_INVALID_CACHE_PATH => "VMHost Agent failed to access the Local Gold Cache location Probable causes: - Invalid Local gold cache Location - No read/write permission to VMHA service/VMHA Machine account - Non local gold cache location Event log gives more detailed info about the failure.",
            SB::E_SB_PROV_INVALID_VM_CREATION_PATH => "VMHost Agent failed to access the VM creation path Probable causes: - Invalid Local vm creation path - No read/write permission to VMHA service/VMHA Machine account - Non local VM creation path Event log gives more detailed info about the failure.",
            SB::E_SB_PROV_INVALID_SMB_SHARE_PATH => "VMHost Agent failed to access the SMB Probable causes: - Invalid SMB path - No read/write permission to VMHA service/VMHA Machine account - local Path - Network connectivity issues to SMB share Event log gives more detailed info about the failure.",
            SB::E_SB_PROV_MASTER_VHD_CACHING_FAILED => "VMHost Agent failed to cache the Gold VM from Base VM location to LocalGOldCache Probable causes: - N/w connectivity issues - Disk ran out of space Event log gives more detailed info about the failure.",
            SB::E_SB_PROV_CREATE_DIFF_VHD_FAILED => "VMHost Agent failed to create the diff disk Probable causes: - Gold VHD is inaccessbile - HyperV specific error VMHA and HyperV Event log gives more detailed info about the failure.",
            SB::E_SB_PROV_CREATE_CLONE_VHD_FAILED => "TODO: Applicable when we move to V2 namespace.",
            SB::E_SB_PROV_VM_START_FAIL => "VMHost Agent failed to start the VM Probable causes: - System low on resources (RAM, Procs) - HyperV specific error VMHA and HyperV Event log gives more detailed info about the failure.",
            SB::E_SB_PROV_VM_TURN_OFF_FAIL => "VMHost Agent failed to turn off the VM Probable causes: - Invalid VM state - HyperV specific failure VMHA and HyperV Event log gives more detailed info about the failure.",
            SB::E_SB_PROV_VM_SNAPSHOT_FAIL => "VMHost Agent failed to take a snap shot of the temp pool VM Probable causes: - HyperV specific failure VMHA and HyperV Event log gives more detailed info about the failure.",
            SB::E_SB_PROV_VM_BOOT_FAIL => "VMHost Agent failed to detect BOOT event from VM, VM failed to boot. Probable causes: - invalid unattend file - unexpected failure during sysprep specialization Event log gives more detailed info about the failure.",
            SB::E_SB_PROV_MIGRATE_STORAGE_FAILED => "VMHost Agent failed to migrate the VM storage to SMB Probable causes: - SMB ran out of space - network connectivity issues - HyperV specific failure VMHA and HyperV Event Event log gives more detailed info about the failure.",
            SB::E_SB_PROV_IMPORT_FAILED => "VMHost Agent failed to import from BASE VM Location Probable causes: - Invalid import files in BaseVmLocation - HyperV configuration does not match the host from where gold was exported (e.g. External Virtual N/W switch name) - HyperV server does not have required resources VMHA and HyperV Event log gives more detailed info about the failure.",
            SB::E_SB_PROV_CONNECT_VHD_FAILED => "VMHA service failed to attach the VHD to the VM Probable causes: - IDE controller not present - VM in invalid state VMHA and HyperV Event log gives more detailed info about the failure.",
            SB::E_SB_PROV_DISCONNECT_VHD_FAILED => "VMHost Agent failed to disconnect VHD from VM Probable causes: - Invalid vm state, e.g. VM running - HyperV specific failure VMHA and HyperV Event log gives more detailed info about the failure.",
            SB::E_SB_PROV_SETUP_VHD_FAILED => "VMHost Agent failed to Probable causes: - Failed to mount the VHD - Invalid gold image VMHA and HyperV Event log gives more detailed info about the failure.",
            SB::E_SB_PROV_GEN_QUERY_OFFVM_FAILURE => "VMHost Agent failed to Probable causes: - VM not in turned off state, e.g. VM running VMHA and HyperV Event log gives more detailed info about the failure.",
            SB::E_SB_PROV_VM_NOT_TURNED_OFF => "VMHost Agent failed to Probable causes: - VM not in turned off state, e.g. VM running VMHA and HyperV Event log gives more detailed info about the failure.",
            SB::E_SB_PROV_MULTIPLE_VHD_DISKS => "VMHost Agent failed to Probable causes: - Multiple VHDs are attached to the VM, we support only one VMHA and HyperV Event log gives more detailed info about the failure.",
            SB::E_SB_PROV_NO_VHD_FOUND => "VMHost Agent failed to Probable causes: - No VHD found for this VM VMHA and HyperV Event log gives more detailed info about the failure.",
            SB::E_SB_PROV_MULTIPLE_WINDOWS_FOLDERS => "VMHost Agent failed to Probable causes: - Multiple WINDOWS folders in a single VHD VMHA and HyperV Event log gives more detailed info about the failure.",
            SB::E_SB_PROV_GOLDVM_NOT_GENERALIZED => "VMHost Agent failed to Probable causes: - Gold VM is not in generalized state.",
            SB::E_SB_PROV_GOLDVM_IC_UPGRADE_REQUIRED => "VMHost Agent failed to Probable causes: - Gold VM does not have the latest IC package.",
            SB::E_SB_PROV_GOLDVM_INVALID_EXPORT_SHARE => "VMHost Agent failed to verify gold VM export share Probable causes: - Import operation failed on gold cache during initial validation - VmHost there gold exited and the Vmhost which returned this error are not configured identically - The export share is corrupted.",
            SB::E_SB_PROV_NO_NETWORK_ADAPTER_ATTACHED => "VMHost Agent did not find any network adapter attached to the VM.",
            SB::E_SB_PROV_NETWORK_NOT_CONNECTED => "VMHost Agent did not find the Network connected to the network adapter of the VM.",
            SB::E_VM_NEEDS_SCSI_CONTROLLER => "VMHost Agent failed to add a VM to a Farm, because the VM does not have a SCSI controller. Please configure  a SCSI controller in the gold image of the Farm, re-create the VM and re-try adding it to the Farm.",
            SB::E_SB_PROV_GOLDVM_NOT_ENOUGH_RAM => "VMHost Agent failed to provision VM because there is not enough RAM assigned to Gold VM  If Dynamic Memory is enabled, then the Max RAM must be set to at least 1024MB,  otherwise, Startup RAM must be at least 1024GB.",
            SB::E_SB_PROV_SERVER_SKU_UNSUPPORTED => "ServerSKUs are not supported as the GuestOS.",
            SB::E_SB_PROV_VM_SYSPREP_SPECIALIZE_INCOMPLETE => "Timed out waiting for SysPrep specialize to be complete.",
            SB::E_SB_PROV_VM_SYSPREP_SPECIALIZE_COMPLETE => "Sysprep specialize is complete, not an error in most cases.",
            SB::E_SB_PROV_VM_FAILED_DOMAIN_CHECK => "Failed to successfully detect VM's domain-joinedness.",
            SB::E_SB_PROV_VM_FAILED_GUEST_COMMUNICATION => "Failed to communicate with the guest.",
            SB::E_SB_PROV_VM_NOT_DEPLOYABLE_STATE => "VM is not in fully deployable state.",
            SB::E_SB_PROV_INVALID_VM_STATE => "VM is in an invalid state for provisioning operation.",
            SB::E_SB_PROV_VM_GENERATION_NOT_SUPPORTED => "VM is in an invalid state for provisioning operation.",
        }
    }
}
