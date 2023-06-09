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

// It is strictly forbidden to use the from_code method, respected Microsoft decided to define
// duplicate codes, but by defining different constant names, therefore, when using these codes,
// it is mandatory to use through from_name, and not from_code. Otherwise, it may cause undefined
// behavior or an unknown exception. Because one code corresponds to several constants.

use super::{RawError};

#[derive(Clone, Debug, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum STORAGE {
    STG_S_CONVERTED,
    STG_S_BLOCK,
    STG_S_RETRYNOW,
    STG_S_MONITORING,
    STG_S_MULTIPLEOPENS,
    STG_S_CONSOLIDATIONFAILED,
    STG_S_CANNOTCONSOLIDATE,
    STG_S_POWER_CYCLE_REQUIRED,
    STG_E_INVALIDFUNCTION,
    STG_E_FILENOTFOUND,
    STG_E_PATHNOTFOUND,
    STG_E_TOOMANYOPENFILES,
    STG_E_ACCESSDENIED,
    STG_E_INVALIDHANDLE,
    STG_E_INSUFFICIENTMEMORY,
    STG_E_INVALIDPOINTER,
    STG_E_NOMOREFILES,
    STG_E_DISKISWRITEPROTECTED,
    STG_E_SEEKERROR,
    STG_E_WRITEFAULT,
    STG_E_READFAULT,
    STG_E_SHAREVIOLATION,
    STG_E_LOCKVIOLATION,
    STG_E_FILEALREADYEXISTS,
    STG_E_INVALIDPARAMETER,
    STG_E_MEDIUMFULL,
    STG_E_PROPSETMISMATCHED,
    STG_E_ABNORMALAPIEXIT,
    STG_E_INVALIDHEADER,
    STG_E_INVALIDNAME,
    STG_E_UNKNOWN,
    STG_E_UNIMPLEMENTEDFUNCTION,
    STG_E_INVALIDFLAG,
    STG_E_INUSE,
    STG_E_NOTCURRENT,
    STG_E_REVERTED,
    STG_E_CANTSAVE,
    STG_E_OLDFORMAT,
    STG_E_OLDDLL,
    STG_E_SHAREREQUIRED,
    STG_E_NOTFILEBASEDSTORAGE,
    STG_E_EXTANTMARSHALLINGS,
    STG_E_DOCFILECORRUPT,
    STG_E_BADBASEADDRESS,
    STG_E_DOCFILETOOLARGE,
    STG_E_NOTSIMPLEFORMAT,
    STG_E_INCOMPLETE,
    STG_E_TERMINATED,
    STG_E_FIRMWARE_SLOT_INVALID,
    STG_E_FIRMWARE_IMAGE_INVALID,
    STG_E_DEVICE_UNRESPONSIVE,
    STG_E_STATUS_COPY_PROTECTION_FAILURE,
    STG_E_CSS_AUTHENTICATION_FAILURE,
    STG_E_CSS_KEY_NOT_PRESENT,
    STG_E_CSS_KEY_NOT_ESTABLISHED,
    STG_E_CSS_SCRAMBLED_SECTOR,
    STG_E_CSS_REGION_MISMATCH,
    STG_E_RESETS_EXHAUSTED,
}

