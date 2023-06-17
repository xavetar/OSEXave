# Copyright 2023 Stanislav Mikhailov (xavetar)
#
# Licensed under the Creative Commons Zero v1.0 Universal (CC0) License.
# You may obtain a copy of the License at
#
#     http://creativecommons.org/publicdomain/zero/1.0/
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the CC0 license is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

import os
import csv

input_file = "HRESULT.csv"
output_folder = "error_enums"

header = '''/*
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
 */'''

facility_types = {
    0: "NULL",
    1: "RPC",
    2: "DISPATCH",
    3: "STORAGE",
    4: "ITF",
    5: "MCA",
    6: "CODCLASS",
    7: "WIN32",
    8: "WINDOWS",
    9: "SECURITY_SSPI",
    10: "CONTROL",
    11: "CERT",
    12: "INTERNET",
    13: "MEDIASERVER",
    14: "MSMQ",
    15: "SETUPAPI",
    16: "SCARD",
    17: "COMPLUS",
    18: "AAF",
    19: "URT",
    20: "ACS",
    21: "DPLAY",
    22: "UMI",
    23: "SXS",
    24: "WINDOWS_CE",
    25: "HTTP",
    26: "USERMODE_COMMONLOG",
    27: "WER",
    28: "USERMODE_FILTER_MANAGER",
    29: "MONITOR",
    30: "GRAPHICS_KERNEL",
    31: "USERMODE_FILTER_MANAGER",
    32: "BACKGROUNDCOPY",
    33: "CONFIGURATION",
    34: "STATE_MANAGEMENT",
    35: "METADIRECTORY",
    36: "WINDOWSUPDATE",
    37: "DIRECTORYSERVICE",
    38: "GRAPHICS",
    39: "SHELL_NAP",
    40: "TPM_SERVICES",
    41: "TPM_SOFTWARE",
    42: "UI",
    43: "XAML",
    44: "ACTION_QUEUE",
    48: "WINDOWS_SETUP_PLA",
    49: "FVE",
    50: "FWP",
    51: "WINRM",
    52: "NDIS",
    53: "USERMODE_HYPERVISOR",
    54: "CMI",
    55: "USERMODE_VIRTUALIZATION",
    56: "USERMODE_VOLMGR",
    57: "BCD",
    58: "USERMODE_VHD",
    59: "USERMODE_HNS",
    60: "SDIAG",
    61: "WEBSERVICES_WINPE",
    62: "WPN",
    63: "WINDOWS_STORE",
    64: "INPUT",
    65: "QUIC",
    66: "EAP",
    67: "SECUREBOOT",
    68: "AUDIO_KERNEL",
    69: "VSM",
    70: "IORING",
    80: "WINDOWS_DEFENDER",
    81: "OPC",
    82: "XPS",
    83: "RAS",
    84: "MBN_POWERSHELL",
    85: "EAS",
    92: "SHARED_VHDX",
    93: "SMB",
    94: "XVS",
    98: "P2P_INT",
    99: "P2P",
    100: "DAF",
    101: "BLUETOOTH_ATT",
    102: "AUDIO",
    103: "STATEREPOSITORY",
    109: "VISUALCPP",
    112: "SCRIPT",
    113: "PARSE",
    120: "BLB",
    121: "BLB_CLI",
    122: "WSBAPP",
    128: "BLBUI",
    129: "USN",
    130: "USERMODE_VOLSNAP",
    131: "TIERING",
    133: "WSB_ONLINE",
    134: "ONLINE_ID",
    135: "DEVICE_UPDATE_AGENT",
    136: "DRVSERVICING",
    153: "DLS",
    160: "SOS",
    170: "IMAPI2",
    173: "OCP_UPDATE_AGENT",
    176: "DEBUGGERS",
    208: "DELIVERY_OPTIMIZATION",
    231: "USERMODE_SPACES",
    232: "SECURITY_CORE",
    232: "USER_MODE_SECURITY_CORE",
    233: "SYSTEM_INTEGRITY",
    234: "USERMODE_LICENSING",
    235: "PLATFORM_MANIFEST",
    236: "APP_EXEC",
    237: "MAXIMUM_VALUE",
    238: "RTC_INTERFACE",
    239: "SIP_STATUS_CODE",
    240: "PINT_STATUS_CODE",
    256: "SPP_RESTORE_DMSERVER",
    257: "WDS_SERVER",
    258: "WDS_IMAGING",
    259: "WDS_MANAGEMENT",
    260: "WDS_UTIL",
    261: "WDS_BINLSVC",
    263: "WDS_PXE",
    264: "WDS_TFTP",
    272: "WDS_TM",
    278: "WDS_DP",
    289: "WDS_MC_SERVER",
    290: "WDS_MC_CLIENT",
    293: "WDS_CONTENT_PROVIDER",
    296: "HSP_SERVICES",
    297: "HSP_SOFTWARE",
    305: "LINGUISTIC_SERVICES",
    885: "WEB",
    886: "WEB_SOCKET",
    1094: "AUDIOSTREAMING",
    1490: "TTD",
    1536: "ACCELERATOR",
    1793: "MOBILE",
    1967: "SQLITE",
    1968: "SERVICE_FABRIC",
    1989: "UTC",
    1996: "WMAAECMA",
    2047: "BACKUP",
    2048: "NTDSB",
    2049: "WEP",
    2050: "SYNCENGINE",
    2067: "SB",
    2085: "RDDB",
    2114: "EAP_MESSAGE",
    2166: "DIRECT_DRAW",
    2168: "DIRECTMUSIC",
    2169: "DIRECT3D10",
    2170: "DXGI",
    2171: "DXGI_DDI",
    2172: "DIRECT3D11",
    2173: "DIRECT3D11_DEBUG",
    2174: "DIRECT3D12",
    2175: "DIRECT3D12_DEBUG",
    2176: "DXCORE",
    2177: "PRESENTATION",
    2184: "LEAP",
    2185: "AUDCLNT",
    2192: "WINML",
    2198: "XAUDIO2",
    2199: "XAPO",
    2200: "WINCODEC_DWRITE_DWM",
    2201: "DIRECT2D",
    2201: "D2D",
    2304: "DEFRAG",
    2305: "USERMODE_SDBUS",
    2306: "JSCRIPT",
    2339: "XBOX",
    2340: "GAME",
    2457: "WPC",
    2561: "PIDGENX",
    2748: "PIX",
    2759: "XACTENGINE",
    3527: "JsDEBUG",
    3562: "DLT"
}

