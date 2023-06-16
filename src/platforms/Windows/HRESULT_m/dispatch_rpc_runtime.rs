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

pub enum DISPATCH_RPC_RUNTIME {
    DISP_E_UNKNOWNINTERFACE = 0x80020001,
    DISP_E_MEMBERNOTFOUND = 0x80020003,
    DISP_E_PARAMNOTFOUND = 0x80020004,
    DISP_E_TYPEMISMATCH = 0x80020005,
    DISP_E_UNKNOWNNAME = 0x80020006,
    DISP_E_NONAMEDARGS = 0x80020007,
    DISP_E_BADVARTYPE = 0x80020008,
    DISP_E_EXCEPTION = 0x80020009,
    DISP_E_OVERFLOW = 0x8002000A,
    DISP_E_BADINDEX = 0x8002000B,
    DISP_E_UNKNOWNLCID = 0x8002000C,
    DISP_E_ARRAYISLOCKED = 0x8002000D,
    DISP_E_BADPARAMCOUNT = 0x8002000E,
    DISP_E_PARAMNOTOPTIONAL = 0x8002000F,
    DISP_E_BADCALLEE = 0x80020010,
    DISP_E_NOTACOLLECTION = 0x80020011,
    DISP_E_DIVBYZERO = 0x80020012,
    DISP_E_BUFFERTOOSMALL = 0x80020013,
    TYPE_E_BUFFERTOOSMALL = 0x80028016,
    TYPE_E_FIELDNOTFOUND = 0x80028017,
    TYPE_E_INVDATAREAD = 0x80028018,
    TYPE_E_UNSUPFORMAT = 0x80028019,
    TYPE_E_REGISTRYACCESS = 0x8002801C,
    TYPE_E_LIBNOTREGISTERED = 0x8002801D,
    TYPE_E_UNDEFINEDTYPE = 0x80028027,
    TYPE_E_QUALIFIEDNAMEDISALLOWED = 0x80028028,
    TYPE_E_INVALIDSTATE = 0x80028029,
    TYPE_E_WRONGTYPEKIND = 0x8002802A,
    TYPE_E_ELEMENTNOTFOUND = 0x8002802B,
    TYPE_E_AMBIGUOUSNAME = 0x8002802C,
    TYPE_E_NAMECONFLICT = 0x8002802D,
    TYPE_E_UNKNOWNLCID = 0x8002802E,
    TYPE_E_DLLFUNCTIONNOTFOUND = 0x8002802F,
    TYPE_E_BADMODULEKIND = 0x800288BD,
    TYPE_E_SIZETOOBIG = 0x800288C5,
    TYPE_E_DUPLICATEID = 0x800288C6,
    TYPE_E_INVALIDID = 0x800288CF,
    TYPE_E_TYPEMISMATCH = 0x80028CA0,
    TYPE_E_OUTOFBOUNDS = 0x80028CA1,
    TYPE_E_IOERROR = 0x80028CA2,
    TYPE_E_CANTCREATETMPFILE = 0x80028CA3,
    TYPE_E_CANTLOADLIBRARY = 0x80029C4A,
    TYPE_E_INCONSISTENTPROPFUNCS = 0x80029C83,
    TYPE_E_CIRCULARTYPE = 0x80029C84,
}