impl STORAGE {
    pub fn code(&self) -> u32 {
        return match self {
            STORAGE::STG_S_CONVERTED => 0x00030200 as u32,
            STORAGE::STG_S_BLOCK => 0x00030201 as u32,
            STORAGE::STG_S_RETRYNOW => 0x00030202 as u32,
            STORAGE::STG_S_MONITORING => 0x00030203 as u32,
            STORAGE::STG_S_MULTIPLEOPENS => 0x00030204 as u32,
            STORAGE::STG_S_CONSOLIDATIONFAILED => 0x00030205 as u32,
            STORAGE::STG_S_CANNOTCONSOLIDATE => 0x00030206 as u32,
            STORAGE::STG_S_POWER_CYCLE_REQUIRED => 0x00030207 as u32,
            STORAGE::STG_E_INVALIDFUNCTION => 0x80030001 as u32,
            STORAGE::STG_E_FILENOTFOUND => 0x80030002 as u32,
            STORAGE::STG_E_PATHNOTFOUND => 0x80030003 as u32,
            STORAGE::STG_E_TOOMANYOPENFILES => 0x80030004 as u32,
            STORAGE::STG_E_ACCESSDENIED => 0x80030005 as u32,
            STORAGE::STG_E_INVALIDHANDLE => 0x80030006 as u32,
            STORAGE::STG_E_INSUFFICIENTMEMORY => 0x80030008 as u32,
            STORAGE::STG_E_INVALIDPOINTER => 0x80030009 as u32,
            STORAGE::STG_E_NOMOREFILES => 0x80030012 as u32,
            STORAGE::STG_E_DISKISWRITEPROTECTED => 0x80030013 as u32,
            STORAGE::STG_E_SEEKERROR => 0x80030019 as u32,
            STORAGE::STG_E_WRITEFAULT => 0x8003001D as u32,
            STORAGE::STG_E_READFAULT => 0x8003001E as u32,
            STORAGE::STG_E_SHAREVIOLATION => 0x80030020 as u32,
            STORAGE::STG_E_LOCKVIOLATION => 0x80030021 as u32,
            STORAGE::STG_E_FILEALREADYEXISTS => 0x80030050 as u32,
            STORAGE::STG_E_INVALIDPARAMETER => 0x80030057 as u32,
            STORAGE::STG_E_MEDIUMFULL => 0x80030070 as u32,
            STORAGE::STG_E_PROPSETMISMATCHED => 0x800300F0 as u32,
            STORAGE::STG_E_ABNORMALAPIEXIT => 0x800300FA as u32,
            STORAGE::STG_E_INVALIDHEADER => 0x800300FB as u32,
            STORAGE::STG_E_INVALIDNAME => 0x800300FC as u32,
            STORAGE::STG_E_UNKNOWN => 0x800300FD as u32,
            STORAGE::STG_E_UNIMPLEMENTEDFUNCTION => 0x800300FE as u32,
            STORAGE::STG_E_INVALIDFLAG => 0x800300FF as u32,
            STORAGE::STG_E_INUSE => 0x80030100 as u32,
            STORAGE::STG_E_NOTCURRENT => 0x80030101 as u32,
            STORAGE::STG_E_REVERTED => 0x80030102 as u32,
            STORAGE::STG_E_CANTSAVE => 0x80030103 as u32,
            STORAGE::STG_E_OLDFORMAT => 0x80030104 as u32,
            STORAGE::STG_E_OLDDLL => 0x80030105 as u32,
            STORAGE::STG_E_SHAREREQUIRED => 0x80030106 as u32,
            STORAGE::STG_E_NOTFILEBASEDSTORAGE => 0x80030107 as u32,
            STORAGE::STG_E_EXTANTMARSHALLINGS => 0x80030108 as u32,
            STORAGE::STG_E_DOCFILECORRUPT => 0x80030109 as u32,
            STORAGE::STG_E_BADBASEADDRESS => 0x80030110 as u32,
            STORAGE::STG_E_DOCFILETOOLARGE => 0x80030111 as u32,
            STORAGE::STG_E_NOTSIMPLEFORMAT => 0x80030112 as u32,
            STORAGE::STG_E_INCOMPLETE => 0x80030201 as u32,
            STORAGE::STG_E_TERMINATED => 0x80030202 as u32,
            STORAGE::STG_E_FIRMWARE_SLOT_INVALID => 0x80030208 as u32,
            STORAGE::STG_E_FIRMWARE_IMAGE_INVALID => 0x80030209 as u32,
            STORAGE::STG_E_DEVICE_UNRESPONSIVE => 0x8003020A as u32,
            STORAGE::STG_E_STATUS_COPY_PROTECTION_FAILURE => 0x80030305 as u32,
            STORAGE::STG_E_CSS_AUTHENTICATION_FAILURE => 0x80030306 as u32,
            STORAGE::STG_E_CSS_KEY_NOT_PRESENT => 0x80030307 as u32,
            STORAGE::STG_E_CSS_KEY_NOT_ESTABLISHED => 0x80030308 as u32,
            STORAGE::STG_E_CSS_SCRAMBLED_SECTOR => 0x80030309 as u32,
            STORAGE::STG_E_CSS_REGION_MISMATCH => 0x8003030A as u32,
            STORAGE::STG_E_RESETS_EXHAUSTED => 0x8003030B as u32,
        }
    }

