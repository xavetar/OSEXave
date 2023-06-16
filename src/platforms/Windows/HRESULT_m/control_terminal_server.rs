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

pub enum CONTROL_TERMINAL_SERVER {
    CTL_E_ILLEGALFUNCTIONCALL = 0x800A0005,
    CTL_E_OVERFLOW = 0x800A0006,
    CTL_E_OUTOFMEMORY = 0x800A0007,
    CTL_E_DIVISIONBYZERO = 0x800A000B,
    CTL_E_OUTOFSTRINGSPACE = 0x800A000E,
    CTL_E_OUTOFSTACKSPACE = 0x800A001C,
    CTL_E_BADFILENAMEORNUMBER = 0x800A0034,
    CTL_E_FILENOTFOUND = 0x800A0035,
    CTL_E_BADFILEMODE = 0x800A0036,
    CTL_E_FILEALREADYOPEN = 0x800A0037,
    CTL_E_DEVICEIOERROR = 0x800A0039,
    CTL_E_FILEALREADYEXISTS = 0x800A003A,
    CTL_E_BADRECORDLENGTH = 0x800A003B,
    CTL_E_DISKFULL = 0x800A003D,
    CTL_E_BADRECORDNUMBER = 0x800A003F,
    CTL_E_BADFILENAME = 0x800A0040,
    CTL_E_TOOMANYFILES = 0x800A0043,
    CTL_E_DEVICEUNAVAILABLE = 0x800A0044,
    CTL_E_PERMISSIONDENIED = 0x800A0046,
    CTL_E_DISKNOTREADY = 0x800A0047,
    CTL_E_PATHFILEACCESSERROR = 0x800A004B,
    CTL_E_PATHNOTFOUND = 0x800A004C,
    CTL_E_INVALIDPATTERNSTRING = 0x800A005D,
    CTL_E_INVALIDUSEOFNULL = 0x800A005E,
    CTL_E_INVALIDFILEFORMAT = 0x800A0141,
    CTL_E_INVALIDPROPERTYVALUE = 0x800A017C,
    CTL_E_INVALIDPROPERTYARRAYINDEX = 0x800A017D,
    CTL_E_SETNOTSUPPORTEDATRUNTIME = 0x800A017E,
    CTL_E_SETNOTSUPPORTED = 0x800A017F,
    CTL_E_NEEDPROPERTYARRAYINDEX = 0x800A0181,
    CTL_E_SETNOTPERMITTED = 0x800A0183,
    CTL_E_GETNOTSUPPORTEDATRUNTIME = 0x800A0189,
    CTL_E_GETNOTSUPPORTED = 0x800A018A,
    CTL_E_PROPERTYNOTFOUND = 0x800A01A6,
    CTL_E_INVALIDCLIPBOARDFORMAT = 0x800A01CC,
    CTL_E_INVALIDPICTURE = 0x800A01E1,
    CTL_E_PRINTERERROR = 0x800A01E2,
    CTL_E_CUSTOM_FIRST = 0x800A0258,
    CTL_E_CANTSAVEFILETOTEMP = 0x800A02DF,
    CTL_E_SEARCHTEXTNOTFOUND = 0x800A02E8,
    CTL_E_REPLACEMENTSTOOLONG = 0x800A02EA,
    adErrProviderFailed = 0x800A0BB8,
    adErrInvalidArgument = 0x800A0BB9,
    adErrOpeningFile = 0x800A0BBA,
    adErrReadFile = 0x800A0BBB,
    adErrWriteFile = 0x800A0BBC,
    adErrNoCurrentRecord = 0x800A0BCD,
    adErrIllegalOperation = 0x800A0C93,
    adErrCantChangeProvider = 0x800A0C94,
    adErrInTransaction = 0x800A0CAE,
    adErrFeatureNotAvailable = 0x800A0CB3,
    adErrItemNotFound = 0x800A0CC1,
    adErrObjectInCollection = 0x800A0D27,
    adErrObjectNotSet = 0x800A0D5C,
    adErrDataConversion = 0x800A0D5D,
    adErrObjectClosed = 0x800A0E78,
    adErrObjectOpen = 0x800A0E79,
    adErrProviderNotFound = 0x800A0E7A,
    adErrBoundToCommand = 0x800A0E7B,
    adErrInvalidParamInfo = 0x800A0E7C,
    adErrInvalidConnection = 0x800A0E7D,
    adErrNotReentrant = 0x800A0E7E,
    adErrStillExecuting = 0x800A0E7F,
    adErrOperationCancelled = 0x800A0E80,
    adErrStillConnecting = 0x800A0E81,
    adErrInvalidTransaction = 0x800A0E82,
    adErrNotExecuting = 0x800A0E83,
    adErrUnsafeOperation = 0x800A0E84,
    adWrnSecurityDialog = 0x800A0E85,
    adWrnSecurityDialogHeader = 0x800A0E86,
    adErrIntegrityViolation = 0x800A0E87,
    adErrPermissionDenied = 0x800A0E88,
    adErrDataOverflow = 0x800A0E89,
    adErrSchemaViolation = 0x800A0E8A,
    adErrSignMismatch = 0x800A0E8B,
    adErrCantConvertvalue = 0x800A0E8C,
    adErrCantCreate = 0x800A0E8D,
    adErrColumnNotOnThisRow = 0x800A0E8E,
    adErrURLDoesNotExist = 0x800A0E8F,
    adErrURLIntegrViolSetColumns = 0x800A0E8F,
    adErrTreePermissionDenied = 0x800A0E90,
    adErrInvalidURL = 0x800A0E91,
    adErrResourceLocked = 0x800A0E92,
    adErrResourceExists = 0x800A0E93,
    adErrCannotComplete = 0x800A0E94,
    adErrVolumeNotFound = 0x800A0E95,
    adErrOutOfSpace = 0x800A0E96,
    adErrResourceOutOfScope = 0x800A0E97,
    adErrUnavailable = 0x800A0E98,
    adErrURLNamedRowDoesNotExist = 0x800A0E99,
    adErrDelResOutOfScope = 0x800A0E9A,
    adErrPropInvalidColumn = 0x800A0E9B,
    adErrPropInvalidOption = 0x800A0E9C,
    adErrPropInvalidValue = 0x800A0E9D,
    adErrPropConflicting = 0x800A0E9E,
    adErrPropNotAllSettable = 0x800A0E9F,
    adErrPropNotSet = 0x800A0EA0,
    adErrPropNotSettable = 0x800A0EA1,
    adErrPropNotSupported = 0x800A0EA2,
    adErrCatalogNotSet = 0x800A0EA3,
    adErrCantChangeConnection = 0x800A0EA4,
    adErrFieldsUpdateFailed = 0x800A0EA5,
    adErrDenyNotSupported = 0x800A0EA6,
    adErrDenyTypeNotSupported = 0x800A0EA7,
    adErrProviderNotSpecified = 0x800A0EA9,
    adErrConnectionStringTooLong = 0x800A0EAA,
}