if not os.path.exists(output_folder):
    os.makedirs(output_folder)

# Read CSV data
errors = []
with open(input_file, 'r') as file:
    reader = csv.reader(file, delimiter=';', quotechar='"')
    next(reader)  # Pass Header
    for row in reader:
        _, code, name, message = row
        errors.append((name, code, message))

# Get Facility types from codes
facilities = set()
for error in errors:
    _, code, _ = error
    facility = (int(code, 16) >> 16) & 0x0FFF
    facilities.add(facility)

# Generate code and save to separate files
for facility in facilities:
    # Get name from code Facility
    facility_name = facility_types.get(facility)

    rust_code = header + "\n\n"
    
    # Warning message
    rust_code += "// It is strictly forbidden to use the from_code method, respected Microsoft decided to define\n"
    rust_code += "// duplicate codes, but by defining different constant names, therefore, when using these codes,\n"
    rust_code += "// it is mandatory to use through from_name, and not from_code. Otherwise, it may cause undefined\n"
    rust_code += "// behavior or an unknown exception. Because one code corresponds to several constants.\n\n"
    
    # Imports
    rust_code += "use super::{RawError};\n\n"
    
    rust_code += "#[derive(Clone, Debug, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]\n"
    
    # Generate Enum
    rust_code += f"pub enum {facility_name} {{\n"
    for error in errors:
        name, code, _ = error
        error_facility = (int(code, 16) >> 16) & 0x0FFF
        if error_facility == facility:
            rust_code += f"    {name},\n"
    rust_code += "}\n\n"

    # Generate implementation
    rust_code += f"impl {facility_name} {{\n"
    
    # Generate method code
    init_code = False
    for error in errors:
        name, code, message = error
        error_facility = (int(code, 16) >> 16) & 0x0FFF
        if error_facility == facility:
            if init_code == False:
                # Генерация метода code
                rust_code += f"    pub fn code(&self) -> u32 {{\n"
                rust_code += "        return match self {\n"
                init_code = True
            rust_code += f"            {facility_name}::{name} => {code} as u32,\n"
    rust_code += "        }\n"
    rust_code += "    }\n\n"
    
    # Generate method error
    init_error = False
    for error in errors:
        name, code, message = error
        error_facility = (int(code, 16) >> 16) & 0x0FFF
        if error_facility == facility:
            if init_error == False:
                # Генерация метода error
                rust_code += f"    pub fn error(&self) -> RawError {{\n"
                rust_code += "        return match self {\n"
                init_error = True
            rust_code += f"            {facility_name}::{name} => RawError::Kind({facility_name}::{name}),\n"
    rust_code += "        }\n"
    rust_code += "    }\n\n"
    
    # Generate method description
    init_description = False
    for error in errors:
        name, code, message = error
        error_facility = (int(code, 16) >> 16) & 0x0FFF
#        print(f"{error_facility}, {facility}")
        if error_facility == facility:
            if init_description == False:
                # Генерация метода description
                rust_code += f"    pub fn description(&self) -> &'static str {{\n"
                rust_code += "        return match self {\n"
                init_description = True
            rust_code += f"            {facility_name}::{name} => \"{message}\",\n"
    rust_code += "        }\n"
    rust_code += "    }\n\n"
    
    # Generate method from_name
    init_from_name = False
    for error in errors:
        name, code, message = error
        error_facility = (int(code, 16) >> 16) & 0x0FFF
        if error_facility == facility:
            if init_from_name == False:
                # Генерация метода from_name
                rust_code += f"    pub fn from_name(name: &str) -> {facility_name} {{\n"
                rust_code += "        return match name {\n"
                init_from_name = True
            rust_code += f"            \"{name}\" => {facility_name}::{name},\n"
    rust_code += "        }\n"
    rust_code += "    }\n"
    
    # Generate method from_code
    init_from_code = False
    for error in errors:
        name, code, message = error
        error_facility = (int(code, 16) >> 16) & 0x0FFF
        if error_facility == facility:
            if init_from_code == False:
                rust_code += f"    pub fn from_code(code: u32) -> {facility_name} {{\n"
                rust_code += "        return match code {\n"
                init_from_code = True
            rust_code += f"            {code} => {facility_name}::{name},\n"
    rust_code += "        }\n"
    rust_code += "    }\n"
    
    # End of file
    rust_code += "}\n"

    filename = f"{facility_name.lower()}.rs"
    output_file = os.path.join(output_folder, filename)
    with open(output_file, 'w') as file:
        file.write(rust_code)

    print(f"{facility_name} save to: {output_file}")