    pub fn error(&self) -> RawError {
        return match self {
            STORAGE::STG_S_CONVERTED => RawError::Kind(STORAGE::STG_S_CONVERTED),
            STORAGE::STG_S_BLOCK => RawError::Kind(STORAGE::STG_S_BLOCK),
            STORAGE::STG_S_RETRYNOW => RawError::Kind(STORAGE::STG_S_RETRYNOW),
            STORAGE::STG_S_MONITORING => RawError::Kind(STORAGE::STG_S_MONITORING),
            STORAGE::STG_S_MULTIPLEOPENS => RawError::Kind(STORAGE::STG_S_MULTIPLEOPENS),
            STORAGE::STG_S_CONSOLIDATIONFAILED => RawError::Kind(STORAGE::STG_S_CONSOLIDATIONFAILED),
            STORAGE::STG_S_CANNOTCONSOLIDATE => RawError::Kind(STORAGE::STG_S_CANNOTCONSOLIDATE),
            STORAGE::STG_S_POWER_CYCLE_REQUIRED => RawError::Kind(STORAGE::STG_S_POWER_CYCLE_REQUIRED),
            STORAGE::STG_E_INVALIDFUNCTION => RawError::Kind(STORAGE::STG_E_INVALIDFUNCTION),
            STORAGE::STG_E_FILENOTFOUND => RawError::Kind(STORAGE::STG_E_FILENOTFOUND),
            STORAGE::STG_E_PATHNOTFOUND => RawError::Kind(STORAGE::STG_E_PATHNOTFOUND),
            STORAGE::STG_E_TOOMANYOPENFILES => RawError::Kind(STORAGE::STG_E_TOOMANYOPENFILES),
            STORAGE::STG_E_ACCESSDENIED => RawError::Kind(STORAGE::STG_E_ACCESSDENIED),
            STORAGE::STG_E_INVALIDHANDLE => RawError::Kind(STORAGE::STG_E_INVALIDHANDLE),
            STORAGE::STG_E_INSUFFICIENTMEMORY => RawError::Kind(STORAGE::STG_E_INSUFFICIENTMEMORY),
            STORAGE::STG_E_INVALIDPOINTER => RawError::Kind(STORAGE::STG_E_INVALIDPOINTER),
            STORAGE::STG_E_NOMOREFILES => RawError::Kind(STORAGE::STG_E_NOMOREFILES),
            STORAGE::STG_E_DISKISWRITEPROTECTED => RawError::Kind(STORAGE::STG_E_DISKISWRITEPROTECTED),
            STORAGE::STG_E_SEEKERROR => RawError::Kind(STORAGE::STG_E_SEEKERROR),
            STORAGE::STG_E_WRITEFAULT => RawError::Kind(STORAGE::STG_E_WRITEFAULT),
            STORAGE::STG_E_READFAULT => RawError::Kind(STORAGE::STG_E_READFAULT),
            STORAGE::STG_E_SHAREVIOLATION => RawError::Kind(STORAGE::STG_E_SHAREVIOLATION),
            STORAGE::STG_E_LOCKVIOLATION => RawError::Kind(STORAGE::STG_E_LOCKVIOLATION),
            STORAGE::STG_E_FILEALREADYEXISTS => RawError::Kind(STORAGE::STG_E_FILEALREADYEXISTS),
            STORAGE::STG_E_INVALIDPARAMETER => RawError::Kind(STORAGE::STG_E_INVALIDPARAMETER),
            STORAGE::STG_E_MEDIUMFULL => RawError::Kind(STORAGE::STG_E_MEDIUMFULL),
            STORAGE::STG_E_PROPSETMISMATCHED => RawError::Kind(STORAGE::STG_E_PROPSETMISMATCHED),
            STORAGE::STG_E_ABNORMALAPIEXIT => RawError::Kind(STORAGE::STG_E_ABNORMALAPIEXIT),
            STORAGE::STG_E_INVALIDHEADER => RawError::Kind(STORAGE::STG_E_INVALIDHEADER),
            STORAGE::STG_E_INVALIDNAME => RawError::Kind(STORAGE::STG_E_INVALIDNAME),
            STORAGE::STG_E_UNKNOWN => RawError::Kind(STORAGE::STG_E_UNKNOWN),
            STORAGE::STG_E_UNIMPLEMENTEDFUNCTION => RawError::Kind(STORAGE::STG_E_UNIMPLEMENTEDFUNCTION),
            STORAGE::STG_E_INVALIDFLAG => RawError::Kind(STORAGE::STG_E_INVALIDFLAG),
            STORAGE::STG_E_INUSE => RawError::Kind(STORAGE::STG_E_INUSE),
            STORAGE::STG_E_NOTCURRENT => RawError::Kind(STORAGE::STG_E_NOTCURRENT),
            STORAGE::STG_E_REVERTED => RawError::Kind(STORAGE::STG_E_REVERTED),
            STORAGE::STG_E_CANTSAVE => RawError::Kind(STORAGE::STG_E_CANTSAVE),
            STORAGE::STG_E_OLDFORMAT => RawError::Kind(STORAGE::STG_E_OLDFORMAT),
            STORAGE::STG_E_OLDDLL => RawError::Kind(STORAGE::STG_E_OLDDLL),
            STORAGE::STG_E_SHAREREQUIRED => RawError::Kind(STORAGE::STG_E_SHAREREQUIRED),
            STORAGE::STG_E_NOTFILEBASEDSTORAGE => RawError::Kind(STORAGE::STG_E_NOTFILEBASEDSTORAGE),
            STORAGE::STG_E_EXTANTMARSHALLINGS => RawError::Kind(STORAGE::STG_E_EXTANTMARSHALLINGS),
            STORAGE::STG_E_DOCFILECORRUPT => RawError::Kind(STORAGE::STG_E_DOCFILECORRUPT),
            STORAGE::STG_E_BADBASEADDRESS => RawError::Kind(STORAGE::STG_E_BADBASEADDRESS),
            STORAGE::STG_E_DOCFILETOOLARGE => RawError::Kind(STORAGE::STG_E_DOCFILETOOLARGE),
            STORAGE::STG_E_NOTSIMPLEFORMAT => RawError::Kind(STORAGE::STG_E_NOTSIMPLEFORMAT),
            STORAGE::STG_E_INCOMPLETE => RawError::Kind(STORAGE::STG_E_INCOMPLETE),
            STORAGE::STG_E_TERMINATED => RawError::Kind(STORAGE::STG_E_TERMINATED),
            STORAGE::STG_E_FIRMWARE_SLOT_INVALID => RawError::Kind(STORAGE::STG_E_FIRMWARE_SLOT_INVALID),
            STORAGE::STG_E_FIRMWARE_IMAGE_INVALID => RawError::Kind(STORAGE::STG_E_FIRMWARE_IMAGE_INVALID),
            STORAGE::STG_E_DEVICE_UNRESPONSIVE => RawError::Kind(STORAGE::STG_E_DEVICE_UNRESPONSIVE),
            STORAGE::STG_E_STATUS_COPY_PROTECTION_FAILURE => RawError::Kind(STORAGE::STG_E_STATUS_COPY_PROTECTION_FAILURE),
            STORAGE::STG_E_CSS_AUTHENTICATION_FAILURE => RawError::Kind(STORAGE::STG_E_CSS_AUTHENTICATION_FAILURE),
            STORAGE::STG_E_CSS_KEY_NOT_PRESENT => RawError::Kind(STORAGE::STG_E_CSS_KEY_NOT_PRESENT),
            STORAGE::STG_E_CSS_KEY_NOT_ESTABLISHED => RawError::Kind(STORAGE::STG_E_CSS_KEY_NOT_ESTABLISHED),
            STORAGE::STG_E_CSS_SCRAMBLED_SECTOR => RawError::Kind(STORAGE::STG_E_CSS_SCRAMBLED_SECTOR),
            STORAGE::STG_E_CSS_REGION_MISMATCH => RawError::Kind(STORAGE::STG_E_CSS_REGION_MISMATCH),
            STORAGE::STG_E_RESETS_EXHAUSTED => RawError::Kind(STORAGE::STG_E_RESETS_EXHAUSTED),
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            STORAGE::STG_S_CONVERTED => "The underlying file was converted to compound file format.",
            STORAGE::STG_S_BLOCK => "The storage operation should block until more data is available.",
            STORAGE::STG_S_RETRYNOW => "The storage operation should retry immediately.",
            STORAGE::STG_S_MONITORING => "The notified event sink will not influence the storage operation.",
            STORAGE::STG_S_MULTIPLEOPENS => "Multiple opens prevent consolidated. (commit succeeded).",
            STORAGE::STG_S_CONSOLIDATIONFAILED => "Consolidation of the storage file failed. (commit succeeded).",
            STORAGE::STG_S_CANNOTCONSOLIDATE => "Consolidation of the storage file is inappropriate. (commit succeeded).",
            STORAGE::STG_S_POWER_CYCLE_REQUIRED => "The device needs to be power cycled. (commit succeeded).",
            STORAGE::STG_E_INVALIDFUNCTION => "Unable to perform requested operation.",
            STORAGE::STG_E_FILENOTFOUND => "%1 could not be found.",
            STORAGE::STG_E_PATHNOTFOUND => "The path %1 could not be found.",
            STORAGE::STG_E_TOOMANYOPENFILES => "There are insufficient resources to open another file.",
            STORAGE::STG_E_ACCESSDENIED => "Access Denied.",
            STORAGE::STG_E_INVALIDHANDLE => "Attempted an operation on an invalid object.",
            STORAGE::STG_E_INSUFFICIENTMEMORY => "There is insufficient memory available to complete operation.",
            STORAGE::STG_E_INVALIDPOINTER => "Invalid pointer error.",
            STORAGE::STG_E_NOMOREFILES => "There are no more entries to return.",
            STORAGE::STG_E_DISKISWRITEPROTECTED => "Disk is write-protected.",
            STORAGE::STG_E_SEEKERROR => "An error occurred during a seek operation.",
            STORAGE::STG_E_WRITEFAULT => "A disk error occurred during a write operation.",
            STORAGE::STG_E_READFAULT => "A disk error occurred during a read operation.",
            STORAGE::STG_E_SHAREVIOLATION => "A share violation has occurred.",
            STORAGE::STG_E_LOCKVIOLATION => "A lock violation has occurred.",
            STORAGE::STG_E_FILEALREADYEXISTS => "%1 already exists.",
            STORAGE::STG_E_INVALIDPARAMETER => "Invalid parameter error.",
            STORAGE::STG_E_MEDIUMFULL => "There is insufficient disk space to complete operation.",
            STORAGE::STG_E_PROPSETMISMATCHED => "Illegal write of non-simple property to simple property set.",
            STORAGE::STG_E_ABNORMALAPIEXIT => "An API call exited abnormally.",
            STORAGE::STG_E_INVALIDHEADER => "The file %1 is not a valid compound file.",
            STORAGE::STG_E_INVALIDNAME => "The name %1 is not valid.",
            STORAGE::STG_E_UNKNOWN => "An unexpected error occurred.",
            STORAGE::STG_E_UNIMPLEMENTEDFUNCTION => "That function is not implemented.",
            STORAGE::STG_E_INVALIDFLAG => "Invalid flag error.",
            STORAGE::STG_E_INUSE => "Attempted to use an object that is busy.",
            STORAGE::STG_E_NOTCURRENT => "The storage has been changed since the last commit.",
            STORAGE::STG_E_REVERTED => "Attempted to use an object that has ceased to exist.",
            STORAGE::STG_E_CANTSAVE => "Can't save.",
            STORAGE::STG_E_OLDFORMAT => "The compound file %1 was produced with an incompatible version of storage.",
            STORAGE::STG_E_OLDDLL => "The compound file %1 was produced with a newer version of storage.",
            STORAGE::STG_E_SHAREREQUIRED => "Share.exe or equivalent is required for operation.",
            STORAGE::STG_E_NOTFILEBASEDSTORAGE => "Illegal operation called on non-file based storage.",
            STORAGE::STG_E_EXTANTMARSHALLINGS => "Illegal operation called on object with extant marshallings.",
            STORAGE::STG_E_DOCFILECORRUPT => "The docfile has been corrupted.",
            STORAGE::STG_E_BADBASEADDRESS => "OLE32.DLL has been loaded at the wrong address.",
            STORAGE::STG_E_DOCFILETOOLARGE => "The compound file is too large for the current implementation",
            STORAGE::STG_E_NOTSIMPLEFORMAT => "The compound file was not created with the STGM_SIMPLE flag",
            STORAGE::STG_E_INCOMPLETE => "The file download was aborted abnormally. The file is incomplete.",
            STORAGE::STG_E_TERMINATED => "The file download has been terminated.",
            STORAGE::STG_E_FIRMWARE_SLOT_INVALID => "The specified firmware slot is invalid.",
            STORAGE::STG_E_FIRMWARE_IMAGE_INVALID => "The specified firmware image is invalid.",
            STORAGE::STG_E_DEVICE_UNRESPONSIVE => "The storage device is unresponsive.",
            STORAGE::STG_E_STATUS_COPY_PROTECTION_FAILURE => "Generic Copy Protection Error.",
            STORAGE::STG_E_CSS_AUTHENTICATION_FAILURE => "Copy Protection Error - DVD CSS Authentication failed.",
            STORAGE::STG_E_CSS_KEY_NOT_PRESENT => "Copy Protection Error - The given sector does not have a valid CSS key.",
            STORAGE::STG_E_CSS_KEY_NOT_ESTABLISHED => "Copy Protection Error - DVD session key not established.",
            STORAGE::STG_E_CSS_SCRAMBLED_SECTOR => "Copy Protection Error - The read failed because the sector is encrypted.",
            STORAGE::STG_E_CSS_REGION_MISMATCH => "Copy Protection Error - The current DVD's region does not correspond to the region setting of the drive.",
            STORAGE::STG_E_RESETS_EXHAUSTED => "Copy Protection Error - The drive's region setting may be permanent or the number of user resets has been exhausted.",
        }
    }