impl CONTROL_TERMINAL_SERVER {
    pub fn description(&self) -> &'static str {
        match self {
            CONTROL_TERMINAL_SERVER::CTL_E_ILLEGALFUNCTIONCALL => "DirectShow: The syntax in the function call was incorrect.",
            CONTROL_TERMINAL_SERVER::CTL_E_OVERFLOW => "DirectShow: The result is too large to be represented in function's return type.",
            CONTROL_TERMINAL_SERVER::CTL_E_OUTOFMEMORY => "DirectShow: There is insufficient memory to perform the requested operation.",
            CONTROL_TERMINAL_SERVER::CTL_E_DIVISIONBYZERO => "DirectShow: The function attempted to divide by zero.",
            CONTROL_TERMINAL_SERVER::CTL_E_OUTOFSTRINGSPACE => "DirectShow: There is insufficient string space to perform the requested operation. Certain operations (copying, for example) involve temporary strings that use up string space.",
            CONTROL_TERMINAL_SERVER::CTL_E_OUTOFSTACKSPACE => "DirectShow: There is insufficient string space to perform the requested operation. Each thread has its own stack while other resources, such as heap memory, are shared by all threads in the process. You must specify how much memory to allocate for a separate stack for each additional thread your program needs. The operating system will allocate additional stack space for the thread, if necessary, but you must specify a default value. If the operating system cannot allocate the necessary space, you receive this error.",
            CONTROL_TERMINAL_SERVER::CTL_E_BADFILENAMEORNUMBER => "DirectShow: The function attempted to access a file with a bad file name or number.",
            CONTROL_TERMINAL_SERVER::CTL_E_FILENOTFOUND => "DirectShow: The function attempted to access a file that could not be found.",
            CONTROL_TERMINAL_SERVER::CTL_E_BADFILEMODE => "DirectShow: The function attempted to perform an operation on a file that has an incompatible file mode. File modes are Append, Binary, Input, Output, or Random.",
            CONTROL_TERMINAL_SERVER::CTL_E_FILEALREADYOPEN => "DirectShow: The function attempted to open a file that was already open.",
            CONTROL_TERMINAL_SERVER::CTL_E_DEVICEIOERROR => "DirectShow: There was a device I/O error during the function execution.",
            CONTROL_TERMINAL_SERVER::CTL_E_FILEALREADYEXISTS => "DirectShow: The function attempted to create a file that already exists.",
            CONTROL_TERMINAL_SERVER::CTL_E_BADRECORDLENGTH => "DirectShow: The function attempted to access a file record that is incorrect. For example, it is assumed that a file opened for random access is composed of a set of identical-length records.",
            CONTROL_TERMINAL_SERVER::CTL_E_DISKFULL => "DirectShow: The function attempted to write to the disk, but the disk is full.",
            CONTROL_TERMINAL_SERVER::CTL_E_BADRECORDNUMBER => "DirectShow: The function attempted to access a file record that is incorrect. The record number is the position of the position in a recordset (a set of records in a data source).",
            CONTROL_TERMINAL_SERVER::CTL_E_BADFILENAME => "DirectShow: The function attempted to access a file name that is incorrect.",
            CONTROL_TERMINAL_SERVER::CTL_E_TOOMANYFILES => "DirectShow: The function attempted to open a file when too many files were already open.",
            CONTROL_TERMINAL_SERVER::CTL_E_DEVICEUNAVAILABLE => "DirectShow: The function attempted to access a device that was not available.",
            CONTROL_TERMINAL_SERVER::CTL_E_PERMISSIONDENIED => "DirectShow: The function attempted to access a file without the proper permission setting.",
            CONTROL_TERMINAL_SERVER::CTL_E_DISKNOTREADY => "DirectShow: The function attempted to access a file when the disk was not ready.",
            CONTROL_TERMINAL_SERVER::CTL_E_PATHFILEACCESSERROR => "DirectShow: The function attempted to access a file or path that was incorrect or had the wrong permissions.",
            CONTROL_TERMINAL_SERVER::CTL_E_PATHNOTFOUND => "DirectShow: The function attempted to access a file with an incorrect path.",
            CONTROL_TERMINAL_SERVER::CTL_E_INVALIDPATTERNSTRING => "DirectShow: The function was called with an invalid string.",
            CONTROL_TERMINAL_SERVER::CTL_E_INVALIDUSEOFNULL => "DirectShow: The function was called with an invalid NULL.",
            CONTROL_TERMINAL_SERVER::CTL_E_INVALIDFILEFORMAT => "DirectShow: The function attempted to perform an operation on a file that has an incompatible file format.",
            CONTROL_TERMINAL_SERVER::CTL_E_INVALIDPROPERTYVALUE => "DirectShow: The function was called with an invalid property value.",
            CONTROL_TERMINAL_SERVER::CTL_E_INVALIDPROPERTYARRAYINDEX => "DirectShow: The function was called with an invalid property array index.",
            CONTROL_TERMINAL_SERVER::CTL_E_SETNOTSUPPORTEDATRUNTIME => "DirectShow: The function attempted to set a property that cannot be set at run time.",
            CONTROL_TERMINAL_SERVER::CTL_E_SETNOTSUPPORTED => "DirectShow: The function attempted to set a property that cannot be set.",
            CONTROL_TERMINAL_SERVER::CTL_E_NEEDPROPERTYARRAYINDEX => "DirectShow: The function attempted to access a property that needs an array index without the array index.",
            CONTROL_TERMINAL_SERVER::CTL_E_SETNOTPERMITTED => "DirectShow: The function attempted to set a property without the proper permissions to set the property.",
            CONTROL_TERMINAL_SERVER::CTL_E_GETNOTSUPPORTEDATRUNTIME => "DirectShow: The function attempted to get a property that cannot be retrieved at run time.",
            CONTROL_TERMINAL_SERVER::CTL_E_GETNOTSUPPORTED => "DirectShow: The function attempted to get a property that cannot be retrieved.",
            CONTROL_TERMINAL_SERVER::CTL_E_PROPERTYNOTFOUND => "DirectShow: The function attempted to get a property that could not be found.",
            CONTROL_TERMINAL_SERVER::CTL_E_INVALIDCLIPBOARDFORMAT => "DirectShow: The function attempted an operation with an invalid Clipboard format.",
            CONTROL_TERMINAL_SERVER::CTL_E_INVALIDPICTURE => "DirectShow: The function attempted an operation with an invalid picture.",
            CONTROL_TERMINAL_SERVER::CTL_E_PRINTERERROR => "DirectShow: The function attempted to print, and there was a printer error.",
            CONTROL_TERMINAL_SERVER::CTL_E_CUSTOM_FIRST => "DirectShow: A custom error.",
            CONTROL_TERMINAL_SERVER::CTL_E_CANTSAVEFILETOTEMP => "DirectShow: The file cannot be saved to the TEMP directory.",
            CONTROL_TERMINAL_SERVER::CTL_E_SEARCHTEXTNOTFOUND => "DirectShow: The searched-for text cannot be found.",
            CONTROL_TERMINAL_SERVER::CTL_E_REPLACEMENTSTOOLONG => "DirectShow: The text replacement is too long.",
            CONTROL_TERMINAL_SERVER::adErrProviderFailed => "Error ADO: Provider failed to perform the requested operation.",
            CONTROL_TERMINAL_SERVER::adErrInvalidArgument => "Error ADO: Arguments are of the wrong type, are out of acceptable range, or are in conflict with one another.",
            CONTROL_TERMINAL_SERVER::adErrOpeningFile => "Error ADO: File could not be opened.",
            CONTROL_TERMINAL_SERVER::adErrReadFile => "Error ADO: File could not be read.",
            CONTROL_TERMINAL_SERVER::adErrWriteFile => "Error ADO: Write to file failed.",
            CONTROL_TERMINAL_SERVER::adErrNoCurrentRecord => "Error ADO: Either BOF or EOF is True, or the current record has been deleted. Requested operation requires a current record.",
            CONTROL_TERMINAL_SERVER::adErrIllegalOperation => "Error ADO: Operation is not allowed in this context.",
            CONTROL_TERMINAL_SERVER::adErrCantChangeProvider => "Error ADO: Supplied provider is different from the one already in use.",
            CONTROL_TERMINAL_SERVER::adErrInTransaction => "Error ADO: Connection object cannot be explicitly closed while in a transaction.",
            CONTROL_TERMINAL_SERVER::adErrFeatureNotAvailable => "Error ADO: Object or provider is not capable of performing requested operation.",
            CONTROL_TERMINAL_SERVER::adErrItemNotFound => "Error ADO: Item cannot be found in the collection corresponding to the requested name or ordinal.",
            CONTROL_TERMINAL_SERVER::adErrObjectInCollection => "Error ADO: Object is already in collection. Cannot append.",
            CONTROL_TERMINAL_SERVER::adErrObjectNotSet => "Error ADO: Object is no longer valid.",
            CONTROL_TERMINAL_SERVER::adErrDataConversion => "Error ADO: Application uses a value of the wrong type for the current operation.",
            CONTROL_TERMINAL_SERVER::adErrObjectClosed => "Error ADO: Operation is not allowed when the object is closed.",
            CONTROL_TERMINAL_SERVER::adErrObjectOpen => "Error ADO: Operation is not allowed when the object is open.",
            CONTROL_TERMINAL_SERVER::adErrProviderNotFound => "Error ADO: Provider cannot be found. It may not be properly installed.",
            CONTROL_TERMINAL_SERVER::adErrBoundToCommand => "Error ADO: Cannot change the ActiveConnection property of a Recordset object which has a Command object as its source.",
            CONTROL_TERMINAL_SERVER::adErrInvalidParamInfo => "Error ADO: Parameter object is improperly defined. Inconsistent or incomplete information was provided.",
            CONTROL_TERMINAL_SERVER::adErrInvalidConnection => "Error ADO: The connection cannot be used to perform this operation. It is either closed or invalid in this context.",
            CONTROL_TERMINAL_SERVER::adErrNotReentrant => "Error ADO: Operation cannot be performed while processing event.",
            CONTROL_TERMINAL_SERVER::adErrStillExecuting => "Error ADO: Operation cannot be performed while executing asynchronously.",
            CONTROL_TERMINAL_SERVER::adErrOperationCancelled => "Error ADO: Operation has been cancelled by the user.",
            CONTROL_TERMINAL_SERVER::adErrStillConnecting => "Error ADO: Operation cannot be performed while connecting aynchronously.",
            CONTROL_TERMINAL_SERVER::adErrInvalidTransaction => "Error ADO: Coordinating transaction is invalid or has not started.",
            CONTROL_TERMINAL_SERVER::adErrNotExecuting => "Error ADO: Operation cannot be performed while not executing.",
            CONTROL_TERMINAL_SERVER::adErrUnsafeOperation => "Error ADO: Safety settings on this computer prohibit accessing a data source on another domain.",
            CONTROL_TERMINAL_SERVER::adWrnSecurityDialog => "Error ADO: For internal use only. Don't use.",
            CONTROL_TERMINAL_SERVER::adWrnSecurityDialogHeader => "Error ADO: For internal use only. Don't use.",
            CONTROL_TERMINAL_SERVER::adErrIntegrityViolation => "Error ADO: Data value conflicts with the integrity constraints of the field.",
            CONTROL_TERMINAL_SERVER::adErrPermissionDenied => "Error ADO: Insufficent permission prevents writing to the field.",
            CONTROL_TERMINAL_SERVER::adErrDataOverflow => "Error ADO: Data value is too large to be represented by the field data type.",
            CONTROL_TERMINAL_SERVER::adErrSchemaViolation => "Error ADO: Data value conflicts with the data type or constraints of the field.",
            CONTROL_TERMINAL_SERVER::adErrSignMismatch => "Error ADO: Conversion failed because the data value was signed and the field data type used by the provider was unsigned.",
            CONTROL_TERMINAL_SERVER::adErrCantConvertvalue => "Error ADO: Data value cannot be converted for reasons other than sign mismatch or data overflow. For example, conversion would have truncated data.",
            CONTROL_TERMINAL_SERVER::adErrCantCreate => "Error ADO: Data value cannot be set or retrieved because the field data type was unknown, or the provider had insufficient resources to perform the operation.",
            CONTROL_TERMINAL_SERVER::adErrColumnNotOnThisRow => "Error ADO: Record does not contain this field.",
            CONTROL_TERMINAL_SERVER::adErrURLDoesNotExist => "Error ADO: Either the source URL or the parent of the destination URL does not exist.",
            CONTROL_TERMINAL_SERVER::adErrURLIntegrViolSetColumns => "Error ADO: Either the source URL or the parent of the destination URL violates the column integrity.",
            CONTROL_TERMINAL_SERVER::adErrTreePermissionDenied => "Error ADO: Permissions are insufficient to access tree or subtree.",
            CONTROL_TERMINAL_SERVER::adErrInvalidURL => "Error ADO: URL contains invalid characters. Make sure the URL is typed correctly.",
            CONTROL_TERMINAL_SERVER::adErrResourceLocked => "Error ADO: Object represented by the specified URL is locked by one or more other processes. Wait until the process has finished and attempt the operation again.",
            CONTROL_TERMINAL_SERVER::adErrResourceExists => "Error ADO: Copy operation cannot be performed. Object named by destination URL already exists. Specify adCopyOverwrite to replace the object.",
            CONTROL_TERMINAL_SERVER::adErrCannotComplete => "Error ADO: Server cannot complete the operation.",
            CONTROL_TERMINAL_SERVER::adErrVolumeNotFound => "Error ADO: Provider cannot locate the storage device indicated by the URL. Make sure the URL is typed correctly.",
            CONTROL_TERMINAL_SERVER::adErrOutOfSpace => "Error ADO: Operation cannot be performed. Provider cannot obtain enough storage space.",
            CONTROL_TERMINAL_SERVER::adErrResourceOutOfScope => "Error ADO: Source or destination URL is outside the scope of the current record.",
            CONTROL_TERMINAL_SERVER::adErrUnavailable => "Error ADO: Operation failed to complete and the status is unavailable. The field may be unavailable or the operation was not attempted.",
            CONTROL_TERMINAL_SERVER::adErrURLNamedRowDoesNotExist => "Error ADO: Record named by this URL does not exist.",
            CONTROL_TERMINAL_SERVER::adErrDelResOutOfScope => "Error ADO: URL of the object to be deleted is outside the scope of the current record.",
            CONTROL_TERMINAL_SERVER::adErrPropInvalidColumn => "Error ADO: Property cannot apply to the specified field.",
            CONTROL_TERMINAL_SERVER::adErrPropInvalidOption => "Error ADO: Property attribute is invalid.",
            CONTROL_TERMINAL_SERVER::adErrPropInvalidValue => "Error ADO: Property value is invalid. Make sure the value is typed correctly.",
            CONTROL_TERMINAL_SERVER::adErrPropConflicting => "Error ADO: Property value conflicts with a related property.",
            CONTROL_TERMINAL_SERVER::adErrPropNotAllSettable => "Error ADO: Property is read-only or cannot be set.",
            CONTROL_TERMINAL_SERVER::adErrPropNotSet => "Error ADO: Optional property value was not set.",
            CONTROL_TERMINAL_SERVER::adErrPropNotSettable => "Error ADO: Read-only property value was not set.",
            CONTROL_TERMINAL_SERVER::adErrPropNotSupported => "Error ADO: Provider does not support the property.",
            CONTROL_TERMINAL_SERVER::adErrCatalogNotSet => "Error ADO: Operation requires a valid ParentCatalog.",
            CONTROL_TERMINAL_SERVER::adErrCantChangeConnection => "Error ADO: Connection was denied. New connection you requested has different characteristics than the one already in use.",
            CONTROL_TERMINAL_SERVER::adErrFieldsUpdateFailed => "Error ADO: Fields update failed. For further information, examine the Status property of individual field objects.",
            CONTROL_TERMINAL_SERVER::adErrDenyNotSupported => "Error ADO: Provider does not support sharing restrictions.",
            CONTROL_TERMINAL_SERVER::adErrDenyTypeNotSupported => "Error ADO: Provider does not support the requested kind of sharing restriction.",
            CONTROL_TERMINAL_SERVER::adErrProviderNotSpecified => "Error ADO: Provider is not specified",
            CONTROL_TERMINAL_SERVER::adErrConnectionStringTooLong => "Error ADO: Connection string is too long",
        }
    }
}