impl DISPATCH_RPC_RUNTIME {
    pub fn description(&self) -> &'static str {
        match self {
            DISPATCH_RPC_RUNTIME::DISP_E_UNKNOWNINTERFACE => "Unknown interface.",
            DISPATCH_RPC_RUNTIME::DISP_E_MEMBERNOTFOUND => "Member not found.",
            DISPATCH_RPC_RUNTIME::DISP_E_PARAMNOTFOUND => "Parameter not found.",
            DISPATCH_RPC_RUNTIME::DISP_E_TYPEMISMATCH => "Type mismatch.",
            DISPATCH_RPC_RUNTIME::DISP_E_UNKNOWNNAME => "Unknown name.",
            DISPATCH_RPC_RUNTIME::DISP_E_NONAMEDARGS => "No named arguments.",
            DISPATCH_RPC_RUNTIME::DISP_E_BADVARTYPE => "Bad variable type.",
            DISPATCH_RPC_RUNTIME::DISP_E_EXCEPTION => "Exception occurred.",
            DISPATCH_RPC_RUNTIME::DISP_E_OVERFLOW => "Out of present range.",
            DISPATCH_RPC_RUNTIME::DISP_E_BADINDEX => "Invalid index.",
            DISPATCH_RPC_RUNTIME::DISP_E_UNKNOWNLCID => "Unknown language.",
            DISPATCH_RPC_RUNTIME::DISP_E_ARRAYISLOCKED => "Memory is locked.",
            DISPATCH_RPC_RUNTIME::DISP_E_BADPARAMCOUNT => "Invalid number of parameters.",
            DISPATCH_RPC_RUNTIME::DISP_E_PARAMNOTOPTIONAL => "Parameter not optional.",
            DISPATCH_RPC_RUNTIME::DISP_E_BADCALLEE => "Invalid callee.",
            DISPATCH_RPC_RUNTIME::DISP_E_NOTACOLLECTION => "Does not support a collection.",
            DISPATCH_RPC_RUNTIME::DISP_E_DIVBYZERO => "Division by zero.",
            DISPATCH_RPC_RUNTIME::DISP_E_BUFFERTOOSMALL => "Buffer too small",
            DISPATCH_RPC_RUNTIME::TYPE_E_BUFFERTOOSMALL => "Buffer too small.",
            DISPATCH_RPC_RUNTIME::TYPE_E_FIELDNOTFOUND => "Field name not defined in the record.",
            DISPATCH_RPC_RUNTIME::TYPE_E_INVDATAREAD => "Old format or invalid type library.",
            DISPATCH_RPC_RUNTIME::TYPE_E_UNSUPFORMAT => "Old format or invalid type library.",
            DISPATCH_RPC_RUNTIME::TYPE_E_REGISTRYACCESS => "Error accessing the OLE registry.",
            DISPATCH_RPC_RUNTIME::TYPE_E_LIBNOTREGISTERED => "Library not registered.",
            DISPATCH_RPC_RUNTIME::TYPE_E_UNDEFINEDTYPE => "Bound to unknown type.",
            DISPATCH_RPC_RUNTIME::TYPE_E_QUALIFIEDNAMEDISALLOWED => "Qualified name disallowed.",
            DISPATCH_RPC_RUNTIME::TYPE_E_INVALIDSTATE => "Invalid forward reference, or reference to uncompiled type.",
            DISPATCH_RPC_RUNTIME::TYPE_E_WRONGTYPEKIND => "Type mismatch.",
            DISPATCH_RPC_RUNTIME::TYPE_E_ELEMENTNOTFOUND => "Element not found.",
            DISPATCH_RPC_RUNTIME::TYPE_E_AMBIGUOUSNAME => "Ambiguous name.",
            DISPATCH_RPC_RUNTIME::TYPE_E_NAMECONFLICT => "Name already exists in the library.",
            DISPATCH_RPC_RUNTIME::TYPE_E_UNKNOWNLCID => "Unknown LCID.",
            DISPATCH_RPC_RUNTIME::TYPE_E_DLLFUNCTIONNOTFOUND => "Function not defined in specified DLL.",
            DISPATCH_RPC_RUNTIME::TYPE_E_BADMODULEKIND => "Wrong module kind for the operation.",
            DISPATCH_RPC_RUNTIME::TYPE_E_SIZETOOBIG => "Size may not exceed 64K.",
            DISPATCH_RPC_RUNTIME::TYPE_E_DUPLICATEID => "Duplicate ID in inheritance hierarchy.",
            DISPATCH_RPC_RUNTIME::TYPE_E_INVALIDID => "Incorrect inheritance depth in standard OLE hmember.",
            DISPATCH_RPC_RUNTIME::TYPE_E_TYPEMISMATCH => "Type mismatch.",
            DISPATCH_RPC_RUNTIME::TYPE_E_OUTOFBOUNDS => "Invalid number of arguments.",
            DISPATCH_RPC_RUNTIME::TYPE_E_IOERROR => "I/O Error.",
            DISPATCH_RPC_RUNTIME::TYPE_E_CANTCREATETMPFILE => "Error creating unique tmp file.",
            DISPATCH_RPC_RUNTIME::TYPE_E_CANTLOADLIBRARY => "Error loading type library/DLL.",
            DISPATCH_RPC_RUNTIME::TYPE_E_INCONSISTENTPROPFUNCS => "Inconsistent property functions.",
            DISPATCH_RPC_RUNTIME::TYPE_E_CIRCULARTYPE => "Circular dependency between types/modules.",
        }
    }
}