    pub fn from_name(name: &str) -> STORAGE {
        return match name {
            "STG_S_CONVERTED" => STORAGE::STG_S_CONVERTED,
            "STG_S_BLOCK" => STORAGE::STG_S_BLOCK,
            "STG_S_RETRYNOW" => STORAGE::STG_S_RETRYNOW,
            "STG_S_MONITORING" => STORAGE::STG_S_MONITORING,
            "STG_S_MULTIPLEOPENS" => STORAGE::STG_S_MULTIPLEOPENS,
            "STG_S_CONSOLIDATIONFAILED" => STORAGE::STG_S_CONSOLIDATIONFAILED,
            "STG_S_CANNOTCONSOLIDATE" => STORAGE::STG_S_CANNOTCONSOLIDATE,
            "STG_S_POWER_CYCLE_REQUIRED" => STORAGE::STG_S_POWER_CYCLE_REQUIRED,
            "STG_E_INVALIDFUNCTION" => STORAGE::STG_E_INVALIDFUNCTION,
            "STG_E_FILENOTFOUND" => STORAGE::STG_E_FILENOTFOUND,
            "STG_E_PATHNOTFOUND" => STORAGE::STG_E_PATHNOTFOUND,
            "STG_E_TOOMANYOPENFILES" => STORAGE::STG_E_TOOMANYOPENFILES,
            "STG_E_ACCESSDENIED" => STORAGE::STG_E_ACCESSDENIED,
            "STG_E_INVALIDHANDLE" => STORAGE::STG_E_INVALIDHANDLE,
            "STG_E_INSUFFICIENTMEMORY" => STORAGE::STG_E_INSUFFICIENTMEMORY,
            "STG_E_INVALIDPOINTER" => STORAGE::STG_E_INVALIDPOINTER,
            "STG_E_NOMOREFILES" => STORAGE::STG_E_NOMOREFILES,
            "STG_E_DISKISWRITEPROTECTED" => STORAGE::STG_E_DISKISWRITEPROTECTED,
            "STG_E_SEEKERROR" => STORAGE::STG_E_SEEKERROR,
            "STG_E_WRITEFAULT" => STORAGE::STG_E_WRITEFAULT,
            "STG_E_READFAULT" => STORAGE::STG_E_READFAULT,
            "STG_E_SHAREVIOLATION" => STORAGE::STG_E_SHAREVIOLATION,
            "STG_E_LOCKVIOLATION" => STORAGE::STG_E_LOCKVIOLATION,
            "STG_E_FILEALREADYEXISTS" => STORAGE::STG_E_FILEALREADYEXISTS,
            "STG_E_INVALIDPARAMETER" => STORAGE::STG_E_INVALIDPARAMETER,
            "STG_E_MEDIUMFULL" => STORAGE::STG_E_MEDIUMFULL,
            "STG_E_PROPSETMISMATCHED" => STORAGE::STG_E_PROPSETMISMATCHED,
            "STG_E_ABNORMALAPIEXIT" => STORAGE::STG_E_ABNORMALAPIEXIT,
            "STG_E_INVALIDHEADER" => STORAGE::STG_E_INVALIDHEADER,
            "STG_E_INVALIDNAME" => STORAGE::STG_E_INVALIDNAME,
            "STG_E_UNKNOWN" => STORAGE::STG_E_UNKNOWN,
            "STG_E_UNIMPLEMENTEDFUNCTION" => STORAGE::STG_E_UNIMPLEMENTEDFUNCTION,
            "STG_E_INVALIDFLAG" => STORAGE::STG_E_INVALIDFLAG,
            "STG_E_INUSE" => STORAGE::STG_E_INUSE,
            "STG_E_NOTCURRENT" => STORAGE::STG_E_NOTCURRENT,
            "STG_E_REVERTED" => STORAGE::STG_E_REVERTED,
            "STG_E_CANTSAVE" => STORAGE::STG_E_CANTSAVE,
            "STG_E_OLDFORMAT" => STORAGE::STG_E_OLDFORMAT,
            "STG_E_OLDDLL" => STORAGE::STG_E_OLDDLL,
            "STG_E_SHAREREQUIRED" => STORAGE::STG_E_SHAREREQUIRED,
            "STG_E_NOTFILEBASEDSTORAGE" => STORAGE::STG_E_NOTFILEBASEDSTORAGE,
            "STG_E_EXTANTMARSHALLINGS" => STORAGE::STG_E_EXTANTMARSHALLINGS,
            "STG_E_DOCFILECORRUPT" => STORAGE::STG_E_DOCFILECORRUPT,
            "STG_E_BADBASEADDRESS" => STORAGE::STG_E_BADBASEADDRESS,
            "STG_E_DOCFILETOOLARGE" => STORAGE::STG_E_DOCFILETOOLARGE,
            "STG_E_NOTSIMPLEFORMAT" => STORAGE::STG_E_NOTSIMPLEFORMAT,
            "STG_E_INCOMPLETE" => STORAGE::STG_E_INCOMPLETE,
            "STG_E_TERMINATED" => STORAGE::STG_E_TERMINATED,
            "STG_E_FIRMWARE_SLOT_INVALID" => STORAGE::STG_E_FIRMWARE_SLOT_INVALID,
            "STG_E_FIRMWARE_IMAGE_INVALID" => STORAGE::STG_E_FIRMWARE_IMAGE_INVALID,
            "STG_E_DEVICE_UNRESPONSIVE" => STORAGE::STG_E_DEVICE_UNRESPONSIVE,
            "STG_E_STATUS_COPY_PROTECTION_FAILURE" => STORAGE::STG_E_STATUS_COPY_PROTECTION_FAILURE,
            "STG_E_CSS_AUTHENTICATION_FAILURE" => STORAGE::STG_E_CSS_AUTHENTICATION_FAILURE,
            "STG_E_CSS_KEY_NOT_PRESENT" => STORAGE::STG_E_CSS_KEY_NOT_PRESENT,
            "STG_E_CSS_KEY_NOT_ESTABLISHED" => STORAGE::STG_E_CSS_KEY_NOT_ESTABLISHED,
            "STG_E_CSS_SCRAMBLED_SECTOR" => STORAGE::STG_E_CSS_SCRAMBLED_SECTOR,
            "STG_E_CSS_REGION_MISMATCH" => STORAGE::STG_E_CSS_REGION_MISMATCH,
            "STG_E_RESETS_EXHAUSTED" => STORAGE::STG_E_RESETS_EXHAUSTED,
        }
    }
    pub fn from_code(code: u32) -> STORAGE {
        return match code {
            0x00030200 => STORAGE::STG_S_CONVERTED,
            0x00030201 => STORAGE::STG_S_BLOCK,
            0x00030202 => STORAGE::STG_S_RETRYNOW,
            0x00030203 => STORAGE::STG_S_MONITORING,
            0x00030204 => STORAGE::STG_S_MULTIPLEOPENS,
            0x00030205 => STORAGE::STG_S_CONSOLIDATIONFAILED,
            0x00030206 => STORAGE::STG_S_CANNOTCONSOLIDATE,
            0x00030207 => STORAGE::STG_S_POWER_CYCLE_REQUIRED,
            0x80030001 => STORAGE::STG_E_INVALIDFUNCTION,
            0x80030002 => STORAGE::STG_E_FILENOTFOUND,
            0x80030003 => STORAGE::STG_E_PATHNOTFOUND,
            0x80030004 => STORAGE::STG_E_TOOMANYOPENFILES,
            0x80030005 => STORAGE::STG_E_ACCESSDENIED,
            0x80030006 => STORAGE::STG_E_INVALIDHANDLE,
            0x80030008 => STORAGE::STG_E_INSUFFICIENTMEMORY,
            0x80030009 => STORAGE::STG_E_INVALIDPOINTER,
            0x80030012 => STORAGE::STG_E_NOMOREFILES,
            0x80030013 => STORAGE::STG_E_DISKISWRITEPROTECTED,
            0x80030019 => STORAGE::STG_E_SEEKERROR,
            0x8003001D => STORAGE::STG_E_WRITEFAULT,
            0x8003001E => STORAGE::STG_E_READFAULT,
            0x80030020 => STORAGE::STG_E_SHAREVIOLATION,
            0x80030021 => STORAGE::STG_E_LOCKVIOLATION,
            0x80030050 => STORAGE::STG_E_FILEALREADYEXISTS,
            0x80030057 => STORAGE::STG_E_INVALIDPARAMETER,
            0x80030070 => STORAGE::STG_E_MEDIUMFULL,
            0x800300F0 => STORAGE::STG_E_PROPSETMISMATCHED,
            0x800300FA => STORAGE::STG_E_ABNORMALAPIEXIT,
            0x800300FB => STORAGE::STG_E_INVALIDHEADER,
            0x800300FC => STORAGE::STG_E_INVALIDNAME,
            0x800300FD => STORAGE::STG_E_UNKNOWN,
            0x800300FE => STORAGE::STG_E_UNIMPLEMENTEDFUNCTION,
            0x800300FF => STORAGE::STG_E_INVALIDFLAG,
            0x80030100 => STORAGE::STG_E_INUSE,
            0x80030101 => STORAGE::STG_E_NOTCURRENT,
            0x80030102 => STORAGE::STG_E_REVERTED,
            0x80030103 => STORAGE::STG_E_CANTSAVE,
            0x80030104 => STORAGE::STG_E_OLDFORMAT,
            0x80030105 => STORAGE::STG_E_OLDDLL,
            0x80030106 => STORAGE::STG_E_SHAREREQUIRED,
            0x80030107 => STORAGE::STG_E_NOTFILEBASEDSTORAGE,
            0x80030108 => STORAGE::STG_E_EXTANTMARSHALLINGS,
            0x80030109 => STORAGE::STG_E_DOCFILECORRUPT,
            0x80030110 => STORAGE::STG_E_BADBASEADDRESS,
            0x80030111 => STORAGE::STG_E_DOCFILETOOLARGE,
            0x80030112 => STORAGE::STG_E_NOTSIMPLEFORMAT,
            0x80030201 => STORAGE::STG_E_INCOMPLETE,
            0x80030202 => STORAGE::STG_E_TERMINATED,
            0x80030208 => STORAGE::STG_E_FIRMWARE_SLOT_INVALID,
            0x80030209 => STORAGE::STG_E_FIRMWARE_IMAGE_INVALID,
            0x8003020A => STORAGE::STG_E_DEVICE_UNRESPONSIVE,
            0x80030305 => STORAGE::STG_E_STATUS_COPY_PROTECTION_FAILURE,
            0x80030306 => STORAGE::STG_E_CSS_AUTHENTICATION_FAILURE,
            0x80030307 => STORAGE::STG_E_CSS_KEY_NOT_PRESENT,
            0x80030308 => STORAGE::STG_E_CSS_KEY_NOT_ESTABLISHED,
            0x80030309 => STORAGE::STG_E_CSS_SCRAMBLED_SECTOR,
            0x8003030A => STORAGE::STG_E_CSS_REGION_MISMATCH,
            0x8003030B => STORAGE::STG_E_RESETS_EXHAUSTED,
        }
    }
}
